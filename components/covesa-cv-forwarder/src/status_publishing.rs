// SPDX-FileCopyrightText: 2023, 2024 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use crate::vehicle_abstraction::vss;
use crate::ChosenSignals;
use clap::ArgMatches;
use influxrs::InfluxClient;
use influxrs::Measurement;
use tokio::time::Duration;

pub const PARAM_INFLUXDB_BUCKET: &str = "influxdb-bucket";
pub const PARAM_INFLUXDB_ORG: &str = "influxdb-org";
pub const PARAM_INFLUXDB_URI: &str = "influxdb-uri";
pub const PARAM_INFLUXDB_TOKEN: &str = "influxdb-token";
pub const PARAM_INFLUXDB_TOKEN_FILE: &str = "influxdb-token-file";

/// A connection to an InfluxDB server.
pub struct CovesaInfluxConnection {
    pub client: InfluxClient,
    pub bucket: String,
}

impl CovesaInfluxConnection {
    /// Creates a new connection to an InfluxDB server.
    ///
    /// Determines the parameters necessary for creating the connection from values specified on
    /// the command line or via environment variables as defined by [`add_command_line_args`].
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use influx_client::connection::InfluxConnection;
    ///
    /// let command = influx_client::connection::add_command_line_args(Command::new("influx_client"));
    /// let matches = command.get_matches_from(vec![
    ///     "influx_client",
    ///     "--influxdb-uri", "http://my-influx.io",
    ///     "--influxdb-token", "some-token",
    ///     "--influxdb-bucket", "the-bucket",
    /// ]);
    /// let connection = InfluxConnection::new(&matches)?;
    /// assert_eq!(connection.bucket, "the-bucket".to_string());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(args: &ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {
        let influx_uri = args
            .get_one::<String>(crate::status_publishing::PARAM_INFLUXDB_URI)
            .unwrap()
            .to_owned();
        let influx_token =
            match args.get_one::<String>(crate::status_publishing::PARAM_INFLUXDB_TOKEN) {
                Some(token) => token.to_string(),
                None => {
                    let file_name = args
                        .get_one::<String>(crate::status_publishing::PARAM_INFLUXDB_TOKEN_FILE)
                        .unwrap();
                    match read_token_from_file(file_name) {
                        Ok(token) => token,
                        Err(e) => return Err(Box::new(e)),
                    }
                }
            };
        let influx_org = args
            .get_one::<String>(crate::status_publishing::PARAM_INFLUXDB_ORG)
            .unwrap()
            .to_owned();
        let influx_bucket = args
            .get_one::<String>(crate::status_publishing::PARAM_INFLUXDB_BUCKET)
            .unwrap()
            .to_owned();
        let client = InfluxClient::builder(influx_uri, influx_token, influx_org)
            .build()
            .unwrap();
        Ok(CovesaInfluxConnection {
            client,
            bucket: influx_bucket,
        })
    }
}

fn read_token_from_file(filename: &str) -> std::io::Result<String> {
    log::info!("reading token from file {filename}");
    std::fs::read_to_string(filename)
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            log::error!("failed to read token from file [{filename}]: {e}");
            e
        })
}

fn build_measurement(chosen_signals: &ChosenSignals) -> Option<Measurement> {
    log::info!("Building measurement...");
    let mut builder = Measurement::builder("Curvelogging");

    if chosen_signals.lat.is_some() && chosen_signals.lon.is_some() {
        builder = builder
            .field(
                vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE,
                chosen_signals.lat.unwrap(),
            )
            .field(
                vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE,
                chosen_signals.lon.unwrap(),
            );
    }
    if chosen_signals.speed.is_some() {
        builder = builder.field(
            "Vehicle.CurrentLocation.Speed",
            chosen_signals.speed.unwrap(),
        );
    }

    builder = builder.timestamp_ms(chosen_signals.time);

    match builder.build() {
        Ok(measurement) => Some(measurement),
        Err(e) => {
            log::debug!("failed to create curvelogging Measurement: {e}");
            None
        }
    }
}

/// A facade to an InfluxDB server for publishing Vehicle status information.
pub struct InfluxWriter {
    pub influx_con: CovesaInfluxConnection,
}

impl InfluxWriter {
    /// Creates a new writer.
    ///
    /// Determines the parameters necessary for creating the writer from values specified on
    /// the command line or via environment variables as defined by [`super::add_command_line_args`].
    pub fn new(args: &ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {
        CovesaInfluxConnection::new(args).map(|con| InfluxWriter { influx_con: con })
    }

    /// Writes Curvelogging information as measurements to the InfluxDB server.
    ///
    /// The measurements are being written to the *bucket* in the *organization* that have been
    /// configured via command line arguments and/or environment variables passed in to [`self::InfluxWriter::new()`].
    pub async fn write_chosen_signals(&self, chosen_signals: &ChosenSignals) {
        let mut measurements: Vec<Measurement> = Vec::new();
        if let Some(measurement) = build_measurement(chosen_signals) {
            println!(
                "writing snapshot measurement to influxdb: {:#?}\n\n",
                measurement
            );
            measurements.push(measurement);
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
        if !measurements.is_empty() {
            if let Err(e) = self
                .influx_con
                .client
                .write(
                    self.influx_con.bucket.as_str(),
                    measurements[..].try_into().unwrap(),
                )
                .await
            {
                println!("failed to write data to influx: {e}");
            } else {
                println!("successfully wrote data to influx");
            }
        }
    }
}

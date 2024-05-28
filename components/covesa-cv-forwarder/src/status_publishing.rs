// SPDX-FileCopyrightText: 2023 Contributors to the Eclipse Foundation
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

use crate::vehicle_abstraction::CovesaInfluxConnection;
use crate::ChosenSignals;

use clap::ArgMatches;
use influxrs::Measurement;
use crate::vehicle_abstraction::vss;

fn build_measurement(
    created_date_time: u128,
    chosen_signals: &ChosenSignals,
) -> Option<Measurement> {
    log::info!("Building measurement...");
    let mut builder = Measurement::builder("curvelogging")
        .field("createdDateTime", created_date_time);

    if chosen_signals.lat.is_some() && chosen_signals.lon.is_some() {
        builder = builder
            .field(vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE, chosen_signals.lat.unwrap())
            .field(vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE, chosen_signals.lon.unwrap());
    }
    if chosen_signals.speed.is_some() {
        builder = builder.field(vss::VSS_VEHICLE_SPEED, chosen_signals.speed.unwrap());
    }

    if chosen_signals.time.is_some() {
        builder = builder.field(vss::VSS_VEHICLE_CURRENTLOCATION_TIMESTAMP, chosen_signals.time.unwrap());
    } else {
        log::error!("ERROR: Each measurement must contain at least one signal positionDateTime");
    }

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
        let created_timestamp: u128 = chosen_signals.time.clone().unwrap() as u128;
        let mut measurements: Vec<Measurement> = Vec::new();
        if let Some(measurement) = build_measurement(
            created_timestamp,
            chosen_signals,
        ) {
            log::info!("writing snapshot measurement to influxdb: {:#?}\n\n", measurement);
            measurements.push(measurement);
        }

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
                log::info!("failed to write data to influx: {e}");
            }
        }
    }
}

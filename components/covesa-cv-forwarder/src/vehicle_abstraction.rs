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

//! An abstraction of a vehicle's (current) status based on
//! [Eclipse kuksa.val Databroker](https://github.com/eclipse/kuksa.val).
//!
use crate::curvelogging::CurveLogActorHandler;
use clap::{Arg, ArgMatches, Command};
pub mod kuksa;
use self::kuksa::DataEntry;
use kuksa::{val_client::ValClient, EntryRequest, Field, GetRequest, View};
use log::error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{error::Error, fmt::Display};
use tokio::time::Duration;
use tonic::{
    transport::{Channel, Endpoint},
    Request,
};

pub mod vss;

pub struct FetchedSignals {
    speed: Option<f32>,
    longitude: Option<f64>,
    latitude: Option<f64>,
}

pub enum Trigger {
    Timer,
}

pub const SLLT_VSS_PATHS: &[&str] = &[
    vss::VSS_VEHICLE_SPEED,
    vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE,
    vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE,
];

const PARAM_DATABROKER_URI: &str = "databroker-uri";
const COVESA_PARAM_TIMER_INTERVAL: &str = "timer-interval";

/// Sets up a connection to the Databroker and registers callbacks for
/// signals that trigger the reporting of the vehicle's current status.
///
/// Expects to find parameters as defined by [`add_command_line_args`] in the passed
/// in *args*.
///
pub async fn init(
    args: &ArgMatches,
    curve_log_handler: CurveLogActorHandler,
) -> Result<(), DatabrokerError> {
    let mut databroker = KuksaValDatabroker::new(args).await?;

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Trigger>(50);

    tokio::task::spawn(async move {
        while let Some(_trigger) = rx.recv().await {
            let signals = databroker.fetch_data().await.unwrap();

            curve_log_handler
                .send_signals(
                    signals.speed,
                    signals.longitude,
                    signals.latitude,
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                )
                .await;
        }
    });
    let timer_sender = tx.clone();
    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let _ = timer_sender.send(Trigger::Timer).await;
        }
    });
    Ok(())
}

/// Adds arguments to an existing command line which can be
/// used to configure the component's behavior.
///
/// The following arguments are being added:
///
/// | long name           | environment variable | default value | description |
/// |---------------------|----------------------|---------------|-------------|
/// | *databroker-uri*    | *KUKSA_DATA_BROKER_URI*| `http://127.0.0.1:55555` | The HTTP(S) URI of the kuksa.val Databroker's gRPC endpoint. |
/// | *timer-interval*    | *TIMER_INTERVAL*     | `5s`          | The time period to wait after polling FMS snapshot data from the kuksa.val Databroker, e.g 5m10s or 1h15m. |
///
pub fn add_command_line_args(command_line: Command) -> Command {
    command_line
        .arg(
            Arg::new(PARAM_DATABROKER_URI)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_DATABROKER_URI)
                .alias("uri")
                .help("The HTTP(S) URI of the kuksa.val Databroker's gRPC endpoint.")
                .value_name("URI")
                .required(false)
                .env("KUKSA_DATA_BROKER_URI")
                .default_value("http://127.0.0.1:55555"),
        )
        .arg(
            Arg::new(COVESA_PARAM_TIMER_INTERVAL)
                .value_parser(|s: &_| duration_str::parse(s))
                .long(COVESA_PARAM_TIMER_INTERVAL)
                .alias("timer")
                .help("The time period to wait after polling FMS snapshot data from the kuksa.val Databroker, e.g 5m10s or 1h15m.")
                .value_name("DURATION_SPEC")
                .required(false)
                .env("TIMER_INTERVAL")
                .default_value("1s"),
        )
        .arg(
            Arg::new(crate::curvelogging::PARAM_WINDOW_CAPACITY)
                .value_parser(clap::value_parser!(usize))
                .long(crate::curvelogging::PARAM_WINDOW_CAPACITY)
                .help("The capacity of the window for data processing.")
                .value_name("CAPACITY")
                .required(false)
                .env("WINDOW_CAPACITY")
                .default_value("25"),
        )
}

/// Indicates a problem while invoking a Databroker operation.
#[derive(Debug)]
pub struct DatabrokerError {
    description: String,
}

impl Error for DatabrokerError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl Display for DatabrokerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error invoking Databroker: {:?}", self.description)
    }
}

pub struct KuksaValDatabroker {
    client: Box<ValClient<Channel>>,
}

impl KuksaValDatabroker {
    pub async fn new(args: &ArgMatches) -> Result<Self, DatabrokerError> {
        let databroker_uri = args
            .get_one::<String>(PARAM_DATABROKER_URI)
            .unwrap()
            .to_owned();

        log::info!(
            "creating client for kuksa.val Databroker at {}",
            databroker_uri
        );
        Endpoint::from_shared(databroker_uri.to_owned())
            .map_err(|e| {
                error!("invalid Databroker URI: {}", e);
                DatabrokerError {
                    description: e.to_string(),
                }
            })
            .map(|builder| {
                let channel = builder
                    .connect_timeout(Duration::from_secs(5))
                    .timeout(Duration::from_secs(5))
                    .connect_lazy();
                let client = ValClient::new(channel);
                KuksaValDatabroker {
                    client: Box::new(client),
                }
            })
    }

    pub async fn fetch_data(&mut self) -> Result<FetchedSignals, DatabrokerError> {
        let mut speed: Option<f32> = None;
        let mut latitude: Option<f64> = None;
        let mut longitude: Option<f64> = None;
        let entry_requests: Vec<EntryRequest> = SLLT_VSS_PATHS
            .iter()
            .map(|path| EntryRequest {
                path: path.to_string(),
                view: View::CurrentValue as i32,
                fields: vec![Field::Value as i32],
            })
            .collect();

        let mut vss_data: Vec<DataEntry> = Vec::new();
        match self
            .client
            .get(Request::new(GetRequest {
                entries: entry_requests,
            }))
            .await
            .map(|res| res.into_inner())
        {
            Err(status) => {
                log::info!("failed to retrieve snapshot data points from Databroker {status}");
                Err(DatabrokerError {
                    description: format!("status code {}", status.code()),
                })
            }
            Ok(get_response) => {
                if let Some(error) = get_response.error {
                    log::info!(
                        "response from Databroker contains global error [code: {}, message: {}]",
                        error.code,
                        error.message
                    );
                } else {
                    get_response
                        .errors
                        .into_iter()
                        .for_each(|data_entry_error| {
                            if let Some(err) = data_entry_error.error {
                                log::info!(
                                    "response from Databroker contains error [path: {}, error: {:?}]",
                                    data_entry_error.path, err
                                );
                            }
                        });
                    get_response.entries.into_iter().for_each(|data_entry| {
                        vss_data.push(data_entry);
                    });
                }
                // loop trough dataentries to extract speed,lat,lon
                // reforfm into matching statement
                for entry in vss_data {
                    if entry.path == *vss::VSS_VEHICLE_SPEED {
                        if let Some(ref _value) = entry.value {
                            let speed_as_value = entry.value.and_then(|dp| dp.value).unwrap();
                            speed = Some(f64::try_from(speed_as_value).unwrap() as f32);
                        }
                    } else if entry.path == *vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE {
                        if let Some(ref _value) = entry.value {
                            let latitude_as_value = entry.value.and_then(|dp| dp.value).unwrap();
                            latitude = Some(f64::try_from(latitude_as_value).unwrap());
                        }
                    } else if entry.path == *vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE {
                        if let Some(ref _value) = entry.value {
                            let longitude_as_value = entry.value.and_then(|dp| dp.value).unwrap();
                            longitude = Some(f64::try_from(longitude_as_value).unwrap());
                        }
                    }
                }
                let signals = FetchedSignals {
                    speed,
                    longitude,
                    latitude,
                };
                Ok(signals)
            }
        }
    }
}

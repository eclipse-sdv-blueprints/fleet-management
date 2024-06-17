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
use clap::{Arg, ArgMatches, Command};
pub use kuksa::proto::v1::datapoint::Value::{Double, Float};
use kuksa::DataEntry;
use log::error;
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
// use databroker_proto::kuksa::val::v1::DataEntry;

use std::{error::Error, fmt::Display};

use crate::curvelogging::CurveLogActorHandler;

pub use kuksa::KuksaClient;
// {val_client::ValClient, EntryRequest, Field, GetRequest, View};

// use self::kuksa::{DataEntry, SubscribeEntry};
use tokio::time::Duration;

pub struct FetchedSignals {
    speed: Option<f32>,
    longitude: Option<f64>,
    latitude: Option<f64>,
}

pub enum Trigger {
    Timer,
}

// impl fmt::Display for SubscribeEntry {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         write!(f, "{}", self.path)
//     }
// }

pub const SLLT_VSS_PATHS: &[&str] = &[
    vss::VSS_VEHICLE_SPEED,
    vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE,
    vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE,
];

const PARAM_DATABROKER_URI: &str = "databroker-uri";
const COVESA_PARAM_TIMER_INTERVAL: &str = "timer-interval";

// pub mod kuksa;
pub mod vss;

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
    let databroker_uri = args
        .get_one::<String>(PARAM_DATABROKER_URI)
        .unwrap()
        .to_owned();
    let uri = tonic::transport::Uri::from_str(&databroker_uri).unwrap();

    let mut databroker = KuksaClient::new(uri);
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Trigger>(50);

    tokio::task::spawn(async move {
        while let Some(_trigger) = rx.recv().await {
            //get  DataEntry directly from databroker
            let paths = SLLT_VSS_PATHS
                .iter()
                .map(|path| path.to_string())
                .collect::<Vec<String>>();
            let vss_data = databroker.get_current_values(paths).await.unwrap();
            let signals = fetch_data(vss_data).await.unwrap();
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            curve_log_handler
                .send_signals(
                    signals.speed,
                    signals.longitude,
                    signals.latitude,
                    current_time,
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

pub async fn fetch_data(vss_data: Vec<DataEntry>) -> Result<FetchedSignals, DatabrokerError> {
    let mut speed: Option<f32> = None;
    let mut latitude: Option<f64> = None;
    let mut longitude: Option<f64> = None;

    // loop trough dataentries to extract speed,lat,lon
    for entry in vss_data {
        if entry.path == *vss::VSS_VEHICLE_SPEED {
            if let Some(ref _value) = entry.value {
                let speed_as_value = entry.value.and_then(|dp| dp.value);
                match speed_as_value {
                    Some(Float(value)) => {
                        speed = Some(value);
                    }
                    Some(Double(value)) => {
                        speed = Some(value as f32);
                    }
                    _ => {
                        error!("Invalid value type for speed");
                    }
                }
            }
        } else if entry.path == *vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE {
            if let Some(ref _value) = entry.value {
                let latitude_as_value = entry.value.and_then(|dp| dp.value);
                match latitude_as_value {
                    Some(Float(value)) => {
                        latitude = Some(value as f64);
                    }
                    Some(Double(value)) => {
                        latitude = Some(value);
                    }
                    _ => {
                        error!("Invalid value type for latitude");
                    }
                }
            }
        } else if entry.path == *vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE {
            if let Some(ref _value) = entry.value {
                let longitude_as_value = entry.value.and_then(|dp| dp.value);
                match longitude_as_value {
                    Some(Float(value)) => {
                        longitude = Some(value as f64);
                    }
                    Some(Double(value)) => {
                        longitude = Some(value);
                    }
                    _ => {
                        error!("Invalid value type for longitude");
                    }
                }
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

// pub struct KuksaValDatabroker {
//     client: Box<ValClient<Channel>>,
// }

// impl KuksaValDatabroker {
//     pub async fn new(args: &ArgMatches) -> Result<Self, DatabrokerError> {
//         let databroker_uri = args
//             .get_one::<String>(PARAM_DATABROKER_URI)
//             .unwrap()
//             .to_owned();

//         info!(
//             "creating client for kuksa.val Databroker at {}",
//             databroker_uri
//         );
//         Endpoint::from_shared(databroker_uri.to_owned())
//             .map_err(|e| {
//                 error!("invalid Databroker URI: {}", e);
//                 DatabrokerError {
//                     description: e.to_string(),
//                 }
//             })
//             .map(|builder| {
//                 let channel = builder
//                     .connect_timeout(Duration::from_secs(5))
//                     .timeout(Duration::from_secs(5))
//                     .connect_lazy();
//                 let client = ValClient::new(channel);
//                 KuksaValDatabroker {
//                     client: Box::new(client),
//                 }
//             })
//     }
// }

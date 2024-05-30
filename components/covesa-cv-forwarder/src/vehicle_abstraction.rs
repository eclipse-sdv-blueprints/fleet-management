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

//! An abstraction of a vehicle's (current) status based on
//! [Eclipse kuksa.val Databroker](https://github.com/eclipse/kuksa.val).
//!
use clap::{Arg, ArgMatches, Command};
use influxrs::InfluxClient;
use log::{error, info, warn};
use std::fmt;
use tokio::sync::mpsc::Sender;

use std::{collections::HashMap, error::Error, fmt::Display};

use crate::curvelogging::{ChosenSignals, CurveLogActorHandle};
use tonic::{
    transport::{Channel, Endpoint},
    Request,
};

pub use fms_proto::fms::{TellTaleInfo, Trigger};
use kuksa::{datapoint::Value, val_client::ValClient, EntryRequest, Field, GetRequest, View};

use self::kuksa::{DataEntry, SubscribeEntry, SubscribeRequest, UnsupportedValueTypeError};
use tokio::time::Duration;

impl fmt::Display for SubscribeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.path)
    }
}

const PARAM_INFLUXDB_BUCKET: &str = "influxdb-bucket";
const PARAM_INFLUXDB_ORG: &str = "influxdb-org";
const PARAM_INFLUXDB_URI: &str = "influxdb-uri";
const PARAM_INFLUXDB_TOKEN: &str = "influxdb-token";
const PARAM_INFLUXDB_TOKEN_FILE: &str = "influxdb-token-file";

pub const SLLT_VSS_PATHS: &[&str] = &[
    vss::VSS_VEHICLE_SPEED,
    vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE,
    vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE,
    vss::VSS_VEHICLE_CURRENTLOCATION_TIMESTAMP,
];

const _TRIGGER_VSS_PATHS: &[&str] = &[
    vss::FMS_VEHICLE_CABIN_TELLTALE_ECT_STATUS,
    vss::FMS_VEHICLE_CABIN_TELLTALE_ENGINEOIL_STATUS,
    vss::FMS_VEHICLE_CABIN_TELLTALE_ENGINE_STATUS,
    vss::FMS_VEHICLE_CABIN_TELLTALE_FUELLEVEL_STATUS,
    vss::FMS_VEHICLE_CABIN_TELLTALE_PARKINGBRAKE_STATUS,
    vss::VSS_VEHICLE_CHASSIS_PARKINGBRAKE_ISENGAGED,
    vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_ISRUNNING,
    vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_ISCARDPRESENT,
    vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_WORKINGSTATE,
    vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_ISCARDPRESENT,
    vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_WORKINGSTATE,
];

const PARAM_DATABROKER_URI: &str = "databroker-uri";
const COVESA_PARAM_TIMER_INTERVAL: &str = "timer-interval";
pub const PARAM_WINDOW_CAPACITY: &str = "window-capacity"; 

const TELL_TALE_NAME_ECT: &str = "ENGINE_COOLANT_TEMPERATURE";
const TELL_TALE_NAME_ENGINE_OIL: &str = "ENGINE_OIL";
const TELL_TALE_NAME_ENGINE_MIL_INDICATOR: &str = "ENGINE_MIL_INDICATOR";
const TELL_TALE_NAME_FUEL_LEVEL: &str = "FUEL_LEVEL";
const TELL_TALE_NAME_PARKING_BRAKE: &str = "PARKING_BRAKE";

pub mod kuksa;
pub mod vss;

/// Sets up a connection to the Databroker and registers callbacks for
/// signals that trigger the reporting of the vehicle's current status.
///
/// Expects to find parameters as defined by [`add_command_line_args`] in the passed
/// in *args*.
///
pub async fn init(
    args: &ArgMatches,
    publisher_channel: Sender<Vec<ChosenSignals>>,
    actor_handle: CurveLogActorHandle,
) -> Result<(), DatabrokerError> {
    let mut databroker = KuksaValDatabroker::new(args).await?;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<FmsTrigger>(50);
    let _ = &databroker.register_sllt_vss_triggers(tx.clone()).await?;

    tokio::task::spawn(async move {
        while let Some(_fms_trigger) = rx.recv().await {
            match databroker.fetch_data().await {
                Err(e) => {
                    warn!(
                        "failed to retrieve signals from databroker: {}",
                        e
                    );
                }
                Ok(vss_data) => {
                    match filter_relevant_signals(vss_data) {
                        Ok(sllt) => actor_handle.send_data(sllt).await,
                        Err(e) => {
                            log::info!("{}", e);
                            std::process::exit(1);
                        }
                    }
                    if let Some(reduced_signals) = actor_handle.get_curved_results().await {
                        match publisher_channel.send(reduced_signals).await {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("failed to send curvelogged signlas via channel: {}", e);
                            }
                        }
                    }
                }
            }
        }
    });
    let timer_sender = tx.clone();
    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let _ = timer_sender.send(FmsTrigger::Timer).await;
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
                .value_parser(duration_str::parse)
                .long(COVESA_PARAM_TIMER_INTERVAL)
                .alias("timer")
                .help("The time period to wait after polling FMS snapshot data from the kuksa.val Databroker, e.g 5m10s or 1h15m.")
                .value_name("DURATION_SPEC")
                .required(false)
                .env("TIMER_INTERVAL")
                .default_value("1s"),
        )
        .arg(
            Arg::new(PARAM_WINDOW_CAPACITY)
                .value_parser(clap::value_parser!(usize))
                .long(PARAM_WINDOW_CAPACITY)
                .help("The capacity of the window for data processing.")
                .value_name("CAPACITY")
                .required(false)
                .env("WINDOW_CAPACITY")
                .default_value("10"),
        )
}

fn read_token_from_file(filename: &str) -> std::io::Result<String> {
    info!("reading token from file {filename}");
    std::fs::read_to_string(filename)
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            error!("failed to read token from file [{filename}]: {e}");
            e
        })
}

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
            .get_one::<String>(PARAM_INFLUXDB_URI)
            .unwrap()
            .to_owned();
        let influx_token = match args.get_one::<String>(PARAM_INFLUXDB_TOKEN) {
            Some(token) => token.to_string(),
            None => {
                let file_name = args.get_one::<String>(PARAM_INFLUXDB_TOKEN_FILE).unwrap();
                match read_token_from_file(file_name) {
                    Ok(token) => token,
                    Err(e) => return Err(Box::new(e)),
                }
            }
        };
        let influx_org = args
            .get_one::<String>(PARAM_INFLUXDB_ORG)
            .unwrap()
            .to_owned();
        let influx_bucket = args
            .get_one::<String>(PARAM_INFLUXDB_BUCKET)
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

pub enum FmsTrigger {
    Timer,
    Driver1Login,
    Driver1Logout,
    Driver1WorkingStateChanged(String),
    Driver2Login,
    Driver2Logout,
    Driver2WorkingStateChanged(String),
    EngineOn,
    EngineOff,
    ParkingBreakSwitchChanged(bool),
    TellTale(TellTaleInfo),
}

impl FmsTrigger {
    fn _new_trigger(type_: &str) -> Trigger {
        let mut trigger = Trigger::new();
        trigger.context = "RFMS".to_string();
        trigger.type_ = type_.to_string();
        trigger
    }

    fn new_tell_tale_trigger(
        data_entry: DataEntry,
        name: &str,
    ) -> Result<FmsTrigger, UnsupportedValueTypeError> {
        if let Some(value) = data_entry.clone().value.and_then(|v| v.value) {
            match String::try_from(value) {
                Ok(status) => {
                    let mut tell_tale_info = TellTaleInfo::new();
                    tell_tale_info.tell_tale = name.to_string();
                    tell_tale_info.status = status;
                    Ok(FmsTrigger::TellTale(tell_tale_info))
                }
                Err(e) => Err(e),
            }
        } else {
            Err(UnsupportedValueTypeError {})
        }
    }

    fn new_boolean_trigger<P: FnOnce(bool) -> FmsTrigger>(
        data_entry: DataEntry,
        trigger_producer: P,
    ) -> Result<FmsTrigger, UnsupportedValueTypeError> {
        if let Some(data_point) = data_entry.clone().value {
            bool::try_from(data_point.value.unwrap()).map(trigger_producer)
        } else {
            Err(UnsupportedValueTypeError {})
        }
    }

    fn new_string_value_trigger<P: FnOnce(String) -> FmsTrigger>(
        data_entry: DataEntry,
        trigger_producer: P,
    ) -> Result<FmsTrigger, UnsupportedValueTypeError> {
        if let Some(data_point) = data_entry.clone().value {
            String::try_from(data_point.value.unwrap()).map(trigger_producer)
        } else {
            Err(UnsupportedValueTypeError {})
        }
    }
}

impl TryFrom<DataEntry> for FmsTrigger {
    type Error = UnsupportedValueTypeError;

    fn try_from(data_entry: DataEntry) -> Result<Self, Self::Error> {
        match data_entry.path.as_str() {
            vss::FMS_VEHICLE_CABIN_TELLTALE_ECT_STATUS => {
                FmsTrigger::new_tell_tale_trigger(data_entry, TELL_TALE_NAME_ECT)
            }
            vss::FMS_VEHICLE_CABIN_TELLTALE_ENGINEOIL_STATUS => {
                FmsTrigger::new_tell_tale_trigger(data_entry, TELL_TALE_NAME_ENGINE_OIL)
            }
            vss::FMS_VEHICLE_CABIN_TELLTALE_ENGINE_STATUS => {
                FmsTrigger::new_tell_tale_trigger(data_entry, TELL_TALE_NAME_ENGINE_MIL_INDICATOR)
            }
            vss::FMS_VEHICLE_CABIN_TELLTALE_FUELLEVEL_STATUS => {
                FmsTrigger::new_tell_tale_trigger(data_entry, TELL_TALE_NAME_FUEL_LEVEL)
            }
            vss::FMS_VEHICLE_CABIN_TELLTALE_PARKINGBRAKE_STATUS => {
                FmsTrigger::new_tell_tale_trigger(data_entry, TELL_TALE_NAME_PARKING_BRAKE)
            }
            vss::VSS_VEHICLE_CHASSIS_PARKINGBRAKE_ISENGAGED => {
                FmsTrigger::new_boolean_trigger(data_entry, FmsTrigger::ParkingBreakSwitchChanged)
            }
            vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_ISRUNNING => {
                FmsTrigger::new_boolean_trigger(data_entry, |is_running| {
                    if is_running {
                        FmsTrigger::EngineOn
                    } else {
                        FmsTrigger::EngineOff
                    }
                })
            }
            vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_ISCARDPRESENT => {
                FmsTrigger::new_boolean_trigger(data_entry, |card_is_present| {
                    if card_is_present {
                        FmsTrigger::Driver1Login
                    } else {
                        FmsTrigger::Driver1Logout
                    }
                })
            }
            vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_WORKINGSTATE => {
                FmsTrigger::new_string_value_trigger(
                    data_entry,
                    FmsTrigger::Driver1WorkingStateChanged,
                )
            }
            vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_ISCARDPRESENT => {
                FmsTrigger::new_boolean_trigger(data_entry, |card_is_present| {
                    if card_is_present {
                        FmsTrigger::Driver2Login
                    } else {
                        FmsTrigger::Driver2Logout
                    }
                })
            }
            vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_WORKINGSTATE => {
                FmsTrigger::new_string_value_trigger(
                    data_entry,
                    FmsTrigger::Driver2WorkingStateChanged,
                )
            }
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

pub fn filter_relevant_signals(vss_data: HashMap<String, Value>) -> Result<ChosenSignals, String> {
    let mut sllt = ChosenSignals::new();
    for path in SLLT_VSS_PATHS.iter().map(|path| path.to_string()) {
        if vss_data.contains_key(&path) {
            if path == vss::VSS_VEHICLE_SPEED.to_string() {
                let value = vss_data
                    .get(&vss::VSS_VEHICLE_SPEED.to_string())
                    .unwrap()
                    .to_owned();
                let speed = f64::try_from(value).unwrap();
                sllt.add_speed(speed as f32);
            } else if path == vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE.to_string() {
                let value = vss_data
                    .get(&vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE.to_string())
                    .unwrap()
                    .to_owned();
                let lat = f64::try_from(value).unwrap();
                sllt.add_lat(lat);
            } else if path == vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE.to_string() {
                let value = vss_data
                    .get(&vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE.to_string())
                    .unwrap()
                    .to_owned();
                let lon = f64::try_from(value).unwrap();
                sllt.add_lon(lon);
            }
        }
    }

    match sllt.is_full() {
        false => Err("Databroker could not extract all the relevant signals".to_string()),
        true => Ok(sllt),
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

        info!(
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

    pub async fn fetch_data(&mut self) -> Result<HashMap<String, Value>, DatabrokerError> {
        let entry_requests: Vec<EntryRequest> = SLLT_VSS_PATHS
            .iter()
            .map(|path| EntryRequest {
                path: path.to_string(),
                view: View::CurrentValue as i32,
                fields: vec![Field::Value as i32],
            })
            .collect();

        let mut vss_data: HashMap<String, Value> = HashMap::new();
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
                        error.code, error.message
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
                        let name = data_entry.path.to_owned();
                        if let Some(value) = data_entry.value.and_then(|dp| dp.value) {
                            vss_data.insert(name, value);
                        }
                    });
                }
                Ok(vss_data)
            }
        }
    }

    pub async fn register_sllt_vss_triggers(
        &mut self,
        sender: Sender<FmsTrigger>,
    ) -> Result<(), DatabrokerError> {
        let subscribe_entries: Vec<SubscribeEntry> = SLLT_VSS_PATHS
            .iter()
            .map(|path| SubscribeEntry {
                path: path.to_string(),
                view: View::CurrentValue as i32,
                fields: vec![Field::Value as i32],
            })
            .collect();

        let req = SubscribeRequest {
            entries: subscribe_entries,
        };

        match self.client.subscribe(req).await {
            Ok(response) => {
                let mut stream = response.into_inner();
                tokio::task::spawn(async move {
                    while let Ok(message) = stream.message().await {
                        if let Some(response) = message {
                            for update in response.updates {
                                match update.entry {
                                    Some(data_entry) => {
                                        if let Ok(trigger) = FmsTrigger::try_from(data_entry) {
                                            let _ = sender.send(trigger).await;
                                        }
                                    }
                                    None => {
                                        log::info!(
                                            "ignoring notification from Databroker containing no data"
                                        );
                                    }
                                }
                            }
                        }
                    }
                });
                Ok(())
            }
            Err(e) => {
                log::info!("failed to register triggers for signals: {}", e);
                Err(DatabrokerError {
                    description: e.message().to_string(),
                })
            }
        }
    }
}

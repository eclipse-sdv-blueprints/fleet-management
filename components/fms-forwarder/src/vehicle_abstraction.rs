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
use clap::Args;
use log::{debug, error, info, warn};
use protobuf::MessageField;
use tokio::sync::mpsc::Sender;

use std::{collections::HashMap, error::Error, fmt::Display, time::Duration};

use tonic::{
    transport::{Channel, Endpoint},
    Request,
};

use fms_proto::fms::{TellTaleInfo, Trigger, VehicleStatus};
use kuksa::{datapoint::Value, val_client::ValClient, EntryRequest, Field, GetRequest, View};

use self::kuksa::{DataEntry, SubscribeEntry, SubscribeRequest, UnsupportedValueTypeError};

const SNAPSHOT_VSS_PATHS: &[&str] = &[
    vss::VSS_VEHICLE_CHASSIS_PARKINGBRAKE_ISENGAGED,
    vss::VSS_VEHICLE_CURRENTLOCATION_ALTITUDE,
    vss::VSS_VEHICLE_CURRENTLOCATION_HEADING,
    vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE,
    vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE,
    vss::FMS_VEHICLE_CURRENTLOCATION_SPEED,
    vss::VSS_VEHICLE_CURRENTLOCATION_TIMESTAMP,
    vss::VSS_VEHICLE_CURRENTOVERALLWEIGHT,
    vss::VSS_VEHICLE_EXTERIOR_AIRTEMPERATURE,
    vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_DIESELEXHAUSTFLUID_LEVEL,
    vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_ENGINEHOURS,
    vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_SPEED,
    vss::FMS_VEHICLE_POWERTRAIN_CURRENTFUELTYPE,
    vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_ACCUMULATEDCONSUMPTION,
    vss::VSS_VEHICLE_POWERTRAIN_FUELSYSTEM_RANGE,
    vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_TANK_FIRST_LEVEL,
    vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_TANK_SECOND_LEVEL,
    vss::VSS_VEHICLE_POWERTRAIN_RANGE,
    vss::VSS_VEHICLE_SPEED,
    vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_CARDISSUINGMEMBERSTATE,
    vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_IDENTIFICATION,
    vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_WORKINGSTATE,
    vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_WORKINGSTATE,
    vss::FMS_VEHICLE_TACHOGRAPH_VEHICLESPEED,
    vss::FMS_VEHICLE_TRAVELED_DISTANCE_HIGH_RES,
    vss::VSS_VEHICLE_VEHICLEIDENTIFICATION_VIN,
];

const TRIGGER_VSS_PATHS: &[&str] = &[
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

const TELL_TALE_NAME_ECT: &str = "ENGINE_COOLANT_TEMPERATURE";
const TELL_TALE_NAME_ENGINE_OIL: &str = "ENGINE_OIL";
const TELL_TALE_NAME_ENGINE_MIL_INDICATOR: &str = "ENGINE_MIL_INDICATOR";
const TELL_TALE_NAME_FUEL_LEVEL: &str = "FUEL_LEVEL";
const TELL_TALE_NAME_PARKING_BRAKE: &str = "PARKING_BRAKE";

const TRIGGER_DRIVER1_WORKING_STATE_CHANGED: &str = "DRIVER_1_WORKING_STATE_CHANGED";
const TRIGGER_DRIVER2_WORKING_STATE_CHANGED: &str = "DRIVER_2_WORKING_STATE_CHANGED";
const TRIGGER_PARKING_BRAKE_SWITCH_CHANGE: &str = "PARKING_BRAKE_SWITCH_CHANGE";
const TRIGGER_DRIVER_LOGIN: &str = "DRIVER_LOGIN";
const TRIGGER_DRIVER_LOGOUT: &str = "DRIVER_LOGOUT";
const TRIGGER_ENGINE_ON: &str = "ENGINE_ON";
const TRIGGER_ENGINE_OFF: &str = "ENGINE_OFF";
const TRIGGER_TELL_TALE: &str = "TELL_TALE";
const TRIGGER_TIMER: &str = "TIMER";

const PARAM_DATABROKER_URI: &str = "databroker-uri";
const PARAM_DEFAULT_VIN: &str = "default-vin";
const PARAM_TIMER_INTERVAL: &str = "timer-interval";

mod kuksa;
mod vss;

#[derive(Args)]
pub struct KuksaDatabrokerClientConfig {
    /// The HTTP(S) URI of the Eclipse Kuksa Databroker's gRPC endpoint.
    #[arg(long = PARAM_DATABROKER_URI, value_name = "URI", env = "KUKSA_DATABROKER_URI", default_value = "http://127.0.0.1:55555", value_parser = clap::builder::NonEmptyStringValueParser::new() )]
    databroker_uri: String,

    /// The default VIN to use if the kuksa.val Databroker does not contain the vehicle's VIN.
    /// The VIN is used as a tag on measurements written to the InfluxDB server.
    #[arg(long = PARAM_DEFAULT_VIN, value_name = "IDENTIFIER", env = "DEFAULT_VIN", default_value = "YV2E4C3A5VB180691", value_parser = clap::builder::NonEmptyStringValueParser::new() )]
    default_vin: String,

    /// The time period to wait after polling FMS snapshot data from the kuksa.val Databroker, e.g 5m10s or 1h15m.
    #[arg(long = PARAM_TIMER_INTERVAL, value_name = "DURATION_SPEC", env = "TIMER_INTERVAL", default_value = "5s", value_parser = |s: &str| duration_str::parse(s) )]
    timer_interval: Duration,
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

enum FmsTrigger {
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
    fn new_trigger(type_: &str) -> Trigger {
        let mut trigger = Trigger::new();
        trigger.context = "RFMS".to_string();
        trigger.type_ = type_.to_string();
        trigger
    }

    pub fn as_trigger(&self) -> Trigger {
        match self {
            Self::Timer => FmsTrigger::new_trigger(TRIGGER_TIMER),
            Self::Driver1Login => FmsTrigger::new_trigger(TRIGGER_DRIVER_LOGIN),
            Self::Driver1Logout => FmsTrigger::new_trigger(TRIGGER_DRIVER_LOGOUT),
            Self::Driver1WorkingStateChanged(_status) => {
                FmsTrigger::new_trigger(TRIGGER_DRIVER1_WORKING_STATE_CHANGED)
            }
            Self::Driver2Login => FmsTrigger::new_trigger(TRIGGER_DRIVER_LOGIN),
            Self::Driver2Logout => FmsTrigger::new_trigger(TRIGGER_DRIVER_LOGOUT),
            Self::Driver2WorkingStateChanged(_status) => {
                FmsTrigger::new_trigger(TRIGGER_DRIVER2_WORKING_STATE_CHANGED)
            }
            Self::EngineOn => FmsTrigger::new_trigger(TRIGGER_ENGINE_ON),
            Self::EngineOff => FmsTrigger::new_trigger(TRIGGER_ENGINE_OFF),
            Self::ParkingBreakSwitchChanged(_is_engaged) => {
                FmsTrigger::new_trigger(TRIGGER_PARKING_BRAKE_SWITCH_CHANGE)
            }
            Self::TellTale(info) => {
                let mut trigger = FmsTrigger::new_trigger(TRIGGER_TELL_TALE);
                trigger.tell_tale_info = MessageField::some(info.clone());
                trigger
            }
        }
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

struct KuksaValDatabroker {
    client: Box<ValClient<Channel>>,
    default_vin: String,
}

impl KuksaValDatabroker {
    async fn new(config: &KuksaDatabrokerClientConfig) -> Result<Self, DatabrokerError> {
        info!(
            "creating client for Eclipse Kuksa Databroker at {}",
            config.databroker_uri
        );
        Endpoint::from_shared(config.databroker_uri.to_owned())
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
                    default_vin: config.default_vin.to_owned(),
                }
            })
    }

    pub async fn get_vehicle_status(&mut self) -> Result<VehicleStatus, DatabrokerError> {
        let entry_requests: Vec<EntryRequest> = SNAPSHOT_VSS_PATHS
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
                warn!("failed to retrieve snapshot data points from Databroker {status}");
                Err(DatabrokerError {
                    description: format!("status code {}", status.code()),
                })
            }
            Ok(get_response) => {
                if let Some(error) = get_response.error {
                    warn!(
                        "response from Databroker contains global error [code: {}, message: {}]",
                        error.code, error.message
                    );
                } else {
                    get_response
                        .errors
                        .into_iter()
                        .for_each(|data_entry_error| {
                            if let Some(err) = data_entry_error.error {
                                warn!(
                                    "response from Databroker contains error [path: {}, error: {:?}]",
                                    data_entry_error.path, err
                                );
                            }
                        });
                    get_response.entries.into_iter().for_each(|data_entry| {
                        let name = data_entry.path.to_owned();
                        if let Some(value) = data_entry.value.and_then(|dp| dp.value) {
                            debug!("got value [path: {}]: {:?}", name, value);
                            vss_data.insert(name, value);
                        }
                    });
                }
                Ok(kuksa::new_vehicle_status(vss_data, &self.default_vin))
            }
        }
    }

    pub async fn register_triggers(
        &mut self,
        sender: Sender<FmsTrigger>,
    ) -> Result<(), DatabrokerError> {
        let subscribe_entries: Vec<SubscribeEntry> = TRIGGER_VSS_PATHS
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
                                        debug!(
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
                warn!("failed to register triggers for signals: {}", e);
                Err(DatabrokerError {
                    description: e.message().to_string(),
                })
            }
        }
    }
}

/// Sets up a connection to the Databroker and registers callbacks for
/// signals that trigger the reporting of the vehicle's current status.
pub async fn init(
    config: &KuksaDatabrokerClientConfig,
    status_publisher: Sender<VehicleStatus>,
) -> Result<(), DatabrokerError> {
    let timer_interval = config.timer_interval.to_owned();

    let mut databroker = KuksaValDatabroker::new(config).await?;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<FmsTrigger>(50);
    let _ = &databroker.register_triggers(tx.clone()).await?;

    tokio::task::spawn(async move {
        let mut current_status = VehicleStatus::new();

        while let Some(fms_trigger) = rx.recv().await {
            match databroker.get_vehicle_status().await {
                Err(e) => {
                    warn!(
                        "failed to retrieve current vehicle status from databroker: {}",
                        e
                    );
                }
                Ok(mut new_vehicle_status) => {
                    let last_known_status = current_status.clone();
                    current_status = new_vehicle_status.clone();
                    let mut trigger = fms_trigger.as_trigger();
                    match fms_trigger {
                        FmsTrigger::Driver1Login => {
                            info!("driver one has logged in");
                            trigger.driver = new_vehicle_status.driver1_id.clone();
                        }
                        FmsTrigger::Driver1Logout => {
                            info!("driver one has logged out");
                            trigger.driver = last_known_status.driver1_id.clone();
                        }
                        FmsTrigger::Driver1WorkingStateChanged(status) => {
                            info!(
                                "driver one's working state has changed to status {}",
                                status
                            );
                            trigger.driver = last_known_status.driver1_id.clone();
                        }
                        FmsTrigger::Driver2Login => {
                            info!("driver two has logged in");
                            trigger.driver = new_vehicle_status
                                .snapshot_data
                                .get_or_default()
                                .driver2_id
                                .clone();
                        }
                        FmsTrigger::Driver2Logout => {
                            info!("driver two has logged out");
                            trigger.driver = last_known_status
                                .snapshot_data
                                .get_or_default()
                                .driver2_id
                                .clone();
                        }
                        FmsTrigger::Driver2WorkingStateChanged(status) => {
                            info!(
                                "driver two's working state has changed to status {}",
                                status
                            );
                            trigger.driver = last_known_status
                                .snapshot_data
                                .get_or_default()
                                .driver2_id
                                .clone();
                        }
                        FmsTrigger::EngineOn => {
                            info!("engine has been started");
                        }
                        FmsTrigger::EngineOff => {
                            info!("engine has been stopped");
                        }
                        FmsTrigger::ParkingBreakSwitchChanged(is_engaged) => {
                            info!("parking brake engaged: {}", is_engaged);
                        }
                        FmsTrigger::TellTale(info) => {
                            info!(
                                "tell tale {} has changed to status {}]",
                                info.tell_tale, info.status
                            );
                        }
                        FmsTrigger::Timer => {
                            info!("timer has fired");
                        }
                    }
                    new_vehicle_status.trigger = MessageField::some(trigger);
                    match status_publisher.send(new_vehicle_status).await {
                        Ok(_) => {}
                        Err(e) => {
                            warn!("failed to send new vehicle status via channel: {}", e);
                        }
                    };
                }
            }
        }
    });

    let timer_sender = tx.clone();
    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(timer_interval).await;
            let _ = timer_sender.send(FmsTrigger::Timer).await;
        }
    });
    Ok(())
}

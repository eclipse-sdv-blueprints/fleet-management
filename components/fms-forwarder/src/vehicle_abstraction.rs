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
use std::{collections::HashMap, error::Error, fmt::Display, time::Duration};

use clap::Args;
use http::Uri;
use kuksa_rust_sdk::kuksa::{common::ClientTraitV2, val::v2::KuksaClientV2};
use kuksa_rust_sdk::v2_proto::value::TypedValue;
use kuksa_rust_sdk::v2_proto::IncompatibleValueTypeError;
use log::{debug, error, info, warn};
use protobuf::MessageField;
use tokio::sync::mpsc::Sender;

use fms_proto::fms::{TellTaleInfo, Trigger, VehicleStatus};

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
const PARAM_TIMER_INTERVAL: &str = "timer-interval";

mod kuksa;
mod vss;

#[derive(Args)]
pub struct KuksaDatabrokerClientConfig {
    /// The HTTP(S) URI of the Eclipse Kuksa Databroker's gRPC endpoint.
    #[arg(long = PARAM_DATABROKER_URI, value_name = "URI", env = "KUKSA_DATABROKER_URI", default_value = "http://127.0.0.1:55555", value_parser = clap::builder::NonEmptyStringValueParser::new() )]
    databroker_uri: String,

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
        value: &TypedValue,
        name: &str,
    ) -> Result<FmsTrigger, IncompatibleValueTypeError> {
        String::try_from(value).map(|status| {
            let mut tell_tale_info = TellTaleInfo::new();
            tell_tale_info.tell_tale = name.to_string();
            tell_tale_info.status = status;
            FmsTrigger::TellTale(tell_tale_info)
        })
    }

    fn new_boolean_trigger<P: FnOnce(bool) -> FmsTrigger>(
        value: &TypedValue,
        trigger_producer: P,
    ) -> Result<FmsTrigger, IncompatibleValueTypeError> {
        bool::try_from(value).map(trigger_producer)
    }

    fn new_string_value_trigger<P: FnOnce(String) -> FmsTrigger>(
        value: &TypedValue,
        trigger_producer: P,
    ) -> Result<FmsTrigger, IncompatibleValueTypeError> {
        String::try_from(value).map(trigger_producer)
    }
}

impl TryFrom<(String, &TypedValue)> for FmsTrigger {
    type Error = IncompatibleValueTypeError;

    fn try_from(value: (String, &TypedValue)) -> Result<Self, Self::Error> {
        match value.0.as_str() {
            vss::FMS_VEHICLE_CABIN_TELLTALE_ECT_STATUS => {
                FmsTrigger::new_tell_tale_trigger(value.1, TELL_TALE_NAME_ECT)
            }
            vss::FMS_VEHICLE_CABIN_TELLTALE_ENGINEOIL_STATUS => {
                FmsTrigger::new_tell_tale_trigger(value.1, TELL_TALE_NAME_ENGINE_OIL)
            }
            vss::FMS_VEHICLE_CABIN_TELLTALE_ENGINE_STATUS => {
                FmsTrigger::new_tell_tale_trigger(value.1, TELL_TALE_NAME_ENGINE_MIL_INDICATOR)
            }
            vss::FMS_VEHICLE_CABIN_TELLTALE_FUELLEVEL_STATUS => {
                FmsTrigger::new_tell_tale_trigger(value.1, TELL_TALE_NAME_FUEL_LEVEL)
            }
            vss::FMS_VEHICLE_CABIN_TELLTALE_PARKINGBRAKE_STATUS => {
                FmsTrigger::new_tell_tale_trigger(value.1, TELL_TALE_NAME_PARKING_BRAKE)
            }
            vss::VSS_VEHICLE_CHASSIS_PARKINGBRAKE_ISENGAGED => {
                FmsTrigger::new_boolean_trigger(value.1, FmsTrigger::ParkingBreakSwitchChanged)
            }
            vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_ISRUNNING => {
                FmsTrigger::new_boolean_trigger(value.1, |is_running| {
                    if is_running {
                        FmsTrigger::EngineOn
                    } else {
                        FmsTrigger::EngineOff
                    }
                })
            }
            vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_ISCARDPRESENT => {
                FmsTrigger::new_boolean_trigger(value.1, |card_is_present| {
                    if card_is_present {
                        FmsTrigger::Driver1Login
                    } else {
                        FmsTrigger::Driver1Logout
                    }
                })
            }
            vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_WORKINGSTATE => {
                FmsTrigger::new_string_value_trigger(
                    value.1,
                    FmsTrigger::Driver1WorkingStateChanged,
                )
            }
            vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_ISCARDPRESENT => {
                FmsTrigger::new_boolean_trigger(value.1, |card_is_present| {
                    if card_is_present {
                        FmsTrigger::Driver2Login
                    } else {
                        FmsTrigger::Driver2Logout
                    }
                })
            }
            vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_WORKINGSTATE => {
                FmsTrigger::new_string_value_trigger(
                    value.1,
                    FmsTrigger::Driver2WorkingStateChanged,
                )
            }
            _ => Err(IncompatibleValueTypeError {}),
        }
    }
}

struct KuksaValDatabroker {
    client: Box<KuksaClientV2>,
}

impl KuksaValDatabroker {
    async fn new(config: &KuksaDatabrokerClientConfig) -> Result<Self, DatabrokerError> {
        info!(
            "creating client for Eclipse Kuksa Databroker at {}",
            config.databroker_uri
        );
        Uri::try_from(config.databroker_uri.clone())
            .map_err(|err| {
                error!("invalid Databroker URI: {err}");
                DatabrokerError {
                    description: err.to_string(),
                }
            })
            .map(|uri| {
                let client = KuksaClientV2::new(uri);
                KuksaValDatabroker {
                    client: Box::new(client),
                }
            })
    }

    pub async fn get_vehicle_status(&mut self) -> Result<VehicleStatus, DatabrokerError> {
        let paths = SNAPSHOT_VSS_PATHS.iter().map(|v| v.to_string()).collect();

        match self.client.get_values(paths).await {
            Err(kuksa_rust_sdk::kuksa::common::ClientError::Connection(msg)) => {
                warn!("failed to retrieve snapshot data points from Databroker: {msg}");
                Err(DatabrokerError { description: msg })
            }
            Err(kuksa_rust_sdk::kuksa::common::ClientError::Status(status)) => {
                warn!(
                    "failed to retrieve snapshot data points from Databroker: {}",
                    status.message()
                );
                Err(DatabrokerError {
                    description: status.message().to_string(),
                })
            }
            Err(kuksa_rust_sdk::kuksa::common::ClientError::Function(errors)) => {
                errors.iter().for_each(|error| {
                    warn!("failed to retrieve snapshot data points from Databroker: {error:?}");
                });
                Err(DatabrokerError {
                    description: "multiple errors while retrieving snapshot data".to_string(),
                })
            }
            Ok(get_response) => {
                let mut vss_data = HashMap::new();
                let mut idx = 0usize;
                get_response.iter().for_each(|data_entry| {
                    if let (name, Some(value)) = (
                        SNAPSHOT_VSS_PATHS[idx],
                        data_entry
                            .value
                            .as_ref()
                            .and_then(|v| v.typed_value.as_ref()),
                    ) {
                        debug!("got value [path: {name}]: {value:?}");
                        vss_data.insert(name.to_owned(), value.to_owned());
                    }
                    idx += 1;
                });
                kuksa::new_vehicle_status(vss_data)
            }
        }
    }

    pub async fn register_triggers(
        &mut self,
        sender: Sender<FmsTrigger>,
    ) -> Result<(), DatabrokerError> {
        let paths = TRIGGER_VSS_PATHS.iter().map(|v| v.to_string()).collect();

        match self.client.subscribe(paths, None, None).await {
            Ok(mut response) => {
                tokio::task::spawn(async move {
                    while let Ok(message) = response.message().await {
                        if let Some(response) = message {
                            for (path, datapoint) in response.entries {
                                if let Some(value) = datapoint
                                    .value
                                    .as_ref()
                                    .and_then(|v| v.typed_value.as_ref())
                                {
                                    if let Ok(trigger) = FmsTrigger::try_from((path, value)) {
                                        let _ = sender.send(trigger).await;
                                    }
                                } else {
                                    debug!(
                                        "ignoring notification from Databroker containing no data"
                                    );
                                }
                            }
                        }
                    }
                });
                Ok(())
            }
            Err(err) => {
                warn!("failed to register triggers for signals: {err}");
                Err(DatabrokerError {
                    description: err.to_string(),
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
    databroker.register_triggers(tx.clone()).await?;

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

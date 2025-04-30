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

use kuksa_rust_sdk::v2_proto::value::TypedValue;
use log::debug;
use protobuf::{well_known_types::timestamp::Timestamp, MessageField};

use std::collections::HashMap;

use crate::vehicle_abstraction::vss;
use fms_proto::fms::VehicleStatus;

use super::DatabrokerError;

pub fn new_vehicle_status(
    data: HashMap<String, TypedValue>,
) -> Result<VehicleStatus, DatabrokerError> {
    let Some(vin) = data
        .get(vss::VSS_VEHICLE_VEHICLEIDENTIFICATION_VIN)
        .and_then(|v| String::try_from(v).ok())
    else {
        return Err(DatabrokerError {
            description: "Databroker does not contain VIN (yet)".to_string(),
        });
    };

    let mut vehicle_status = VehicleStatus::new();
    vehicle_status.created = MessageField::some(Timestamp::now());
    vehicle_status.vin = vin;

    if let Some(value) = data.get(vss::VSS_VEHICLE_CHASSIS_PARKINGBRAKE_ISENGAGED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .parking_brake_engaged = bool::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .latitude = f64::try_from(value).unwrap_or_default();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .longitude = f64::try_from(value).unwrap_or_default();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_ALTITUDE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .altitude = i32::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_HEADING) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .heading = u32::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_CURRENTLOCATION_SPEED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .speed = f64::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_TIMESTAMP) {
        // this will succeed because we know that the Databroker will only accept a String as
        // this VSS Data Entry's value
        let iso_date_time: String = String::try_from(value).unwrap();
        match chrono::DateTime::parse_from_rfc3339(&iso_date_time) {
            Ok(instant) => {
                let position_instant = vehicle_status
                    .snapshot_data
                    .mut_or_insert_default()
                    .gnss_position
                    .mut_or_insert_default()
                    .instant
                    .mut_or_insert_default();
                position_instant.seconds = instant.timestamp();
                position_instant.nanos = instant.timestamp_subsec_nanos() as i32;
            }
            Err(_e) => debug!("failed to parse value as ISO8601 date-time string"),
        }
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTOVERALLWEIGHT) {
        vehicle_status.gross_combination_vehicle_weight = u32::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_EXTERIOR_AIRTEMPERATURE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .ambient_air_temperature = f64::try_from(value).ok();
    }

    if let Some(value) =
        data.get(vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_DIESELEXHAUSTFLUID_LEVEL)
    {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .catalyst_fuel_level = f64::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_ENGINEHOURS) {
        vehicle_status.total_engine_hours = f64::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_SPEED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .engine_speed = f64::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_POWERTRAIN_CURRENTFUELTYPE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .fuel_type = String::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_ACCUMULATEDCONSUMPTION) {
        vehicle_status.engine_total_fuel_used = u64::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_POWERTRAIN_RANGE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .estimated_distance_to_empty
            .mut_or_insert_default()
            .total = u64::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_POWERTRAIN_FUELSYSTEM_RANGE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .estimated_distance_to_empty
            .mut_or_insert_default()
            .fuel = u64::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_TANK_FIRST_LEVEL) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .fuel_level1 = f64::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_TANK_SECOND_LEVEL) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .fuel_level2 = f64::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_SPEED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .wheel_based_speed = f64::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_IDENTIFICATION) {
        vehicle_status
            .driver1_id
            .mut_or_insert_default()
            .tacho_driver_identification
            .mut_or_insert_default()
            .driver_identification = String::try_from(value).unwrap_or_default();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_WORKINGSTATE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .driver1_working_state = String::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_WORKINGSTATE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .driver2_working_state = String::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_TACHOGRAPH_VEHICLESPEED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .tachograph_speed = f64::try_from(value).ok();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_TRAVELED_DISTANCE_HIGH_RES) {
        vehicle_status.hr_total_vehicle_distance = u64::try_from(value).ok();
    }
    Ok(vehicle_status)
}

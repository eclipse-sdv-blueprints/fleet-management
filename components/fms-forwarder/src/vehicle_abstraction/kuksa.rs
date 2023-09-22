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

tonic::include_proto!("kuksa.val.v1");

use log::debug;
use protobuf::{MessageField, well_known_types::timestamp::Timestamp};

use std::collections::HashMap;

use self::datapoint::Value;
use fms_proto::fms::VehicleStatus;
use crate::vehicle_abstraction::vss;

#[derive(Debug)]
pub struct UnsupportedValueTypeError{}

impl TryFrom<Value> for u32 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<u32> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as u64),
            Value::Uint64(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<u64> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as u64)),
            Value::Uint64(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as i32),
            Value::Int32(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<i32> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as i32)),
            Value::Int32(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as i64),
            Value::Uint64(v) => Ok(v as i64),
            Value::Int32(v) => Ok(v as i64),
            Value::Int64(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<i64> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as i64)),
            Value::Uint64(v) => Ok(Some(v as i64)),
            Value::Int32(v) => Ok(Some(v as i64)),
            Value::Int64(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for f32 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as f32),
            Value::Int32(v) => Ok(v as f32),
            Value::Float(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<f32> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as f32)),
            Value::Int32(v) => Ok(Some(v as f32)),
            Value::Float(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as f64),
            Value::Uint64(v) => Ok(v as f64),
            Value::Int32(v) => Ok(v as f64),
            Value::Int64(v) => Ok(v as f64),
            Value::Double(v) => Ok(v),
            Value::Float(v) => Ok(v as f64),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<f64> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as f64)),
            Value::Uint64(v) => Ok(Some(v as f64)),
            Value::Int32(v) => Ok(Some(v as f64)),
            Value::Int64(v) => Ok(Some(v as f64)),
            Value::Double(v) => Ok(Some(v)),
            Value::Float(v) => Ok(Some(v as f64)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<String> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<bool> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

pub fn new_vehicle_status(data: HashMap<String, Value>, default_vin: &String) -> VehicleStatus {
    let mut vehicle_status = VehicleStatus::new();
    vehicle_status.created = MessageField::some(Timestamp::now());

    vehicle_status.vin = data
        .get(vss::VSS_VEHICLE_VEHICLEIDENTIFICATION_VIN)
        .map_or(default_vin.to_owned(), |value| {
            value.clone().try_into().unwrap()
        });

    if let Some(value) = data.get(vss::VSS_VEHICLE_CHASSIS_PARKINGBRAKE_ISENGAGED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .parking_brake_engaged = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_LATITUDE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .latitude = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_LONGITUDE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .longitude = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_ALTITUDE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .altitude = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_HEADING) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .heading = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_CURRENTLOCATION_SPEED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .speed = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_TIMESTAMP) {
        // this will succeed because we know that the Databroker will only accept a String as
        // this VSS Data Entry's value
        let iso_date_time: String = value.clone().try_into().unwrap();
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
        vehicle_status.gross_combination_vehicle_weight = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_EXTERIOR_AIRTEMPERATURE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .ambient_air_temperature = value.clone().try_into().unwrap();
    }

    if let Some(value) =
        data.get(vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_DIESELEXHAUSTFLUID_LEVEL)
    {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .catalyst_fuel_level = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_ENGINEHOURS) {
        vehicle_status.total_engine_hours = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_POWERTRAIN_COMBUSTIONENGINE_SPEED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .engine_speed = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_POWERTRAIN_CURRENTFUELTYPE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .fuel_type = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_ACCUMULATEDCONSUMPTION) {
        vehicle_status.engine_total_fuel_used = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_POWERTRAIN_RANGE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .estimated_distance_to_empty
            .mut_or_insert_default()
            .total = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_POWERTRAIN_FUELSYSTEM_RANGE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .estimated_distance_to_empty
            .mut_or_insert_default()
            .fuel = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_TANK_FIRST_LEVEL) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .fuel_level1 = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_POWERTRAIN_FUELSYSTEM_TANK_SECOND_LEVEL) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .fuel_level2 = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::VSS_VEHICLE_SPEED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .wheel_based_speed = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_IDENTIFICATION) {
        vehicle_status
            .driver1_id
            .mut_or_insert_default()
            .tacho_driver_identification
            .mut_or_insert_default()
            .driver_identification = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_WORKINGSTATE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .driver1_working_state = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_TACHOGRAPH_DRIVER2_WORKINGSTATE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .driver2_working_state = value.clone().try_into().unwrap();
    }
    if let Some(value) = data.get(vss::FMS_VEHICLE_TACHOGRAPH_VEHICLESPEED) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .tachograph_speed = value.clone().try_into().unwrap();
    }

    if let Some(value) = data.get(vss::FMS_VEHICLE_TRAVELED_DISTANCE_HIGH_RES) {
        vehicle_status.hr_total_vehicle_distance = value.clone().try_into().unwrap();
    }
    vehicle_status
}

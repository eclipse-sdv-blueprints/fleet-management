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

fn rounded_f64_to_i32(value: f64) -> Option<i32> {
    let rounded = value.round();

    if rounded.is_finite() && rounded >= i32::MIN as f64 && rounded <= i32::MAX as f64 {
        Some(rounded as i32)
    } else {
        None
    }
}

fn rounded_f64_to_u32(value: f64) -> Option<u32> {
    let rounded = value.round();

    if rounded.is_finite() && rounded >= 0.0 && rounded <= u32::MAX as f64 {
        Some(rounded as u32)
    } else {
        None
    }
}

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

    if let Some(value) = data.get(vss::VSS_VEHICLE_BODY_LIGHTS_DIRECTIONINDICATOR_RIGHT_ISSIGNALING)
    {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .direction_indicator_right = bool::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_BODY_LIGHTS_DIRECTIONINDICATOR_LEFT_ISSIGNALING)
    {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .direction_indicator_left = bool::try_from(value).ok();
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_BODY_LIGHTS_BRAKE_ISACTIVE) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .brake_light_status = String::try_from(value).ok();
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
            .altitude = f64::try_from(value).ok().and_then(rounded_f64_to_i32);
    }
    if let Some(value) = data.get(vss::VSS_VEHICLE_CURRENTLOCATION_HEADING) {
        vehicle_status
            .snapshot_data
            .mut_or_insert_default()
            .gnss_position
            .mut_or_insert_default()
            .heading = f64::try_from(value).ok().and_then(rounded_f64_to_u32);
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
                if instant.timestamp_subsec_millis() > 999 {
                    // this means that the instant is a leap-second which is
                    // not representable in the protobuf::Timestamp type
                    // so we simply ignore the value
                    debug!("ignoring leap-second timestamp value");
                } else {
                    let position_instant = vehicle_status
                        .snapshot_data
                        .mut_or_insert_default()
                        .gnss_position
                        .mut_or_insert_default()
                        .instant
                        .mut_or_insert_default();
                    position_instant.seconds = instant.timestamp();
                    // we already have checked that the value is at most 999,999,999 nanoseconds,
                    // so the cast is safe
                    position_instant.nanos = instant.timestamp_subsec_nanos() as i32;
                }
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

    if let Some(value) = data
        .get(vss::VSS_VEHICLE_DRIVER_IDENTIFIER_SUBJECT)
        .or_else(|| data.get(vss::FMS_VEHICLE_TACHOGRAPH_DRIVER1_IDENTIFICATION))
    {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(7.49_f64 => Some(7))]
    #[test_case(-7.49_f64 => Some(-7))]
    #[test_case(7.50_f64 => Some(8))]
    #[test_case(-7.50_f64 => Some(-8))]
    #[test_case(f64::INFINITY  => None)]
    #[test_case(f64::NEG_INFINITY  => None)]
    #[test_case(f64::NAN  => None)]
    #[test_case(f64::MAX  => None)]
    #[test_case(f64::MIN  => None)]
    fn rounded_f64_to_i32_test(float_to_be_converted: f64) -> Option<i32> {
        rounded_f64_to_i32(float_to_be_converted)
    }

    #[test_case(7.49_f64 => Some(7))]
    #[test_case(-7.49_f64 => None)]
    #[test_case(7.50_f64 => Some(8))]
    #[test_case(-7.50_f64 => None)]
    #[test_case(f64::INFINITY  => None)]
    #[test_case(f64::NEG_INFINITY  => None)]
    #[test_case(f64::NAN  => None)]
    #[test_case(f64::MAX  => None)]
    #[test_case(f64::MIN  => None)]
    fn rounded_f64_to_u32_test(float_to_be_converted: f64) -> Option<u32> {
        rounded_f64_to_u32(float_to_be_converted)
    }
}

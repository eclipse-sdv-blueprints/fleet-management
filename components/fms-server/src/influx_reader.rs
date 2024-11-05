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

use chrono::{DateTime, Utc};
use const_format::formatcp;
use influx_client::connection::{InfluxConnection, InfluxConnectionConfig};
use influxrs::InfluxError;
use log::error;

use crate::models::position::{GnssPositionObject, VehiclePositionObject};
use crate::models::status::{DriverWorkingStateProperty, SnapshotDataObject, VehicleStatusObject};
use crate::models::vehicle::VehicleObject;
use crate::models::TriggerObject;
use crate::query_parser::QueryParameters;

const FILTER_FIELDS_POSITION: &str = formatcp!(
    r#"filter(fn: (r) => contains(set: ["{}","{}","{}","{}","{}","{}","{}","{}", "{}"], value: r._field))"#,
    influx_client::FIELD_CREATED_DATE_TIME,
    influx_client::FIELD_LATITUDE,
    influx_client::FIELD_LONGITUDE,
    influx_client::FIELD_ALTITUDE,
    influx_client::FIELD_HEADING,
    influx_client::FIELD_SPEED,
    influx_client::FIELD_POSITION_DATE_TIME,
    influx_client::FIELD_TACHOGRAPH_SPEED,
    influx_client::FIELD_WHEEL_BASED_SPEED,
);
const FILTER_MEASUREMENT_SNAPSHOT: &str = formatcp!(
    r#"filter(fn: (r) => r._measurement == "{}")"#,
    influx_client::MEASUREMENT_SNAPSHOT,
);
const FILTER_TAG_ANY_VIN: &str = formatcp!(
    r#"filter(fn: (r) => r["{}"] =~ /.*/)"#,
    influx_client::TAG_VIN
);
const FILTER_TAG_ANY_TRIGGER: &str = formatcp!(
    r#"filter(fn: (r) => r["{}"] =~ /.*/)"#,
    influx_client::TAG_TRIGGER
);

fn unpack_value_i32(value: Option<&String>) -> Option<i32> {
    value.and_then(|v| v.parse().ok())
}

fn unpack_value_i64(value: Option<&String>) -> Option<i64> {
    value.and_then(|v| v.parse().ok())
}

fn unpack_value_f64(value: Option<&String>) -> Option<f64> {
    value.and_then(|v| v.parse().ok())
}

fn unpack_value_bool(value: Option<&String>) -> Option<bool> {
    value.and_then(|v| v.parse().ok())
}

fn unpack_time(value: Option<&String>) -> Option<DateTime<Utc>> {
    let timestamp = unpack_value_i64(value)?;
    DateTime::from_timestamp_millis(timestamp)
}

fn unpack_driver_working_state(value: Option<&String>) -> Option<DriverWorkingStateProperty> {
    if let Some(state) = value {
        let working_state = match state.as_str() {
            "REST" => Some(DriverWorkingStateProperty::Rest),
            "DRIVER_AVAILABLE" => Some(DriverWorkingStateProperty::DriverAvailable),
            "WORK" => Some(DriverWorkingStateProperty::Work),
            "DRIVE" => Some(DriverWorkingStateProperty::Drive),
            "ERROR" => Some(DriverWorkingStateProperty::Error),
            "NOT_AVAILABLE" => Some(DriverWorkingStateProperty::NotAvailable),
            _ => None,
        };
        return working_state;
    }
    None
}

pub struct InfluxReader {
    influx_con: InfluxConnection,
}

impl InfluxReader {
    pub fn new(args: &InfluxConnectionConfig) -> Result<Self, Box<dyn std::error::Error>> {
        InfluxConnection::new(args).map(|con| InfluxReader { influx_con: con })
    }

    pub async fn get_vehicles(&self) -> Result<Vec<VehicleObject>, InfluxError> {
        let read_query = influxrs::Query::new(format!(
            r#"
                import "influxdata/influxdb/schema"
                schema.tagValues(bucket: "{}", tag: "{}")
            "#,
            self.influx_con.bucket,
            influx_client::TAG_VIN,
        ));

        self.influx_con.client.query(read_query).await.map(|vins| {
            vins.into_iter()
                .filter_map(|entry| {
                    entry
                        .get("_value")
                        .map(|vin| VehicleObject::new(vin.to_string()))
                })
                .collect()
        })
    }

    pub async fn get_vehicleposition(
        &self,
        parameters: &QueryParameters,
    ) -> Result<Vec<VehiclePositionObject>, InfluxError> {
        // Build Query
        let time_filter = format!(
            "range(start: {}, stop: {})",
            parameters.start_time, parameters.stop_time
        );
        let vin_filter = match &parameters.vin {
            Some(v) => format!(
                r#"filter(fn: (r) => r["{}"] == "{}""#,
                influx_client::TAG_VIN,
                v
            ),
            None => FILTER_TAG_ANY_VIN.to_string(),
        };
        let trigger_filter = match &parameters.trigger_filter {
            Some(t) => format!(
                r#"filter(fn: (r) => r["{}"] == "{}")"#,
                influx_client::TAG_TRIGGER,
                t
            ),
            None => FILTER_TAG_ANY_TRIGGER.to_string(),
        };

        let mut read_query =
            influxrs::Query::new(format!(r#"from(bucket: "{}")"#, self.influx_con.bucket))
                .then(time_filter)
                .then(FILTER_MEASUREMENT_SNAPSHOT)
                .then(vin_filter)
                .then(trigger_filter)
                .then(FILTER_FIELDS_POSITION);
        if Some(true) == parameters.latest_only {
            read_query = read_query.then("last()");
        }
        read_query = read_query
            .then(r#"pivot(rowKey: ["_time"], columnKey: ["_field"], valueColumn: "_value")"#);

        self.influx_con
            .client
            .query(read_query)
            .await
            .map(|measurements| {
                measurements
                    .into_iter()
                    .filter_map(|entry| {
                        match (
                            entry.get(influx_client::TAG_VIN),
                            entry.get(influx_client::TAG_TRIGGER),
                            unpack_time(entry.get(influx_client::FIELD_CREATED_DATE_TIME)),
                        ) {
                            (Some(vin), Some(trigger), Some(created_date_time)) => {
                                let gnss_position = match (
                                    unpack_time(entry.get(influx_client::FIELD_POSITION_DATE_TIME)),
                                    unpack_value_f64(entry.get(influx_client::FIELD_LONGITUDE)),
                                    unpack_value_f64(entry.get(influx_client::FIELD_LATITUDE)),
                                ) {
                                    (Some(position_date_time), Some(longitude), Some(latitude)) => {
                                        Some(GnssPositionObject {
                                            latitude,
                                            longitude,
                                            heading: unpack_value_i32(
                                                entry.get(influx_client::FIELD_HEADING),
                                            ),
                                            altitude: unpack_value_i32(
                                                entry.get(influx_client::FIELD_ALTITUDE),
                                            ),
                                            speed: unpack_value_f64(
                                                entry.get(influx_client::FIELD_SPEED),
                                            ),
                                            position_date_time,
                                        })
                                    }
                                    _ => None,
                                };

                                // set vehicle positions from result
                                Some(VehiclePositionObject {
                                    vin: vin.to_string(),
                                    trigger_type: TriggerObject::new(
                                        trigger.to_string(),
                                        "RFMS".to_string(),
                                    ),
                                    created_date_time,
                                    received_date_time: chrono::Utc::now(),
                                    gnss_position,
                                    wheel_based_speed: unpack_value_f64(
                                        entry.get(influx_client::FIELD_WHEEL_BASED_SPEED),
                                    ),
                                    tachograph_speed: unpack_value_f64(
                                        entry.get(influx_client::FIELD_TACHOGRAPH_SPEED),
                                    ),
                                })
                            }
                            _ => None,
                        }
                    })
                    .collect()
            })
    }

    pub async fn get_vehiclesstatuses(
        &self,
        parameters: &QueryParameters,
    ) -> Result<Vec<VehicleStatusObject>, InfluxError> {
        // Build Query
        let time_filter = format!(
            "range(start: {}, stop: {})",
            parameters.start_time, parameters.stop_time
        );
        let vin_filter = match &parameters.vin {
            Some(v) => format!(
                r#"filter(fn: (r) => r["{}"] == "{}""#,
                influx_client::TAG_VIN,
                v
            ),
            None => FILTER_TAG_ANY_VIN.to_string(),
        };
        let trigger_filter = match &parameters.trigger_filter {
            Some(t) => format!(
                r#"filter(fn: (r) => r["{}"] == "{}")"#,
                influx_client::TAG_TRIGGER,
                t
            ),
            None => FILTER_TAG_ANY_TRIGGER.to_string(),
        };

        let mut read_query =
            influxrs::Query::new(format!(r#"from(bucket: "{}")"#, self.influx_con.bucket))
                .then(time_filter)
                .then(vin_filter)
                .then(trigger_filter)
                .then(r#"aggregateWindow(every: 500ms, fn: last, createEmpty: false)"#);
        if Some(true) == parameters.latest_only {
            read_query = read_query
                .then(r#"group(columns: ["_measurement", "_field", "vin"], mode:"by")"#)
                .then("last()");
        }
        read_query = read_query
            .then(r#"group(columns: ["_field", "trigger", "vin"], mode:"by")"#)
            .then(r#"pivot(rowKey: ["_time"], columnKey: ["_field"], valueColumn: "_value")"#)
            .then(r#"group(columns: ["_time"], mode:"by")"#);

        self.influx_con
            .client
            .query(read_query)
            .await
            .map_err(|e| {
                error!("Error during the query for vehicle statuses: {}", e);
                e
            })
            .map(|measurements| {
                measurements
                    .into_iter()
                    .filter_map(|entry| {
                        let vin = entry.get(influx_client::TAG_VIN);
                        let trigger = entry.get(influx_client::TAG_TRIGGER);
                        let date_time = entry.get(influx_client::FIELD_CREATED_DATE_TIME);
                        let unpacked_time = unpack_time(date_time);
                        match (vin, trigger, unpacked_time) {
                            (Some(vin), Some(trigger), Some(created_date_time)) => {
                                let gnss_position = match (
                                    unpack_time(entry.get(influx_client::FIELD_POSITION_DATE_TIME)),
                                    unpack_value_f64(entry.get(influx_client::FIELD_LONGITUDE)),
                                    unpack_value_f64(entry.get(influx_client::FIELD_LATITUDE)),
                                ) {
                                    (Some(position_date_time), Some(longitude), Some(latitude)) => {
                                        Some(GnssPositionObject {
                                            latitude,
                                            longitude,
                                            heading: unpack_value_i32(
                                                entry.get(influx_client::FIELD_HEADING),
                                            ),
                                            altitude: unpack_value_i32(
                                                entry.get(influx_client::FIELD_ALTITUDE),
                                            ),
                                            speed: unpack_value_f64(
                                                entry.get(influx_client::FIELD_SPEED),
                                            ),
                                            position_date_time,
                                        })
                                    }
                                    _ => None,
                                };

                                let snapshot_data = Some(SnapshotDataObject {
                                    gnss_position,
                                    wheel_based_speed: unpack_value_f64(
                                        entry.get(influx_client::FIELD_WHEEL_BASED_SPEED),
                                    ),
                                    tachograph_speed: unpack_value_f64(
                                        entry.get(influx_client::FIELD_TACHOGRAPH_SPEED),
                                    ),
                                    engine_speed: unpack_value_f64(
                                        entry.get(influx_client::FIELD_ENGINE_SPEED),
                                    ),
                                    electric_motor_speed: None,
                                    fuel_type: None,
                                    fuel_level1: unpack_value_f64(
                                        entry.get(influx_client::FIELD_FUEL_LEVEL1),
                                    ),
                                    fuel_level2: unpack_value_f64(
                                        entry.get(influx_client::FIELD_FUEL_LEVEL2),
                                    ),
                                    catalyst_fuel_level: unpack_value_f64(
                                        entry.get(influx_client::FIELD_CATALYST_FUEL_LEVEL),
                                    ),
                                    driver1_working_state: unpack_driver_working_state(
                                        entry.get(influx_client::FIELD_DRIVER1_WORKING_STATE),
                                    ),
                                    driver2_id: None,
                                    driver2_working_state: unpack_driver_working_state(
                                        entry.get(influx_client::FIELD_DRIVER2_WORKING_STATE),
                                    ),
                                    ambient_air_temperature: unpack_value_f64(
                                        entry.get(influx_client::FIELD_AMBIENT_AIR_TEMP),
                                    ),
                                    parking_brake_switch: unpack_value_bool(
                                        entry.get(influx_client::FIELD_PARKING_BREAK_SWITCH),
                                    ),
                                    hybrid_battery_pack_remaining_charge: None,
                                    battery_pack_charging_status: None,
                                    battery_pack_charging_connection_status: None,
                                    battery_pack_charging_device: None,
                                    battery_pack_charging_power: None,
                                    estimated_time_battery_pack_charging_completed: None,
                                    estimated_distance_to_empty: None,
                                    vehicle_axles: None,
                                    trailers: None,
                                });

                                Some(VehicleStatusObject {
                                    vin: vin.to_string(),
                                    trigger_type: TriggerObject::new(
                                        trigger.to_string(),
                                        "RFMS".to_string(),
                                    ),
                                    created_date_time,
                                    received_date_time: Utc::now(),
                                    hr_total_vehicle_distance: unpack_value_i64(
                                        entry.get(influx_client::FIELD_HR_TOTAL_VEHICLE_DISTANCE),
                                    ),
                                    total_engine_hours: unpack_value_f64(
                                        entry.get(influx_client::FIELD_TOTAL_ENGINE_HOURS),
                                    ),
                                    total_electric_motor_hours: unpack_value_f64(
                                        entry.get(influx_client::FIELD_TOTAL_ELECTRIC_MOTOR_HOURS),
                                    ),
                                    driver1_id: None,
                                    gross_combination_vehicle_weight: unpack_value_i32(entry.get(
                                        influx_client::FIELD_GROSS_COMBINATION_VEHICLE_WEIGHT,
                                    )),
                                    engine_total_fuel_used: unpack_value_i64(
                                        entry.get(influx_client::FIELD_ENGINE_TOTAL_FUEL_USED),
                                    ),
                                    total_fuel_used_gaseous: None,
                                    total_electric_energy_used: None,
                                    status2_of_doors: None,
                                    door_status: None,
                                    accumulated_data: None,
                                    snapshot_data,
                                    uptime_data: None,
                                })
                            }
                            _ => None,
                        }
                    })
                    .collect()
            })
    }
}

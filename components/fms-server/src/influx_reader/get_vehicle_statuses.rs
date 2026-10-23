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

use std::collections::HashMap;

use const_format::formatcp;
use log::{debug, error};

use crate::{
    data_access::{GetVehicleStatuses, StatusesQueryParameters},
    models::{
        error::FmsError,
        position::GnssPositionObject,
        status::{DriverWorkingStateProperty, SnapshotDataObject, VehicleStatusObject},
        TriggerObject,
    },
};

const FILTER_MEASUREMENT_HEADER_SNAPSHOT: &str = formatcp!(
    r#"filter(fn: (r) => r._measurement == "{}" or r._measurement == "{}")"#,
    influx_client::MEASUREMENT_HEADER,
    influx_client::MEASUREMENT_SNAPSHOT,
);

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

impl StatusesQueryParameters {
    pub fn measurement_filter(&self) -> String {
        // for now, we only support filtering by the header and snapshot measurements
        FILTER_MEASUREMENT_HEADER_SNAPSHOT.to_string()
    }

    pub fn range_filter(&self) -> String {
        match (self.common.start_time, self.common.stop_time) {
            (Some(start), None) => format!("range(start: {})", start.to_rfc3339()),
            (Some(start), Some(stop)) => {
                format!(
                    "range(start: {}, stop: {})",
                    start.to_rfc3339(),
                    stop.to_rfc3339()
                )
            }
            (None, None) | (None, Some(_)) => "range(start: -3h)".to_string(),
        }
    }

    pub fn vin_filter(&self) -> Option<String> {
        if let Some(vin) = self.common.vin.as_ref() {
            return Some(format!(
                r#"filter(fn: (r) => r["{}"] == "{}")"#,
                influx_client::TAG_VIN,
                vin
            ));
        }
        if let Some(vin) = self.common.basic.last_vin.as_ref() {
            return Some(format!(
                r#"filter(fn: (r) => r["{}"] > "{}")"#,
                influx_client::TAG_VIN,
                vin
            ));
        }
        None
    }

    pub fn trigger_filter(&self) -> Option<String> {
        if self.trigger_filter.is_empty() {
            return None;
        }
        let filter_expr = self
            .trigger_filter
            .iter()
            .map(|trigger| format!(r#"r["{}"] == "{}""#, influx_client::TAG_TRIGGER, trigger))
            .collect::<Vec<_>>()
            .join(" or ");
        Some(format!(r#"filter(fn: (r) => {filter_expr})"#))
    }

    pub fn is_latest_only(&self) -> bool {
        self.common.latest_only.is_some_and(|v| v)
    }
}

#[async_trait::async_trait]
impl GetVehicleStatuses for super::InfluxReader {
    async fn get_vehiclestatuses(
        &self,
        parameters: &StatusesQueryParameters,
    ) -> Result<Vec<VehicleStatusObject>, FmsError> {
        // Build Query
        let mut read_query =
            influxrs::Query::new(format!(r#"from(bucket: "{}")"#, self.influx_con.bucket))
                .then(parameters.range_filter())
                .then(parameters.measurement_filter());

        if let Some(vin_filter) = parameters.vin_filter() {
            read_query = read_query.then(vin_filter);
        }

        if let Some(trigger_filter) = parameters.trigger_filter() {
            read_query = read_query.then(trigger_filter);
        }

        read_query = read_query
            .then(r#"aggregateWindow(every: 500ms, fn: last, createEmpty: false)"#)
            .then(format!(
                // r#"group(columns: ["_measurement", "_field", "{}"])"#,
                r#"group(columns: ["_field", "{}"])"#,
                influx_client::TAG_VIN,
            ));

        // if the client has specified the lastOnly parameter, we return the latest entry for
        // each vehicle only, sorted by VIN in ascending order
        if parameters.is_latest_only() {
            read_query = read_query.then("last()").then(format!(
                r#"sort(columns: ["{}"], desc: false)"#,
                influx_client::TAG_VIN
            ));
        } else {
            // if the client has not specified the latestOnly parameter, we return all snapshot data
            // for all vehicles, sorted by time and trigger in ascending order (least recent values first)
            read_query = read_query.then(format!(
                r#"sort(columns: ["_time", "{}"], desc: false)"#,
                influx_client::TAG_TRIGGER
            ));
        }
        read_query = read_query
            .then(super::PIVOT_ROW_KEY_TIME_TRIGGER)
            // apply the page size limit
            // we use page_size + 1 to allow the client to determine if
            // there are more results available
            .then(format!(
                "limit(n: {})",
                parameters.common.basic.page_size + 1
            ));

        debug!("Executing query for vehicle statuses: {}", read_query);

        self.influx_con
            .client
            .query(read_query)
            .await
            .map_err(|e| {
                error!("error retrieving vehicle statuses: {e}");
                FmsError::DataAccessError("vehicle statuses".to_string())
            })
            .map(|measurements| {
                measurements
                    .into_iter()
                    .filter_map(|entry| {
                        if let (
                            Some(vin),
                            Some(trigger),
                            Some(created_date_time),
                            Some(received_date_time),
                        ) = (
                            entry.get(influx_client::TAG_VIN),
                            entry.get(influx_client::TAG_TRIGGER),
                            super::unpack_unix_epoch_timestamp(
                                entry.get(influx_client::FIELD_CREATED_DATE_TIME),
                            ),
                            super::unpack_rfc3339_date_time(entry.get("_time")),
                        ) {
                            let snapshot_data = get_snapshot_data(&entry);

                            Some(VehicleStatusObject {
                                vin: vin.to_string(),
                                trigger_type: TriggerObject::new(
                                    trigger.to_string(),
                                    "RFMS".to_string(),
                                ),
                                created_date_time,
                                received_date_time,
                                hr_total_vehicle_distance: super::unpack_value_i64(
                                    entry.get(influx_client::FIELD_HR_TOTAL_VEHICLE_DISTANCE),
                                ),
                                total_engine_hours: super::unpack_value_f64(
                                    entry.get(influx_client::FIELD_TOTAL_ENGINE_HOURS),
                                ),
                                total_electric_motor_hours: super::unpack_value_f64(
                                    entry.get(influx_client::FIELD_TOTAL_ELECTRIC_MOTOR_HOURS),
                                ),
                                driver1_id: None,
                                gross_combination_vehicle_weight: super::unpack_value_i32(
                                    entry
                                        .get(influx_client::FIELD_GROSS_COMBINATION_VEHICLE_WEIGHT),
                                ),
                                engine_total_fuel_used: super::unpack_value_i64(
                                    entry.get(influx_client::FIELD_ENGINE_TOTAL_FUEL_USED),
                                ),
                                total_fuel_used_gaseous: None,
                                total_electric_energy_used: None,
                                status2_of_doors: None,
                                door_status: None,
                                accumulated_data: None,
                                snapshot_data: Some(snapshot_data),
                                uptime_data: None,
                            })
                        } else {
                            debug!("ignoring measurement lacking mandatory field(s)");
                            None
                        }
                    })
                    .collect()
            })
    }
}

fn get_snapshot_data(entry: &HashMap<String, String>) -> SnapshotDataObject {
    let gnss_position = match (
        super::unpack_unix_epoch_timestamp(entry.get(influx_client::FIELD_POSITION_DATE_TIME)),
        super::unpack_value_f64(entry.get(influx_client::FIELD_LONGITUDE)),
        super::unpack_value_f64(entry.get(influx_client::FIELD_LATITUDE)),
    ) {
        (Some(position_date_time), Some(longitude), Some(latitude)) => Some(GnssPositionObject {
            position_date_time,
            longitude,
            latitude,
            heading: super::unpack_value_i32(entry.get(influx_client::FIELD_HEADING)),
            altitude: super::unpack_value_i32(entry.get(influx_client::FIELD_ALTITUDE)),
            speed: super::unpack_value_f64(entry.get(influx_client::FIELD_SPEED)),
        }),
        _ => None,
    };

    SnapshotDataObject {
        gnss_position,
        wheel_based_speed: super::unpack_value_f64(
            entry.get(influx_client::FIELD_WHEEL_BASED_SPEED),
        ),
        tachograph_speed: super::unpack_value_f64(entry.get(influx_client::FIELD_TACHOGRAPH_SPEED)),
        engine_speed: super::unpack_value_f64(entry.get(influx_client::FIELD_ENGINE_SPEED)),
        electric_motor_speed: None,
        fuel_type: None,
        fuel_level1: super::unpack_value_f64(entry.get(influx_client::FIELD_FUEL_LEVEL1)),
        fuel_level2: super::unpack_value_f64(entry.get(influx_client::FIELD_FUEL_LEVEL2)),
        catalyst_fuel_level: super::unpack_value_f64(
            entry.get(influx_client::FIELD_CATALYST_FUEL_LEVEL),
        ),
        driver1_working_state: unpack_driver_working_state(
            entry.get(influx_client::FIELD_DRIVER1_WORKING_STATE),
        ),
        driver2_id: None,
        driver2_working_state: unpack_driver_working_state(
            entry.get(influx_client::FIELD_DRIVER2_WORKING_STATE),
        ),
        ambient_air_temperature: super::unpack_value_f64(
            entry.get(influx_client::FIELD_AMBIENT_AIR_TEMP),
        ),
        parking_brake_switch: super::unpack_value_bool(
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
        // extra fields not defined by rFMS
        direction_indicator_left: super::unpack_value_bool(
            entry.get(influx_client::FIELD_DIRECTION_INDICATOR_LEFT),
        ),
        direction_indicator_right: super::unpack_value_bool(
            entry.get(influx_client::FIELD_DIRECTION_INDICATOR_RIGHT),
        ),
        brake_light_status: entry.get(influx_client::FIELD_BRAKE_LIGHT_STATUS).cloned(),
    }
}

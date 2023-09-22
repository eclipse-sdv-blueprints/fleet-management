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
use clap::ArgMatches;
use const_format::formatcp;
use influx_client::connection::InfluxConnection;
use influxrs::InfluxError;

use crate::models::{self, GnssPositionObject, TriggerObject, VehiclePositionObject};

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
const FILTER_TAG_ANY_VIN: &str = formatcp!(r#"filter(fn: (r) => r["{}"] =~ /.*/)"#, influx_client::TAG_VIN);
const FILTER_TAG_ANY_TRIGGER: &str = formatcp!(r#"filter(fn: (r) => r["{}"] =~ /.*/)"#, influx_client::TAG_TRIGGER);

fn unpack_value_i32(value: Option<&String>) -> Option<i32> {
    value.and_then(|v| v.parse().ok())
}

fn unpack_value_f64(value: Option<&String>) -> Option<f64> {
    value.and_then(|v| v.parse().ok())
}

fn unpack_time(value: Option<&String>) -> Option<DateTime<Utc>> {
    value.and_then(|v| v.parse().ok())
}

pub struct InfluxReader {
    influx_con: InfluxConnection,
}

impl InfluxReader {

    pub fn new(args: &ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {
        InfluxConnection::new(args).map(|con| InfluxReader { influx_con: con })
    }

    pub async fn get_vehicles(&self) -> Result<Vec<models::VehicleObject>, InfluxError> {
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
                        .map(|vin| models::VehicleObject::new(vin.to_string()))
                })
                .collect()
        })
    }

    pub async fn get_vehicleposition(
        &self,
        start_time: i64,
        stop_time: i64,
        vin: Option<&String>,
        trigger: Option<&String>,
        latest_only: bool,
    ) -> Result<Vec<models::VehiclePositionObject>, InfluxError> {
        // Build Query
        let time_filter = format!("range(start: {}, stop: {})", start_time, stop_time);
        let vin_filter = match vin {
            Some(v) => format!(r#"filter(fn: (r) => r["{}"] == "{}""#, influx_client::TAG_VIN, v),
            None => FILTER_TAG_ANY_VIN.to_string(),
        };
        let trigger_filter = match trigger {
            Some(t) => format!(r#"filter(fn: (r) => r["{}"] == "{}")"#, influx_client::TAG_TRIGGER, t),
            None => FILTER_TAG_ANY_TRIGGER.to_string(),
        };

        let mut read_query = influxrs::Query::new(format!(r#"from(bucket: "{}")"#, self.influx_con.bucket))
            .then(time_filter)
            .then(FILTER_MEASUREMENT_SNAPSHOT)
            .then(vin_filter)
            .then(trigger_filter)
            .then(FILTER_FIELDS_POSITION);
        if latest_only {
            read_query = read_query.then("last()");
        }
        read_query = read_query
            .then(r#"pivot(rowKey: ["_time"], columnKey: ["_field"], valueColumn: "_value")"#);

        self.influx_con.client.query(read_query).await.map(|measurements| {
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
                                        heading: unpack_value_i32(entry.get(influx_client::FIELD_HEADING)),
                                        altitude: unpack_value_i32(entry.get(influx_client::FIELD_ALTITUDE)),
                                        speed: unpack_value_f64(entry.get(influx_client::FIELD_SPEED)),
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
                                wheel_based_speed: unpack_value_f64(entry.get(influx_client::FIELD_WHEEL_BASED_SPEED)),
                                tachograph_speed: unpack_value_f64(entry.get(influx_client::FIELD_TACHOGRAPH_SPEED)),
                            })
                        }
                        _ => None,
                    }
                })
                .collect()
        })
    }
}

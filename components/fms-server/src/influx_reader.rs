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
use influx_client::connection::{InfluxConnection, InfluxConnectionConfig};

use crate::data_access::DataAccess;

mod get_vehicle_positions;
mod get_vehicle_statuses;
mod get_vehicles;

const PIVOT_ROW_KEY_TIME_VIN_TRIGGER: &str = const_format::formatcp!(
    r#"pivot(rowKey: ["_time", "{}", "{}"], columnKey: ["_field"], valueColumn: "_value")"#,
    influx_client::TAG_VIN,
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

fn unpack_unix_epoch_timestamp(value: Option<&String>) -> Option<DateTime<Utc>> {
    let timestamp = unpack_value_i64(value)?;
    DateTime::from_timestamp_millis(timestamp)
}

fn unpack_rfc3339_date_time(value: Option<&String>) -> Option<DateTime<Utc>> {
    value.and_then(|v| {
        DateTime::parse_from_rfc3339(v)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    })
}

pub struct InfluxReader {
    influx_con: InfluxConnection,
}

impl InfluxReader {
    pub fn new(args: &InfluxConnectionConfig) -> Result<Self, Box<dyn std::error::Error>> {
        InfluxConnection::new(args).map(|con| InfluxReader { influx_con: con })
    }
}

#[async_trait::async_trait]
impl DataAccess for InfluxReader {}

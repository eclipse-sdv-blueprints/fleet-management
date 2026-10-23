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

use const_format::formatcp;
use log::error;

use crate::{
    data_access::{GetVehiclePositions, PositionsQueryParameters},
    models::{
        error::FmsError,
        position::{GnssPositionObject, VehiclePositionObject},
        TriggerObject,
    },
};

const FILTER_FIELDS_POSITION: &str = formatcp!(
    r#"filter(fn: (r) => r._field == "{}" or r._field == "{}" or r._field == "{}" or r._field == "{}" or r._field == "{}" or r._field == "{}" or r._field == "{}" or r._field == "{}" or r._field == "{}")"#,
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

impl PositionsQueryParameters {
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
        if let Some(trigger) = self.trigger_filter.as_ref() {
            return Some(format!(
                r#"filter(fn: (r) => r["{}"] == "{}")"#,
                influx_client::TAG_TRIGGER,
                trigger
            ));
        }
        None
    }

    pub fn is_latest_only(&self) -> bool {
        self.common.latest_only.is_some_and(|v| v)
    }
}

#[async_trait::async_trait]
impl GetVehiclePositions for super::InfluxReader {
    async fn get_vehiclepositions(
        &self,
        parameters: &PositionsQueryParameters,
    ) -> Result<Vec<VehiclePositionObject>, FmsError> {
        // Build Query
        let mut read_query =
            influxrs::Query::new(format!(r#"from(bucket: "{}")"#, self.influx_con.bucket))
                .then(parameters.range_filter())
                .then(FILTER_MEASUREMENT_SNAPSHOT);

        if let Some(vin_filter) = parameters.vin_filter() {
            read_query = read_query.then(vin_filter);
        }

        if let Some(trigger_filter) = parameters.trigger_filter() {
            read_query = read_query.then(trigger_filter);
        }

        read_query = read_query.then(FILTER_FIELDS_POSITION).then(format!(
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
            // if the client has not specified the latestOnly parameter, we return all position data
            // for all vehicles, sorted by time and trigger in ascending order (least recent values first)
            read_query = read_query.then(format!(
                r#"sort(columns: ["_time", "{}"], desc: false)"#,
                influx_client::TAG_TRIGGER
            ));
        }
        read_query = read_query.then(super::PIVOT_ROW_KEY_TIME_TRIGGER);
        // apply the page size limit
        // we use page_size + 1 to allow the client to determine if
        // there are more results available
        read_query = read_query.then(format!(
            "limit(n: {})",
            parameters.common.basic.page_size + 1
        ));

        self.influx_con
            .client
            .query(read_query)
            .await
            .map_err(|e| {
                error!("error retrieving vehicle positions: {e}");
                FmsError::DataAccessError("vehicle positions".to_string())
            })
            .map(|measurements| {
                measurements
                    .into_iter()
                    .filter_map(|entry| {
                        match (
                            entry.get(influx_client::TAG_VIN),
                            entry.get(influx_client::TAG_TRIGGER),
                            super::unpack_unix_epoch_timestamp(
                                entry.get(influx_client::FIELD_CREATED_DATE_TIME),
                            ),
                            super::unpack_rfc3339_date_time(entry.get("_time")),
                        ) {
                            (
                                Some(vin),
                                Some(trigger),
                                Some(created_date_time),
                                Some(received_date_time),
                            ) => {
                                let gnss_position = match (
                                    super::unpack_unix_epoch_timestamp(
                                        entry.get(influx_client::FIELD_POSITION_DATE_TIME),
                                    ),
                                    super::unpack_value_f64(
                                        entry.get(influx_client::FIELD_LONGITUDE),
                                    ),
                                    super::unpack_value_f64(
                                        entry.get(influx_client::FIELD_LATITUDE),
                                    ),
                                ) {
                                    (Some(position_date_time), Some(longitude), Some(latitude)) => {
                                        Some(GnssPositionObject {
                                            latitude,
                                            longitude,
                                            heading: super::unpack_value_i32(
                                                entry.get(influx_client::FIELD_HEADING),
                                            ),
                                            altitude: super::unpack_value_i32(
                                                entry.get(influx_client::FIELD_ALTITUDE),
                                            ),
                                            speed: super::unpack_value_f64(
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
                                    received_date_time,
                                    gnss_position,
                                    wheel_based_speed: super::unpack_value_f64(
                                        entry.get(influx_client::FIELD_WHEEL_BASED_SPEED),
                                    ),
                                    tachograph_speed: super::unpack_value_f64(
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
}

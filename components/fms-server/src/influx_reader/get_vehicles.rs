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
    data_access::{GetVehicles, VehiclesQueryParameters},
    models::{error::FmsError, vehicle::VehicleObject},
};

const FILTER_MEASUREMENT_METADATA: &str = formatcp!(
    r#"filter(fn: (r) => r._measurement == "{}")"#,
    influx_client::MEASUREMENT_METADATA,
);

impl VehiclesQueryParameters {
    pub fn vin_filter(&self) -> Option<String> {
        self.last_vin.as_ref().map(|vin| {
            format!(
                r#"filter(fn: (r) => r["{}"] > "{}")"#,
                influx_client::TAG_VIN,
                vin
            )
        })
    }
}

#[async_trait::async_trait]
impl GetVehicles for super::InfluxReader {
    async fn get_vehicles(
        &self,
        query_params: &VehiclesQueryParameters,
    ) -> Result<Vec<VehicleObject>, FmsError> {
        let mut read_query =
            influxrs::Query::new(format!(r#"from(bucket: "{}")"#, self.influx_con.bucket))
                .then(r#"range(start: -1y)"#)
                .then(FILTER_MEASUREMENT_METADATA);

        // apply pagination
        if let Some(vin_filter) = query_params.vin_filter() {
            read_query = read_query.then(vin_filter);
        }

        read_query = read_query
            .then(format!(
                r#"filter(fn: (r) => contains(value: r._field, set: ["{}", "{}"]))"#,
                influx_client::FIELD_METADATA_BRAND,
                influx_client::FIELD_METADATA_MODEL
            ))
            .then(format!(
                r#"group(columns: ["_field", "{}"])"#,
                influx_client::TAG_VIN
            ))
            .then(r#"last()"#)
            .then(format!(r#"sort(columns: ["{}"])"#, influx_client::TAG_VIN))
            .then(super::PIVOT_ROW_KEY_TIME_TRIGGER)
            // apply the page size limit
            // we use page_size + 1 to allow the client to determine if
            // there are more results available
            .then(format!("limit(n: {})", query_params.page_size + 1));

        self.influx_con
            .client
            .query(read_query)
            .await
            .map_err(|e| {
                error!("error retrieving vehicles: {e}");
                FmsError::DataAccessError("vehicles".to_string())
            })
            .map(|vins| {
                vins.into_iter()
                    .filter_map(|row| {
                        let vin = row.get(influx_client::TAG_VIN)?;
                        Some(VehicleObject::for_metadata(
                            vin.to_owned(),
                            row.get(influx_client::FIELD_METADATA_MODEL)
                                .map(|v| v.to_string()),
                            row.get(influx_client::FIELD_METADATA_BRAND)
                                .map(|v| v.to_string()),
                        ))
                    })
                    .collect()
            })
    }
}

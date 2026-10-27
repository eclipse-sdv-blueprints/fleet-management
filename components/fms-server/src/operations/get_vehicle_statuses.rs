// SPDX-FileCopyrightText: 2026 Contributors to the Eclipse Foundation
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

use std::sync::Arc;

use axum::{
    extract::{OriginalUri, State},
    http::{HeaderMap, HeaderValue},
    Json,
};
use axum_extra::extract::{Query, WithRejection};

use chrono::{DateTime, TimeDelta, Utc};
use log::debug;
use serde_qs::Config;

use crate::{
    data_access::{DataAccess, DateType, StatusesQueryParameters},
    models::{
        error::FmsError,
        status::{VehicleStatusResponseObject, VehicleStatusResponseObjectVehicleStatusResponse},
    },
};

const CONTENT_TYPE_VEHICLESTATUSES: HeaderValue =
    HeaderValue::from_static("application/json; rfms=vehiclestatuses.v4.0");

impl StatusesQueryParameters {
    pub fn validate(&self) -> Result<(), FmsError> {
        self.common.validate_vin_param()?;
        self.common.basic.validate_last_vin_param()?;
        for trigger in &self.trigger_filter {
            crate::data_access::validate_trigger(trigger)?;
        }

        if self.common.date_type == DateType::Created {
            return Err(FmsError::ParameterValidationError(
                "server does not support 'created' datetype (yet)".to_string(),
            ));
        }

        if self.common.start_time.is_none() && self.common.latest_only.is_none() {
            return Err(FmsError::ParameterValidationError(
                "either starttime or latestOnly must be supplied".to_string(),
            ));
        }

        if self.common.start_time.is_some() && self.common.latest_only.is_some() {
            return Err(FmsError::ParameterValidationError(
                "cannot set both starttime and latestOnly simultaneously".to_string(),
            ));
        }

        if self.common.start_time.is_none() && self.common.stop_time.is_some() {
            return Err(FmsError::ParameterValidationError(
                "stoptime cannot be specified without starttime".to_string(),
            ));
        }

        if self.common.vin.is_some() && self.common.basic.last_vin.is_some() {
            return Err(FmsError::ParameterValidationError(
                "cannot set both vin and lastVin simultaneously".to_string(),
            ));
        }

        // when a client wants to scroll through historical data,
        // the starttime param is used to delimit the pages
        // otherwise, the lastVin parameter is used to paginate through the
        // (latest only) results
        if self.common.start_time.is_some() && self.common.basic.last_vin.is_some() {
            return Err(FmsError::ParameterValidationError(
                "lastVin cannot be specified with starttime".to_string(),
            ));
        }

        if self
            .content_filter
            .contains(&crate::data_access::StatusContentType::Accumulated)
            || self
                .content_filter
                .contains(&crate::data_access::StatusContentType::Uptime)
        {
            return Err(FmsError::ParameterValidationError(
                "server (currently) supports SNAPSHOT contentFilter only".to_string(),
            ));
        }
        Ok(())
    }
}

fn next_statuses_page_query_params(
    request_params: &StatusesQueryParameters,
    last_received_date_time_from_result_set: &DateTime<Utc>,
    last_vin_from_result_set: &str,
) -> Result<String, FmsError> {
    let mut new_params = request_params.clone();
    if request_params.common.latest_only.is_some_and(|v| v) {
        // use last VIN to mark next page start
        new_params.common.basic.last_vin = Some(last_vin_from_result_set.to_string());
    } else {
        // client wants to scroll through historical data
        // new starttime according to OpenAPI specification for pagination is
        // last received data + 1 second
        let new_start_time = last_received_date_time_from_result_set
            .checked_add_signed(TimeDelta::seconds(1))
            .ok_or_else(|| {
                FmsError::ParameterValidationError(
                    "Failed to create new startime for pagination".to_string(),
                )
            })?;
        new_params.common.start_time = Some(new_start_time);
    }
    Config::new()
        .array_format(serde_qs::ArrayFormat::Unindexed)
        .serialize_string(&new_params)
        .map_err(|_e| {
            FmsError::ParameterValidationError("error encoding query parameters".to_string())
        })
}

pub(crate) async fn get_vehicle_statuses(
    State(influx_server): State<Arc<dyn DataAccess>>,
    WithRejection(Query(query_parameters), _): WithRejection<
        Query<StatusesQueryParameters>,
        FmsError,
    >,
    OriginalUri(uri): OriginalUri,
) -> Result<(HeaderMap, Json<VehicleStatusResponseObject>), FmsError> {
    debug!(
        "get_vehicle_statuses called with query_params: {:?}",
        query_parameters
    );

    query_parameters.validate()?;
    let request_server_date_time = Utc::now();

    influx_server
        .get_vehiclestatuses(&query_parameters)
        .await
        .and_then(|mut vehicles_statuses| {
            let more_data_available =
                vehicles_statuses.len() > query_parameters.common.basic.page_size;
            // trim the result set to PAGE_SIZE if more data is available
            vehicles_statuses.truncate(query_parameters.common.basic.page_size);
            let more_data_available_link = if more_data_available {
                let query = next_statuses_page_query_params(
                    &query_parameters,
                    &vehicles_statuses.last().unwrap().received_date_time,
                    &vehicles_statuses.last().unwrap().vin,
                )?;
                Some(format!("{}?{}", uri.path(), query))
            } else {
                None
            };
            let vehicle_status_response = VehicleStatusResponseObjectVehicleStatusResponse {
                vehicle_statuses: Some(vehicles_statuses),
            };

            let result_object = VehicleStatusResponseObject {
                vehicle_status_response,
                more_data_available,
                more_data_available_link,
                request_server_date_time,
            };
            let mut headers = HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                CONTENT_TYPE_VEHICLESTATUSES,
            );
            Ok((headers, Json(result_object)))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_next_statuses_page_query_params_uses_non_indexed_arrays() {
        let request_params = StatusesQueryParameters {
            common: Default::default(),
            trigger_filter: vec!["ENGINE_ON".to_string(), "ENGINE_OFF".to_string()],
            content_filter: vec![],
        };
        let last_received_date_time_from_result_set = &Utc::now();
        let last_vin_from_result_set = "VIN123";

        let result = next_statuses_page_query_params(
            &request_params,
            last_received_date_time_from_result_set,
            last_vin_from_result_set,
        );
        assert!(result.is_ok_and(|query| {
            query.contains("triggerFilter=ENGINE_ON") && query.contains("triggerFilter=ENGINE_OFF")
        }));
    }

    #[tokio::test]
    async fn test_next_statuses_page_query_params_creates_new_starttime() {
        let last_received_date_time_from_result_set =
            DateTime::<Utc>::from(DateTime::parse_from_rfc3339("2024-06-05T12:34:56Z").unwrap());
        let last_vin_from_result_set = "VIN123";

        let request_params = StatusesQueryParameters {
            common: Default::default(),
            trigger_filter: vec![],
            content_filter: vec![],
        };

        let result = next_statuses_page_query_params(
            &request_params,
            &last_received_date_time_from_result_set,
            last_vin_from_result_set,
        );
        assert!(result.is_ok_and(|query| { query.contains("starttime=2024-06-05T12:34:57Z") }));
    }
}

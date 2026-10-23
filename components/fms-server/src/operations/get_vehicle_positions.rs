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
use chrono::{DateTime, Utc};
use log::debug;
use serde_qs::Config;

use crate::{
    data_access::{DataAccess, DateType, PositionsQueryParameters},
    models::{
        error::FmsError,
        position::{
            VehiclePositionResponseObject, VehiclePositionResponseObjectVehiclePositionResponse,
        },
    },
};

const CONTENT_TYPE_VEHICLEPOSITIONS: HeaderValue =
    HeaderValue::from_static("application/json; rfms=vehiclepositions.v4.0");

impl PositionsQueryParameters {
    pub fn validate(&self) -> Result<(), FmsError> {
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
        Ok(())
    }
}

fn next_positions_page_query_params(
    request_params: &PositionsQueryParameters,
    most_recent_received_date_time: &DateTime<Utc>,
    last_vin_from_result_set: &str,
) -> Result<String, FmsError> {
    let mut new_params = request_params.clone();

    if request_params.common.latest_only.is_some() {
        // use last VIN to mark next page start
        new_params.common.basic.last_vin = Some(last_vin_from_result_set.to_string());
    } else {
        // client wants to scroll through historical data
        new_params.common.start_time = Some(*most_recent_received_date_time);
    }
    Config::new()
        .array_format(serde_qs::ArrayFormat::Unindexed)
        .serialize_string(&new_params)
        .map_err(|_e| {
            FmsError::ParameterValidationError("error encoding query parameters".to_string())
        })
}

pub(crate) async fn get_vehicle_positions(
    State(influx_server): State<Arc<dyn DataAccess>>,
    WithRejection(Query(query_parameters), _): WithRejection<
        Query<PositionsQueryParameters>,
        FmsError,
    >,
    OriginalUri(uri): OriginalUri,
) -> Result<(HeaderMap, Json<VehiclePositionResponseObject>), FmsError> {
    debug!(
        "get_vehicle_positions called with query_params: {:?}",
        query_parameters
    );

    query_parameters.validate()?;
    let request_server_date_time = chrono::Utc::now();

    influx_server
        .get_vehiclepositions(&query_parameters)
        .await
        .and_then(|mut positions| {
            let more_data_available = positions.len() > query_parameters.common.basic.page_size;
            // trim the positions to PAGE_SIZE if more data is available
            positions.truncate(query_parameters.common.basic.page_size);
            let more_data_available_link = if more_data_available {
                let query = next_positions_page_query_params(
                    &query_parameters,
                    &positions.last().unwrap().received_date_time,
                    &positions.last().unwrap().vin,
                )?;
                Some(format!("{}?{}", uri.path(), query))
            } else {
                None
            };
            let result = VehiclePositionResponseObject {
                vehicle_position_response: VehiclePositionResponseObjectVehiclePositionResponse {
                    vehicle_positions: Some(positions),
                },
                more_data_available,
                more_data_available_link,
                request_server_date_time,
            };

            let mut headers = HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                CONTENT_TYPE_VEHICLEPOSITIONS,
            );
            Ok((headers, Json(result)))
        })
}

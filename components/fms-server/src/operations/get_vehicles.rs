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
use log::debug;

use crate::{
    data_access::{DataAccess, VehiclesQueryParameters},
    models::{
        error::FmsError,
        vehicle::{VehicleResponseObject, VehicleResponseObjectVehicleResponse},
    },
};

const CONTENT_TYPE_VEHICLES: HeaderValue =
    HeaderValue::from_static("application/json; rfms=vehicles.v4.0");

impl VehiclesQueryParameters {
    pub fn validate(&self) -> Result<(), FmsError> {
        self.validate_last_vin_param()
    }
}

pub(crate) async fn get_vehicles(
    State(data_access): State<Arc<dyn DataAccess>>,
    WithRejection(Query(query_params), _): WithRejection<Query<VehiclesQueryParameters>, FmsError>,
    OriginalUri(uri): OriginalUri,
) -> Result<(HeaderMap, Json<VehicleResponseObject>), FmsError> {
    query_params.validate()?;
    debug!("get_vehicles called with query_params: {:?}", query_params);

    data_access
        .get_vehicles(&query_params)
        .await
        .map(|mut vehicles| {
            let more_data_available = vehicles.len() > query_params.page_size;
            // trim the vehicles to page_size if more data is available
            vehicles.truncate(query_params.page_size);
            let more_data_available_link = if more_data_available {
                Some(format!(
                    "{}?lastVin={}",
                    uri.path(),
                    vehicles.last().unwrap().vin
                ))
            } else {
                None
            };
            let result_object = VehicleResponseObject {
                vehicle_response: VehicleResponseObjectVehicleResponse {
                    vehicles: Some(vehicles),
                },
                more_data_available,
                more_data_available_link,
            };
            let mut headers = HeaderMap::new();
            headers.insert(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_VEHICLES);
            (headers, Json(result_object))
        })
}

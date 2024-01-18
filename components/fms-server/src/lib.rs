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

use axum::http::StatusCode;
use axum::{routing::get, Json, Router};

use axum::extract::{Query, State};
use clap::{ArgMatches, Command};
use log::{error, info};

use serde_json::json;
use std::collections::HashMap;
use std::process;
use std::sync::Arc;

use chrono::Utc;

use influx_reader::InfluxReader;

mod influx_reader;
mod models;
mod query_parser;

use models::position::{
    VehiclePositionResponseObject, VehiclePositionResponseObjectVehiclePositionResponse,
};
use models::status::{
    VehicleStatusResponseObject, VehicleStatusResponseObjectVehicleStatusResponse,
};
use query_parser::parse_query_parameters;

pub fn app(args: ArgMatches) -> Router {
    let influx_reader = InfluxReader::new(&args).map_or_else(
        |e| {
            error!("failed to create InfluxDB client: {e}");
            process::exit(1);
        },
        Arc::new,
    );
    info!("starting rFMS server");
    Router::new()
        .route("/", get(root))
        .route("/rfms/vehiclepositions", get(get_vehicleposition))
        .route("/rfms/vehicles", get(get_vehicles))
        .route("/rfms/vehiclestatuses", get(get_vehiclesstatuses))
        .with_state(influx_reader)
}

pub fn add_command_line_args(command_line: Command) -> Command {
    influx_client::connection::add_command_line_args(command_line)
}

async fn root() -> &'static str {
    "Welcome to the rFMS server. The following endpoints are implemented: '/rfms/vehicleposition', '/rfms/vehicles', and '/rfms/vehiclestatuses'"
}

async fn get_vehicleposition(
    State(influx_server): State<Arc<InfluxReader>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let query_parameters = parse_query_parameters(&params)?;

    influx_server
        .get_vehicleposition(&query_parameters)
        .await
        .map(|positions| {
            let result = json!(VehiclePositionResponseObject {
                vehicle_position_response: VehiclePositionResponseObjectVehiclePositionResponse {
                    vehicle_positions: Some(positions)
                },
                more_data_available: false,
                more_data_available_link: None,
                request_server_date_time: chrono::Utc::now()
            });

            Json(result)
        })
        .map_err(|e| {
            error!("error retrieving vehicle positions: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn get_vehicles(
    State(influx_server): State<Arc<InfluxReader>>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    influx_server
        .get_vehicles()
        .await
        .map(|vehicles| {
            let response = models::vehicle::VehicleResponseObjectVehicleResponse {
                vehicles: Some(vehicles),
            };

            let result_object = json!(models::vehicle::VehicleResponseObject {
                vehicle_response: response,
                more_data_available: false,
                more_data_available_link: None,
            });
            Json(result_object)
        })
        .map_err(|e| {
            error!("error retrieving vehicles: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn get_vehiclesstatuses(
    State(influx_server): State<Arc<InfluxReader>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let query_parameters = parse_query_parameters(&params)?;
    influx_server
        .get_vehiclesstatuses(&query_parameters)
        .await
        .map(|vehicles_statuses| {
            let response = VehicleStatusResponseObjectVehicleStatusResponse {
                vehicle_statuses: Some(vehicles_statuses),
            };

            //TODO for request_server_date_time
            // put in start time used in influx query instead of now
            let result_object = json!(VehicleStatusResponseObject {
                vehicle_status_response: response,
                more_data_available: false,
                more_data_available_link: None,
                request_server_date_time: Utc::now()
            });
            Json(result_object)
        })
        .map_err(|e| {
            error!("error retrieving vehicle statuses: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

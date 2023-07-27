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
use clap::Command;
use log::{error, info};

use serde_json;
use serde_json::json;
use std::collections::HashMap;
use std::process;
use std::str::FromStr;
use std::sync::Arc;

use chrono::{DateTime, Utc};

use influx_reader::InfluxReader;

mod influx_reader;
mod models;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut parser = Command::new("rFMS server")
        .about("Exposes data from an InfluxDB via an rFMS API endpoint.");
    parser = influx_client::connection::add_command_line_args(parser);
    let args = parser.get_matches();
    let influx_reader = InfluxReader::new(&args).map_or_else(
        |e| {
            error!("failed to create InfluxDB client: {e}");
            process::exit(1);
        },
        Arc::new,
    );
    info!("starting rFMS server");
    let app = Router::new()
        .route("/", get(root))
        .route("/rfms/vehicleposition", get(get_vehicleposition))
        .route("/rfms/vehicles", get(get_vehicles))
        .with_state(influx_reader);
    axum::Server::bind(&"0.0.0.0:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Welcome to the rFMS server. The following endpoints are implemented: '/rfms/vehicleposition and /rfms/vehicles'"
}

async fn get_vehicleposition(
    State(influx_server): State<Arc<InfluxReader>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let start_time = params.get("starttime").map_or(0, |text| {
        DateTime::<Utc>::from_str(text).map_or(0, |time| time.timestamp())
    });

    let stop_time = params
        .get("stoptime")
        .map_or_else(|| Utc::now().timestamp(), |text| {
            DateTime::<Utc>::from_str(text).map_or_else(|_| Utc::now().timestamp(), |time| time.timestamp())
        });

    let vin = params.get("vin");
    let trigger_filter = params.get("triggerFilter");
    let latest_only = params
        .get("latestOnly")
        .map_or(false, |text| text.parse().unwrap_or(false));

    influx_server
        .get_vehicleposition(start_time, stop_time, vin, trigger_filter, latest_only)
        .await
        .map(|positions| {
            let result = json!(models::VehiclePositionResponseObject {
                vehicle_position_response:
                    models::VehiclePositionResponseObjectVehiclePositionResponse {
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
            let response = models::VehicleResponseObjectVehicleResponse {
                vehicles: Some(vehicles),
            };

            let result_object = json!(models::VehicleResponseObject {
                vehicle_response: response,
                more_data_available: false,
                more_data_available_link: None,
            });
            Json(result_object)
        })
        .map_err(|e| {
            error!("error retrieving vehicle status: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

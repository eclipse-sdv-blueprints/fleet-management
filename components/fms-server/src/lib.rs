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

use axum::{routing::get, Router};

use influx_client::connection::InfluxConnectionConfig;
use log::{error, info};

use std::sync::Arc;

mod data_access;
mod influx_reader;
mod models;
mod operations;

use crate::data_access::DataAccess;

pub fn app(
    influx_connection_params: &InfluxConnectionConfig,
) -> Result<Router, Box<dyn std::error::Error>> {
    let data_access: Arc<dyn DataAccess> =
        influx_reader::InfluxReader::new(influx_connection_params)
            .inspect_err(|e| error!("failed to create InfluxDB client: {e}"))
            .map(Arc::new)?;

    info!("starting rFMS server");

    Ok(Router::new()
        .route("/", get(root))
        .route("/rfms/vehicles", get(operations::get_vehicles))
        .route(
            "/rfms/vehiclepositions",
            get(operations::get_vehicle_positions),
        )
        .route(
            "/rfms/vehiclestatuses",
            get(operations::get_vehicle_statuses),
        )
        .with_state(data_access))
}

async fn root() -> &'static str {
    "Welcome to the rFMS 4.0 server."
}

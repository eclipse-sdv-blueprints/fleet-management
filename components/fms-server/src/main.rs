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

use clap::Parser;
use influx_client::connection::InfluxConnectionConfig;

/// Exposes data from an InfluxDB via an rFMS API endpoint.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct FmsServerArgs {
    #[command(flatten)]
    influx_connection: InfluxConnectionConfig,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // let mut parser = Command::new("rFMS server")
    //     .about("Exposes data from an InfluxDB via an rFMS API endpoint.");
    // parser = fms_server::add_command_line_args(parser);
    // let args = parser.get_matches();
    let args = FmsServerArgs::parse();
    let router = fms_server::app(&args.influx_connection);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

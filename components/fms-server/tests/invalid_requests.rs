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
use clap::Command;
use axum_test::TestServer;

 #[tokio::test]
 async fn vehiclestatuses_with_no_latestonly_or_time_returns_400() {
    let server = init_server();
    let response = server.get("/rfms/vehiclestatuses").await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclestatuses_with_query_but_no_latestonly_or_time_returns_400() {
    let server = init_server();
    let response = server
        .get("/rfms/vehiclestatuses")
        .add_query_param("vin", "1234")
        .await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclestatuses_with_latestonly_and_starttime_returns_400() {
    let server = init_server();
    let response = server
        .get("/rfms/vehiclestatuses")
        .add_query_params(&[
            ("latestOnly", "true"),
            ("stoptime", "2022-12-15T15:12:00Z"),
        ])
        .await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclestatuses_with_latestonly_but_not_bool_returns_400() {
    let server = init_server();
    let response = server
        .get("/rfms/vehiclestatuses")
        .add_query_param("latestOnly", "123")
        .await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclestatuses_with_startime_but_not_timestamp_returns_400() {
    let server = init_server();
    let response = server
        .get("/rfms/vehiclestatuses")
        .add_query_param("starttime", "Random")
        .await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclestatuses_with_latestonly_and_stoptime_returns_400() {
    let server = init_server();
    let response = server
        .get("/rfms/vehiclestatuses")
        .add_query_params(&[
            ("latestOnly", "false"),
            ("starttime", "2022-12-15T15:12:00Z"),
        ])
        .await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclepositions_with_no_latestonly_or_time_returns_400() {
    let server = init_server();
    let response = server.get("/rfms/vehiclepositions").await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclepositions_with_query_but_no_latestonly_or_time_returns_400() {
    let server = init_server();
    let response = server
        .get("/rfms/vehiclepositions")
        .add_query_param("vin", "1234")
        .await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclepositions_with_latestonly_and_starttime_returns_400() {
    let server = init_server();
    let response = server
        .get("/rfms/vehiclepositions")
        .add_query_params(&[
            ("latestOnly", "true"),
            ("starttime", "2022-12-15T15:12:00Z"),
        ])
        .await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

 #[tokio::test]
 async fn vehiclepositions_with_latestonly_and_stoptime_returns_400() {
    let server = init_server();
    let response = server
    .get("/rfms/vehiclepositions")
    .add_query_params(&[
        ("latestOnly", "false"),
        ("stoptime", "2022-12-15T15:12:00Z"),
    ])
    .await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status_code());
 }

fn init_server() -> TestServer {
    let mut parser = Command::new("rFMS server")
        .about("Exposes data from an InfluxDB via an rFMS API endpoint.");
    parser = fms_server::add_command_line_args(parser);
    let args = parser.get_matches_from(vec![
        "fms_server",
        "--influxdb-uri", "http://influx.io",
        "--influxdb-token", "the-token",
        ]); 

    let app = fms_server::app(args);
    TestServer::new(app).expect("Creating test server failed")
 }
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

use std::process;

use clap::Command;
use fms_proto::fms::VehicleStatus;
use hono_publisher::HonoPublisher;
use influx_client::writer::InfluxWriter;
use log::{error, info};
use status_publishing::StatusPublisher;
use tokio::sync::mpsc;
use zenoh_publisher::ZenohPublisher;

mod hono_publisher;
mod mqtt_connection;
mod status_publishing;
mod vehicle_abstraction;
mod zenoh_publisher;

const SUBCOMMAND_HONO: &str = "hono";
const SUBCOMMAND_INFLUX: &str = "influx";
const SUBCOMMAND_ZENOH: &str = "zenoh";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let version = option_env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT")
        .unwrap_or(option_env!("VERGEN_GIT_SHA").unwrap_or("unknown"));

    let mut parser = Command::new("fms-forwarder")
        .arg_required_else_help(true)
        .version(version)
        .about("Forwards FMS related VSS data points to a back end system");
    parser = vehicle_abstraction::add_command_line_args(parser);
    parser = parser
        .subcommand_required(true)
        .subcommand(hono_publisher::add_command_line_args(
            Command::new(SUBCOMMAND_HONO).about("Forwards VSS data to Hono's MQTT adapter"),
        ))
        .subcommand(influx_client::connection::add_command_line_args(
            Command::new(SUBCOMMAND_INFLUX).about("Forwards VSS data to an Influx DB server"),
        ))
        .subcommand(zenoh_publisher::add_command_line_args(
            Command::new(SUBCOMMAND_ZENOH).about("Forwards VSS data to Zenoh"),
        ));

    let args = parser.get_matches();

    let publisher: Box<dyn StatusPublisher> = match args.subcommand_name() {
        Some(SUBCOMMAND_HONO) => {
            let hono_args = args.subcommand_matches(SUBCOMMAND_HONO).unwrap();
            match HonoPublisher::new(hono_args).await {
                Ok(writer) => Box::new(writer),
                Err(e) => {
                    error!("failed to create Hono publisher: {}", e);
                    process::exit(1);
                }
            }
        }
        Some(SUBCOMMAND_INFLUX) => {
            let influx_args = args.subcommand_matches(SUBCOMMAND_INFLUX).unwrap();
            match InfluxWriter::new(influx_args) {
                Ok(writer) => Box::new(writer),
                Err(e) => {
                    error!("failed to create InfluxDB writer: {e}");
                    process::exit(1);
                }
            }
        }
        Some(SUBCOMMAND_ZENOH) => {
            let zenoh_args = args.subcommand_matches(SUBCOMMAND_ZENOH).unwrap();
            match ZenohPublisher::new(zenoh_args).await {
                Ok(writer) => Box::new(writer),
                Err(e) => {
                    error!("failed to create Zenoh Publisher: {e}");
                    process::exit(1);
                }
            }
        }
        Some(_) => {
            // cannot happen because subcommand is required
            process::exit(1);
        }
        None => {
            // cannot happen because subcommand is required
            process::exit(1);
        }
    };

    info!("starting FMS forwarder");

    let (tx, mut rx) = mpsc::channel::<VehicleStatus>(30);
    vehicle_abstraction::init(&args, tx).await?;

    while let Some(vehicle_status) = rx.recv().await {
        publisher
            .as_ref()
            .publish_vehicle_status(&vehicle_status)
            .await;
    }
    Ok(())
}

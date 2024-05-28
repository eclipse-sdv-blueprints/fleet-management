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
// use std::env;
use std::process;
use clap::Command;
use curvelogging::*;
use log::{error, info};
use status_publishing::InfluxWriter as CovesaInfluxWriter;
use tokio::sync::mpsc;

mod curvelogging;
mod status_publishing;
mod vehicle_abstraction;

const SUBCOMMAND_INFLUX: &str = "influx";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();
    let version = option_env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT")
        .unwrap_or(option_env!("VERGEN_GIT_SHA").unwrap_or("unknown"));

    let mut parser = Command::new("covesa-forwarder")
        .arg_required_else_help(true)
        .version(version)
        .about("Forwards VSS data points to a back end system after applying a curvelogging algorithm to remove unnecessary values");

    parser = vehicle_abstraction::add_command_line_args(parser);
    parser = parser
        .subcommand_required(true)
        .subcommand(influx_client::connection::add_command_line_args(
            Command::new(SUBCOMMAND_INFLUX).about("Forwards VSS data to an Influx DB server"),
        ));

    let args = parser.get_matches();
    let publisher = if let Some(SUBCOMMAND_INFLUX) = args.subcommand_name() {
        let influx_args = args.subcommand_matches(SUBCOMMAND_INFLUX).unwrap();
        match CovesaInfluxWriter::new(influx_args) {
            Ok(writer) => Box::new(writer),
            Err(e) => {
                error!("failed to create InfluxDB writer: {e}");
                process::exit(1);
            }
        }
    } else {
        error!("failed to create InfluxDB writer");
        process::exit(1);
    };

    info!("starting COVESA CV forwarder");

    let (tx, mut rx) = mpsc::channel::<Vec<ChosenSignals>>(30);
    let default_vin = args
        .get_one::<String>(crate::vehicle_abstraction::PARAM_DEFAULT_VIN)
        .unwrap()
        .to_string();
    let window_capacity = args.get_one::<usize>(crate::vehicle_abstraction::PARAM_WINDOW_CAPACITY).unwrap();
    let actor_handle = CurveLogActorHandle::new(window_capacity.to_owned());
    vehicle_abstraction::init(&args, tx, actor_handle).await?;
    while let Some(chosen_signals_collection) = rx.recv().await {
        for signal in chosen_signals_collection {
            publisher
                .write_chosen_signals(&signal, &default_vin)
                .await;
        }
    }

    Ok(())
}

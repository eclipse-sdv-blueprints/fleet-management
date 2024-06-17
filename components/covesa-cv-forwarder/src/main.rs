// SPDX-FileCopyrightText: 2023, 2024 Contributors to the Eclipse Foundation
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
use clap::Command;
use curvelogging::*;
use log::info;
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
    parser = parser.subcommand_required(true).subcommand(
        influx_client::connection::add_command_line_args(
            Command::new(SUBCOMMAND_INFLUX).about("Forwards VSS data to an Influx DB server"),
        ),
    );

    let args = parser.get_matches();
    let influx_args = args.subcommand_matches(SUBCOMMAND_INFLUX).unwrap();
    let publisher = Box::new(CovesaInfluxWriter::new(influx_args).unwrap());

    info!("starting COVESA CV forwarder");

    let (tx, mut rx) = mpsc::channel::<Vec<Option<ChosenSignals>>>(30);
    let window_capacity = args
        .get_one::<usize>(crate::curvelogging::PARAM_WINDOW_CAPACITY)
        .unwrap();
    let curve_log_handler = CurveLogActorHandler::new(window_capacity.to_owned(), tx.clone());
    vehicle_abstraction::init(&args, curve_log_handler).await?;
    while let Some(chosen_signals_collection) = rx.recv().await {
        for signal in chosen_signals_collection {
            // collected all of the chosen signals incoming Vec
            if signal.is_some() {
                publisher.write_chosen_signals(&signal.unwrap()).await;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    }

    Ok(())
}

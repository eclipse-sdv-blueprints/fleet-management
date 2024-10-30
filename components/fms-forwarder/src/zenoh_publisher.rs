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

use crate::status_publishing::StatusPublisher;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use fms_proto::fms::VehicleStatus;
use log::{debug, warn};
use protobuf::Message;
use zenoh::{pubsub::Publisher, Config, Session};

const KEY_EXPR: &str = "fms/vehicleStatus";

pub fn add_command_line_args(command: Command) -> Command {
    command.arg(
        Arg::new("config")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .long("config")
            .short('c')
            .help("A file to read the Zenoh configuration from")
            .required(false),
    )
}

pub fn parse_args(args: &ArgMatches) -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(conf_file) = args.get_one::<String>("config") {
        Config::from_file(conf_file)
    } else {
        Ok(Config::default())
    }
}

pub struct ZenohPublisher<'a> {
    // we need to keep a reference to the Session in order to
    // prevent Zenoh from closing it prematurely
    _session: Session,
    publisher: Publisher<'a>,
}

impl<'a> ZenohPublisher<'a> {
    pub async fn new(
        args: &ArgMatches,
    ) -> Result<ZenohPublisher<'a>, Box<dyn std::error::Error + Send + Sync>> {
        let config = parse_args(args)?;
        let session = zenoh::open(config).await?;
        let publisher = session.declare_publisher(KEY_EXPR).await?;
        Ok(ZenohPublisher {
            _session: session,
            publisher,
        })
    }
}

#[async_trait]
impl<'a> StatusPublisher for ZenohPublisher<'a> {
    async fn publish_vehicle_status(&self, vehicle_status: &VehicleStatus) {
        match vehicle_status.write_to_bytes() {
            Ok(payload) => {
                match self.publisher.put(payload).await {
                    Ok(_t) => debug!("successfully published vehicle status to Zenoh",),
                    Err(e) => {
                        warn!("error publishing vehicle status to Zenoh: {}", e);
                    }
                };
                return;
            }
            Err(e) => warn!(
                "error serializing vehicle status to protobuf message: {}",
                e
            ),
        }
    }
}

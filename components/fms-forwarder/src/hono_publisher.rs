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

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use fms_proto::fms::VehicleStatus;
use log::{debug, warn};
use mqtt::MessageBuilder;
use paho_mqtt as mqtt;
use protobuf::Message;

use crate::{status_publishing::StatusPublisher, mqtt_connection::{self, MqttConnection}};

const TOPIC_TELEMETRY: &str = "telemetry/?content-type=application%2Fvnd.google.protobuf";

/// Adds arguments to an existing command line which can be
/// used to configure the connection to a Hono MQTT protocol adapter.
/// 
/// See [`mqtt_connection::add_command_line_args`]
/// 
pub fn add_command_line_args(command: Command) -> Command {
    mqtt_connection::add_command_line_args(command)
}

pub struct HonoPublisher {
    mqtt_connection: MqttConnection,
}

impl HonoPublisher {

    /// Creates a new publisher.
    /// 
    /// Determines the parameters necessary for creating the publisher from values specified on
    /// the command line or via environment variables as defined by [`add_command_line_args`].
    /// 
    /// The publisher returned is configured to keep trying to (re-)connect to the configured
    /// MQTT endpoint using a client certificate of username/password credentials. 
    pub async fn new(args: &ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {

        MqttConnection::new(&args).await
            .map(|con| {
                HonoPublisher { mqtt_connection: con }
                })
            }
        }

#[async_trait]
impl StatusPublisher for HonoPublisher {
    async fn publish_vehicle_status(&self, vehicle_status: &VehicleStatus) {
        match vehicle_status.write_to_bytes() {
            Ok(payload) => {
                let msg = MessageBuilder::new()
                    .topic(TOPIC_TELEMETRY)
                    .payload(payload)
                    .finalize();
                match self.mqtt_connection.mqtt_client.publish(msg).await {
                    Ok(_t) => debug!(
                        "successfully published vehicle status to MQTT endpoint [uri: {}, topic: {}]",
                        self.mqtt_connection.uri, TOPIC_TELEMETRY
                    ),
                    Err(e) => {
                        warn!(
                            "error publishing vehicle status to MQTT endpoint [uri: {}, topic: {}]: {}",
                            self.mqtt_connection.uri, TOPIC_TELEMETRY, e
                        );
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

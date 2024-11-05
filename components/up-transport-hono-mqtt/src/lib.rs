// SPDX-FileCopyrightText: 2024 Contributors to the Eclipse Foundation
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

use clap::Args;
use log::{debug, warn};
use paho_mqtt::{AsyncClient, MessageBuilder};
use protobuf::Message;
use up_rust::{CloudEvent, UCode, UMessage, UStatus, UTransport};
use url_encor::Encoder;

mod mqtt_connection;

// TODO: this should be replaced with a constant from up-rust
// see https://github.com/eclipse-uprotocol/up-rust/issues/223
const MIME_TYPE_CLOUDEVENTS_PROTOBUF: &str = "application/cloudevents+protobuf";

#[derive(Args)]
pub struct HonoMqttTransportConfig {
    #[command(flatten)]
    mqtt_client_options: mqtt_connection::MqttClientOptions,
}

pub struct HonoMqttTransport {
    mqtt_client: AsyncClient,
    topic: String,
}

/// A simple uProtocol transport implementation for sending messages to Eclipse Hono's
/// MQTT adapter (using MQTT v3.1.1).
impl HonoMqttTransport {
    /// Creates a new transport.
    ///
    /// Determines the parameters necessary for creating the publisher from values specified on
    /// the command line or via environment variables as defined by [`add_command_line_args`].
    ///
    /// The publisher returned is configured to keep trying to (re-)connect to the configured
    /// MQTT endpoint using a client certificate of username/password credentials.
    pub async fn new(config: &HonoMqttTransportConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let content_type = MIME_TYPE_CLOUDEVENTS_PROTOBUF.to_string().url_encode();
        let topic = format!("telemetry/?content-type={}", content_type);

        config
            .mqtt_client_options
            .connect()
            .await
            .map(|mqtt_client| HonoMqttTransport { mqtt_client, topic })
    }
}

#[async_trait::async_trait]
impl UTransport for HonoMqttTransport {
    async fn send(&self, message: UMessage) -> Result<(), UStatus> {
        let event = CloudEvent::try_from(message)
            .map_err(|e| UStatus::fail_with_code(UCode::INVALID_ARGUMENT, e.to_string()))?;
        let payload = event.write_to_bytes().map_err(|_e| {
            UStatus::fail_with_code(UCode::INTERNAL, "failed to serialize CloudEvent to JSON")
        })?;
        let msg = MessageBuilder::new()
            .topic(self.topic.clone())
            .payload(payload)
            .finalize();
        match self.mqtt_client.publish(msg).await {
            Ok(_t) => {
                debug!(
                    "successfully published vehicle status to MQTT endpoint [uri: {}, topic: {}]",
                    self.mqtt_client.server_uri(),
                    self.topic
                );
                Ok(())
            }
            Err(e) => {
                warn!(
                    "error publishing vehicle status to MQTT endpoint [uri: {}, topic: {}]: {}",
                    self.mqtt_client.server_uri(),
                    self.topic,
                    e
                );
                Err(UStatus::fail_with_code(UCode::INTERNAL, e.to_string()))
            }
        }
    }
}

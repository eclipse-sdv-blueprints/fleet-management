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

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use clap::Args;
use futures::TryStreamExt;
use log::{debug, error, info, trace};
use protobuf::Message as ProtoMessage;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    message::{BorrowedHeaders, BorrowedMessage, Headers},
    ClientConfig, Message,
};
use up_rust::{
    local_transport::LocalTransport, CloudEvent, UCode, UListener, UMessage, UStatus, UTransport,
    UUri,
};

const HEADER_NAME_ORIG_ADDRESS: &str = "orig_address";

const PARAM_KAFKA_PROPERTIES_FILE: &str = "kafka-properties-file";
const PARAM_HONO_TENANT_ID: &str = "hono-tenant-id";

// TODO: this should be replaced with a constant from up-rust
// see https://github.com/eclipse-uprotocol/up-rust/issues/223
const MIME_TYPE_CLOUDEVENTS_PROTOBUF: &str = "application/cloudevents+protobuf";

fn add_property_bag_to_map(property_bag: String, headers: &mut HashMap<String, String>) {
    property_bag.split('&').for_each(|p| {
        trace!("processing property: {p}");
        if let Some((key, value)) = p.split_once('=') {
            if headers.contains_key(key) {
                trace!("skipping property [{key}] from property bag because header with same name exists");
            } else {
                trace!("adding propery [key: {key}, value: {value}] to header map");
                headers.insert(key.to_string(), value.to_string());
            }
        }
    });
}

fn get_headers_as_map(headers: &BorrowedHeaders) -> HashMap<String, String> {
    let mut result = HashMap::new();
    headers.iter().for_each(|header| {
        match (
            header.key,
            header
                .value
                .and_then(|v| String::from_utf8(v.to_vec()).ok()),
        ) {
            (HEADER_NAME_ORIG_ADDRESS, Some(value)) => {
                if let Some((_topic, props)) = value.rsplit_once("/?") {
                    debug!("found property bag in {HEADER_NAME_ORIG_ADDRESS} header: {props}");
                    add_property_bag_to_map(props.to_string(), &mut result);
                }
            }
            (_, Some(value)) => {
                result.insert(header.key.to_string(), value);
            }
            (_, None) => {
                debug!("message contains empty header [{}]", header.key);
            }
        };
    });

    result
}

fn extract_umessage_from_cloudevent(payload: &[u8]) -> Option<UMessage> {
    match CloudEvent::parse_from_bytes(payload) {
        Ok(cloudevent) => match UMessage::try_from(cloudevent) {
            Err(e) => {
                info!("failed to extract payload from CloudEvent: {}", e);
                None
            }
            Ok(msg) => Some(msg),
        },
        Err(e) => {
            info!("failed to deserialize CloudEvent: {}", e);
            None
        }
    }
}

fn extract_umessage_from_kafka_message(m: &BorrowedMessage<'_>) -> Option<UMessage> {
    if let Some(headers) = m.headers() {
        let message_properties = get_headers_as_map(headers);

        if let Some(device_id) = message_properties.get("device_id") {
            debug!("received message from vehicle {}", device_id);
        } else {
            debug!("discarding message from unknown device");
            return None;
        };

        match (
            message_properties.get("content-type").map(String::as_str),
            m.payload(),
        ) {
            (Some(MIME_TYPE_CLOUDEVENTS_PROTOBUF), Some(payload)) => {
                debug!("received message containing CloudEvent");
                extract_umessage_from_cloudevent(payload)
            }
            (_, None) => {
                debug!("ignoring message without payload");
                None
            }
            _ => None,
        }
    } else {
        debug!("ignoring message without headers");
        None
    }
}

#[derive(Args)]
pub struct HonoKafkaTransportConfig {
    /// The path to a file containing Kafka client properties for connecting to Hono's Kafka broker(s).
    #[arg(long = PARAM_KAFKA_PROPERTIES_FILE, value_name = "PATH", env = "KAFKA_PROPERTIES_FILE", value_parser = clap::builder::PathBufValueParser::new())]
    kafka_property_file: PathBuf,

    /// The identifier of the Hono tenant to consume messages for.
    #[arg(long = PARAM_HONO_TENANT_ID, value_name = "ID", env = "HONO_TENANT_ID")]
    hono_tenant_id: String,
}

impl HonoKafkaTransportConfig {
    fn read_lines(&self) -> Result<std::io::Lines<BufReader<File>>, Box<dyn std::error::Error>> {
        // Open the file in read-only mode.
        match File::open(&self.kafka_property_file) {
            Ok(file) => {
                // Read the file line by line, and return an iterator of the lines of the file.
                Ok(BufReader::new(file).lines())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn get_kafka_client_config(&self) -> Result<ClientConfig, Box<dyn std::error::Error>> {
        self.read_lines().map(|lines| {
            let mut client_config = ClientConfig::new();
            for line in lines {
                match line {
                    Ok(property) => match property.split_once('=') {
                        Some((key, value)) => {
                            client_config.set(key, value);
                        }
                        None => {
                            debug!("cannot parse line into property: {}", property);
                        }
                    },
                    Err(e) => {
                        debug!("cannot read line from file: {e}");
                    }
                }
            }
            client_config
        })
    }
}

/// A simple uProtocol transport implementation for receiving messages from Eclipse Hono's
/// Apache Kafka based messaging infrastructure.
pub struct HonoKafkaTransport {
    local_transport: Arc<LocalTransport>,
}

impl HonoKafkaTransport {
    pub fn new(
        config_params: HonoKafkaTransportConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_config = config_params.get_kafka_client_config()?;

        // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
        let consumer = client_config
            .set_log_level(RDKafkaLogLevel::Debug)
            .create::<StreamConsumer>()?;

        let telemetry_topic = format!("hono.telemetry.{}", config_params.hono_tenant_id);
        let event_topic = format!("hono.event.{}", config_params.hono_tenant_id);
        let response_topic = format!("hono.command_response.{}", config_params.hono_tenant_id);

        let topics = &[
            telemetry_topic.as_str(),
            event_topic.as_str(),
            response_topic.as_str(),
        ];
        for topic_name in topics {
            match consumer.fetch_metadata(Some(topic_name), Duration::from_secs(10)) {
                Err(e) => {
                    error!(
                        "could not retrieve meta data for topic [{topic_name}] from broker: {e}"
                    );
                }
                Ok(metadata) => match metadata
                    .topics()
                    .iter()
                    .find(|topic| topic.name().eq(*topic_name))
                {
                    Some(topic) => {
                        if topic.partitions().is_empty() {
                            error!("topic [{topic_name}] does not exist (yet)");
                        }
                    }
                    None => {
                        error!("broker did not return meta data for topic [{topic_name}]");
                    }
                },
            }
        }

        if let Err(e) = consumer.subscribe(topics) {
            error!("failed to subscribe to topic: {}", e);
            return Err(Box::new(e));
        } else {
            info!("successfully subscribed to tenant's topics");
        }

        let local_transport = Arc::new(LocalTransport::default());
        let local_transport_ref = local_transport.clone();
        tokio::spawn(async move {
            info!("starting message consumer");
            consumer
                .stream()
                .try_for_each(|borrowed_message| {
                    let cloned_local_transport = local_transport_ref.clone();
                    async move {
                        if let Some(msg) = extract_umessage_from_kafka_message(&borrowed_message) {
                            let _ = cloned_local_transport.send(msg).await;
                        }
                        Ok(())
                    }
                })
                .await
                .unwrap_or_else(|e| {
                    error!("could not start consumer for Hono topics: {}", e);
                });
        });
        Ok(HonoKafkaTransport { local_transport })
    }
}

#[async_trait::async_trait]
impl UTransport for HonoKafkaTransport {
    async fn send(&self, _message: UMessage) -> Result<(), UStatus> {
        Err(UStatus::fail_with_code(
            UCode::UNIMPLEMENTED,
            "not implemented",
        ))
    }

    async fn register_listener(
        &self,
        source_filter: &UUri,
        sink_filter: Option<&UUri>,
        listener: Arc<dyn UListener>,
    ) -> Result<(), UStatus> {
        self.local_transport
            .register_listener(source_filter, sink_filter, listener)
            .await
    }

    async fn unregister_listener(
        &self,
        source_filter: &UUri,
        sink_filter: Option<&UUri>,
        listener: Arc<dyn UListener>,
    ) -> Result<(), UStatus> {
        self.local_transport
            .unregister_listener(source_filter, sink_filter, listener)
            .await
    }
}

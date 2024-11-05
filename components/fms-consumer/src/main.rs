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

use std::sync::Arc;
use std::thread;

use clap::{Parser, Subcommand};
use fms_proto::fms::VehicleStatus;
use fms_util::zenoh::ZenohTransportConfig;
use influx_client::connection::InfluxConnectionConfig;
use influx_client::writer::InfluxWriter;
use log::{debug, info};

use fms_util::StaticUriProvider;
use up_rust::communication::{Notifier, SimpleNotifier};
use up_rust::{UListener, UMessage, UTransport, UUri};
use up_transport_hono_kafka::{HonoKafkaTransport, HonoKafkaTransportConfig};

const SUBCOMMAND_HONO: &str = "hono";
const SUBCOMMAND_ZENOH: &str = "zenoh";

struct NotificationListener {
    influx_writer: InfluxWriter,
}

#[async_trait::async_trait]
impl UListener for NotificationListener {
    async fn on_receive(&self, msg: UMessage) {
        if let Ok(vehicle_status) = msg.extract_protobuf::<VehicleStatus>() {
            self.influx_writer
                .write_vehicle_status(&vehicle_status)
                .await;
        } else {
            debug!("ignoring Notification message with invalid/unknown payload");
        }
    }
}

/// Receives FMS related VSS data points via Hono's Kafka based Telemetry API or Eclipse Zenoh instance
/// and writes them to an InfluxDB server.
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct FmsConsumerCommand {
    /// The uProtocol URI to use as the source filter for vehicle status notifications.
    #[arg(long = fms_util::PARAM_UE_SOURCE, value_name = "URI", env = "UE_SOURCE_FILTER", default_value = "up://fms-forwarder/D100/1/D100", value_parser = fms_util::read_uuri )]
    notification_source_filter: UUri,

    /// The local uService address.
    #[arg(long = fms_util::PARAM_UE_SINK, value_name = "URI", env = "LOCAL_USERVICE_URI", default_value = "up://fms-consumer/D100/1/0", value_parser = fms_util::read_uuri )]
    local_uservice_uri: UUri,

    #[command(flatten)]
    influxdb_connection: InfluxConnectionConfig,

    #[command(subcommand)]
    transport: TransportType,
}

#[derive(Subcommand)]
#[command(subcommand_required = true)]
enum TransportType {
    /// Consumes VSS data using the Eclipse Hono/Kafka based uProtocol transport.
    #[command(name = SUBCOMMAND_HONO)]
    Hono(HonoKafkaTransportConfig),

    /// Consumes VSS data using the Eclipse Zenoh based uProtocol transport.
    #[command(name = SUBCOMMAND_ZENOH)]
    Zenoh(ZenohTransportConfig),
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let command = FmsConsumerCommand::parse();
    let uri_provider = StaticUriProvider::new(&command.local_uservice_uri).map(Arc::new)?;

    let transport: Arc<dyn UTransport> = match command.transport {
        TransportType::Hono(config) => {
            HonoKafkaTransport::new(config, &command.local_uservice_uri).map(Arc::new)?
        }
        TransportType::Zenoh(config) => {
            fms_util::zenoh::new_transport(&command.local_uservice_uri, config)
                .await
                .map(Arc::new)?
        }
    };

    let notifier = SimpleNotifier::new(transport.clone(), uri_provider);

    info!(
        "Registering uProtocol Notification listener [source filter: {}]",
        &command.notification_source_filter.to_uri(false)
    );
    let influx_writer = InfluxWriter::new(&command.influxdb_connection)?;
    let listener = Arc::new(NotificationListener { influx_writer });
    notifier
        .start_listening(&command.notification_source_filter, listener)
        .await
        .map_err(Box::new)?;
    // do not let the Notifier that we use to receive and process
    // Vehicle status notifications go out of scope
    thread::park();

    Ok(())
}

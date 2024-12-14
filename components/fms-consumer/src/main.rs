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

use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use clap::{Parser, Subcommand};
use fms_proto::fms::VehicleStatus;
use fms_util::ZenohTransportConfig;
use influx_client::connection::InfluxConnectionConfig;
use influx_client::writer::InfluxWriter;
use log::info;

use up_rust::{UListener, UMessage, UTransport, UUri};
use up_transport_hono_kafka::{HonoKafkaTransport, HonoKafkaTransportConfig};
use up_transport_zenoh::UPTransportZenoh;

struct VehicleStatusListener {
    influx_writer: InfluxWriter,
}

#[async_trait::async_trait]
impl UListener for VehicleStatusListener {
    async fn on_receive(&self, msg: UMessage) {
        if let Ok(vehicle_status) = msg.extract_protobuf::<VehicleStatus>() {
            self.influx_writer
                .write_vehicle_status(&vehicle_status)
                .await;
        } else {
            info!("ignoring event with invalid/unknown payload");
        }
    }
}

/// Receives FMS related VSS data points via Hono's Kafka based Telemetry API or Eclipse Zenoh instance
/// and writes them to an InfluxDB server.
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct FmsConsumerCommand {
    /// The topic URI pattern to use for consuming vehicle status events.
    #[arg(long = "topic-filter", value_name = "URI", env = "TOPIC_FILTER", default_value = "up://*/D100/1/D100", value_parser = up_rust::UUri::from_str )]
    vehicle_status_topic_filter: UUri,

    /// The local uService address.
    #[arg(long = "uservice-uri", value_name = "URI", env = "USERVICE_URI", default_value = "up://fms-consumer/D101/1/0", value_parser = up_rust::UUri::from_str )]
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
    #[command(name = "hono")]
    Hono(HonoKafkaTransportConfig),

    /// Consumes VSS data using the Eclipse Zenoh based uProtocol transport.
    #[command(name = "zenoh")]
    Zenoh(ZenohTransportConfig),
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let command = FmsConsumerCommand::parse();

    let transport: Arc<dyn UTransport> = match command.transport {
        TransportType::Hono(config) => HonoKafkaTransport::new(config).map(Arc::new)?,
        TransportType::Zenoh(config) => {
            let config = config.try_into()?;
            UPTransportZenoh::new(config, command.local_uservice_uri)
                .await
                .map(Arc::new)?
        }
    };

    let influx_writer = InfluxWriter::new(&command.influxdb_connection)?;
    let listener = Arc::new(VehicleStatusListener { influx_writer });
    info!(
        "Registering listener for vehicle status events [source filter: {}]",
        &command.vehicle_status_topic_filter.to_uri(false)
    );
    transport
        .register_listener(&command.vehicle_status_topic_filter, None, listener)
        .await
        .map_err(Box::new)?;
    // do not let the Notifier that we use to receive and process
    // Vehicle status notifications go out of scope
    thread::park();

    Ok(())
}

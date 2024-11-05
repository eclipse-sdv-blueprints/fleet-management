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

use std::{str::FromStr, sync::Arc};

use clap::{Parser, Subcommand};
use fms_proto::fms::VehicleStatus;
use fms_util::ZenohTransportConfig;
use log::{info, warn};
use tokio::sync::mpsc;
use up_rust::{
    communication::{CallOptions, Publisher, SimplePublisher, UPayload},
    LocalUriProvider, StaticUriProvider, UTransport, UUri,
};
use up_transport_hono_mqtt::{HonoMqttTransport, HonoMqttTransportConfig};
use up_transport_zenoh::UPTransportZenoh;

mod vehicle_abstraction;

/// Forwards FMS related VSS data points to a back end system using uProtocol.
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct FmsForwarderCommand {
    /// The topic to publish vehicle status events to.
    #[arg(long = "topic", value_name = "URI", env = "TOPIC", default_value = "up://fms-forwarder/D100/1/D100", value_parser = up_rust::UUri::from_str )]
    vehicle_status_topic: UUri,

    #[command(flatten)]
    databroker_connection: vehicle_abstraction::KuksaDatabrokerClientConfig,

    #[command(subcommand)]
    transport: TransportType,
}

#[derive(Subcommand)]
#[command(subcommand_required = true)]
enum TransportType {
    /// Forwards VSS data via Eclipse uProtocol using Eclipse Hono based transport.
    #[command(name = "hono")]
    Hono(HonoMqttTransportConfig),

    /// Forwards VSS data via Eclipse uProtocol using Eclipse Zenoh based transport.
    #[command(name = "zenoh")]
    Zenoh(ZenohTransportConfig),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let command = FmsForwarderCommand::parse();
    let uri_provider = StaticUriProvider::try_from(&command.vehicle_status_topic).map(Arc::new)?;

    let transport: Arc<dyn UTransport> = match command.transport {
        TransportType::Hono(config) => HonoMqttTransport::new(&config).await.map(Arc::new)?,
        TransportType::Zenoh(config) => {
            let zenoh_config = config.try_into()?;
            UPTransportZenoh::new(zenoh_config, uri_provider.get_source_uri())
                .await
                .map(Arc::new)?
        }
    };

    let origin_resource_id = u16::try_from(command.vehicle_status_topic.resource_id)?;
    let publisher = Arc::new(SimplePublisher::new(transport, uri_provider));
    info!("starting FMS forwarder");

    let (tx, mut rx) = mpsc::channel::<VehicleStatus>(30);
    vehicle_abstraction::init(&command.databroker_connection, tx).await?;

    while let Some(vehicle_status) = rx.recv().await {
        match UPayload::try_from_protobuf(vehicle_status) {
            Ok(payload) => {
                if let Err(e) = publisher
                    .publish(
                        origin_resource_id,
                        CallOptions::for_publish(None, None, None),
                        Some(payload),
                    )
                    .await
                {
                    warn!("failed to publish vehicle status event: {}", e);
                }
            }
            Err(e) => {
                warn!("failed to serialize vehicle status: {}", e);
            }
        }
    }
    Ok(())
}

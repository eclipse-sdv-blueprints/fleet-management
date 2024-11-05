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

use std::{process, sync::Arc};

use clap::{Parser, Subcommand};
use fms_proto::fms::VehicleStatus;
use fms_util::{zenoh::ZenohTransportConfig, StaticUriProvider};
use log::{error, info, warn};
use tokio::sync::mpsc;
use up_rust::{
    communication::{CallOptions, Notifier, SimpleNotifier, UPayload},
    UTransport, UUri,
};
use up_transport_hono_mqtt::{HonoMqttTransport, MqttClientOptions};

mod vehicle_abstraction;

const SUBCOMMAND_HONO: &str = "hono";
const SUBCOMMAND_ZENOH: &str = "zenoh";

/// Forwards FMS related VSS data points to a back end system using uProtocol.
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct FmsForwarderCommand {
    /// The uProtocol URI to use as the destination for vehicle status notifications.
    #[arg(long = fms_util::PARAM_UE_SINK, value_name = "URI", env = "UE_SINK", default_value = "up://fms-consumer/D100/1/0", value_parser = fms_util::read_uuri )]
    notification_destination: UUri,

    /// The uProtocol URI to use as the origin address for vehicle status notifications.
    #[arg(long = fms_util::PARAM_UE_SOURCE, value_name = "URI", env = "UE_SOURCE", default_value = "up://fms-forwarder/D100/1/D100", value_parser = fms_util::read_uuri )]
    notification_origin: UUri,

    #[command(flatten)]
    databroker_connection: vehicle_abstraction::KuksaDatabrokerClientConfig,

    #[command(subcommand)]
    transport: TransportType,
}

#[derive(Subcommand)]
#[command(subcommand_required = true)]
enum TransportType {
    /// Forwards VSS data via Eclipse uProtocol using Eclipse Hono based transport.
    #[command(name = SUBCOMMAND_HONO)]
    Hono(MqttClientOptions),

    /// Forwards VSS data via Eclipse uProtocol using Eclipse Zenoh based transport.
    #[command(name = SUBCOMMAND_ZENOH)]
    Zenoh(ZenohTransportConfig),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let command = FmsForwarderCommand::parse();

    let transport: Arc<dyn UTransport> = match command.transport {
        TransportType::Hono(config) => match HonoMqttTransport::new(config).await {
            Ok(transport) => Arc::new(transport),
            Err(e) => {
                error!("failed to create Hono transport: {}", e);
                process::exit(1);
            }
        },
        TransportType::Zenoh(config) => {
            match fms_util::zenoh::new_transport(&command.notification_origin, config).await {
                Ok(transport) => Arc::new(transport),
                Err(e) => {
                    error!("failed to create Zenoh based UTransport: {e}");
                    process::exit(1);
                }
            }
        }
    };

    let origin_resource_id = u16::try_from(command.notification_origin.resource_id)?;
    let uri_provider = StaticUriProvider::new(&command.notification_origin).map(Arc::new)?;
    let notifier = Arc::new(SimpleNotifier::new(transport, uri_provider));

    info!("starting FMS forwarder");

    let (tx, mut rx) = mpsc::channel::<VehicleStatus>(30);
    vehicle_abstraction::init(&command.databroker_connection, tx).await?;

    while let Some(vehicle_status) = rx.recv().await {
        match UPayload::try_from_protobuf(vehicle_status) {
            Ok(payload) => {
                if let Err(e) = notifier
                    .notify(
                        origin_resource_id,
                        &command.notification_destination,
                        CallOptions::for_notification(None, None, None),
                        Some(payload),
                    )
                    .await
                {
                    warn!("failed to send vehicle status notification: {}", e);
                }
            }
            Err(e) => {
                warn!("failed to forward vehicle status: {}", e);
            }
        }
    }
    Ok(())
}

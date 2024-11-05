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

use std::path::PathBuf;

use clap::Args;
use log::info;
use up_rust::UUri;
use up_transport_zenoh::UPTransportZenoh;
use zenoh::config::Config;

/// Parameters for configuring the connection to an Eclipse Zenoh router.
#[derive(Args)]
pub struct ZenohTransportConfig {
    /// A file to read the Zenoh configuration from.
    #[arg(long = "config", short, value_parser = clap::builder::PathBufValueParser::new())]
    config_file: Option<PathBuf>,
}

impl ZenohTransportConfig {
    fn zenoh_config(&self) -> Result<Config, Box<dyn std::error::Error>> {
        if let Some(path) = &self.config_file {
            Config::from_file(path).map_err(|e| e as Box<dyn std::error::Error>)
        } else {
            Ok(Config::default())
        }
    }
}

// Creates a new Eclipse Zenoh based uProtocol transport.
//
pub async fn new_transport(
    local_uri: &UUri,
    config_params: ZenohTransportConfig,
) -> Result<UPTransportZenoh, Box<dyn std::error::Error>> {
    let mut owned_local_uri = local_uri.to_owned();
    owned_local_uri.resource_id = 0x0000;
    let config = config_params.zenoh_config()?;

    info!("Creating Zenoh based UTransport...");
    UPTransportZenoh::new(config, owned_local_uri)
        .await
        .map_err(Box::from)
}

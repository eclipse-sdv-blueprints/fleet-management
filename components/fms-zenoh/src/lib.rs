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

use std::path::PathBuf;

use clap::Args;
use zenoh::config::Config;

/// Parameters for configuring the connection to an Eclipse Zenoh router.
#[derive(Args)]
pub struct ZenohTransportConfig {
    /// A file to read the Zenoh configuration from.
    #[arg(long = "config", short)]
    config_file: Option<PathBuf>,
}

impl TryFrom<ZenohTransportConfig> for Config {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: ZenohTransportConfig) -> Result<Self, Self::Error> {
        if let Some(path) = &value.config_file {
            Config::from_file(path).map_err(|e| e as Box<dyn std::error::Error>)
        } else {
            Ok(Config::default())
        }
    }
}

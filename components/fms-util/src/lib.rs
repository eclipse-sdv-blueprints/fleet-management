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

use up_rust::{LocalUriProvider, UUri, UUriError};

pub mod zenoh;

pub const PARAM_UE_SOURCE: &str = "ue-source";
pub const PARAM_UE_SINK: &str = "ue-sink";

pub fn read_uuri(uri: &str) -> Result<UUri, UUriError> {
    up_rust::UUri::try_from(uri)
}

pub struct StaticUriProvider {
    local_uri: UUri,
}

impl StaticUriProvider {
    pub fn new(source_uri: &UUri) -> Result<Self, Box<dyn std::error::Error>> {
        let local_uri = UUri::try_from_parts(
            &source_uri.authority_name,
            source_uri.ue_id,
            u8::try_from(source_uri.ue_version_major)?,
            0x0000,
        )?;
        Ok(StaticUriProvider { local_uri })
    }
}

impl LocalUriProvider for StaticUriProvider {
    fn get_authority(&self) -> String {
        self.local_uri.authority_name.clone()
    }

    fn get_resource_uri(&self, resource_id: u16) -> UUri {
        let mut uri = self.local_uri.clone();
        uri.resource_id = resource_id as u32;
        uri
    }

    fn get_source_uri(&self) -> UUri {
        self.local_uri.clone()
    }
}

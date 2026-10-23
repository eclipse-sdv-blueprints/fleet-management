// SPDX-FileCopyrightText: 2026 Contributors to the Eclipse Foundation
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

use chrono::{DateTime, Utc};

use crate::models::error::FmsError;

const PAGE_SIZE: usize = 30;

pub type VehiclesQueryParameters = BasicQueryParameters;

fn deserialize_optional_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = <Option<String> as serde::Deserialize>::deserialize(deserializer)?;
    value
        .map(|text| text.parse::<bool>())
        .transpose()
        .map_err(serde::de::Error::custom)
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DateType {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "received")]
    #[default]
    Received,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BasicQueryParameters {
    #[serde(rename = "lastVin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_vin: Option<String>,
    #[serde(skip)]
    pub page_size: usize,
}

impl Default for BasicQueryParameters {
    fn default() -> Self {
        Self {
            last_vin: None,
            page_size: PAGE_SIZE,
        }
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CommonQueryParameters {
    #[serde(flatten)]
    pub basic: BasicQueryParameters,
    #[serde(rename = "datetype")]
    #[serde(default)]
    pub date_type: DateType,
    #[serde(rename = "starttime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(rename = "stoptime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<String>,
    // using the deserialize_optional_bool function is necessary, because the flattening of fields
    // results in the URL deserialzer seeing a string instead of a boolean.
    #[serde(
        rename = "latestOnly",
        default,
        deserialize_with = "deserialize_optional_bool"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_only: Option<bool>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PositionsQueryParameters {
    #[serde(flatten)]
    pub common: CommonQueryParameters,
    #[serde(rename = "triggerFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_filter: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StatusContentType {
    #[serde(rename = "ACCUMULATED")]
    Accumulated,
    #[serde(rename = "SNAPSHOT")]
    Snapshot,
    #[serde(rename = "UPTIME")]
    Uptime,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct StatusesQueryParameters {
    #[serde(flatten)]
    pub common: CommonQueryParameters,
    #[serde(rename = "contentFilter")]
    pub content_filter: Vec<StatusContentType>,
    #[serde(rename = "triggerFilter")]
    pub trigger_filter: Vec<String>,
}

#[async_trait::async_trait]
pub(crate) trait GetVehicles {
    async fn get_vehicles(
        &self,
        query_params: &VehiclesQueryParameters,
    ) -> Result<Vec<crate::models::vehicle::VehicleObject>, FmsError>;
}

#[async_trait::async_trait]
pub(crate) trait GetVehiclePositions {
    async fn get_vehiclepositions(
        &self,
        parameters: &PositionsQueryParameters,
    ) -> Result<Vec<crate::models::position::VehiclePositionObject>, FmsError>;
}

#[async_trait::async_trait]
pub(crate) trait GetVehicleStatuses {
    async fn get_vehiclestatuses(
        &self,
        parameters: &StatusesQueryParameters,
    ) -> Result<Vec<crate::models::status::VehicleStatusObject>, FmsError>;
}

#[async_trait::async_trait]
pub(crate) trait DataAccess:
    GetVehicles + GetVehiclePositions + GetVehicleStatuses + Sync + Send
{
}

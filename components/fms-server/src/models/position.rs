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

use crate::models;

//use serde::Deserialize;
//use serde::Serialize;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehiclePositionObject {
    /// vehicle identification number. See ISO 3779 (17 characters)
    #[serde(rename = "vin")]
    pub vin: String,

    #[serde(rename = "triggerType")]
    pub trigger_type: models::TriggerObject,

    /// When the data was retrieved in the vehicle in iso8601 format.
    #[serde(rename = "createdDateTime")]
    pub created_date_time: chrono::DateTime<chrono::Utc>,

    /// Reception at Server. To be used for handling of \"more data available\" in iso8601 format.
    #[serde(rename = "receivedDateTime")]
    pub received_date_time: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "gnssPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnss_position: Option<GnssPositionObject>,

    /// Wheel-Based Vehicle Speed in km/h (Speed of the vehicle as calculated from wheel or tailshaft speed)
    #[serde(rename = "wheelBasedSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheel_based_speed: Option<f64>,

    /// Tachograph vehicle speed in km/h (Speed of the vehicle registered by the tachograph)
    #[serde(rename = "tachographSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tachograph_speed: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GnssPositionObject {
    /// Latitude (WGS84 based)
    #[serde(rename = "latitude")]
    pub latitude: f64,

    /// Longitude (WGS84 based)
    #[serde(rename = "longitude")]
    pub longitude: f64,

    /// The direction of the vehicle (0-359)
    #[serde(rename = "heading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i32>,

    /// The altitude of the vehicle. Where 0 is sea level, negative values below sealevel and positive above sealevel. Unit in meters.
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<i32>,

    /// The GNSS(e.g. GPS)-speed in km/h
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,

    /// The time of the position data in iso8601 format.
    #[serde(rename = "positionDateTime")]
    pub position_date_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehiclePositionResponseObject {
    #[serde(rename = "vehiclePositionResponse")]
    pub vehicle_position_response: VehiclePositionResponseObjectVehiclePositionResponse,

    /// This will be set to true if the result set was too large to be sent back in one reply. A new request must be sent to get the rest of the vehicle positions, where the starttime parameter must be supplied. The starttime should be set to the latest ReceivedDateTime + 1 second of the last vehicle in the result set of this message.
    #[serde(rename = "moreDataAvailable")]
    pub more_data_available: bool,

    /// Populated with the link to the next part of the result when moreDataAvailable is true. The link is relative, i.e. starts with /rfms/vehiclepositions, and preserves any query parameters from the original request.
    #[serde(rename = "moreDataAvailableLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_data_available_link: Option<String>,

    /// Time to be used to ask for historical data at customers (for starttime), to solve the problem of having different times at server and clients. This is the time at the server when this request was received. To avoid losing any messages or get duplicates, this is the time that should be supplied in the startTime parameter in the next request in iso8601 format.
    #[serde(rename = "requestServerDateTime")]
    pub request_server_date_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehiclePositionResponseObjectVehiclePositionResponse {
    #[serde(rename = "vehiclePositions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_positions: Option<Vec<VehiclePositionObject>>,
}

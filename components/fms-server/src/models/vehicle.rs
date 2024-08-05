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

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum VehiclesGetResponse {
    /// OK
    OK(VehicleResponseObject),
    /// The server cannot or will not process the request due to an apparent client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing)  Possible reason: Mandatory field missing, e.g. Authentication Header empty or missing  The comments for the 4xx codes are from the Wikipedia article [List of HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_Client_errors), which is released under the [Creative Commons Attribution-Share-Alike License 3.0](https://creativecommons.org/licenses/by-sa/3.0/). View authors on this [page](https://en.wikipedia.org/w/index.php?title=List_of_HTTP_status_codes&action=history).
    TheServerCannotOrWillNotProcessTheRequestDueToAnApparentClientError(ErrorObject),
    /// Similar to 403 Forbidden, but specifically for use when authentication is required and has failed or has not yet been provided. The response must include a WWW-Authenticate header field containing a challenge applicable to the requested resource. See Basic access authentication and Digest access authentication.  Possible reasons: Wrong credentials, Login credentials expired and/or Access token not valid or expired  The comments for the 4xx codes are from the Wikipedia article [List of HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_Client_errors), which is released under the [Creative Commons Attribution-Share-Alike License 3.0](https://creativecommons.org/licenses/by-sa/3.0/). View authors on this [page](https://en.wikipedia.org/w/index.php?title=List_of_HTTP_status_codes&action=history).
    SimilarTo(ErrorObject),
    /// The request was a valid request, but the server is refusing to respond to it. Unlike a 401 Unauthorized response, authenticating will make no difference. On servers where authentication is required, this commonly means that the provided credentials were successfully authenticated but that the credentials still do not grant the client permission to access the resource (e.g. a recognized user attempting to access restricted content)  Possible reason: Insufficient rights for the service, no rights on any service of this vehicle and/or Response is too large  The comments for the 4xx codes are from the Wikipedia article [List of HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_Client_errors), which is released under the [Creative Commons Attribution-Share-Alike License 3.0](https://creativecommons.org/licenses/by-sa/3.0/). View authors on this [page](https://en.wikipedia.org/w/index.php?title=List_of_HTTP_status_codes&action=history).
    TheRequestWasAValidRequest(ErrorObject),
    /// The requested resource could not be found but may be available again in the future. Subsequent requests by the client are permissible  Possible reason: vehicle unknown and/or rFMS-Version not supported  The comments for the 4xx codes are from the Wikipedia article [List of HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_Client_errors), which is released under the [Creative Commons Attribution-Share-Alike License 3.0](https://creativecommons.org/licenses/by-sa/3.0/). View authors on this [page](https://en.wikipedia.org/w/index.php?title=List_of_HTTP_status_codes&action=history).
    TheRequestedResourceCouldNotBeFoundButMayBeAvailableAgainInTheFuture(ErrorObject),
    /// Possible reason: unsupported Accept parameter sent
    PossibleReason(ErrorObject),
    /// The user has sent too many requests in a given amount of time. Intended for use with rate limiting schemes Possible reason Request sent too often and/or Max concurrent calls
    TheUserHasSentTooManyRequestsInAGivenAmountOfTime(ErrorObject),
}

/// Optional responses for error codes, detailing the error if needed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ErrorObject {
    /// An identifier for this error
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// A description of the error
    #[serde(rename = "error_description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,

    /// A URI providing more information
    #[serde(rename = "error_uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VehicleResponseObject {
    #[serde(rename = "vehicleResponse")]
    pub vehicle_response: VehicleResponseObjectVehicleResponse,

    /// This will be set to true if the result set was too large to be sent back in one reply. A new request must be sent to get the rest of the vehicles, where the lastVin parameter must be supplied. The lastVin should be set to the VIN of the last vehicle in the result set of this message.
    #[serde(rename = "moreDataAvailable")]
    pub more_data_available: bool,

    /// Populated with the link to the next part of the result when moreDataAvailable is true. The link is relative, i.e. starts with /rfms/vehicles, and preserves any query parameters from the original request.
    #[serde(rename = "moreDataAvailableLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_data_available_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VehicleResponseObjectVehicleResponse {
    #[serde(rename = "vehicles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicles: Option<Vec<VehicleObject>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VehicleObject {
    /// vehicle identification number. See ISO 3779 (17 characters)
    #[serde(rename = "vin")]
    pub vin: String,

    /// The customer's name for the vehicle.
    #[serde(rename = "customerVehicleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vehicle_name: Option<String>,

    /// The vehicle brand. rFMS standard values VOLVO TRUCKS, SCANIA, DAIMLER, IVECO, DAF, MAN, RENAULT TRUCKS, VDL, VOLVO BUSES, IVECO BUS, IRISBUS
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    #[serde(rename = "productionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_date: Option<VehicleObjectProductionDate>,

    /// Indicates the type of vehicle. rFMS standard values TRUCK, BUS, VAN
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Indicates the model of the vehicle. OEM specific value.
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The possible fuel types supported by this vehicle, formatted as the HEX id number according to SPN 5837. This does NOT indicate which fuel type that is presently being used.
    #[serde(rename = "possibleFuelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_fuel_type: Option<Vec<String>>,

    /// The emission level this vehicle supports. Possible values:  European Union, Heavy-Duty Truck and Bus Engines:  EURO_III, EURO_III_EEV, EURO_IV, EURO_V, EURO_VI  European Union, Nonroad Engines:  EURO_STAGE_III, EURO_STAGE_IV, EURO_STAGE_V  United_States, Heavy-Duty Truck and Bus Engines:  EPA_2004, EPA_2007, EPA_2010, EPA_2015_NOX10, EPA_2015_NOX05, EPA_2015_NOX02  United_States, Nonroad Engines:  EPA_TIER_2, EPA_TIER_3, EPA_TIER_4_2008, EPA_TIER_4_2013  Brazil, Heavy-Duty Truck and Bus Engines:  PROCONVE_P5, PROCONVE_P6, PROCONVE_P7  Brazil, Nonroad Engines:  PROCONVE_MARI
    #[serde(rename = "emissionLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_level: Option<String>,

    /// This parameter indicates how the tell tales shall be interpreted, the code is unique for each OEM. One OEM can have different interpretations  depending on vehicle type.
    #[serde(rename = "tellTaleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tell_tale_code: Option<String>,

    /// The chassis type of the vehicle. OEM specific value. This is used mainly for buses
    #[serde(rename = "chassisType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis_type: Option<String>,

    /// Number of axles on the vehicle. This is used mainly for buses
    #[serde(rename = "noOfAxles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_of_axles: Option<i32>,

    /// Total fuel tank volume for all tanks in milliltres.
    #[serde(rename = "totalFuelTankVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_fuel_tank_volume: Option<i32>,

    /// Total gas tank capacity for all tanks in kilograms.
    #[serde(rename = "totalFuelTankCapacityGaseous")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_fuel_tank_capacity_gaseous: Option<i32>,

    /// Total battery pack capacity in watt hours.
    #[serde(rename = "totalBatteryPackCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_battery_pack_capacity: Option<i32>,

    /// The type of tachograph in the vehicle. rFMS standard values MTCO, DTCO, TSU, DTCO_G1, DTCO_G2, NONE  DTCO - Digital tachograph, unknown generation  DTCO_G1 - Digital tachograph generation 1  DTCO_G2 - Digital tachograph generation 2  NONE - No tachograph in the vehicle  MTCO - Modular tachograph  TSU - Tachograph simulator
    #[serde(rename = "tachographType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tachograph_type: Option<String>,

    /// The type of gearbox the vehicle is equipped with. rFMS standard values MANUAL, AUTOMATIC, SEMI_AUTOMATIC, NO_GEAR (e.g electrical)
    #[serde(rename = "gearboxType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gearbox_type: Option<String>,

    /// The type of body on the chassis. rFMS standard values CITY_BUS, INTERCITY_BUS, COACH. This is used mainly for buses.
    #[serde(rename = "bodyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_type: Option<String>,

    /// The door configuration. The door order definition is OEM specific. E.g. [1, 2, 2] means the bus has 3 doors: 1 front door, double doors for door 2 and 3. This is used mainly for buses.
    #[serde(rename = "doorConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_configuration: Option<Vec<i32>>,

    /// If the vehicle is equipped with a ramp or not. This is used mainly for buses.
    #[serde(rename = "hasRampOrLift")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_ramp_or_lift: Option<bool>,

    /// Paths that the client is authorized to call
    #[serde(rename = "authorizedPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_paths: Option<Vec<String>>,
}

/// Indicates when the vehicle was produced.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VehicleObjectProductionDate {
    /// Day of the month where first day of the month is 1
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,

    /// Month of the year, where January is value 1
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,

    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl VehicleObject {
    pub fn new(vin: String) -> VehicleObject {
        VehicleObject {
            vin,
            customer_vehicle_name: None,
            brand: None,
            production_date: None,
            r#type: None,
            model: None,
            possible_fuel_type: None,
            emission_level: None,
            tell_tale_code: None,
            chassis_type: None,
            no_of_axles: None,
            total_fuel_tank_volume: None,
            total_fuel_tank_capacity_gaseous: None,
            total_battery_pack_capacity: None,
            tachograph_type: None,
            gearbox_type: None,
            body_type: None,
            door_configuration: None,
            has_ramp_or_lift: None,
            authorized_paths: None,
        }
    }
}

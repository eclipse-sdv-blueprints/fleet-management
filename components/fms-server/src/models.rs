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

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehiclePositionObject {
    /// vehicle identification number. See ISO 3779 (17 characters)
    #[serde(rename = "vin")]
    pub vin: String,

    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerObject,

    /// When the data was retrieved in the vehicle in iso8601 format.
    #[serde(rename = "createdDateTime")]
    pub created_date_time: chrono::DateTime::<chrono::Utc>,

    /// Reception at Server. To be used for handling of \"more data available\" in iso8601 format.
    #[serde(rename = "receivedDateTime")]
    pub received_date_time: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "gnssPosition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gnss_position: Option<GnssPositionObject>,

    /// Wheel-Based Vehicle Speed in km/h (Speed of the vehicle as calculated from wheel or tailshaft speed)
    #[serde(rename = "wheelBasedSpeed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub wheel_based_speed: Option<f64>,

    /// Tachograph vehicle speed in km/h (Speed of the vehicle registered by the tachograph)
    #[serde(rename = "tachographSpeed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tachograph_speed: Option<f64>,

}

/// This description is placed here due to limitations of describing references in OpenAPI  Property __driverId__:  The driver id of driver. (independant whether it is driver or Co-driver)  This is only set if the TriggerType = DRIVER_LOGIN, DRIVER_LOGOUT, DRIVER_1_WORKING_STATE_CHANGED or DRIVER_2_WORKING_STATE_CHANGED  For DRIVER_LOGIN it is the id of the driver that logged in  For DRIVER_LOGOUT it is the id of the driver that logged out  For DRIVER_1_WORKING_STATE_CHANGED it is the id of driver 1  For DRIVER_2_WORKING_STATE_CHANGED it is the id of driver 2  Property __tellTaleInfo__:  The tell tale(s) that triggered this message.  This is only set if the TriggerType = TELL_TALE
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TriggerObject {
    /// Trigger types for Context=RFMS:  TIMER - Data was sent due to a timer trigger. (Timer value set outside rFMS scope)  IGNITION_ON - Data was sent due to an ignition on  IGNITION_OFF - Data was sent due to an ignition off  PTO_ENABLED - Data was sent due to that a PTO was enabled, will be sent for each PTO that gets enabled  PTO_DISABLED - Data was sent due to that a PTO was disabled, will be sent for each PTO that gets disabled.  DRIVER_LOGIN - Data was sent due to a successful driver login.  DRIVER_LOGOUT - Data was sent due to a driver logout  TELL_TALE - Data was sent due to that at least one tell tale changed state  ENGINE_ON - Data was sent due to an engine on. For electric motor crank is on  ENGINE_OFF - Data was sent due to an engine off. For electric motor crank is off  DRIVER_1_WORKING_STATE_CHANGED - Data was sent due to that driver 1 changed working state  DRIVER_2_WORKING_STATE_CHANGED - Data was sent due to that driver 2 changed working state  DISTANCE_TRAVELLED - Data was sent due to that a set distance was travelled. (Distance set outside rFMS scope)  FUEL_TYPE_CHANGE - Data was sent due to that the type of fuel currently being utilized by the vehicle changed  PARKING_BRAKE_SWITCH_CHANGE - Data was sent due to that the parking brake state has changed  BATTERY_PACK_CHARGING_STATUS_CHANGE - Data was sent due to a change in the battery pack charging status.  BATTERY_PACK_CHARGING_CONNECTION_STATUS_CHANGE - Data was sent due to a change in the battery pack charging connection status.  TRAILER_CONNECTED - One or several trailers were connected  TRAILER_DISCONNECTED - One or several trailers were disconnected
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    /// The context defines if this is part of the standard or OEM specific. rFMS standard values VOLVO TRUCKS, SCANIA, DAIMLER, IVECO, DAF, MAN, RENAULT TRUCKS, VDL, VOLVO BUSES, IVECO BUS, IRISBUS If the Trigger is defined in the rFMS standard, the Context = RFMS
    #[serde(rename = "context")]
    pub context: String,

    /// Additional TriggerInfo content for OEM specific triggers E.g. TRAILER_ATTACHED_TRIGGER [id of trailer]
    #[serde(rename = "triggerInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trigger_info: Option<Vec<String>>,

    #[serde(rename = "driverId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driver_id: Option<DriverIdObject>,

    /// The id of a PTO. This is only set if the TriggerType = PTO_ENABLED or PTO_DISABLED
    #[serde(rename = "ptoId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pto_id: Option<String>,

    #[serde(rename = "tellTaleInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tell_tale_info: Option<TellTaleObject>,

    #[serde(rename = "chargingStatusInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub charging_status_info: Option<TriggerObjectChargingStatusInfo>,

    #[serde(rename = "chargingConnectionStatusInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub charging_connection_status_info: Option<TriggerObjectChargingConnectionStatusInfo>,

}

impl TriggerObject {
    #[allow(clippy::new_without_default)]
    pub fn new(trigger_type: String, context: String, ) -> TriggerObject {
        TriggerObject {
            trigger_type,
            context,
            trigger_info: None,
            driver_id: None,
            pto_id: None,
            tell_tale_info: None,
            charging_status_info: None,
            charging_connection_status_info: None,
        }
    }
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub heading: Option<i32>,

    /// The altitude of the vehicle. Where 0 is sea level, negative values below sealevel and positive above sealevel. Unit in meters.
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub altitude: Option<i32>,

    /// The GNSS(e.g. GPS)-speed in km/h
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub speed: Option<f64>,

    /// The time of the position data in iso8601 format.
    #[serde(rename = "positionDateTime")]
    pub position_date_time: chrono::DateTime::<chrono::Utc>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DriverIdObject {
    #[serde(rename = "tachoDriverIdentification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tacho_driver_identification: Option<models::DriverIdObjectTachoDriverIdentification>,

    #[serde(rename = "oemDriverIdentification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub oem_driver_identification: Option<models::DriverIdObjectOemDriverIdentification>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TellTaleObject {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "tellTale")]
    pub tell_tale: String,

    /// The OemTellTale is only set when the TellTale == OEM_SPECIFIC_TELL_TALE. This is an OEM specific string defining a tell tale in the OEM context.
    #[serde(rename = "oemTellTale")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub oem_tell_tale: Option<String>,

    /// The current state of the tell tale.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "state")]
    pub state: String,

}

/// Additional information can be provided if the trigger type is BATTERY_PACK_CHARGING_STATUS_CHANGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TriggerObjectChargingStatusInfo {
    /// CHARGING_STARTED - Charging has started  CHARGING_COMPLETED - Charging is completed  CHARGING_INTERRUPTED - Charging has been interrupted (no error)  ERROR - An error occurred when charging  ESTIMATED_COMPLETION_TIME_CHANGED - The estimated time for completed charging has changed. (Threshold is outside scope of rFMS)  TIMER - A predefined time has passed since last charge status update. (Frequency is outside the scope of rFMS)  CHARGING_LEVEL - The charging level has reached a predefined level. (Charging levels are outside the scope of rFMS)
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,

    /// Details regarding the event. Content is OEM specific
    #[serde(rename = "eventDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_detail: Option<String>,

}

/// Additional information can be provided if the trigger type is BATTERY_PACK_CHARGING_CONNECTION_STATUS_CHANGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TriggerObjectChargingConnectionStatusInfo {
    /// CONNECTING - Vehicle is being connected to a charger  CONNECTED - Vehicle is connected to a charger  DISCONNECTING - Vehicle is being disconnected from the charger  DISCONNECTED - Vehicle is not connected to a charger  ERROR - An error occurred
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,

    /// Details regarding the event. Content is OEM specific
    #[serde(rename = "eventDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_detail: Option<String>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DriverIdObjectTachoDriverIdentification {
    /// The unique identification of a driver in a Member State. This fields is formatted according the definition for driverIdentification in COMMISSION REGULATION (EC) No 1360/2002 Annex 1b
    #[serde(rename = "driverIdentification")]
    pub driver_identification: String,

    /// The country alpha code of the Member State having issued the card. This fields is formatted according the definition for NationAlpha in COMMISSION REGULATION (EC) No 1360/2002 Annex 1b
    #[serde(rename = "cardIssuingMemberState")]
    pub card_issuing_member_state: String,

    /// Code to distinguish different types of equipment for the tachograph application. See description of the field 'DriverAuthenticationEquipment' in COMMISSION REGULATION (EC) No 1360/2002 Annex 1b
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "driverAuthenticationEquipment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driver_authentication_equipment: Option<String>,

    /// A card replacement index. This fields is formatted according the definition for CardReplacementIndex (chap 2.26) in: COMMISSION REGULATION (EC) No 1360/2002 Annex 1b
    #[serde(rename = "cardReplacementIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub card_replacement_index: Option<String>,

    /// A card renewal index. This fields is formatted according the definition for CardRenewalIndex (chap 2.25) in: COMMISSION REGULATION (EC) No 1360/2002 Annex 1b
    #[serde(rename = "cardRenewalIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub card_renewal_index: Option<String>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DriverIdObjectOemDriverIdentification {
    /// Contains an optional id type (e.g. pin, USB, encrypted EU id...)
    #[serde(rename = "idType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id_type: Option<String>,

    /// An OEM specific driver id.
    #[serde(rename = "oemDriverIdentification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub oem_driver_identification: Option<String>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehiclePositionResponseObject {
    #[serde(rename = "vehiclePositionResponse")]
    pub vehicle_position_response: models::VehiclePositionResponseObjectVehiclePositionResponse,

    /// This will be set to true if the result set was too large to be sent back in one reply. A new request must be sent to get the rest of the vehicle positions, where the starttime parameter must be supplied. The starttime should be set to the latest ReceivedDateTime + 1 second of the last vehicle in the result set of this message.
    #[serde(rename = "moreDataAvailable")]
    pub more_data_available: bool,

    /// Populated with the link to the next part of the result when moreDataAvailable is true. The link is relative, i.e. starts with /rfms/vehiclepositions, and preserves any query parameters from the original request.
    #[serde(rename = "moreDataAvailableLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub more_data_available_link: Option<String>,

    /// Time to be used to ask for historical data at customers (for starttime), to solve the problem of having different times at server and clients. This is the time at the server when this request was received. To avoid losing any messages or get duplicates, this is the time that should be supplied in the startTime parameter in the next request in iso8601 format.
    #[serde(rename = "requestServerDateTime")]
    pub request_server_date_time: chrono::DateTime::<chrono::Utc>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehiclePositionResponseObjectVehiclePositionResponse {
    #[serde(rename = "vehiclePositions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicle_positions: Option<Vec<models::VehiclePositionObject>>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleResponseObject {
    #[serde(rename = "vehicleResponse")]
    pub vehicle_response: models::VehicleResponseObjectVehicleResponse,

    /// This will be set to true if the result set was too large to be sent back in one reply. A new request must be sent to get the rest of the vehicles, where the lastVin parameter must be supplied. The lastVin should be set to the VIN of the last vehicle in the result set of this message.
    #[serde(rename = "moreDataAvailable")]
    pub more_data_available: bool,

    /// Populated with the link to the next part of the result when moreDataAvailable is true. The link is relative, i.e. starts with /rfms/vehicles, and preserves any query parameters from the original request.
    #[serde(rename = "moreDataAvailableLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub more_data_available_link: Option<String>,

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum VehiclesGetResponse {
    /// OK
    OK
    (models::VehicleResponseObject)
    ,
    /// The server cannot or will not process the request due to an apparent client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing)  Possible reason: Mandatory field missing, e.g. Authentication Header empty or missing  The comments for the 4xx codes are from the Wikipedia article [List of HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_Client_errors), which is released under the [Creative Commons Attribution-Share-Alike License 3.0](https://creativecommons.org/licenses/by-sa/3.0/). View authors on this [page](https://en.wikipedia.org/w/index.php?title=List_of_HTTP_status_codes&action=history).
    TheServerCannotOrWillNotProcessTheRequestDueToAnApparentClientError
    (models::ErrorObject)
    ,
    /// Similar to 403 Forbidden, but specifically for use when authentication is required and has failed or has not yet been provided. The response must include a WWW-Authenticate header field containing a challenge applicable to the requested resource. See Basic access authentication and Digest access authentication.  Possible reasons: Wrong credentials, Login credentials expired and/or Access token not valid or expired  The comments for the 4xx codes are from the Wikipedia article [List of HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_Client_errors), which is released under the [Creative Commons Attribution-Share-Alike License 3.0](https://creativecommons.org/licenses/by-sa/3.0/). View authors on this [page](https://en.wikipedia.org/w/index.php?title=List_of_HTTP_status_codes&action=history).
    SimilarTo
    (models::ErrorObject)
    ,
    /// The request was a valid request, but the server is refusing to respond to it. Unlike a 401 Unauthorized response, authenticating will make no difference. On servers where authentication is required, this commonly means that the provided credentials were successfully authenticated but that the credentials still do not grant the client permission to access the resource (e.g. a recognized user attempting to access restricted content)  Possible reason: Insufficient rights for the service, no rights on any service of this vehicle and/or Response is too large  The comments for the 4xx codes are from the Wikipedia article [List of HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_Client_errors), which is released under the [Creative Commons Attribution-Share-Alike License 3.0](https://creativecommons.org/licenses/by-sa/3.0/). View authors on this [page](https://en.wikipedia.org/w/index.php?title=List_of_HTTP_status_codes&action=history).
    TheRequestWasAValidRequest
    (models::ErrorObject)
    ,
    /// The requested resource could not be found but may be available again in the future. Subsequent requests by the client are permissible  Possible reason: vehicle unknown and/or rFMS-Version not supported  The comments for the 4xx codes are from the Wikipedia article [List of HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_Client_errors), which is released under the [Creative Commons Attribution-Share-Alike License 3.0](https://creativecommons.org/licenses/by-sa/3.0/). View authors on this [page](https://en.wikipedia.org/w/index.php?title=List_of_HTTP_status_codes&action=history).
    TheRequestedResourceCouldNotBeFoundButMayBeAvailableAgainInTheFuture
    (models::ErrorObject)
    ,
    /// Possible reason: unsupported Accept parameter sent
    PossibleReason
    (models::ErrorObject)
    ,
    /// The user has sent too many requests in a given amount of time. Intended for use with rate limiting schemes Possible reason Request sent too often and/or Max concurrent calls 
    TheUserHasSentTooManyRequestsInAGivenAmountOfTime
    (models::ErrorObject)
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleResponseObjectVehicleResponse {
    #[serde(rename = "vehicles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicles: Option<Vec<models::VehicleObject>>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleObject {
    /// vehicle identification number. See ISO 3779 (17 characters)
    #[serde(rename = "vin")]
    pub vin: String,

    /// The customer's name for the vehicle.
    #[serde(rename = "customerVehicleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_vehicle_name: Option<String>,

    /// The vehicle brand. rFMS standard values VOLVO TRUCKS, SCANIA, DAIMLER, IVECO, DAF, MAN, RENAULT TRUCKS, VDL, VOLVO BUSES, IVECO BUS, IRISBUS
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub brand: Option<String>,

    #[serde(rename = "productionDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub production_date: Option<models::VehicleObjectProductionDate>,

    /// Indicates the type of vehicle. rFMS standard values TRUCK, BUS, VAN
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<String>,

    /// Indicates the model of the vehicle. OEM specific value.
    #[serde(rename = "model")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub model: Option<String>,

    /// The possible fuel types supported by this vehicle, formatted as the HEX id number according to SPN 5837. This does NOT indicate which fuel type that is presently being used.
    #[serde(rename = "possibleFuelType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub possible_fuel_type: Option<Vec<String>>,

    /// The emission level this vehicle supports. Possible values:  European Union, Heavy-Duty Truck and Bus Engines:  EURO_III, EURO_III_EEV, EURO_IV, EURO_V, EURO_VI  European Union, Nonroad Engines:  EURO_STAGE_III, EURO_STAGE_IV, EURO_STAGE_V  United_States, Heavy-Duty Truck and Bus Engines:  EPA_2004, EPA_2007, EPA_2010, EPA_2015_NOX10, EPA_2015_NOX05, EPA_2015_NOX02  United_States, Nonroad Engines:  EPA_TIER_2, EPA_TIER_3, EPA_TIER_4_2008, EPA_TIER_4_2013  Brazil, Heavy-Duty Truck and Bus Engines:  PROCONVE_P5, PROCONVE_P6, PROCONVE_P7  Brazil, Nonroad Engines:  PROCONVE_MARI
    #[serde(rename = "emissionLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emission_level: Option<String>,

    /// This parameter indicates how the tell tales shall be interpreted, the code is unique for each OEM. One OEM can have different interpretations  depending on vehicle type.
    #[serde(rename = "tellTaleCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tell_tale_code: Option<String>,

    /// The chassis type of the vehicle. OEM specific value. This is used mainly for buses
    #[serde(rename = "chassisType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chassis_type: Option<String>,

    /// Number of axles on the vehicle. This is used mainly for buses
    #[serde(rename = "noOfAxles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub no_of_axles: Option<i32>,

    /// Total fuel tank volume for all tanks in milliltres.
    #[serde(rename = "totalFuelTankVolume")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_fuel_tank_volume: Option<i32>,

    /// Total gas tank capacity for all tanks in kilograms.
    #[serde(rename = "totalFuelTankCapacityGaseous")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_fuel_tank_capacity_gaseous: Option<i32>,

    /// Total battery pack capacity in watt hours.
    #[serde(rename = "totalBatteryPackCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_battery_pack_capacity: Option<i32>,

    /// The type of tachograph in the vehicle. rFMS standard values MTCO, DTCO, TSU, DTCO_G1, DTCO_G2, NONE  DTCO - Digital tachograph, unknown generation  DTCO_G1 - Digital tachograph generation 1  DTCO_G2 - Digital tachograph generation 2  NONE - No tachograph in the vehicle  MTCO - Modular tachograph  TSU - Tachograph simulator
    #[serde(rename = "tachographType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tachograph_type: Option<String>,

    /// The type of gearbox the vehicle is equipped with. rFMS standard values MANUAL, AUTOMATIC, SEMI_AUTOMATIC, NO_GEAR (e.g electrical)
    #[serde(rename = "gearboxType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gearbox_type: Option<String>,

    /// The type of body on the chassis. rFMS standard values CITY_BUS, INTERCITY_BUS, COACH. This is used mainly for buses.
    #[serde(rename = "bodyType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_type: Option<String>,

    /// The door configuration. The door order definition is OEM specific. E.g. [1, 2, 2] means the bus has 3 doors: 1 front door, double doors for door 2 and 3. This is used mainly for buses.
    #[serde(rename = "doorConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub door_configuration: Option<Vec<i32>>,

    /// If the vehicle is equipped with a ramp or not. This is used mainly for buses.
    #[serde(rename = "hasRampOrLift")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_ramp_or_lift: Option<bool>,

    /// Paths that the client is authorized to call
    #[serde(rename = "authorizedPaths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authorized_paths: Option<Vec<String>>,

}

impl VehicleObject {
    pub fn new(vin : String) -> VehicleObject {
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

/// Indicates when the vehicle was produced.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleObjectProductionDate {
    /// Day of the month where first day of the month is 1
    #[serde(rename = "day")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub day: Option<i32>,

    /// Month of the year, where January is value 1
    #[serde(rename = "month")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub month: Option<i32>,

    #[serde(rename = "year")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub year: Option<i32>,

}

/// Optional responses for error codes, detailing the error if needed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorObject {
    /// An identifier for this error
    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<String>,

    /// A description of the error
    #[serde(rename = "error_description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_description: Option<String>,

    /// A URI providing more information
    #[serde(rename = "error_uri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_uri: Option<String>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleStatusResponseObject {
    #[serde(rename = "vehicleStatusResponse")]
    pub vehicle_status_response: models::VehicleStatusResponseObjectVehicleStatusResponse,

    /// This will be set to true if the result set was too large to be sent back in one reply. A new request must be done to get the rest of the vehicle statuses, where the starttime parameter must be supplied. The starttime should be set to the ReceivedDateTime + 1 second of the last vehicle in the result set of this message.
    #[serde(rename = "moreDataAvailable")]
    pub more_data_available: bool,

    /// Populated with the link to the next part of the result when moreDataAvailable is true. The link is relative, i.e. starts with /rfms/vehiclestatuses, and preserves any query parameters from the original request.
    #[serde(rename = "moreDataAvailableLink")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub more_data_available_link: Option<String>,

    /// Time in UTC to be used to ask for historical data (for starttime), to solve the problem of having different times at server and clients. This is the time at the server when this request was received. To avoid losing any messages or get duplicates, this is the time that should be supplied in the startTime parameter in the next request in iso8601 format.
    #[serde(rename = "requestServerDateTime")]
    pub request_server_date_time: chrono::DateTime::<chrono::Utc>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleStatusResponseObjectVehicleStatusResponse {
    #[serde(rename = "vehicleStatuses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicle_statuses: Option<Vec<models::VehicleStatusObject>>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleStatusObject {
    /// vehicle identification number. See ISO 3779 (17 characters)
    #[serde(rename = "vin")]
    pub vin: String,

    #[serde(rename = "triggerType")]
    pub trigger_type: models::TriggerObject,

    /// When the data was retrieved in the vehicle in iso8601 format.
    #[serde(rename = "createdDateTime")]
    pub created_date_time: chrono::DateTime::<chrono::Utc>,

    /// Reception at Server. To be used for handling of \"more data available\" in iso8601 format.
    #[serde(rename = "receivedDateTime")]
    pub received_date_time: chrono::DateTime::<chrono::Utc>,

    /// Accumulated distance travelled by the vehicle during its operation in meter
    #[serde(rename = "hrTotalVehicleDistance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hr_total_vehicle_distance: Option<i64>,

    /// The total hours of operation for the vehicle combustion engine. At least one of totalEngineHours or totalElectricMotorHours is Mandatory
    #[serde(rename = "totalEngineHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_engine_hours: Option<f64>,

    /// The total hours the electric motor is ready for propulsion (i.e. crank mode). At least one of totalEngineHours or totalElectricMotorHours is mandatory
    #[serde(rename = "totalElectricMotorHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_electric_motor_hours: Option<f64>,

    #[serde(rename = "driver1Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driver1_id: Option<models::DriverIdObject>,

    /// The full vehicle weight in kg
    #[serde(rename = "grossCombinationVehicleWeight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gross_combination_vehicle_weight: Option<i32>,

    /// The total fuel the vehicle has used during its lifetime in MilliLitres. At least one of engineTotalFuelUsed, totalFuelUsedGaseous or totalElectricEnergyUsed is mandatory.
    #[serde(rename = "engineTotalFuelUsed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_total_fuel_used: Option<i64>,

    /// Total fuel consumed in kg (trip drive fuel + trip PTO governor moving fuel + trip PTO governor non-moving fuel + trip idle fuel) over the life of the engine. At least one of engineTotalFuelUsed, totalFuelUsedGaseous or totalElectricEnergyUsed is mandatory.
    #[serde(rename = "totalFuelUsedGaseous")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_fuel_used_gaseous: Option<i64>,

    /// Total electric energy consumed by the vehicle, excluding when plugged in (vehicle coupler) for charging, (incl. motor, PTO, cooling, etc.) in watt hours. Recuperation is subtracted from the value.  At least one of engineTotalFuelUsed, totalFuelUsedGaseous or totalElectricEnergyUsed is mandatory.
    #[serde(rename = "totalElectricEnergyUsed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_electric_energy_used: Option<i64>,

    /// Composite indication of all bus door statuses. Bus specific parameter
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status2OfDoors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status2_of_doors: Option<String>,

    /// Individual status for each door. Bus specific parameter
    #[serde(rename = "doorStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub door_status: Option<Vec<models::VehicleStatusObjectDoorStatusInner>>,

    #[serde(rename = "accumulatedData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accumulated_data: Option<models::AccumulatedDataObject>,

    #[serde(rename = "snapshotData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_data: Option<models::SnapshotDataObject>,

    #[serde(rename = "uptimeData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uptime_data: Option<models::UptimeDataObject>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleStatusObjectDoorStatusInner {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "DoorEnabledStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub door_enabled_status: Option<String>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "DoorOpenStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub door_open_status: Option<String>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "DoorLockStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub door_lock_status: Option<String>,

    #[serde(rename = "DoorNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub door_number: Option<i32>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AccumulatedDataObject {
    /// The time the vehicle speed has been over zero.
    #[serde(rename = "durationWheelbasedSpeedOverZero")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration_wheelbased_speed_over_zero: Option<i64>,

    /// The distance the vehicle has been driven with cruise control active
    #[serde(rename = "distanceCruiseControlActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub distance_cruise_control_active: Option<i64>,

    /// The time the vehicle has been driven with cruise control active
    #[serde(rename = "durationCruiseControlActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration_cruise_control_active: Option<i64>,

    /// The fuel the vehicle has consumed while driven with cruise control active, in millilitres
    #[serde(rename = "fuelConsumptionDuringCruiseActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_consumption_during_cruise_active: Option<i64>,

    /// The gas the vehicle has consumed while driven with cruise control active, in kilograms.
    #[serde(rename = "fuelConsumptionDuringCruiseActiveGaseous")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_consumption_during_cruise_active_gaseous: Option<i64>,

    /// The electric energy the vehicle has consumed while driven with cruise control active, in watt-hours.
    #[serde(rename = "electricEnergyConsumptionDuringCruiseActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_energy_consumption_during_cruise_active: Option<i64>,

    /// The time the vehicle speed has been equal to zero, in seconds. Engine on (RPM>0 or electic motor in crank mode) and no PTO active
    #[serde(rename = "durationWheelbasedSpeedZero")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration_wheelbased_speed_zero: Option<i64>,

    /// The fuel the vehicle has consumed while the vehicle speed has been equal to zero. Engine on (RPM>0) and no PTO active. Unit in millilitres.
    #[serde(rename = "fuelWheelbasedSpeedZero")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_wheelbased_speed_zero: Option<i64>,

    /// The gas the vehicle has consumed while the vehicle speed has been equal to zero. Engine on (RPM>0) and no PTO active. Unit in kilograms.
    #[serde(rename = "fuelWheelbasedSpeedZeroGaseous")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_wheelbased_speed_zero_gaseous: Option<i64>,

    /// The electric energy the vehicle has consumed while the vehicle speed has been equal to zero. Electric motor is in crank mode and no PTO active. Unit in watt-hours.
    #[serde(rename = "electricEnergyWheelbasedSpeedZero")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_energy_wheelbased_speed_zero: Option<i64>,

    /// The fuel the vehicle has consumed while the vehicle speed has been over zero. Engine on (RPM>0). Unit in millilitres.
    #[serde(rename = "fuelWheelbasedSpeedOverZero")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_wheelbased_speed_over_zero: Option<i64>,

    /// The gas the vehicle has consumed while the vehicle speed has been over zero. Engine on (RPM>0). Unit in kilograms.
    #[serde(rename = "fuelWheelbasedSpeedOverZeroGaseous")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_wheelbased_speed_over_zero_gaseous: Option<i64>,

    /// The electric energy the vehicle has consumed (including recuperation) while the vehicle speed has been over zero. Electric motor is in crank mode. Unit in watt-hours.
    #[serde(rename = "electricEnergyWheelbasedSpeedOverZero")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_energy_wheelbased_speed_over_zero: Option<i64>,

    /// The electric energy the auxiliary systems have consumed, in watt hours. Auxiliary systems are all consumers except electric motor(s) and PTO(s). 
    #[serde(rename = "electricEnergyAux")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_energy_aux: Option<i64>,

    /// Label WHEELBASED_SPEED_ZERO  At least one PTO active during wheelbased speed=0  Counters for time (seconds) and consumption (millilitres, kilograms, watt-hours)  Label WHEELBASED_SPEED_OVER_ZERO  At least one PTO active during wheelbased speed>0  Counters for time (seconds), distance (meter) and consumption (millilitres, kilograms, watt-hours)
    #[serde(rename = "ptoActiveClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pto_active_class: Option<Vec<models::LabelObject>>,

    /// The total number of times the brake pedal has been used while the vehicle was driving.
    #[serde(rename = "brakePedalCounterSpeedOverZero")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub brake_pedal_counter_speed_over_zero: Option<i64>,

    /// The total distance the vehicle has driven where the brake pedal has been used. Unit Meters.
    #[serde(rename = "distanceBrakePedalActiveSpeedOverZero")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub distance_brake_pedal_active_speed_over_zero: Option<i64>,

    /// In percent. Minimum 5 classes [0, 20[ [20, 40[ [40, 60[ [60, 80[ [80, 100]
    #[serde(rename = "accelerationPedalPositionClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceleration_pedal_position_class: Option<Vec<models::FromToClassObject>>,

    /// In percent. Minimum 5 classes [0, 20[ [20, 40[ [40, 60[ [60, 80[ [80, 100]
    #[serde(rename = "brakePedalPositionClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub brake_pedal_position_class: Option<Vec<models::FromToClassObject>>,

    /// In m/s2 Minimum 13 classes. ], -1.1] ]-1.1, -0.9] ]-0.9, -0.7] ]-0.7, -0.5] ]-0.5, -0.3] ]-0.3, -0.1] ]-0.1, 0.1[ [0.1, 0.3[ [0.3, 0.5[ [0.5, 0.7[ [0.7, 0.9[ [0.9, 1.1[ [1.1, [
    #[serde(rename = "accelerationClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceleration_class: Option<Vec<models::FromToClassObject>>,

    /// In m/s2 Minimum 11 classes ], -3.0] ]-3.0, -2.5] ]-2.5, -2.0] ]-2.0, -1.5] ]-1.5, -1.1] ]-1.1, 1.1[ [1.1, 1.5[ [1.5, 2.0[ [2.0, 2.5[ [2.5, 3.0[ [3.0, [
    #[serde(rename = "highAccelerationClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub high_acceleration_class: Option<Vec<models::FromToClassObject>>,

    /// In percent (how the retarder is used as a positive value). Minimum 5 classes ]0, 20[ [20, 40[ [40, 60[ [60, 80[ [80, 100]
    #[serde(rename = "retarderTorqueClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retarder_torque_class: Option<Vec<models::FromToClassObject>>,

    /// Driving without torque, with gear (clutch is engaged) Labels DRIVING_WITHOUT_TORQUE
    #[serde(rename = "drivingWithoutTorqueClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driving_without_torque_class: Option<Vec<models::LabelObject>>,

    /// In percent based on EEC1 value (Actual Engine-Percent Torque). Minimum 10 classes [0, 10[ [10, 20[ [20, 30[ [30, 40[ [40, 50[ [50, 60[ [60, 70[ [70, 80[ [80, 90[ [90, 100]
    #[serde(rename = "engineTorqueClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_torque_class: Option<Vec<models::FromToClassObjectCombustion>>,

    /// In percent (Actual Engine-Percent Torque). Minimum 10 classes [0, 10[ [10, 20[ [20, 30[ [30, 40[ [40, 50[ [50, 60[ [60, 70[ [70, 80[ [80, 90[ [90, 100]
    #[serde(rename = "electricMotorTorqueClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_motor_torque_class: Option<Vec<models::FromToClassObjectElectrical>>,

    /// In percent based on EEC2 value (Engine Percent Load At Current Speed). Minimum 10 classes [0, 10[ [10, 20[ [20, 30[ [30, 40[ [40, 50[ [50, 60[ [60, 70[ [70, 80[ [80, 90[ [90, 100]
    #[serde(rename = "engineTorqueAtCurrentSpeedClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_torque_at_current_speed_class: Option<Vec<models::FromToClassObjectCombustion>>,

    /// In percent (Engine Percent Load At Current Speed). Minimum 10 classes [0, 10[ [10, 20[ [20, 30[ [30, 40[ [40, 50[ [50, 60[ [60, 70[ [70, 80[ [80, 90[ [90, 100]
    #[serde(rename = "electricMotorTorqueAtCurrentSpeedClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_motor_torque_at_current_speed_class: Option<Vec<models::FromToClassObjectElectrical>>,

    /// In km/h Minimum 40 classes. [0, 4[ [4, 8[ [8, 12[ [12, 16[ [16, 20[ [20, 24[ ... [156, [ Engine on (RPM>0 or electric motor in crank mode)
    #[serde(rename = "vehicleSpeedClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicle_speed_class: Option<Vec<models::FromToClassObject>>,

    /// Classes refer to the RPM of the combustion engine. Only mandatory if the vehicle has a combustion engine for propulsion. Minimum 10 classes [0, 400[ [400, 800[ [800, 1200[ [1200, 1600[ [1600, 2000[ [2000, 2400[ [2400, 2800[ [2800, 3200[ [3200, 3600[ [3600, [ Note: Engine on (RPM>0 or electric motor in crank mode)
    #[serde(rename = "engineSpeedClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_speed_class: Option<Vec<models::FromToClassObject>>,

    /// In m/s2 Minimum 13 classes. ], -1.1] ]-1.1, -0.9] ]-0.9, -0.7] ]-0.7, -0.5] ]-0.5, -0.3] ]-0.3, -0.1] ]-0.1, 0.1[ [0.1, 0.3[ [0.3, 0.5[ [0.5, 0.7[ [0.7, 0.9[ [0.9, 1.1[ [1.1, [
    #[serde(rename = "accelerationDuringBrakeClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceleration_during_brake_class: Option<Vec<models::FromToClassObject>>,

    /// The currently selected gear One class per gear. Neutral is also a gear. Park is also a gear. This is formatted according to SPN 524, supplied as a decimal value. Example 0 = Neutral, 1 = 1:st gear... This is mainly used for Buses.
    #[serde(rename = "selectedGearClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_gear_class: Option<Vec<models::LabelObject>>,

    /// The currently used gear One class per gear. Neutral is also a gear. Park is also a gear. This is formatted according to SPN 523, supplied as a decimal value. Example 0 = Neutral, 1 = 1:st gear... This is mainly used for Buses.
    #[serde(rename = "currentGearClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_gear_class: Option<Vec<models::LabelObject>>,

    /// The total number of times the chairlift has been outside the bus. This is mainly used for Buses
    #[serde(rename = "chairliftCounter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chairlift_counter: Option<i64>,

    /// The total number of stop requests made. This is mainly used for Buses
    #[serde(rename = "stopRequestCounter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stop_request_counter: Option<i64>,

    /// The total number of times the bus has knelt.
    #[serde(rename = "kneelingCounter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kneeling_counter: Option<i64>,

    /// The total number of pram requests made. This is mainly used for Buses
    #[serde(rename = "pramRequestCounter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pram_request_counter: Option<i64>,

    /// Classes refer to the recuperated electric power in kilowatt Minimum 11 classes [0, 100[ [100, 200[ [200, 300[ ... [900, 1000[ [1000, [
    #[serde(rename = "electricPowerRecuperationClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_power_recuperation_class: Option<Vec<models::FromToClassObjectElectrical>>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LabelObject {
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seconds: Option<i64>,

    #[serde(rename = "meters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meters: Option<i64>,

    #[serde(rename = "milliLitres")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milli_litres: Option<i64>,

    #[serde(rename = "kilograms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kilograms: Option<i64>,

    #[serde(rename = "watthours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watthours: Option<i64>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FromToClassObject {
    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<f64>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<f64>,

    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seconds: Option<i64>,

    #[serde(rename = "meters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meters: Option<i64>,

    #[serde(rename = "milliLitres")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milli_litres: Option<i64>,

    #[serde(rename = "kilograms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kilograms: Option<i64>,

    #[serde(rename = "watthours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watthours: Option<i64>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FromToClassObjectCombustion {
    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<f64>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<f64>,

    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seconds: Option<i64>,

    #[serde(rename = "meters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meters: Option<i64>,

    #[serde(rename = "milliLitres")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milli_litres: Option<i64>,

    #[serde(rename = "kilograms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kilograms: Option<i64>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FromToClassObjectElectrical {
    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<f64>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<f64>,

    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seconds: Option<i64>,

    #[serde(rename = "meters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meters: Option<i64>,

    #[serde(rename = "watthours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watthours: Option<i64>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SnapshotDataObject {
    #[serde(rename = "gnssPosition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gnss_position: Option<models::GnssPositionObject>,

    /// The vehicle wheelbased speed
    #[serde(rename = "wheelBasedSpeed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub wheel_based_speed: Option<f64>,

    /// The Tacho speed
    #[serde(rename = "tachographSpeed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tachograph_speed: Option<f64>,

    /// The engine (Diesel/gaseous) speed in rev/min
    #[serde(rename = "engineSpeed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_speed: Option<f64>,

    /// The electric motor speed in rev/min
    #[serde(rename = "electricMotorSpeed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_motor_speed: Option<f64>,

    /// Type of fuel currently being utilized by the vehicle acc. SPN 5837
    #[serde(rename = "fuelType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_type: Option<String>,

    /// The fuel level percentage
    #[serde(rename = "fuelLevel1")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_level1: Option<f64>,

    /// Ratio of volume of fuel to the total volume of fuel storage container, in percent. When Fuel Level 2 is not used, Fuel Level 1 represents the total fuel in all fuel storage containers.  When Fuel Level 2 is used, Fuel Level 1 represents the fuel level in the primary or left-side fuel storage container.
    #[serde(rename = "fuelLevel2")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel_level2: Option<f64>,

    /// The adblue level percentage
    #[serde(rename = "catalystFuelLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub catalyst_fuel_level: Option<f64>,

    #[serde(rename = "driver1WorkingState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driver1_working_state: Option<models::DriverWorkingStateProperty>,

    #[serde(rename = "driver2Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driver2_id: Option<models::DriverIdObject>,

    #[serde(rename = "driver2WorkingState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driver2_working_state: Option<models::DriverWorkingStateProperty>,

    /// The Ambient air temperature in Celsius
    #[serde(rename = "ambientAirTemperature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ambient_air_temperature: Option<f64>,

    /// Switch signal which indicates when the parking brake is set. In general the switch actuated by the operator's park brake control, whether a pedal, lever or other control mechanism  true - parking brake set  false - parking brake not set
    #[serde(rename = "parkingBrakeSwitch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parking_brake_switch: Option<bool>,

    /// Indicates the hybrid battery pack remaining charge.  0% means no charge remaining,  100% means full charge remaining.  Is used as well for full electrical vehicles
    #[serde(rename = "hybridBatteryPackRemainingCharge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hybrid_battery_pack_remaining_charge: Option<f64>,

    /// Indicates the charging status of the battery pack. Recuperation is excluded.  Not charging - No charging  Charging - Charging ongoing (AC or DC is unknown)  Charging AC - AC charging ongoing  Charging DC - DC charging ongoing  Error - An error occurred when charging  Not available - Charging status is not available
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "batteryPackChargingStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub battery_pack_charging_status: Option<String>,

    /// Indicates the charging connection status of the battery pack.  Connecting - A charger is being connected  Connected - A charger is connected  Disconnecting - A charger is being disconnected  Disconnected - No charger is connected  Error - An error occurred when connecting or disconnecting  Not available - Charging connection status is not available
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "batteryPackChargingConnectionStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub battery_pack_charging_connection_status: Option<String>,

    /// Device used to charge the battery pack. Standard rFMS values taken from ISO 15118 (OEM can have additional values):  ACD - Automatic Connection Device  WPT - Wireless Power Transfer  VEHICLE_COUPLER - manual connection of a flexible cable to an EV  NONE - No device connected  NOT_AVAILABLE - Unknown
    #[serde(rename = "batteryPackChargingDevice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub battery_pack_charging_device: Option<String>,

    /// Charging power in watts.
    #[serde(rename = "batteryPackChargingPower")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub battery_pack_charging_power: Option<f64>,

    /// Estimated time when charging has reached the target level.
    #[serde(rename = "estimatedTimeBatteryPackChargingCompleted")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub estimated_time_battery_pack_charging_completed: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "estimatedDistanceToEmpty")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub estimated_distance_to_empty: Option<models::SnapshotDataObjectEstimatedDistanceToEmpty>,

    /// A list of vehicle axles
    #[serde(rename = "vehicleAxles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicle_axles: Option<Vec<models::SnapshotDataObjectVehicleAxlesInner>>,

    /// List of trailers connected to the truck.
    #[serde(rename = "trailers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailers: Option<Vec<models::SnapshotDataObjectTrailersInner>>,

}

/// Tachograph Working state of the driver
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum DriverWorkingStateProperty {
    #[serde(rename = "REST")]
    Rest,
    #[serde(rename = "DRIVER_AVAILABLE")]
    DriverAvailable,
    #[serde(rename = "WORK")]
    Work,
    #[serde(rename = "DRIVE")]
    Drive,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "NOT_AVAILABLE")]
    NotAvailable,
}

/// Estimated distance to empty (tanks and/or battery packs) in meters
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SnapshotDataObjectEstimatedDistanceToEmpty {
    /// Estimated distance to empty, summarizing fuel, gas and battery in meters
    #[serde(rename = "total")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,

    /// Estimated distance to empty, fuel tank, in meters
    #[serde(rename = "fuel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fuel: Option<i64>,

    /// Estimated distance to empty, gas tank, in meters
    #[serde(rename = "gas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gas: Option<i64>,

    /// Estimated distance to empty, battery pack, in meters
    #[serde(rename = "batteryPack")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub battery_pack: Option<i64>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SnapshotDataObjectVehicleAxlesInner {
    /// Axle position from 1 to 15, 1 being in the front of the truck
    #[serde(rename = "vehicleAxlePosition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicle_axle_position: Option<i32>,

    /// The static vertical load of a vehicle axle in kilograms.
    #[serde(rename = "vehicleAxleLoad")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicle_axle_load: Option<f64>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SnapshotDataObjectTrailersInner {
    /// Trailer number from 1 to 5, 1 being closest to the truck, according to ISO 11992-2.
    #[serde(rename = "trailerNo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailer_no: Option<i32>,

    /// The identification data sent by the trailer to the truck in the RGE23 message of ISO 11992-2. An alternative source is the DID (Data identifier definition) record VIN, as specified in ISO 11992-4. Even though both ISO 11992-2 and ISO 11992-4 specifies this as a VIN, the actual data sent from a trailer is not always the true VIN of the trailer.
    #[serde(rename = "trailerIdentificationData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailer_identification_data: Option<String>,

    /// The vehicle identification number of the trailer. See ISO 3779 (17 characters) If the trailerIdentificationData is reporting a true VIN, trailerVin will have the same value. If it is possible to map the trailerIdentificationData to a true VIN using other sources, the value can be provided here.
    #[serde(rename = "trailerVin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailer_vin: Option<String>,

    /// The customer's name for the trailer
    #[serde(rename = "customerTrailerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_trailer_name: Option<String>,

    /// Indicates the type of the trailer. The type is sent in the EBS24 message of  ISO 11992-2.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "trailerType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailer_type: Option<String>,

    /// The sum of the static vertical loads of the trailer axles in kilograms. The load is sent in the EBS22 message of ISO 11992-2.
    #[serde(rename = "trailerAxleLoadSum")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailer_axle_load_sum: Option<i32>,

    /// A list of trailer axles
    #[serde(rename = "trailerAxles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailer_axles: Option<Vec<models::SnapshotDataObjectTrailersInnerTrailerAxlesInner>>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SnapshotDataObjectTrailersInnerTrailerAxlesInner {
    /// Axle position from 1 to 15, 1 being in the front closest to the truck, according to ISO 11992-2.
    #[serde(rename = "trailerAxlePosition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailer_axle_position: Option<i32>,

    /// The static vertical load of a trailer axle in kilograms. The load is sent in the RGE22 message of ISO11992-2.
    #[serde(rename = "trailerAxleLoad")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailer_axle_load: Option<f64>,

}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UptimeDataObject {
    /// List of tell tales with the actual status for each tell tale.
    #[serde(rename = "tellTaleInfo")]
    pub tell_tale_info: Vec<models::TellTaleObject>,

    /// The distance in meter to the next service
    #[serde(rename = "serviceDistance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_distance: Option<i64>,

    /// The temperature of the coolant liquid in Celsius
    #[serde(rename = "engineCoolantTemperature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_coolant_temperature: Option<f64>,

    /// The temperature of the battery pack coolant in Celsius HVESS - High Voltage Energy Storage System
    #[serde(rename = "hvessOutletCoolantTemperature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hvess_outlet_coolant_temperature: Option<f64>,

    /// The temperature of the battery pack in Celsius HVESS - High Voltage Energy Storage System
    #[serde(rename = "hvessTemperature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hvess_temperature: Option<f64>,

    /// The air pressure in circuit 1 in Pascal.
    #[serde(rename = "serviceBrakeAirPressureCircuit1")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_brake_air_pressure_circuit1: Option<i64>,

    /// The air pressure in circuit 2 in Pascal.
    #[serde(rename = "serviceBrakeAirPressureCircuit2")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_brake_air_pressure_circuit2: Option<i64>,

    /// The total time at least one door has been opened in the bus. (seconds) Used mainly for buses.
    #[serde(rename = "durationAtLeastOneDoorOpen")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration_at_least_one_door_open: Option<i64>,

    #[serde(rename = "alternatorInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alternator_info: Option<models::UptimeDataObjectAlternatorInfo>,

    /// The bellow pressure in the front axle left side in Pascal. Used mainly for buses.
    #[serde(rename = "bellowPressureFrontAxleLeft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bellow_pressure_front_axle_left: Option<i64>,

    /// The bellow pressure in the front axle right side in Pascal. Used mainly for buses.
    #[serde(rename = "bellowPressureFrontAxleRight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bellow_pressure_front_axle_right: Option<i64>,

    /// The bellow pressure in the rear axle left side in Pascal. Used mainly for buses.
    #[serde(rename = "bellowPressureRearAxleLeft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bellow_pressure_rear_axle_left: Option<i64>,

    /// The bellow pressure in the rear axle right side in Pascal. Used mainly for buses.
    #[serde(rename = "bellowPressureRearAxleRight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bellow_pressure_rear_axle_right: Option<i64>,

}

/// The alternator status of the up to 4 alternators. Used mainly for buses.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UptimeDataObjectAlternatorInfo {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "alternatorStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alternator_status: Option<String>,

    #[serde(rename = "alternatorNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alternator_number: Option<i64>,

}


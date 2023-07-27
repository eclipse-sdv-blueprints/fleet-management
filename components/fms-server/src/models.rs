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
                vin: vin, 
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



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

pub mod position;
pub mod status;
pub mod vehicle;

/// This description is placed here due to limitations of describing references in OpenAPI  Property __driverId__:  The driver id of driver. (independant whether it is driver or Co-driver)  This is only set if the TriggerType = DRIVER_LOGIN, DRIVER_LOGOUT, DRIVER_1_WORKING_STATE_CHANGED or DRIVER_2_WORKING_STATE_CHANGED  For DRIVER_LOGIN it is the id of the driver that logged in  For DRIVER_LOGOUT it is the id of the driver that logged out  For DRIVER_1_WORKING_STATE_CHANGED it is the id of driver 1  For DRIVER_2_WORKING_STATE_CHANGED it is the id of driver 2  Property __tellTaleInfo__:  The tell tale(s) that triggered this message.  This is only set if the TriggerType = TELL_TALE
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TriggerObject {
    /// Trigger types for Context=RFMS:  TIMER - Data was sent due to a timer trigger. (Timer value set outside rFMS scope)  IGNITION_ON - Data was sent due to an ignition on  IGNITION_OFF - Data was sent due to an ignition off  PTO_ENABLED - Data was sent due to that a PTO was enabled, will be sent for each PTO that gets enabled  PTO_DISABLED - Data was sent due to that a PTO was disabled, will be sent for each PTO that gets disabled.  DRIVER_LOGIN - Data was sent due to a successful driver login.  DRIVER_LOGOUT - Data was sent due to a driver logout  TELL_TALE - Data was sent due to that at least one tell tale changed state  ENGINE_ON - Data was sent due to an engine on. For electric motor crank is on  ENGINE_OFF - Data was sent due to an engine off. For electric motor crank is off  DRIVER_1_WORKING_STATE_CHANGED - Data was sent due to that driver 1 changed working state  DRIVER_2_WORKING_STATE_CHANGED - Data was sent due to that driver 2 changed working state  DISTANCE_TRAVELLED - Data was sent due to that a set distance was travelled. (Distance set outside rFMS scope)  FUEL_TYPE_CHANGE - Data was sent due to that the type of fuel currently being utilized by the vehicle changed  PARKING_BRAKE_SWITCH_CHANGE - Data was sent due to that the parking brake state has changed  BATTERY_PACK_CHARGING_STATUS_CHANGE - Data was sent due to a change in the battery pack charging status.  BATTERY_PACK_CHARGING_CONNECTION_STATUS_CHANGE - Data was sent due to a change in the battery pack charging connection status.  TRAILER_CONNECTED - One or several trailers were connected  TRAILER_DISCONNECTED - One or several trailers were disconnected
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    /// The context defines if this is part of the standard or OEM specific. rFMS standard values VOLVO TRUCKS, SCANIA, DAIMLER, IVECO, DAF, MAN, RENAULT TRUCKS, VDL, VOLVO BUSES, IVECO BUS, IRISBUS If the Trigger is defined in the rFMS standard, the Context = RFMS
    #[serde(rename = "context")]
    pub context: String,

    /// Additional TriggerInfo content for OEM specific triggers E.g. TRAILER_ATTACHED_TRIGGER [id of trailer]
    #[serde(rename = "triggerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_info: Option<Vec<String>>,

    #[serde(rename = "driverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<DriverIdObject>,

    /// The id of a PTO. This is only set if the TriggerType = PTO_ENABLED or PTO_DISABLED
    #[serde(rename = "ptoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pto_id: Option<String>,

    #[serde(rename = "tellTaleInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tell_tale_info: Option<TellTaleObject>,

    #[serde(rename = "chargingStatusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_status_info: Option<TriggerObjectChargingStatusInfo>,

    #[serde(rename = "chargingConnectionStatusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_connection_status_info: Option<TriggerObjectChargingConnectionStatusInfo>,
}

impl TriggerObject {
    #[allow(clippy::new_without_default)]
    pub fn new(trigger_type: String, context: String) -> TriggerObject {
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
pub struct DriverIdObject {
    #[serde(rename = "tachoDriverIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tacho_driver_identification: Option<models::DriverIdObjectTachoDriverIdentification>,

    #[serde(rename = "oemDriverIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oem_driver_identification: Option<models::DriverIdObjectOemDriverIdentification>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TellTaleObject {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "tellTale")]
    pub tell_tale: String,

    /// The OemTellTale is only set when the TellTale == OEM_SPECIFIC_TELL_TALE. This is an OEM specific string defining a tell tale in the OEM context.
    #[serde(rename = "oemTellTale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oem_tell_tale: Option<String>,

    /// The current state of the tell tale.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "state")]
    pub state: String,
}

/// Additional information can be provided if the trigger type is BATTERY_PACK_CHARGING_STATUS_CHANGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TriggerObjectChargingStatusInfo {
    /// CHARGING_STARTED - Charging has started  CHARGING_COMPLETED - Charging is completed  CHARGING_INTERRUPTED - Charging has been interrupted (no error)  ERROR - An error occurred when charging  ESTIMATED_COMPLETION_TIME_CHANGED - The estimated time for completed charging has changed. (Threshold is outside scope of rFMS)  TIMER - A predefined time has passed since last charge status update. (Frequency is outside the scope of rFMS)  CHARGING_LEVEL - The charging level has reached a predefined level. (Charging levels are outside the scope of rFMS)
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    /// Details regarding the event. Content is OEM specific
    #[serde(rename = "eventDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_detail: Option<String>,
}

/// Additional information can be provided if the trigger type is BATTERY_PACK_CHARGING_CONNECTION_STATUS_CHANGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TriggerObjectChargingConnectionStatusInfo {
    /// CONNECTING - Vehicle is being connected to a charger  CONNECTED - Vehicle is connected to a charger  DISCONNECTING - Vehicle is being disconnected from the charger  DISCONNECTED - Vehicle is not connected to a charger  ERROR - An error occurred
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    /// Details regarding the event. Content is OEM specific
    #[serde(rename = "eventDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_authentication_equipment: Option<String>,

    /// A card replacement index. This fields is formatted according the definition for CardReplacementIndex (chap 2.26) in: COMMISSION REGULATION (EC) No 1360/2002 Annex 1b
    #[serde(rename = "cardReplacementIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_replacement_index: Option<String>,

    /// A card renewal index. This fields is formatted according the definition for CardRenewalIndex (chap 2.25) in: COMMISSION REGULATION (EC) No 1360/2002 Annex 1b
    #[serde(rename = "cardRenewalIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_renewal_index: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriverIdObjectOemDriverIdentification {
    /// Contains an optional id type (e.g. pin, USB, encrypted EU id...)
    #[serde(rename = "idType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,

    /// An OEM specific driver id.
    #[serde(rename = "oemDriverIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oem_driver_identification: Option<String>,
}

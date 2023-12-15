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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VehicleStatusResponseObject {
    #[serde(rename = "vehicleStatusResponse")]
    pub vehicle_status_response: VehicleStatusResponseObjectVehicleStatusResponse,

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
    pub vehicle_statuses: Option<Vec<VehicleStatusObject>>,

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
    pub door_status: Option<Vec<VehicleStatusObjectDoorStatusInner>>,

    #[serde(rename = "accumulatedData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accumulated_data: Option<AccumulatedDataObject>,

    #[serde(rename = "snapshotData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_data: Option<SnapshotDataObject>,

    #[serde(rename = "uptimeData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uptime_data: Option<UptimeDataObject>,

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
    pub pto_active_class: Option<Vec<LabelObject>>,

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
    pub acceleration_pedal_position_class: Option<Vec<FromToClassObject>>,

    /// In percent. Minimum 5 classes [0, 20[ [20, 40[ [40, 60[ [60, 80[ [80, 100]
    #[serde(rename = "brakePedalPositionClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub brake_pedal_position_class: Option<Vec<FromToClassObject>>,

    /// In m/s2 Minimum 13 classes. ], -1.1] ]-1.1, -0.9] ]-0.9, -0.7] ]-0.7, -0.5] ]-0.5, -0.3] ]-0.3, -0.1] ]-0.1, 0.1[ [0.1, 0.3[ [0.3, 0.5[ [0.5, 0.7[ [0.7, 0.9[ [0.9, 1.1[ [1.1, [
    #[serde(rename = "accelerationClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceleration_class: Option<Vec<FromToClassObject>>,

    /// In m/s2 Minimum 11 classes ], -3.0] ]-3.0, -2.5] ]-2.5, -2.0] ]-2.0, -1.5] ]-1.5, -1.1] ]-1.1, 1.1[ [1.1, 1.5[ [1.5, 2.0[ [2.0, 2.5[ [2.5, 3.0[ [3.0, [
    #[serde(rename = "highAccelerationClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub high_acceleration_class: Option<Vec<FromToClassObject>>,

    /// In percent (how the retarder is used as a positive value). Minimum 5 classes ]0, 20[ [20, 40[ [40, 60[ [60, 80[ [80, 100]
    #[serde(rename = "retarderTorqueClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retarder_torque_class: Option<Vec<FromToClassObject>>,

    /// Driving without torque, with gear (clutch is engaged) Labels DRIVING_WITHOUT_TORQUE
    #[serde(rename = "drivingWithoutTorqueClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driving_without_torque_class: Option<Vec<LabelObject>>,

    /// In percent based on EEC1 value (Actual Engine-Percent Torque). Minimum 10 classes [0, 10[ [10, 20[ [20, 30[ [30, 40[ [40, 50[ [50, 60[ [60, 70[ [70, 80[ [80, 90[ [90, 100]
    #[serde(rename = "engineTorqueClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_torque_class: Option<Vec<FromToClassObjectCombustion>>,

    /// In percent (Actual Engine-Percent Torque). Minimum 10 classes [0, 10[ [10, 20[ [20, 30[ [30, 40[ [40, 50[ [50, 60[ [60, 70[ [70, 80[ [80, 90[ [90, 100]
    #[serde(rename = "electricMotorTorqueClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_motor_torque_class: Option<Vec<FromToClassObjectElectrical>>,

    /// In percent based on EEC2 value (Engine Percent Load At Current Speed). Minimum 10 classes [0, 10[ [10, 20[ [20, 30[ [30, 40[ [40, 50[ [50, 60[ [60, 70[ [70, 80[ [80, 90[ [90, 100]
    #[serde(rename = "engineTorqueAtCurrentSpeedClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_torque_at_current_speed_class: Option<Vec<FromToClassObjectCombustion>>,

    /// In percent (Engine Percent Load At Current Speed). Minimum 10 classes [0, 10[ [10, 20[ [20, 30[ [30, 40[ [40, 50[ [50, 60[ [60, 70[ [70, 80[ [80, 90[ [90, 100]
    #[serde(rename = "electricMotorTorqueAtCurrentSpeedClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub electric_motor_torque_at_current_speed_class: Option<Vec<FromToClassObjectElectrical>>,

    /// In km/h Minimum 40 classes. [0, 4[ [4, 8[ [8, 12[ [12, 16[ [16, 20[ [20, 24[ ... [156, [ Engine on (RPM>0 or electric motor in crank mode)
    #[serde(rename = "vehicleSpeedClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicle_speed_class: Option<Vec<FromToClassObject>>,

    /// Classes refer to the RPM of the combustion engine. Only mandatory if the vehicle has a combustion engine for propulsion. Minimum 10 classes [0, 400[ [400, 800[ [800, 1200[ [1200, 1600[ [1600, 2000[ [2000, 2400[ [2400, 2800[ [2800, 3200[ [3200, 3600[ [3600, [ Note: Engine on (RPM>0 or electric motor in crank mode)
    #[serde(rename = "engineSpeedClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_speed_class: Option<Vec<FromToClassObject>>,

    /// In m/s2 Minimum 13 classes. ], -1.1] ]-1.1, -0.9] ]-0.9, -0.7] ]-0.7, -0.5] ]-0.5, -0.3] ]-0.3, -0.1] ]-0.1, 0.1[ [0.1, 0.3[ [0.3, 0.5[ [0.5, 0.7[ [0.7, 0.9[ [0.9, 1.1[ [1.1, [
    #[serde(rename = "accelerationDuringBrakeClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acceleration_during_brake_class: Option<Vec<FromToClassObject>>,

    /// The currently selected gear One class per gear. Neutral is also a gear. Park is also a gear. This is formatted according to SPN 524, supplied as a decimal value. Example 0 = Neutral, 1 = 1:st gear... This is mainly used for Buses.
    #[serde(rename = "selectedGearClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_gear_class: Option<Vec<LabelObject>>,

    /// The currently used gear One class per gear. Neutral is also a gear. Park is also a gear. This is formatted according to SPN 523, supplied as a decimal value. Example 0 = Neutral, 1 = 1:st gear... This is mainly used for Buses.
    #[serde(rename = "currentGearClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_gear_class: Option<Vec<LabelObject>>,

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
    pub electric_power_recuperation_class: Option<Vec<FromToClassObjectElectrical>>,
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
    pub gnss_position: Option<models::position::GnssPositionObject>,

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
    pub driver1_working_state: Option<DriverWorkingStateProperty>,

    #[serde(rename = "driver2Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driver2_id: Option<models::DriverIdObject>,

    #[serde(rename = "driver2WorkingState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub driver2_working_state: Option<DriverWorkingStateProperty>,

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
    pub estimated_distance_to_empty: Option<SnapshotDataObjectEstimatedDistanceToEmpty>,

    /// A list of vehicle axles
    #[serde(rename = "vehicleAxles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vehicle_axles: Option<Vec<SnapshotDataObjectVehicleAxlesInner>>,

    /// List of trailers connected to the truck.
    #[serde(rename = "trailers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trailers: Option<Vec<SnapshotDataObjectTrailersInner>>,

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
    pub trailer_axles: Option<Vec<SnapshotDataObjectTrailersInnerTrailerAxlesInner>>,

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
    pub alternator_info: Option<UptimeDataObjectAlternatorInfo>,

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

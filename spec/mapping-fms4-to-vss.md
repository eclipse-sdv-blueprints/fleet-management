<!--
SPDX-FileCopyrightText: 2023 Contributors to the Eclipse Foundation

See the NOTICE file(s) distributed with this work for additional
information regarding copyright ownership.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

SPDX-License-Identifier: Apache-2.0
-->
This document provides a mapping of data/object types used in the rFMS 4.0 OpenAPI specification
to VSS paths that either already exist in the VSS 4.0 spec or paths that are defined in the
[FMS overlay file](fms.vspec).

In the tables below, if the *Impl* column contains `yes` for a property, then
* a mapping from the underlying j1939 signal to VSS has been defined in the [FMS overlay file](fms.vspec),
* the corresponding VSS Data Entries are getting retrieved from the Kuksa.val Databroker as part of the TIMER trigger,
* the values retrieved from the Databroker are included in the data sent by the vehicle to the back end.

# VehiclePositionObject

| rFMS property                   | M/O | rFMS type   | rFMS unit | VSS/FMS Overlay path                            | VSS unit | Impl |
| :------------------------------ | --- | :---------- | :-------- | :---------------------------------------------- | :------- | ---- |
| vin                             |  M  | string      |           | VSS: Vehicle.VehicleIdentification.VIN          |          | yes  |
| gnssPosition.latitude           |  M  | double      | WGS84     | VSS: Vehicle.CurrentLocation.Latitude           | WGS84    | yes  |
| gnssPosition.longitude          |  M  | double      | WGS84     | VSS: Vehicle.CurrentLocation.Longitude          | WGS84    | yes  |
| gnssPosition.heading            |  O  | integer     | degrees   | VSS: Vehicle.CurrentLocation.Heading            | degrees  | yes  |
| gnssPosition.altitude           |  O  | integer     | m         | VSS: Vehicle.CurrentLocation.Altitude           | m        | yes  |
| gnssPosition.speed              |  O  | double      | km/h      | FMS: Vehicle.CurrentLocation.Speed              | km/h     | yes  |
| gnssPosition.positionDateTime   |  M  | date-time   |           | VSS: Vehicle.CurrentLocation.Timestamp          |          | yes  |
| wheelBasedSpeed                 |  M  | double      | km/h      | VSS: Vehicle.Speed                              | km/h     | yes  |
| tachographSpeed                 |  O  | double      | km/h      | FMS: Vehicle.Tachograph.VehicleSpeed            | km/h     | yes  |

# VehicleStatusObject

| rFMS property                   | M/O | rFMS type   | rFMS unit | VSS/FMS Overlay path                                              | VSS unit | Impl |
| :------------------------------ | --- | :---------- | :-------- | :---------------------------------------------------------------- | :------- | ---- |
| vin                             |  M  | string      |           | VSS: Vehicle.VehicleIdentification.VIN                            |          | yes  |
| hrTotalVehicleDistance          |  M  | int64       | m         | FMS: Vehicle.TraveledDistanceHighRes                              | m        | yes  |
| totalEngineHours                | M/O | double      | h         | VSS: Vehicle.Powertrain.CombustionEngine.EngineHours              | h        | yes  |
| totalElectricMotorHours         | M/O | double      | h         | FMS: Vehicle.Powertrain.ElectricMotor.MotorHours                  | h        | no   |
| engineTotalFuelUsed             | M/O | int64       | ml        | FMS: Vehicle.Powertrain.FuelSystem.AccumulatedConsumption         | ml       | yes  |
| totalFuelUsedGaseous            | M/O | int64       | kg        |                                                                   |          | no   |
| totalElectricEnergyUsed         | M/O | int64       | Wh        | VSS: Vehicle.Powertrain.TractionBattery.AccumulatedConsumedEnergy | kWh      | no   |
| grossCombinationVehicleWeight   |  O  | integer     | kg        | VSS: Vehicle.CurrentOverallWeight                                 | kg       | yes  |
| driver1Id                       |  M  | [DriverIdObject](#driveridobject) | | FMS: Vehicle.Tachograph.Driver.Driver1                |          | yes  |
| accumulatedData                 |  O  | [AccumulatedDataObject](#accumulateddataobject) | |                                         |          | no   |
| snapshotData                    |  O  | [SnapshotDataObject](#snapshotdataobject) | |                                               |          | yes  |
| uptimeData                      |  O  | [UptimeDataObject](#uptimedataobject) | |                                                   |          | no   |
| status2OfDoors                  | M(B)| string      | |                                                                             |          | no   |
| doorStatus                      |  O  | array       | |                                                                             |          | no   |

# AccumulatedDataObject

| rFMS property                   | M/O | rFMS format | rFMS unit | VSS/FMS Overlay path                   | VSS unit | Impl |
| :------------------------------ | --- | :---------- | :-------- | :------------------------------------- | :------- | ---- |
| durationWheelbasedSpeedOverZero |  M  |             |           |                                        |          | no   |

TBD

# SnapshotDataObject

| rFMS property                             | M/O | rFMS format | rFMS unit | VSS/FMS Overlay path                     | VSS unit | Impl |
| :---------------------------------------- | --- | :---------- | :-------- | :--------------------------------------- | :------- | ---- |
| gnssPosition.latitude                     |  M  | double      | WGS84     | VSS: Vehicle.CurrentLocation.Latitude    | WGS84    | yes  |
| gnssPosition.longitude                    |  M  | double      | WGS84     | VSS: Vehicle.CurrentLocation.Longitude   | WGS84    | yes  |
| gnssPosition.heading                      |  O  | integer     | degrees   | VSS: Vehicle.CurrentLocation.Heading     | degrees  | yes  |
| gnssPosition.altitude                     |  O  | integer     | m         | VSS: Vehicle.CurrentLocation.Altitude    | m        | yes  |
| gnssPosition.speed                        |  O  | double      | km/h      | FMS: Vehicle.CurrentLocation.Speed       | km/h     | yes  |
| gnssPosition.positionDateTime             |  M  | date-time   |           | VSS: Vehicle.CurrentLocation.Timestamp   |          | yes  |
| wheelBasedSpeed                           |  M  | double      | km/h      | VSS: Vehicle.Speed                       | km/h     | yes  |
| tachographSpeed                           |  O  | double      | km/h      | FMS: Vehicle.Tachograph.VehicleSpeed     | km/h     | yes  |
| engineSpeed                               |  O  | double      | rpm       | VSS: Vehicle.Powertrain.CombustionEngine.Speed | rpm | yes  |
| electricMotorSpeed                        |  O  | double      | rpm       | VSS: Vehicle.Powertrain.ElectricMotor.Speed | rpm   | no   |
| fuelType                                  |  O  | string      |           | FMS: Vehicle.Powertrain.CurrentFuelType  | enum     | no   |
| fuelLevel1                                |  M  | double      | %         | FMS: Vehicle.Powertrain.FuelSystem.Tank.First.RelativeLevel  | % | yes  |
| fuelLevel2                                |  O  | double      | %         | FMS: Vehicle.Powertrain.FuelSystem.Tank.Second.RelativeLevel | % | yes  |
| catalystFuelLevel                         |  O  | double      | %         | VSS: Vehicle.Powertrain.CombustionEngine.DieselExhaustFluid.Level | % | yes  |
| driver1WorkingState                       |  O  | string      |           | FMS: Vehicle.Tachograph.Driver.Driver1.WorkingState | | yes  |
| driver2Id                                 |  O  | [DriverIdObject](#driveridobject) | | FMS: Vehicle.Tachograph.Driver.Driver2  | | no   |
| driver2WorkingState                       |  O  | string      |           | FMS: Vehicle.Tachograph.Driver.Driver2.WorkingState | | yes  |
| ambientAirTemperature                     |  O  | double      | celsius   | VSS: Vehicle.Exterior.AirTemperature     | celsius  | yes  |
| parkingBrakeSwitch                        |  O  | boolean     |           | VSS: Vehicle.Chassis.ParkingBrake.IsEngaged | | yes  |
| hybridBatteryPackRemainingCharge          |  O  | double      | %         | VSS: Vehicle.Powertrain.TractionBattery.StateOfCharge.Current | % | no   |
| batteryPackChargingStatus                 |  O  | string      | enum      | VSS: Vehicle.Powertrain.TractionBattery.Charging.IsCharging | | no   |
| batteryPackChargingConnectionStatus       |  O  | string      | enum      | VSS: Vehicle.Powertrain.TractionBattery.Charging.IsChargingCableConnected | | no   |
| batteryPackChargingDevice                 |  O  | string      | enum      | | | no   |
| batteryPackChargingPower                  |  O  | double      | W         | VSS: Vehicle.Powertrain.TractionBattery.Charging.ChargeCurrent.DC | A | no   |
|                                           |     |             |           | VSS: Vehicle.Powertrain.TractionBattery.Charging.ChargeVoltage.DC | V | no   |
| estimatedTimeBatteryPackChargingCompleted |  O  | date-time   |           | VSS: Vehicle.Powertrain.TractionBattery.Charging.TimeToComplete | s | no   |
| estimatedDistanceToEmpty.total            |  M  | int64       | m         | VSS: Vehicle.Powertrain.Range                 | m        | yes  |
| estimatedDistanceToEmpty.fuel             |  O  | int64       | m         | VSS: Vehicle.Powertrain.FuelSystem.Range      | m        | yes  |
| estimatedDistanceToEmpty.gas              |  O  |             |           |                                               |          | no   |
| estimatedDistanceToEmpty.batteryPack      |  O  | int64       | m         | VSS: Vehicle.Powertrain.TractionBattery.Range | m        | no   |
| vehicleAxles                              |  O  | array       |           |                                               |          | no   |
| trailers                                  |  O  | array       |           |                                               |          | no   |

# UptimeDataObject

| rFMS property                   | M/O | rFMS format | rFMS unit | VSS/FMS Overlay path                                        | VSS unit | Impl |
| :------------------------------ | --- | :---------- | :-------- | :---------------------------------------------------------- | :------- | ---- |
| tellTaleInfo                    |  M  | array       |           | FMS: Vehicle.Cabin.Telltale.*                               | enum     | no   |
| serviceDistance                 |  O  | int64       | m         | VSS: Vehicle.Service.DistanceToService                      | km       | no   |
| engineCoolantTemperature        |  O  | double      | celsius   | VSS: Vehicle.Powertrain.CombustionEngine.ECT                | celsius  | no   |
| hvessOutletCoolantTemperature   |  O  | double      | celsius   | FMS: Vehicle.Powertrain.TractionBattery.CoolantTemperature  | celsius  | no   |
| hvessTemperature                |  O  | double      | celsius   | VSS: Vehicle.Powertrain.TractionBattery.Temperature.Average | celsius  | no   |
| serviceBrakeAirPressureCircuit1 |  O  | int64       | pascal    | FMS: Vehicle.Chassis.Brake.Circuit1.AirPressure             | pascal   | no   |
| serviceBrakeAirPressureCircuit2 |  O  | int64       | pascal    | FMS: Vehicle.Chassis.Brake.Circuit2.AirPressure             | pascal   | no   |
| durationAtLeastOneDoorOpen      |  O  | | | | | no   |
| alternatorInfo.alternatorStatus | M(B)| | | | | no   |
| alternatorInfo.alternatorNumber | M(B)| | | | | no   |
| bellowPressureFrontAxleLeft     |  O  | | | | | no   |
| bellowPressureFrontAxleRight    |  O  | | | | | no   |
| bellowPressureRearAxleLeft      |  O  | | | | | no   |
| bellowPressureRearAxleRight     |  O  | | | | | no   |

# DriverIdObject

## DriverIdObject.tachoDriverIdentification

| rFMS property                   | M/O | VSS/FMS Overlay path                                   | Impl |
| :------------------------------ | --- | :----------------------------------------------------- | ---- |
| driverIdentification            |  M  | FMS: Vehicle.Tachograph.Driver.Identification          | yes  |
| cardIssuingMemberState          |  M  | FMS: Vehicle.Tachograph.Driver.CardIssuingMemberState  | no   |
| driverAuthenticationEquipment   |  O  | FMS: Vehicle.Tachograph.Driver.AuthenticationEquipment | no   |
| cardReplacementIndex            |  O  | FMS: Vehicle.Tachograph.Driver.CardReplacementIndex    | no   |
| cardRenewalIndex                |  O  | FMS: Vehicle.Tachograph.Driver.CardRenewalIndex        | no   |

## DriverIdObject.oemDriverIdentification

| rFMS property                   | M/O | VSS/FMS Overlay path                                 | Impl |
| :------------------------------ | --- | :--------------------------------------------------- | ---- |
| oemDriverIdentification         |  O  | FMS: Vehicle.Tachograph.Driver.OemIdentification     | no   |
| idType                          |  O  | FMS: Vehicle.Tachograph.Driver.OemIdentificationType | no   |

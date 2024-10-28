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

//! Provides means to write a Vehicle's current status properties
//! to an InfluxDB as Influx *measurements*.
use clap::ArgMatches;
use fms_proto::fms::VehicleStatus;
use influxrs::Measurement;
use log::{debug, warn};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::connection::InfluxConnection;

fn build_header_measurement(
    vin: &str,
    trigger: &str,
    created_date_time: u128,
    vehicle_status: &VehicleStatus,
) -> Option<Measurement> {
    let mut builder = Measurement::builder(crate::MEASUREMENT_HEADER)
        .tag(crate::TAG_TRIGGER, trigger)
        .tag(crate::TAG_VIN, vin)
        .field(crate::FIELD_CREATED_DATE_TIME, created_date_time);

    if let Some(value) = vehicle_status.hr_total_vehicle_distance {
        builder = builder.field(crate::FIELD_HR_TOTAL_VEHICLE_DISTANCE, value);
    }
    if let Some(value) = vehicle_status.gross_combination_vehicle_weight {
        builder = builder.field(crate::FIELD_GROSS_COMBINATION_VEHICLE_WEIGHT, value);
    }
    if let Some(value) = vehicle_status.total_engine_hours {
        builder = builder.field(crate::FIELD_TOTAL_ENGINE_HOURS, value);
    }
    if let Some(value) = vehicle_status.total_electric_motor_hours {
        builder = builder.field(crate::FIELD_TOTAL_ELECTRIC_MOTOR_HOURS, value);
    }
    if let Some(value) = vehicle_status.engine_total_fuel_used {
        builder = builder.field(crate::FIELD_ENGINE_TOTAL_FUEL_USED, value);
    }

    if let Some(tacho_driver_id) = vehicle_status
        .driver1_id
        .clone()
        .into_option()
        .and_then(|driver_id| driver_id.tacho_driver_identification.into_option())
    {
        builder = builder.field(
            crate::FIELD_DRIVER1_ID,
            tacho_driver_id.driver_identification,
        );
        builder = builder.field(
            crate::FIELD_DRIVER1_CARD_ISSUER,
            tacho_driver_id.card_issuing_memberState,
        );
    }

    match builder.build() {
        Ok(measurement) => Some(measurement),
        Err(e) => {
            debug!("failed to create header Measurement: {e}");
            None
        }
    }
}

fn build_snapshot_measurement(
    vin: &str,
    trigger: &str,
    created_date_time: u128,
    vehicle_status: &VehicleStatus,
) -> Option<Measurement> {
    let mut builder = Measurement::builder(crate::MEASUREMENT_SNAPSHOT)
        .tag(crate::TAG_TRIGGER, trigger)
        .tag(crate::TAG_VIN, vin)
        .field(crate::FIELD_CREATED_DATE_TIME, created_date_time);

    if let Some(snapshot_data) = vehicle_status.snapshot_data.clone().into_option() {
        if let Some(value) = snapshot_data.wheel_based_speed {
            builder = builder.field(crate::FIELD_WHEEL_BASED_SPEED, value);
        }
        if let Some(value) = snapshot_data.tachograph_speed {
            builder = builder.field(crate::FIELD_TACHOGRAPH_SPEED, value);
        }
        if let Some(value) = snapshot_data.fuel_type {
            builder = builder.field(crate::FIELD_FUEL_TYPE, value);
        }
        if let Some(value) = snapshot_data.engine_speed {
            builder = builder.field(crate::FIELD_ENGINE_SPEED, value);
        }
        if let Some(value) = snapshot_data.catalyst_fuel_level {
            builder = builder.field(crate::FIELD_CATALYST_FUEL_LEVEL, value);
        }
        if let Some(value) = snapshot_data.fuel_level1 {
            builder = builder.field(crate::FIELD_FUEL_LEVEL1, value);
        }
        if let Some(value) = snapshot_data.fuel_level2 {
            builder = builder.field(crate::FIELD_FUEL_LEVEL2, value);
        }
        if let Some(value) = snapshot_data.driver1_working_state {
            builder = builder.field(crate::FIELD_DRIVER1_WORKING_STATE, value);
        }
        if let Some(value) = snapshot_data.driver2_working_state {
            builder = builder.field(crate::FIELD_DRIVER2_WORKING_STATE, value);
        }
        if let Some(value) = snapshot_data.ambient_air_temperature {
            builder = builder.field(crate::FIELD_AMBIENT_AIR_TEMP, value);
        }
        if let Some(value) = snapshot_data.parking_brake_engaged {
            builder = builder.field(crate::FIELD_PARKING_BREAK_SWITCH, value);
        }

        if let Some(current_location) = snapshot_data.gnss_position.into_option() {
            builder = builder
                .field(crate::FIELD_LATITUDE, current_location.latitude)
                .field(crate::FIELD_LONGITUDE, current_location.longitude);

            if let Some(value) = current_location.heading {
                builder = builder.field(crate::FIELD_HEADING, value);
            }

            if let Some(value) = current_location.altitude {
                builder = builder.field(crate::FIELD_ALTITUDE, value);
            }

            if let Some(value) = current_location.speed {
                builder = builder.field(crate::FIELD_SPEED, value);
            }

            if let Some(instant) = current_location.instant.clone().into_option() {
                builder = builder.field(crate::FIELD_POSITION_DATE_TIME, instant.seconds);
            }
        }

        if let Some(distance_to_empty) = snapshot_data.estimated_distance_to_empty.into_option() {
            if let Some(value) = distance_to_empty.fuel {
                builder = builder.field(crate::FIELD_ESTIMATED_DIST_TO_EMPTY_FUEL, value);
            }
            if let Some(value) = distance_to_empty.total {
                builder = builder.field(crate::FIELD_ESTIMATED_DIST_TO_EMPTY_TOTAL, value);
            }
        }

        if let Some(tacho_driver_id) = snapshot_data
            .driver2_id
            .clone()
            .into_option()
            .and_then(|driver_id| driver_id.tacho_driver_identification.into_option())
        {
            builder = builder
                .field(
                    crate::FIELD_DRIVER2_ID,
                    tacho_driver_id.driver_identification,
                )
                .field(
                    crate::FIELD_DRIVER2_CARD_ISSUER,
                    tacho_driver_id.card_issuing_memberState,
                );
        }
    }

    match builder.build() {
        Ok(measurement) => Some(measurement),
        Err(e) => {
            debug!("failed to create snapshot Measurement: {e}");
            None
        }
    }
}

/// A facade to an InfluxDB server for publishing Vehicle status information.
pub struct InfluxWriter {
    influx_con: InfluxConnection,
}

impl InfluxWriter {
    /// Creates a new writer.
    ///
    /// Determines the parameters necessary for creating the writer from values specified on
    /// the command line or via environment variables as defined by [`super::add_command_line_args`].
    pub fn new(args: &ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {
        InfluxConnection::new(args).map(|con| InfluxWriter { influx_con: con })
    }

    /// Writes Vehicle status information as measurements to the InfluxDB server.
    ///
    /// The measurements are being written to the *bucket* in the *organization* that have been
    /// configured via command line arguments and/or environment variables passed in to [`self::InfluxWriter::new()`].
    ///
    /// This function writes the current vehicle status to InfluxDB by means of two measurements:
    ///
    /// * *header* - contains the following tags/fields:

    ///   | Type  | Name            | Description                      |
    ///   | ----- | --------------- | -------------------------------- |
    ///   | tag   | trigger         | The type of event that triggered the reporting of the vehicle status. |
    ///   | tag   | vin             | The vehicle's identification number. |
    ///   | field | createdDateTime | The instant of time (milliseconds since UNIX epoch) at which the vehicle status information had been created. |
    ///   | field | hrTotalVehicleDistance | The accumulated distance travelled by the vehicle during its operation in meter. |
    ///   | field | grossCombinationVehicleWeight | The full vehicle weight in kg. |
    ///   | field | totalEngineHours | The total hours of operation for the vehicle combustion engine. |
    ///   | field | totalElectricMotorHours | The total hours the electric motor is ready for propulsion (i.e. crank mode). |
    ///   | field | engineTotalFuelUsed | The total fuel the vehicle has used during its lifetime in MilliLitres. |
    ///   | field | driver1Id | The unique identification of driver one in a Member State. |
    ///   | field | driver1IdCardIssuer | The country alpha code of the Member State having issued driver one's card. |

    /// * *snapshot* - contains the following tags/fields:

    ///   | Type  | Name            | Description                      |
    ///   | ----- | --------------- | -------------------------------- |
    ///   | tag   | trigger         | The type of event that triggered the reporting of the vehicle status. |
    ///   | tag   | vin             | The vehicle's identification number. |
    ///   | field | createdDateTime | The instant of time (milliseconds since UNIX epoch) at which the vehicle status information had been created. |
    ///   | field | latitude        | Latitude (WGS84 based). |
    ///   | field | longitude       | Longitude (WGS84 based). |
    ///   | field | heading         | The direction of the vehicle (0-359). |
    ///   | field | altitude        | The altitude of the vehicle. Where 0 is sea level, negative values below sealevel and positive above sealevel. Unit in meters. |
    ///   | field | speed           | The GNSS(e.g. GPS)-speed in km/h. |
    ///   | field | positionDateTime | The time of the position data in ISO 8601 format. |
    ///   | field | wheelBasedSpeed | The vehicle's wheel based speed. |
    ///   | field | tachographSpeed | The Tacho speed. |
    ///   | field | engineSpeed     | The engine (Diesel/gaseous) speed in rev/min. |
    ///   | field | fuelType        | Type of fuel currently being utilized by the vehicle acc. SPN 5837. |
    ///   | field | catalystFuelLevel | The AdBlue level percentage. |
    ///   | field | fuelLevel1      | Ratio of volume of fuel to the total volume of fuel storage container, in percent. |
    ///   | field | fuelLevel2      | Ratio of volume of fuel to the total volume of fuel storage container, in percent. When Fuel Level 2 is not used, Fuel Level 1 represents the total fuel in all fuel storage containers.  When Fuel Level 2 is used, Fuel Level 1 represents the fuel level in the primary or left-side fuel storage container. |
    ///   | field | estimatedDistanceToEmptyFuel | Estimated distance to empty, fuel tank, in meters. |
    ///   | field | estimatedDistanceToEmptyTotal | Estimated distance to empty, summarizing fuel, gas and battery in meters. |
    ///   | field | driver1WorkingState | Tachograph Working state of the driver one. |
    ///   | field | driver2Id | The unique identification of driver two in a Member State. |
    ///   | field | driver2IdCardIssuer | The country alpha code of the Member State having issued driver two's card. |
    ///   | field | driver2WorkingState | Tachograph Working state of the driver two. |
    ///   | field | ambientAirTemperature | The Ambient air temperature in Celsius. |
    ///   | field | parkingBrakeSwitch | Switch signal which indicates when the parking brake is set. |
    pub async fn write_vehicle_status(&self, vehicle_status: &VehicleStatus) {
        if vehicle_status.vin.is_empty() {
            debug!("ignoring vehicle status without VIN ...");
            return;
        }
        let Some(ts) = vehicle_status
            .created
            .clone()
            .into_option()
            .and_then(|v| SystemTime::try_from(v).ok())
        else {
            debug!("ignoring vehicle status without created timestamp");
            return;
        };
        let Ok(created_timestamp) = ts
            .duration_since(UNIX_EPOCH)
            .map_err(|e| debug!("error processing 'created' timestamp: {}", e))
            .map(|d| d.as_millis())
        else {
            return;
        };
        let trigger = match vehicle_status.trigger.clone().into_option() {
            Some(t) => match t.context.as_str() {
                "RFMS" => t.type_.clone(),
                _ => {
                    debug!(
                        "ignoring vehicle status with unsupported trigger context [{}]",
                        t.context
                    );
                    return;
                }
            },
            None => {
                debug!("ignoring vehicle status without trigger");
                return;
            }
        };

        let mut measurements = Vec::new();
        if let Some(measurement) = build_header_measurement(
            vehicle_status.vin.as_str(),
            &trigger,
            created_timestamp,
            vehicle_status,
        ) {
            debug!("writing header measurement to influxdb");
            measurements.push(measurement);
        }
        if let Some(measurement) = build_snapshot_measurement(
            vehicle_status.vin.as_str(),
            &trigger,
            created_timestamp,
            vehicle_status,
        ) {
            debug!("writing snapshot measurement to influxdb");
            measurements.push(measurement);
        }
        if !measurements.is_empty() {
            if let Err(e) = self
                .influx_con
                .client
                .write(self.influx_con.bucket.as_str(), measurements.as_slice())
                .await
            {
                warn!("failed to write data to influx: {e}");
            }
        }
    }
}

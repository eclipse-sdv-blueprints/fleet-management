# SPDX-FileCopyrightText: 2025 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
# 
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0

import csv
from pyexpat import model
import random
import math
import argparse

class VehicleMetadata:
    def __init__(self):
        # Generate random vehicle metadata
        brand_models = {
            "VOLVO TRUCKS": ["FH", "FM"],
            "MAN": ["TGX", "TGS"],
            "DAF": ["XF", "XG"],
        }
        brand_codes = {
            "VOLVO TRUCKS": "YV2",
            "MAN": "WMA",
            "DAF": "XLR",
        }
        self.brand = random.choice(list(brand_models))
        self.model = random.choice(brand_models[self.brand])
        self.vin = f"{brand_codes[self.brand]}E4C3A5VB{random.randint(100000, 999999)}"
        self.vehicle_type = "TRUCK"
        self.production_date = f"{random.randint(2010, 2026)}-{random.randint(1, 12):02d}-{random.randint(1, 28):02d}"

    def write_values(self, csv_writer):
        writer.writerow([
            "current",
            "Vehicle.VehicleIdentification.VIN",
            self.vin,
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.VehicleIdentification.Brand",
            self.brand,
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.VehicleIdentification.Model",
            self.model,
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.VehicleIdentification.Type",
            self.vehicle_type,
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.VehicleIdentification.ProductionDate",
            self.production_date,
            0
        ])

class VehicleState:
    def __init__(self):
        # Initialization with standard values
        self.working_state_driver1 = random.choice(["REST", "DRIVE", "WORK"])
        self.working_state_driver2 = random.choice(
            ["REST", "DRIVER_AVAILABLE"])
        self.is_card_present_driver1 = random.choice([True, False])
        self.is_card_present_driver2 = random.choice([True, False])

        # Set initial driving state - 90% chance to start driving
        self.is_driving_period = random.random() < 0.9

        # Fahrzyklus-Management - much shorter driving periods,
        # more frequent rests
        self.driving_cycle_counter = 0
        # Even shorter driving duration (was 40-70)
        self.driving_duration = random.randint(20, 45)
        self.rest_duration = random.randint(
            8, 30)      # More variable rest duration

        # Basic vehicle data
        # Independent 50% chance for each resource to be low
        low_fuel = random.random() < 0.5  # Direct 50% chance fuel is low
        low_adblue = random.random() < 0.5  # Direct 50% chance AdBlue is low

        self.fuel_type = random.choice(["GAS", "DSL"]) # petrol or diesel
        # Set levels based on what's low
        self.fuel_level = random.uniform(
            5, 20) if low_fuel else random.uniform(50, 100)
        self.adblue_level = random.uniform(
            5, 20) if low_adblue else random.uniform(70, 100)

        self.traveled_distance = random.randint(3200000, 55000000)
        self.air_temperature = random.uniform(10, 25)
        self.engine_hours = random.uniform(4800, 33000)
        self.parking_brake_engaged = not self.is_driving_period
        self.current_weight = random.uniform(10000, 35000)

        # SIMPLIFIED APPROACH: Use smooth curve functions for speeds
        # Initialize time counters
        self.time_counter = 0

        # Replace continuous sine waves with more stable speed patterns
        if self.is_driving_period:
            # Initial cruise speed with minimal variation
            self.target_speed = random.uniform(60, 85)
            self.current_speed = self.target_speed
            self.vehicle_speed = self.current_speed
            self.engine_speed = 800 + self.vehicle_speed * 15

            # Counters for cruise control behavior
            # Shorter periods of constant speed
            self.cruise_duration = random.randint(10, 30)
            self.cruise_counter = 0
            self.transition_duration = 0
            self.transition_counter = 0
            self.in_transition = False

            self.working_state_driver1 = "DRIVE"
        else:
            self.base_speed = 0
            self.speed_variation = 0
            self.speed_period = 50
            self.speed_phase = 0
            self.vehicle_speed = 0
            # 70% chance engine is off when parked
            self.engine_speed = 0 if random.random() < 0.7 else 800
            self.working_state_driver1 = "REST"

        # Initialize GPS coordinates in Central Europe
        # (around Germany/Austria/Czech area)
        # Central European coordinates range roughly:
        # Latitude: 48.0 to 52.0 (North)
        # Longitude: 8.0 to 16.0 (East)
        self.latitude = random.uniform(48.0, 52.0)
        self.longitude = random.uniform(8.0, 16.0)

        # Vehicle heading (0-359 degrees, 0=North, 90=East, etc.)
        self.heading = random.uniform(0, 359)

        # Set initial GPS update counter
        self.gps_update_counter = 0

        # Central European cities to make simulation more realistic
        self.cities = [
            {"name": "Munich", "lat": 48.137154, "lng": 11.576124},
            {"name": "Berlin", "lat": 52.520008, "lng": 13.404954},
            {"name": "Vienna", "lat": 48.208176, "lng": 16.373819},
            {"name": "Prague", "lat": 50.073658, "lng": 14.418540},
            {"name": "Warsaw", "lat": 52.229676, "lng": 21.012229},
            {"name": "Frankfurt", "lat": 50.110924, "lng": 8.682127},
            {"name": "Zurich", "lat": 47.376888, "lng": 8.541694}
        ]

        # Randomly pick a starting city and adjust coordinates
        if random.random() < 0.7:  # 70% chance to start in a city
            city = random.choice(self.cities)
            self.latitude = city["lat"] + \
                random.uniform(-0.01, 0.01)  # Small variation
            self.longitude = city["lng"] + random.uniform(-0.01, 0.01)

        # Driver rest cycle tracking - much more aggressive now
        self.driver1_active_cycles = 0
        self.driver2_active_cycles = 0
        self.driver1_rest_cycles = 0
        self.driver2_rest_cycles = 0

        # Constants for driver breaks - much more frequent breaks
        # ~1 minute drive time before requiring break
        self.DRIVE_DURATION_BEFORE_BREAK = 60
        self.DRIVER_BREAK_DURATION = 30  # ~30 second minimum rest duration

        # Driver states (can be REST independent of vehicle state)
        self.driver1_needs_break = False
        self.driver2_needs_break = False

        # Track when breaks were last enforced
        self.last_driver1_break = 0
        self.last_driver2_break = 0
        self.cycle_count = 0

        self.is_engine_running = self.engine_speed > 0

    def update(self):
        # Track total cycles for break timing
        self.cycle_count += 1

        # SIMPLIFIED: Check for phase changes first
        self.driving_cycle_counter += 1

        # State transitions
        if self.is_driving_period:
            if self.driving_cycle_counter >= self.driving_duration:
                # Transition to rest
                self.is_driving_period = False
                self.driving_cycle_counter = 0
                self.rest_duration = random.randint(10, 30)  # Longer rest
                self.working_state_driver1 = "REST"
                self.working_state_driver2 = "REST"
                # Start slowing down
                self.speed_period = random.uniform(
                    10, 20)  # Faster deceleration

            # Random chance for unplanned rest break - significantly increased
            elif random.random() < 0.04:  # 4% chance per cycle (was 1.5%)
                self.is_driving_period = False
                self.driving_cycle_counter = 0
                self.rest_duration = random.randint(
                    5, 20)  # Variable unplanned stops
                self.working_state_driver1 = "REST"
                self.working_state_driver2 = "REST"

            # Additional rest triggers based on conditions
            elif self.fuel_level < 15 and random.random() < 0.1:
                # Low fuel may trigger rest
                self.is_driving_period = False
                self.driving_cycle_counter = 0
                self.rest_duration = random.randint(
                    15, 25)  # Longer rest for refueling
                self.working_state_driver1 = "REST"
                self.working_state_driver2 = "REST"

            # Driver fatigue simulation - longer driving leads
            # to higher rest probability
            elif self.driving_cycle_counter > 15 and random.random() < 0.02:
                self.is_driving_period = False
                self.driving_cycle_counter = 0
                self.rest_duration = random.randint(
                    5, 15)  # Short rest for driver break
                self.working_state_driver1 = "REST"
                self.working_state_driver2 = "REST"
        else:
            if self.driving_cycle_counter >= self.rest_duration:
                # Transition to driving
                self.is_driving_period = True
                self.driving_cycle_counter = 0
                # Even shorter driving periods (was 40-70)
                self.driving_duration = random.randint(20, 45)
                self.working_state_driver1 = "DRIVE"
                self.working_state_driver2 = "DRIVER_AVAILABLE"

                # Set up a new speed profile
                self.base_speed = random.uniform(60, 85)
                self.speed_variation = random.uniform(5, 15)
                self.speed_period = random.uniform(30, 60)
                self.speed_phase = random.uniform(0, 2*math.pi)

        # SIMPLIFIED: 5% chance to start driving immediately if at rest
        if not self.is_driving_period and random.random() < 0.05:
            self.is_driving_period = True
            self.driving_cycle_counter = 0
            self.driving_duration = random.randint(
                40, 70)  # Shorter driving periods
            self.working_state_driver1 = "DRIVE"
            self.working_state_driver2 = "DRIVER_AVAILABLE"

            # Start with a new speed profile
            self.base_speed = random.uniform(60, 85)
            self.speed_variation = random.uniform(5, 15)
            self.speed_period = random.uniform(30, 60)
            self.speed_phase = random.uniform(0, 2*math.pi)

        # Speed pattern simulation with periods of constant speed
        if self.is_driving_period:
            if not self.in_transition:
                # Currently cruising at stable speed
                self.cruise_counter += 1

                # Add tiny variations to simulate real-world minor fluctuations
                small_adjustment = random.uniform(-0.3, 0.3)
                self.vehicle_speed = max(
                    0, min(100, self.current_speed + small_adjustment))

                # Random chance for spontaneous speed change
                # 5% chance per cycle for spontaneous speed change
                if random.random() < 0.05:
                    self.in_transition = True
                    self.transition_counter = 0

                    # Quick speed adjustment (like traffic conditions,
                    # other vehicles)
                    change_amount = random.uniform(5, 15)
                    direction = 1 if random.random() < 0.5 else -1
                    self.target_speed = self.current_speed + \
                        (direction * change_amount)
                    # Keep within reasonable bounds
                    self.target_speed = max(40, min(95, self.target_speed))

                    # Calculate how long the transition should take
                    speed_diff = abs(self.target_speed - self.current_speed)
                    self.transition_duration = int(
                        1 + speed_diff / 3)  # Faster transitions

                # Decide if it's time to change speed
                elif self.cruise_counter >= self.cruise_duration:
                    self.in_transition = True
                    self.transition_counter = 0

                    # Decide new target speed
                    prev_target = self.target_speed

                    # Sometimes make larger changes (like highway exit,
                    # overtaking)
                    # 40% chance of significant speed change
                    if random.random() < 0.4:
                        change_amount = random.uniform(15, 25)
                        direction = 1 if random.random() < 0.5 else -1
                        self.target_speed = prev_target + \
                            (direction * change_amount)
                        # Keep within reasonable bounds
                        self.target_speed = max(40, min(95, self.target_speed))
                    else:
                        # Small adjustments to current speed
                        self.target_speed = prev_target + \
                            random.uniform(-10, 10)
                        # Keep within reasonable bounds
                        self.target_speed = max(40, min(95, self.target_speed))

                    # Calculate how long the transition should take
                    speed_diff = abs(self.target_speed - self.current_speed)
                    self.transition_duration = int(
                        1 + speed_diff / 2)  # 1 cycle per 2 km/h change
            else:
                # Currently in speed transition
                self.transition_counter += 1

                # Calculate smooth progression to target speed
                progress = min(1.0, self.transition_counter /
                               self.transition_duration)
                self.current_speed = self.current_speed + \
                    ((self.target_speed - self.current_speed) * progress * 0.2)

                # Small random variations during transition
                small_adjustment = random.uniform(-0.2, 0.2)
                self.vehicle_speed = max(
                    0, min(100, self.current_speed + small_adjustment))

                # Check if transition is complete
                if self.transition_counter >= self.transition_duration:
                    self.in_transition = False
                    self.cruise_counter = 0
                    self.current_speed = self.target_speed
                    self.cruise_duration = random.randint(
                        10, 30)  # Set next cruise duration

            # Engine speed follows vehicle speed without feedback loops
            target_rpm = 800 + self.vehicle_speed * 15
            self.engine_speed = target_rpm + \
                random.uniform(-20, 20)  # Small variation in RPM

            # Always in DRIVE state when moving with speed >5
            self.working_state_driver1 = "DRIVE" if self.vehicle_speed > 5 else "WORK"
            self.parking_brake_engaged = False
        else:
            # When resting, smoothly decelerate
            if self.vehicle_speed > 0:
                # Decelerate gradually
                self.vehicle_speed = max(0, self.vehicle_speed - 2)

                # Engine speed follows accordingly
                self.engine_speed = 800 + self.vehicle_speed * 15

                # Still driving until fully stopped
                if self.vehicle_speed > 5:
                    self.working_state_driver1 = "DRIVE"
                else:
                    self.working_state_driver1 = "WORK"
            else:
                # Fully stopped
                self.vehicle_speed = 0

                # Either idle or engine off
                if self.engine_speed > 0:
                    # Engine running - either keep idling or turn off
                    if random.random() < 0.1:  # 10% chance to turn off engine per cycle
                        self.engine_speed = max(0, self.engine_speed - 200)
                    else:
                        # Idle fluctuations
                        self.engine_speed = 800 + random.uniform(-20, 20)

                self.working_state_driver1 = "REST"
                self.parking_brake_engaged = True

        # Adjust fuel consumption (simpler, based directly on current speed)
        if self.engine_speed > 0:
            # Higher consumption at higher speed
            base_consumption = 0.01 + (self.vehicle_speed / 1000)
            self.fuel_level = max(0, self.fuel_level - base_consumption)

            # AdBlue consumption (slower than fuel)
            self.adblue_level = max(
                0, self.adblue_level - base_consumption * 0.7)

        # Update distance
        if self.vehicle_speed > 0:
            # km/h to m/s
            self.traveled_distance += round(self.vehicle_speed / 3.6)

        # Update engine hours
        if self.engine_speed > 0:
            self.engine_hours += 1/3600

        # Very small temperature change
        self.air_temperature += random.uniform(-0.1, 0.1)
        self.air_temperature = max(-20, min(40, self.air_temperature))

        # Rare weight changes
        if random.random() < 0.01:
            self.current_weight += random.uniform(-30, 30)
            self.current_weight = max(10000, min(35000, self.current_weight))

        # Driver card changes
        if random.random() < 0.01:
            self.is_card_present_driver1 = not self.is_card_present_driver1

        if random.random() < 0.01:
            self.is_card_present_driver2 = not self.is_card_present_driver2

        # Update GPS position when vehicle is moving
        if self.vehicle_speed > 0:
            # Update GPS position every second based on speed

            # 1. Occasionally change heading for more realistic path
            if random.random() < 0.05:  # 5% chance to change direction slightly
                self.heading = (self.heading + random.uniform(-20, 20)) % 360

            # 2. Calculate distance traveled in this update
            # Speed is km/h, convert to meters per update cycle (assuming ~1 sec per update)
            distance_meters = self.vehicle_speed / 3.6

            # 3. Convert heading and distance to lat/lng changes
            # At these latitudes, 1 degree ~= 111km
            lat_change = distance_meters / 111000 * \
                math.cos(math.radians(self.heading))
            # Longitude degrees get shorter as we move north
            lng_factor = math.cos(math.radians(self.latitude))
            lng_change = distance_meters / \
                (111000 * lng_factor) * math.sin(math.radians(self.heading))

            # 4. Apply the changes
            self.latitude += lat_change
            self.longitude += lng_change

        # Rest periods may include small GPS jitter
        elif random.random() < 0.01:
            # Occasional tiny GPS fluctuation when stationary
            self.latitude += random.uniform(-0.00001, 0.00001)
            self.longitude += random.uniform(-0.00001, 0.00001)

        # Driver break logic and state management - completely rewritten
        # for consistency
        # Regular 5-minute breaks enforced regardless of vehicle state
        if self.cycle_count - self.last_driver1_break > 300:
            # Force break every 5 minutes
            self.driver1_needs_break = True
            self.last_driver1_break = self.cycle_count

        if self.cycle_count - self.last_driver2_break > 300:
            self.driver2_needs_break = True
            self.last_driver2_break = self.cycle_count

        # Update activity counters for driver1
        if self.working_state_driver1 != "REST":
            self.driver1_active_cycles += 1
            self.driver1_rest_cycles = 0

            if self.driver1_active_cycles >= self.DRIVE_DURATION_BEFORE_BREAK:
                self.driver1_needs_break = True
        else:
            self.driver1_rest_cycles += 1
            self.driver1_active_cycles = 0

            if self.driver1_needs_break and self.driver1_rest_cycles >= self.DRIVER_BREAK_DURATION:
                self.driver1_needs_break = False

        # Update activity counters for driver2
        if self.working_state_driver2 != "REST":
            self.driver2_active_cycles += 1
            self.driver2_rest_cycles = 0

            if self.driver2_active_cycles >= self.DRIVE_DURATION_BEFORE_BREAK:
                self.driver2_needs_break = True
        else:
            self.driver2_rest_cycles += 1
            self.driver2_active_cycles = 0

            if self.driver2_needs_break and self.driver2_rest_cycles >= self.DRIVER_BREAK_DURATION:
                self.driver2_needs_break = False

        # IMPORTANT: Force driver breaks regardless of other logic -
        # highest priority
        if self.driver1_needs_break:
            self.working_state_driver1 = "REST"

        if self.driver2_needs_break:
            self.working_state_driver2 = "REST"

        # Ensure driver roles are properly defined in all states
        if self.is_driving_period:
            # Primary driver needs to be driving if not on break
            if not self.driver1_needs_break and self.vehicle_speed > 5:
                self.working_state_driver1 = "DRIVE"
            elif not self.driver1_needs_break:
                # Vehicle stopped but driver active
                self.working_state_driver1 = "WORK"

            # Secondary driver is available or driving depending
            # on primary driver's state
            if not self.driver2_needs_break:
                if self.working_state_driver1 == "REST" and self.vehicle_speed > 5:
                    # If primary driver is resting but vehicle is moving,
                    # second driver takes over
                    self.working_state_driver2 = "DRIVE"
                elif self.working_state_driver1 == "REST":
                    # If both vehicle and primary driver are inactive,
                    # but secondary isn't on break
                    self.working_state_driver2 = "WORK"
                else:
                    # Default state for secondary driver when not active
                    self.working_state_driver2 = "DRIVER_AVAILABLE"
        else:
            # Vehicle not in driving period - default to REST for both unless
            # otherwise specified
            if not self.driver1_needs_break and self.vehicle_speed <= 0:
                self.working_state_driver1 = "REST"

            if not self.driver2_needs_break and self.working_state_driver2 != "DRIVE":
                self.working_state_driver2 = "REST"

        # Additionally, driver swap logic for long drives
        if random.random() < 0.01 and self.is_driving_period and self.vehicle_speed > 20:  # 1% chance per cycle
            # Switch driving duties between drivers if possible
            if self.working_state_driver1 == "DRIVE" and not self.driver2_needs_break:
                # First driver takes a break, second driver takes over
                self.working_state_driver1 = "REST"
                self.working_state_driver2 = "DRIVE"
                self.driver1_active_cycles = 0  # Reset counter
                self.last_driver1_break = self.cycle_count  # Record break time
            elif self.working_state_driver2 == "DRIVE" and not self.driver1_needs_break:
                # Second driver takes a break, first driver takes over
                self.working_state_driver2 = "REST"
                self.working_state_driver1 = "DRIVE"
                self.driver2_active_cycles = 0  # Reset counter
                self.last_driver2_break = self.cycle_count  # Record break time

        self.is_engine_running = self.engine_speed > 0

    def write_values(self, writer):
        writer.writerow([
            "current",
            "Vehicle.Powertrain.CombustionEngine.IsRunning",
            str(self.is_engine_running),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Tachograph.Driver.Driver1.IsCardPresent",
            str(self.is_card_present_driver1),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Tachograph.Driver.Driver1.WorkingState",
            self.working_state_driver1,
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Tachograph.Driver.Driver2.WorkingState",
            self.working_state_driver2,
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Tachograph.Driver.Driver2.IsCardPresent",
            str(self.is_card_present_driver2),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.CurrentLocation.Latitude",
            round(self.latitude, 6),
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.CurrentLocation.Longitude",
            round(self.longitude, 6),
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.CurrentLocation.Heading",
            round(self.heading, 3),
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.CurrentLocation.Speed",
            round(self.vehicle_speed, 1),
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.Speed",
            round(self.vehicle_speed, 1),
            0
        ])
        writer.writerow([
            "current",
            "Vehicle.Tachograph.VehicleSpeed",
            round(self.vehicle_speed, 1),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Powertrain.CurrentFuelType",
            self.fuel_type,
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Powertrain.FuelSystem.Tank.First.RelativeLevel",
            int(round(self.fuel_level)),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.TraveledDistanceHighRes",
            self.traveled_distance,
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Powertrain.CombustionEngine.DieselExhaustFluid.Level",
            int(round(self.adblue_level)),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Powertrain.CombustionEngine.Speed",
            round(self.engine_speed),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Chassis.ParkingBrake.IsEngaged",
            str(self.parking_brake_engaged),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Exterior.AirTemperature",
            round(self.air_temperature, 1),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.Powertrain.CombustionEngine.EngineHours",
            round(self.engine_hours, 3),
            round(random.uniform(0, 1), 4)
        ])
        writer.writerow([
            "current",
            "Vehicle.CurrentOverallWeight",
            int(round(self.current_weight)),
            round(random.uniform(0, 1), 4)
        ])

        # Neither VSS nor FMS do define these signals
        # writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver1.NeedsBreak",
        #                 str(self.driver1_needs_break), round(random.uniform(0, 1), 4)])
        # writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver2.NeedsBreak",
        #                 str(self.driver2_needs_break), round(random.uniform(0, 1), 4)])
        # writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver1.ActiveCycles",
        #                 self.driver1_active_cycles, round(random.uniform(0, 1), 4)])
        # writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver2.ActiveCycles",
        #                 self.driver2_active_cycles, round(random.uniform(0, 1), 4)])
        # writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver1.RestCycles",
        #                 self.driver1_rest_cycles, round(random.uniform(0, 1), 4)])
        # writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver2.RestCycles",
        #                 self.driver2_rest_cycles, round(random.uniform(0, 1), 4)])

    
# Set up argument parser for command line options
parser = argparse.ArgumentParser(description='Generate vehicle data CSV file.')
parser.add_argument('--output', '-o', type=str, default='signals.csv',
                    help='Output CSV file name (default: signals.csv)')
args = parser.parse_args()

# Generate and write data to a new CSV file
with open(args.output, mode='w', newline='') as file:

    id_driver1 = f"DRIVER_ID_{random.randint(100000, 999999)}"
    id_driver2 = f"DRIVER_ID_{random.randint(100000, 999999)}"

    vehicle_metadata = VehicleMetadata()
    vehicle_state = VehicleState()

    writer = csv.writer(file)
    writer.writerow(["field", "signal", "value", "delay"])  # Header

    # Write vehicle metadata so that it will be available in the Databroker from the very beginning
    vehicle_metadata.write_values(writer)

    writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver1.AuthenticationEquipment", "DRIVER_CARD", 0])
    writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver1.CardIssuingMemberState", "DE", 0])
    writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver1.CardRenewalIndex", "B", 0])
    writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver1.CardReplacementIndex", "1", 0])
    writer.writerow(["current", "Vehicle.Tachograph.Driver.Driver1.Identification", id_driver1, 0])

    for _ in range(280):  # Adjust number of rows as needed
        vehicle_state.update()
        vehicle_state.write_values(writer)

        # periodically write vehicle metadata to ensure it is available in the Databroker
        vehicle_metadata.write_values(writer)

print(f"CSV file '{args.output}' generated successfully.")

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
The Fleet Management Service (FMS) specification defines an HTTP based API for retrieving data gathered during operation of
fleets of trucks and/or buses. The syntax and semantics of data exchanged is defined in the rFMS 4.0 OpenAPI definition
file that can be downloaded from the [FMS website](https://www.fms-standard.com).

The [mapping-fms4-to-vss.md file](mapping-fms4-to-vss.md) defines a mapping of the data required by FMS to VSS Data Entries.
The [overlay folder](overlay) contains a VSS overlay that defines additional VSS Data Entries for those FMS data points for
which no standard VSS Data Entry exist (yet). The folder also contains a [pre-compiled JSON model file](spec/overlay/vss.json) that
contains all of the definitions from both the standard VSS as well as the overlay vspec files.
This model file can then be read in by the kuksa.val Databroker during startup using the `--metadata` switch.

Please refer to the [VSS documentation](https://covesa.github.io/vehicle_signal_specification/rule_set/overlay/) for details regarding VSS overlays.

## Creating the JSON Model File

The `vspec2json.py` program from the [COVESA VSS tools project](https://github.com/COVESA/vss-tools) can be used to (re-)create the JSON model file that
contains all of the standard VSS Data Entries plus the ones from the FMS overlay file.

1. Clone the COVESA VSS Signal Specification repository as described in its
   [Getting Started guide](https://github.com/COVESA/vehicle_signal_specification#contribute-to-vss). It is important to include the
   `--recurse-submodules` in order to initialize the submodule that contains the VSS Tools.
2. Check out the `v4.0` tag:
   ```sh
   git checkout --recurse-submodules v4.0
   ```
3. Follow the instructions given in the *Basic Setup* section of the `vss-tools/README.md` file.
4. Run the following command to create the JSON file:
   
   ```sh
   # in folder spec
   ${PATH_TO_CLONED_VSS_REPO}/vss-tools/vspec2json.py --strict --json-pretty -e dbc -o overlay/fms.vspec ${PATH_TO_CLONED_VSS_REPO}/spec/VehicleSignalSpecification.vspec overlay/vss.json
   ```

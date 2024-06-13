<!--
SPDX-FileCopyrightText: 2024 Contributors to the Eclipse Foundation

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

The COVESA CV Forwarder polls the signals Vehicle.Speed, Vehicle.CurrentLocation.{Latitude, Longitude, Altitude} from a [kuksa.val Databroker](https://github.com/eclipse/kuksa.val/tree/master/kuksa_databroker) and HTTP POSTs the data as a set of *measurements* to an InfluxDB server.

The implementation uses the proto definition for [kuksa.val.v1](https://github.com/eclipse/kuksa.val/tree/master/proto/kuksa/val/v1)
which has been copied to [/proto/kuksa/val/v1](/proto/kuksa/val/v1/).

# Building

Building the forwarder requires a [Rust development toolchain](https://rustup.rs/).

# Running

The covesa-cv-forwarder applies a curvelogging algorithm to filter out redundant data from a kuksa.val Databroker and forwards it to one of multiple supported back ends. The type of back end can be selected by means of command line arguments when starting the forwarder.

## Writing directly to an InfluxDB Server

The forwarder can write status information directly to an InfluxDB server using its HTTP based API.
For this to work, the forwarder needs to be configured with the URI of the InfluxDB server and an API token for authenticating to the server.

To run the COVESA CV Forwarder you can run the following script:

If you are running the forwarder for the first time or if you want to make sure that the latest changes were taken into account, run:
the build command:
`docker compose --profile covesa build --no-cache`

To run the covesa forwarder, you can use the .vscode/launch.json configuration 

otherwise, run:
`docker compose --profile covesa build`

then
`docker compose --profile covesa up`

## Log Output

To see the output of the program, run:
`docker compose logs -f covesa-cv-forwarder influxDB csv-provider-covesa databroker`

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
The FMS Forwarder polls FMS related data from a [kuksa.val Databroker](https://github.com/eclipse/kuksa.val/tree/master/kuksa_databroker)
and HTTP POSTs the data as a set of *measurements* to an InfluxDB server.

The implementation uses the proto definition for [kuksa.val.v1](https://github.com/eclipse/kuksa.val/tree/master/proto/kuksa/val/v1)
which has been copied to [/proto/kuksa/val/v1](/proto/kuksa/val/v1/).

# Building

Building the forwarder requires a [Rust development toolchain](https://rustup.rs/).

# Running

THe forwarder reads the current vehicle status data from a kuksa.val Databroker and forwards it to one of multiple supported
back ends. The type of back end can be selected by means of command line arguments when starting the forwarder.

Please refer to the command line help for details:

```sh
fms-forwarder --help
```

## Writing directly to an InfluxDB Server

The forwarder can write status information directly to an InfluxDB server using its HTTP based API.
For this to work, the forwarder needs to be configured with the URI of the InfluxDB server and an API token for
authenticating to the server.

Please refer to the command line help for details:

```sh
fms-forwarder influx --help
```

## Publishing to Eclipse Hono

The forwarder can publish status information to the MQTT adapter of an [Eclipse Hono](https://eclipse.org/hono) instance.
For this to work, the forwarder needs to be configured with the URI of the MQTT adapter endpoint, the credentials to use for
authentication and the name of the tenant that the device belongs to.

Please refer to the command line help for details:

```sh
fms-forwarder hono --help
```

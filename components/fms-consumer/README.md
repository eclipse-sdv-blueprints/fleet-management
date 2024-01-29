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
The FMS Consumer receives vehicle data either via Hono's north bound Kafka based Telemetry API or Zenoh router and writes it to the Influx DB.


# Building

Building the consumer requires a [Rust development toolchain](https://rustup.rs/).

# Running

The FMS Consumer receives vehicle data either via Hono's north bound Kafka based Telemetry API or Zenoh router and writes it to the Influx DB. The type of source can be selected by means of command line arguments when starting the consumer.

Please refer to the command line help for details:

```sh
fms-consumer --help
```

## Received data from Hono's north bound Kafka based Telemetry API 

The consumer can received data from Hono's north bound Kafka based Telemetry API 

Please refer to the command line help for details:

```sh
fms-consumer kafka --help
```


## Received data from Zenoh Router

The consumer can subscriber status from Zenoh Router of an [Eclipse Zenoh](https://projects.eclipse.org/projects/iot.zenoh/) instance.
For this to work, the consumer needs to be configured with the zenoh router end points,

Please refer to the command line help for details:

```sh
fms-consumer zenoh --help
```

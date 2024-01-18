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
The [Eclipse SDV Blueprints](https://github.com/eclipse-sdv-blueprints) project is a collaborative initiative
led by Eclipse SDV members to bring the *software defined vehicle* concepts to life.

The project hosts a collection of blueprints that demonstrate the application of technologies developed in
the context of the [Eclipse SDV Working Group](https://sdv.eclipse.org).

This repository contains the **Fleet Management Blueprint** which is a close to *real-life* showcase
for truck fleet management where trucks run an SDV software stack so that logistics fleet operators can
manage apps, data and services for a diverse set of vehicles.

The use case illustrates how the standard VSS model can be customized and used to report data from a vehicle
to a back end. The following diagram provides an overview of the current architecture:

<img src="img/architecture.drawio.svg">

The overall idea is to enable back end applications to consume data coming from a vehicle using the rFMS API.

Data originates from the vehicle's sensors which are represented by a CSV file that is being played back by the
kuksa.val CSV feeder. The CSV feeder publishes the data to the kuksa.val Databroker. From there, the FMS Forwarder
consumes the data and writes it to an InfluxDB in the back end. The measurements in the InfluxDB can then be
visualized in a web browser by means of a Grafana dashboard. Alternatively, the measurements can be retrieved by
a Fleet Management application via the FMS Server's (HTTP based) rFMS API.

# Quick Start

The easiest way to set up and start the services is by means of using the Docker Compose file in the top level folder:

```sh
docker compose -f ./fms-blueprint-compose.yaml up --detach
```

This will pull or build (if necessary) the container images and create and start all components.

Once all services have been started, the current vehicle status can be viewed on a [Grafana dashboard](http://127.0.0.1:3000),
using *admin*/*admin* as username and password for logging in.


The rFMS API can be used to retrieve the data, e.g.

```sh
curl -v -s http://127.0.0.1:8081/rfms/vehicleposition?latestOnly=true | jq
```

# Using Eclipse Hono to send Vehicle Data to Back End

By default, the Docker Compose file starts the FMS Forwarder configured to write vehicle data directly to the
Influx DB running in the back end.

However, in a real world scenario, this tight coupling between the vehicle and the Influx DB is not desirable.
As an alternative, the blueprint supports configuring the FMS Forwarder to send vehicle data to the MQTT adapter
of an Eclipse Hono instance as shown in the diagram below.

<img src="img/architecture-hono.drawio.svg">

1. Register the vehicle as a device in Hono using the [create-config-hono.sh shell script](./create-config-hono.sh):

   ```sh
   ./create-config-hono.sh --tenant MY_TENANT_ID --device-id MY_DEVICE_ID --device-pwd MY_PWD --provision
   ```

   Make sure to replace `MY_TENANT_ID`, `MY_DEVICE_ID` and `MY_PWD` with your own values.

   The script registers the tenant and device in Hono's Sandbox installation at `hono.eclipseprojects.io` unless the
   `--host` and/or `--kafka-brokers` command line arguments are used. Use the `--help` switch to print usage information.

   The script also creates configuration files in the `OUT_DIR/config/hono` folder. The OUT_DIR can be specified using
   the `--out-dir` option, default value is the current working directory. These files are used to configure the services
   started via Docker Compose in the next step.

2. Start up the vehicle and back end services using Docker Compose:

   ```sh
   docker compose --env-file ./config/hono/hono.env -f ./fms-blueprint-compose.yaml -f ./fms-blueprint-compose-hono.yaml up --detach
   ```

   The path set via the `--env-file` option needs to be adapted to the output folder specified in the previous step.

   The second compose file specified on the command line will also start the [FMS Consumer](./components/fms-consumer)
   back end component which receives vehicle data via Hono's north bound Kafka based Telemetry API and writes it to the
   Influx DB.

# Manual configuration

All information required for setting up the networks, volumes, configs and containers is contained in the
Docker Compose file. Please refer to the Docker and/or Podman documentation for details how to perform the
setup manually.

Additional information can be found in the components' corresponding subfolders.

# Contributing

We are looking forward to your ideas and PRs. Each PRs triggers a GitHub action which checks the formating, performs linting and runs the test. You can performe similar check in your development environment. For more details check the respective [action](.github/workflows/lint_source_code.yaml) where the checks are listed in the bottom of the file.


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
to a back end. It also shows how [Eclipse uProtocol&trade;](https://eclipse-uprotocol.github.io) can be used to connect in-vehicle components to an off-vehicle service in the back end. uProtocol provides a generic API for using the well known pub/sub and RPC message exchange patterns over arbitrary transport protocols like MQTT, Eclipse Zenoh etc.

The following component diagram provides a high level overview of the building blocks and how they are related to each other.

<img src="img/architecture-uprotocol.drawio.svg">

The overall idea is to enable back end applications to consume data coming from a vehicle using the rFMS API.

Data originates from the vehicle's sensors which are represented by a CSV file that is being played back by the
[Eclipse Kuksa&trade; CSV Provider](https://github.com/eclipse-kuksa/kuksa-csv-provider). The *CSV Provider* publishes
the data to the [Kuksa Databroker](https://github.com/eclipse-kuksa/kuksa-databroker).
The *FMS Forwarder* reads the signal values from the Databroker and sends them to the *FMS Consumer* in the back end. The FMS Consumer then writes the measurements to an *InfluxDB* from where it
can be visualized in a web browser by means of a *Grafana* dashboard. Alternatively, the measurements can be
retrieved by a Fleet Management application via the *FMS Server's* (HTTP based) rFMS API.

Both the FMS Forwarder and Consumer are implemened as uProtocol entities (_uEntity_). This allows the FMS Forwarder to send its data to the Consumer by means of a uProtocol _Notification_. The concrete transport protocol being used to transmit the message on the wire is a matter of configuration and has no impact on the implementation of the business logic itself.

# Quick Start

The easiest way to set up and start the services is by means of using the Docker Compose file in the top level folder:

```sh
docker compose -f ./fms-blueprint-compose.yaml -f ./fms-blueprint-compose-zenoh.yaml up --detach
```

This will pull (or build if necessary) the container images and create and start all components.

Once all services have been started, the current vehicle status can be viewed on a [Grafana dashboard](http://127.0.0.1:3000),
using *admin*/*admin* as username and password for logging in.


The rFMS API can be used to retrieve the data, e.g.

```sh
curl -v -s http://127.0.0.1:8081/rfms/vehicleposition?latestOnly=true | jq
```

# Eclipse Zenoh&trade; Transport

The command line from the quick start section will start up containers for the _FMS Forwarder_ and _FMS Consumer_ that are configured to use a [Zenoh](https://zenoh.io) based uProtocol transport as shown in the deployment diagram below:

<img src="img/architecture-zenoh.drawio.svg">

# Eclipse Hono&trade; based Transport

The blueprint can also be configured to use a Hono based uProtocol transport that employs Hono's MQTT adapter
and Apache Kafka&trade; based messaging infrastructure for sending vehicle data to the Consumer.

In order to run the blueprint with the Hono transport, perform the following steps:

1. Register the vehicle as a device in Hono using the [create-config-hono.sh shell script](./create-config-hono.sh):

   ```sh
   ./create-config-hono.sh --tenant MY_TENANT_ID --device-id MY_DEVICE_ID --device-pwd MY_PWD --provision
   ```

   Make sure to replace `MY_TENANT_ID`, `MY_DEVICE_ID` and `MY_PWD` with your own values.

   The script registers the tenant and device in [Hono's Sandbox installation](https://eclipse.dev/hono/sandbox/) at `hono.eclipseprojects.io` unless the
   `--host` and/or `--kafka-brokers` command line arguments are used. Use the `--help` switch to print usage information.

   The script also creates configuration files in the `OUT_DIR/config/hono` folder. The OUT_DIR can be specified using
   the `--out-dir` option, default value is the current working directory. These files are used to configure the services
   started via Docker Compose in the next step.

2. Start up the vehicle and back end services using Docker Compose:

   ```sh
   docker compose --env-file ./config/hono/hono.env -f ./fms-blueprint-compose.yaml -f ./fms-blueprint-compose-hono.yaml up --detach
   ```

   The path set via the `--env-file` option needs to be adapted to the output folder specified in the previous step.

This will result in a deployment as shown below:

<img src="img/architecture-hono.drawio.svg">

# Manual configuration

All information required for setting up the networks, volumes, configs and containers is contained in the
Docker Compose file. Please refer to the Docker and/or Podman documentation for details how to perform the
setup manually.

Additional information can be found in the components' corresponding subfolders.

# Contributing

We are looking forward to your ideas and PRs. Each PRs triggers a GitHub action which checks the formating, performs
linting and runs the test. You can performe similar check in your development environment. For more details check the
respective [action](.github/workflows/lint_source_code.yaml) where the checks are listed in the bottom of the file.

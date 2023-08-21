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
The components of the Fleet Management blueprint fall into two categories: the *kuksa.val Databroker*,
*CSV Provider* and *FMS Forwarder* components are supposed to run in the vehicle, whereas the remaining
components are supposed to run in a (cloud) back end to which the vehicle is connected via (public internet)
networking infrastructure.

All of the components can be run on a single Docker host as described in the [top level README](../README.md). However, the following sections describe a more realistic deployment scenario in which
[Eclipse Leda](https://eclipse-leda.github.io/leda/) is used as the vehicle runtime environment.

This guide was tested with release v0.1.0-M2 of Eclipse Leda.

# Start Back End Components

The containers for the back end components are run on the local host using Docker Compose:

```sh
# in this repository's root folder
docker compose -f fms-blueprint-compose.yaml up influxdb grafana fms-server --detach
```

# Start In-Vehicle Coponents

The containers for the in-vehicle components are deployed to a Leda instance running on the
local host. The *FMS Forwarder* running in Leda will then connect to the *influxdb* server running
on the local host.

Please refer to [Leda's Getting Started](https://eclipse-leda.github.io/leda/docs/general-usage/)
guide for setting up a Leda instance.

## Stop default Containers

Leda comes with a set of default containers (including kuksa.val Databroker) that are managed using
Eclipse Kanto's *container-manager*. These containers are defined by means of JSON manifest files in
Leda's `/data/var/containers/mainfests` folder.

We will stop and disable some of Leda's default containers, make some changes to the configuration of the
Databroker container and also deploy additional containers.

```sh
# in Leda instance's /data/var/containers/mainfests folder
tar cf manifests.orig.tar * 
kanto-cm stop --force -n feedercan
mv feedercan.json feedercan.json.disabled
kanto-cm stop --force -n hvacservice-example
mv hvac.json hvac.json.disabled
kanto-cm stop --force -n seatservice-example
mv seatservice.json seatservice.json.disabled
```

## Deploy in-vehicle Blueprint Components

Some of the Fleet Management blueprint containers require access to configuration files that are not present
in Leda, e.g. the FMS-specific VSS definitions.

Create the folders in Leda from which the containers can mount these files:

```sh
# in Leda instance
mkdir -p /data/usr/fms/csv /data/usr/fms/databroker /data/usr/fms/forwarder
```

Now copy the files required by the containers to the Leda instance:

```sh
# on the (local) host that you have started the back end components on
docker exec -it influxDB cat /tmp/out/fms-demo.token > /tmp/influxdb.token
```

```sh
# in this repository's root folder
scp -P 2222 csv-provider/signalsFmsRecording.csv root@127.0.0.1:/data/usr/fms/csv
scp -P 2222 spec/overlay/vss.json root@127.0.0.1:/data/usr/fms/databroker
scp -P 2222 /tmp/influxdb.token root@127.0.0.1:/data/usr/fms/forwarder
```

Finally, copy the manifest files to Leda, triggering the execution of the in-vehicle containers:

```sh
# in this repository's root folder
scp -P 2222 leda/data/var/containers/manifests/*.json root@127.0.0.1:/data/var/containers/manifests
```

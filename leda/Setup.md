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
# Setting Up a Leda Image

The setup for this FMS scenario assumes a split into components running in the vehicles and components running in a (cloud) backend. 
The following page describes how to configure a Leda instance to become part of this FMS setup. More precisely, we deploy the containers for the feedercan, the fms-forwarder, and the Kuksa.val databroker with an FMS-specific vehicle model.

## Run Leda
The following steps assume a running instance of Eclipse Leda. For more details see the [Eclipse Leda Getting Started](https://eclipse-leda.github.io/leda/docs/general-usage/)
This guide was tested with release v0.1.0-M1 of Eclipse Leda.

## Create GitHub Token to Read containers
The container image for the FMS-Forwarder is currently only available in a private GitHub container registry within the repository https://github.com/SoftwareDefinedVehicle/oss-tech-scouting-demo .
To enable Leda to access the container, you can create a GitHub token with the scope `read:packages`. This requires that your GitHub user has the respective access rights for that repository. 
In the GitHub user settings you can generate the token under Settings -> Developer Settings -> Personal Access Tokens -> Tokens (classic).

Afterwards, select `Configure SSO` and authorize the SoftwareDefinedVehicle organization. 

## Configure Leda to use GitHub Token
We now need to configure Leda with the newly created GitHub token, to enable the download from a private container registry. Execute in Leda:

```
sdv-kanto-ctl add-registry -h ghcr.io -u github -p <Your_GitHub_PersonalAccessToken>
``` 

For more details see the [Leda documentation](https://eclipse-leda.github.io/leda/docs/device-provisioning/container-management/container-registries/)

## Backup existing manifestes
In Leda, there are manifests to manage the execution of containers by Eclipse Kanto. During the next steps we will overwrite some of the default manifests (`databroker.json`, `feedercan.json`). Because of that, we recommend to backup the existing manifests from `/data/var/containers/mainfests`.

## Copy manifests to Leda
To trigger the execution of the required containers for the FMS setup, copy the manifest files from `leda/data/var/containers/manifests` in the host to `/data/var/containers/manifests` in Leda:

```
manifests % scp -P 2222 *.json root@127.0.0.1:/data/var/containers/manifests
```

## create folders in Leda mounted in mainfests
The containers described within the manifests require files that are not present in Leda like the FMS-specific vehicle model. 
Therefore, we need to create the paths (e.g,. `mkdir`) in Leda from which the containers try to mount these files which are:

- `mkdir -p /data/usr/fms/dbc`
- `mkdir -p /data/usr/databroker`

## copy required files to leda folders:
Now you can copy the files required by the containers to Leda. Execute in the root of this repository on the host: 
```
scp -P 2222 dbc-feeder/220421_MAN_Si_RIO_CAN_converted.log root@127.0.0.1:/data/usr/fms/dbc
scp -P 2222 dbc-feeder/j1939_REMODUL_v5.dbc root@127.0.0.1:/data/usr/fms/dbc
scp -P 2222 spec/overlay/vss.json root@127.0.0.1:/data/usr/fms/databroker
```

## start backend on host
Besides the in-vehicle containers in Leda, we still need the backend for the full FMS setup. You can start the remaining containers on the host with: 

```
docker compose -f fms-demo-compose.yaml up influxdb grafana fms-server --detach
```

## configure InfluxDB token
The fms-forwarder needs a token to write data into the InfluxDB which has been started on the host. 
There are multiple ways to retrieve this token, e.g., through the web-interface of InfluxDB (localhost:8086). One approach through the command line is the following:

```
docker exec -it influxDB cat /tmp/out/fms-demo.token 
```

You can then insert the token in the `/data/var/containers/manifestsfms-forwarder.json` manifest in Leda at the bottom of the file as value for `config.env.INFLUXAPI_TOKEN`.
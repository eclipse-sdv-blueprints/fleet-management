#!/bin/bash

# SPDX-FileCopyrightText: 2023 Contributors to the Eclipse Foundation
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

set -e

token=$(influx auth create \
  --hide-headers \
  --description "Token for writing to the FMS demo bucket" \
  --user "${DOCKER_INFLUXDB_INIT_USERNAME}" \
  --read-bucket "${DOCKER_INFLUXDB_INIT_BUCKET_ID}" \
  --write-bucket "${DOCKER_INFLUXDB_INIT_BUCKET_ID}" | awk -F '\t' '{print $3}')

echo "${token}" > /tmp/out/fms-demo.token

cat <<EOF > /tmp/influxdb-datasources/influxdb.yaml
apiVersion: 1
datasources:
- name: "InfluxDB-SDV-Flux"
  uid: "PDC312342D5DCA611"
  type: influxdb
  access: proxy
  url: http://influxdb:8086
  jsonData:
    version: Flux
    organization: ${DOCKER_INFLUXDB_INIT_ORG}
    defaultBucket: ${DOCKER_INFLUXDB_INIT_BUCKET}
    tlsSkipVerify: true
  secureJsonData:
    token: ${token}
EOF


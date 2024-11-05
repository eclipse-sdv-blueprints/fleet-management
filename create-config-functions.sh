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

#
# Utility methods for creating property files to be used with the Docker Compose
# files in this directory.
#

# create file with environment variables that the FMS Forwarder running in the vehicle
# will use to configure its connection to Hono's MQTT adapter
create_common_mqtt_client_env() {
  ENV_FILE_PATH=$1
  URI=$2
  TRUST_STORE=$3
  ENABLE_HOSTNAME_VALIDATION=$4
  echo "Creating MQTT client properties file ${ENV_FILE_PATH} ..."
  cat <<EOF > "${ENV_FILE_PATH}"
MQTT_URI=${URI}
TRUST_STORE_PATH=${TRUST_STORE}
ENABLE_HOSTNAME_VALIDATION=${ENABLE_HOSTNAME_VALIDATION}
EOF
}

# create file with environment variables that the FMS Forwarder running in the vehicle
# will use to configure its connection to Hono's MQTT adapter
create_mqtt_client_env() {
  ENV_FILE_PATH=$1
  URI=$2
  USERNAME=$3
  PASSWORD=$4
  TRUST_STORE=$5
  ENABLE_HOSTNAME_VALIDATION=$6
  create_common_mqtt_client_env \
    "${ENV_FILE_PATH}" \
    "${URI}" \
    "${TRUST_STORE}" \
    "${ENABLE_HOSTNAME_VALIDATION}"
  cat <<EOF >> "${ENV_FILE_PATH}"
MQTT_USERNAME=${USERNAME}
MQTT_PASSWORD=${PASSWORD}
EOF
}

# create file with environment variables that the FMS Forwarder running in the vehicle
# will use to configure its connection to Hono's MQTT adapter
create_mqtt_client_env_with_cert() {
  ENV_FILE_PATH=$1
  URI=$2
  CLIENT_CERT_PATH=$3
  CLIENT_KEY_PATH=$4
  TRUST_STORE=$5
  ENABLE_HOSTNAME_VALIDATION=$6
  create_common_mqtt_client_env \
    "${ENV_FILE_PATH}" \
    "${URI}" \
    "${TRUST_STORE}" \
    "${ENABLE_HOSTNAME_VALIDATION}"
  CONFIG_DIR_FMS_FORWARDER="${ENV_FILE_PATH%/*}/fms-forwarder"
  cp "${CLIENT_CERT_PATH}" "${CLIENT_KEY_PATH}" "${CONFIG_DIR_FMS_FORWARDER}"
  cat <<EOF >> "${ENV_FILE_PATH}"
DEVICE_CERT=/app/config/$(basename "$CLIENT_CERT_PATH")
DEVICE_KEY=/app/config/$(basename "$CLIENT_KEY_PATH")
EOF
}

# create file with environment variables to be used with Docker Compose when
# starting the services:
#
# docker compose --env-file hono.env ...
# 
create_docker_compose_env() {
  ENV_FILE_PATH=$1
  CONSUMER_CONFIG_FOLDER=$2
  CONSUMER_KAFKA_PROPERTIES_FILE_NAME=$3
  CONSUMER_HONO_TENANT_ID=$4
  FORWARDER_CONFIG_FOLDER=$5
  FORWARDER_MQTT_ENV_FILE_PATH=$6
  echo "Creating Docker Compose env file ${ENV_FILE_PATH} ..."
  cat <<EOF > "${ENV_FILE_PATH}"
FMS_CONSUMER_CONFIG_FOLDER=${CONSUMER_CONFIG_FOLDER}
FMS_CONSUMER_KAFKA_PROPERTIES_FILE=${CONSUMER_KAFKA_PROPERTIES_FILE_NAME}
FMS_CONSUMER_HONO_TENANT_ID=${CONSUMER_HONO_TENANT_ID}
FMS_FORWARDER_CONFIG_FOLDER=${FORWARDER_CONFIG_FOLDER}
FMS_FORWARDER_MQTT_ENV_FILE=${FORWARDER_MQTT_ENV_FILE_PATH}
EOF
}

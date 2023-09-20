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
# This script can be used to provision a tenant and a device to an existing
# Hono instance.
#
# By default, the device will be provisioned to the Hono sandbox.
#
set -e

HONO_HOST="hono.eclipseprojects.io"
HONO_REGISTRY_PORT="28443"
HONO_TENANT_ID=""
HONO_TENANT_CA=""
HONO_DEVICE_ID=""
HONO_DEVICE_PASSWORD=""
HONO_DEVICE_CERT=""
HONO_DEVICE_KEY=""
HONO_KAFKA_BROKERS=""
HONO_KAFKA_SECURE_PORT=9094
HONO_KAFKA_USER="hono"
HONO_KAFKA_PASSWORD="hono-secret"
OUT_DIR="."
PROVISION_TO_HONO=""

print_usage() {
  cmd_name=$(basename "$0")
  cat <<EOF >&2 
Usage: ${cmd_name} OPTIONS ...

OPTIONS

-h | --help               Display this usage information.
-t | --tenant             The identifier of the tenant to create the device for.
--tenant-ca               The path to a PEM file containing an X.509 certificate which should be set as the tenant's trust anchor for authenticating devices using a client certificate.
--device-id               The identifier of the device to create.
--device-pwd              The password that the device needs to use for authenticating to Hono's MQTT adapter.
--device-cert             The path to a PEM file containing an X.509 certificate that the device uses for authenticating to Hono's MQTT adapter.
--device-key              The path to a PEM file containing the private key that the device uses for authenticating to Hono's MQTT adapter.
-H | --host               The host name or IP address of Hono's device registry. [${HONO_HOST}]
-p | --port               The TLS port of the Hono device registry. [${HONO_REGISTRY_PORT}]
--kafka-brokers           A comma separated list of host name/IP address:port tuples of the Kafka broker(s) to consume messages from. [${HONO_HOST}:${HONO_KAFKA_SECURE_PORT}]
--kafka-user              The username to use for authenticating to the Kafka broker(s) to consume messages from. [${HONO_KAFKA_USER}]
--kafka-pwd               The password to use for authenticating to the Kafka broker(s) to consume messages from. [${HONO_KAFKA_PASSWORD}]
--out-dir                 The path to the folder to write configuration files to. [${OUT_DIR}]
--provision               Also provision device information to Hono's Device Registry.

Examples:

Create configuration files in current working directory and provision device information to Hono Sandbox

  ${cmd_name} --tenant my-test-tenant --device-id my-device --device-pwd verysecret --provision

Create configuration files in home directory

  ${cmd_name} -t my-test-tenant --device-id my-device --device-pwd verysecret -o ~/

EOF
}

error_and_exit() {
  echo "ERROR: $1" >&2
  echo
  print_usage
  exit 1
}

provision_device() {
  REGISTRY_URI="https://${HONO_HOST}:${HONO_REGISTRY_PORT}/v1"
  OPENSSL_CMD=$(which openssl)

  TRUST_ANCHOR_DEF=""
  if [[ -n ${HONO_TENANT_CA} ]]; then
    if [[ -z "${OPENSSL_CMD}" ]]; then
      echo "Could not find openssl command, skipping registration of trust anchor with tenant ..."
    else
      CERT=$(openssl x509 -in "${HONO_TENANT_CA}" -outform PEM | sed /^---/d | sed -z 's/\n//g')
      TRUST_ANCHOR_DEF='{"cert": "'${CERT}'"}'
    fi
  fi
  curl --silent --show-error --fail -X POST -H "content-type: application/json" --data-binary '{
    "trusted-ca": ['"${TRUST_ANCHOR_DEF}"'],
    "ext": {
      "messaging-type": "kafka"
    }
  }' "${REGISTRY_URI}/tenants/${HONO_TENANT_ID}"
  echo "Successfully created tenant [${HONO_TENANT_ID}] in Hono registry [${REGISTRY_URI}]"

  curl --silent --show-error --fail -X POST "${REGISTRY_URI}/devices/${HONO_TENANT_ID}/${HONO_DEVICE_ID}"
  echo "Successfully added vehicle device [${HONO_DEVICE_ID}] to tenant [${HONO_TENANT_ID}]"

  if [[ -n "${HONO_DEVICE_PASSWORD}" ]]; then
    curl --silent --show-error --fail -X PUT -H "content-type: application/json" --data-binary '[{
      "type": "hashed-password",
      "auth-id": "'"${HONO_DEVICE_ID}"'",
      "secrets": [{
          "pwd-plain": "'"${HONO_DEVICE_PASSWORD}"'"
      }]
    }]' "${REGISTRY_URI}/credentials/${HONO_TENANT_ID}/${HONO_DEVICE_ID}"
    echo "Successfully created hashed password credentials for vehicle device [${HONO_DEVICE_ID}]"
  fi
  if [[ -n "${HONO_DEVICE_CERT}" ]]; then
    if [[ -z "${OPENSSL_CMD}" ]]; then
      echo "Could not find openssl command, skipping registration of X.509 certificate based credentials ..."
    else
      CERT=$(openssl x509 -in "${HONO_DEVICE_CERT}" -outform PEM | sed /^---/d | sed -z 's/\n//g')
      curl --silent --show-error --fail -X PUT -H "content-type: application/json" --data-binary '[{
        "type": "x509-cert",
        "cert": "'"${CERT}"'",
        "secrets": [{}]
      }]' "${REGISTRY_URI}/credentials/${HONO_TENANT_ID}/${HONO_DEVICE_ID}"
    echo "Successfully created X.509 client certificate based credentials for vehicle device [${HONO_DEVICE_ID}]"
    fi
  fi
} 

# shellcheck source=create-config-functions.sh
source "${0%/*}/create-config-functions.sh"

while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do case $1 in
  -h | --help )
    print_usage
    exit 1
    ;;
  -t | --tenant )
    shift; HONO_TENANT_ID=$1
    ;;
  --tenant-ca )
    shift; HONO_TENANT_CA=$1
    ;;
  --device-id )
    shift; HONO_DEVICE_ID=$1
    ;;
  --device-pwd )
    shift; HONO_DEVICE_PASSWORD=$1
    ;;
  --device-cert )
    shift; HONO_DEVICE_CERT=$1
    ;;
  --device-key )
    shift; HONO_DEVICE_KEY=$1
    ;;
  -H | --host )
    shift; HONO_HOST=$1
    ;;
  -p | --port )
    shift; HONO_REGISTRY_PORT=$1
    ;;
  --kafka-brokers )
    shift; HONO_KAFKA_BROKERS=$1
    ;;
  --kafka-user )
    shift; HONO_KAFKA_USER=$1
    ;;
  --kafka-pwd )
    shift; HONO_KAFKA_PASSWORD=$1
    ;;
  -o | --out-dir )
    shift; OUT_DIR=$1
    ;;
  --provision )
    PROVISION_TO_HONO=1
    ;;
  *)
    echo "Ignoring unknown option: $1"
    echo "Run with flag -h for usage"
    ;;
esac; shift; done
if [[ "$1" == '--' ]]; then shift; fi

if [[ -z "${HONO_TENANT_ID}" ]]; then
  error_and_exit "Missing required option: tenant"
fi
if [[ -z "${HONO_DEVICE_ID}" ]]; then
  error_and_exit "Missing required option: device-id"
fi
if [[ -z "${HONO_DEVICE_PASSWORD}" && -z ${HONO_DEVICE_CERT} ]]; then
  error_and_exit "Missing required option: device-pwd | device-cert"
fi
if [[ -n "${HONO_DEVICE_CERT}" && -z "${HONO_DEVICE_KEY}" ]]; then
  error_and_exit "Missing required option: device-key"
fi
if [[ -z "${HONO_DEVICE_CERT}" && -n "${HONO_DEVICE_KEY}" ]]; then
  error_and_exit "Missing required option: device-cert"
fi
if [[ -n ${PROVISION_TO_HONO} && -z ${HONO_TENANT_CA} ]]; then
  error_and_exit "Missing required option: tenant-ca"
fi
if [[ ! -e ${HONO_DEVICE_CERT} || ! -r ${HONO_DEVICE_CERT} ]]; then
  error_and_exit "Cannot read client certificate from ${HONO_DEVICE_CERT}"
fi
if [[ ! -e ${HONO_DEVICE_KEY} || ! -r ${HONO_DEVICE_KEY} ]]; then
  error_and_exit "Cannot read client key from ${HONO_DEVICE_KEY}"
fi

if [[ -z "${HONO_KAFKA_BROKERS}" ]]; then
  HONO_KAFKA_BROKERS=${HONO_HOST}:${HONO_KAFKA_SECURE_PORT}
fi

CONFIG_DIR_BASE="${OUT_DIR}/config/hono"
CONFIG_DIR_FMS_CONSUMER="${CONFIG_DIR_BASE}/fms-consumer"
mkdir -p "${CONFIG_DIR_FMS_CONSUMER}"
CONFIG_DIR_FMS_FORWARDER="${CONFIG_DIR_BASE}/fms-forwarder"
mkdir -p "${CONFIG_DIR_FMS_FORWARDER}"

if [[ -n "${PROVISION_TO_HONO}" ]]; then
  provision_device
fi

# create kafka client properties file for use with the FMS Consumer running in the back end
KAFKA_PROPERTIES_FILE="${CONFIG_DIR_FMS_CONSUMER}/kafka.properties"
echo "Creating Kafka client properties file ${KAFKA_PROPERTIES_FILE} ..."
cat <<EOF > "${KAFKA_PROPERTIES_FILE}"
bootstrap.servers=${HONO_KAFKA_BROKERS}
group.id=fms-demo-consumer
enable.partition.eof=false
session.timeout.ms=6000
enable.auto.commit=true
security.protocol=SASL_SSL
sasl.mechanism=SCRAM-SHA-512
sasl.username=${HONO_KAFKA_USER}
sasl.password=${HONO_KAFKA_PASSWORD}
ssl.ca.location=/etc/ssl/certs/ca-certificates.crt
EOF

# create file with environment variables that the FMS Forwarder running in the vehicle
# will use to configure its connection to Hono's MQTT adapter
MQTT_PROPS_FILE="${CONFIG_DIR_BASE}/mqtt-client.env"
if [[ -n "${HONO_DEVICE_CERT}" ]]; then
  create_mqtt_client_env_with_cert \
    "${MQTT_PROPS_FILE}" \
    "mqtts://${HONO_HOST}:8883" \
    "${HONO_DEVICE_CERT}" \
    "${HONO_DEVICE_KEY}" \
    "/etc/ssl/certs/ca-certificates.crt"
else
  create_mqtt_client_env \
    "${MQTT_PROPS_FILE}" \
    "mqtts://${HONO_HOST}:8883" \
    "${HONO_DEVICE_ID}@${HONO_TENANT_ID}" \
    "${HONO_DEVICE_PASSWORD}" \
    "/etc/ssl/certs/ca-certificates.crt"
fi
# create file with environment variables to be used with Docker Compose when
# starting the services:
#
# docker compose --env-file hono.env ...
# 
create_docker_compose_env \
  "${CONFIG_DIR_BASE}/hono.env" \
  "${CONFIG_DIR_FMS_CONSUMER}" \
  "kafka.properties" \
  "hono.telemetry.${HONO_TENANT_ID}" \
  "${CONFIG_DIR_FMS_FORWARDER}" \
  "${MQTT_PROPS_FILE}"

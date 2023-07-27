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
HONO_DEVICE_ID=""
HONO_DEVICE_PASSWORD=""
HONO_KAFKA_BROKERS=""
HONO_KAFKA_SECURE_PORT=9094
HONO_KAFKA_USER="hono"
HONO_KAFKA_PASSWORD="hono-secret"

print_usage() {
    echo "Usage: provision-vehicle-to-hono --tenant=TENANT --device-id=DEVICE --device-pwd=PASSWORD [--host=HOST] [--port=PORT] [--kafka-brokers=BROKERS] [--kafka-user=USER] [--kafka-pwd=PASSWORD]"
    echo
    echo "--tenant         The identifier of the tenant to create the device for."
    echo "--device-id      The identifier of the device to create."
    echo "--device-pwd     The password that the device needs to use for authenticating to Hono's protocol adapters."
    echo "--host           The host name or IP address of Hono's device registry. [${HONO_HOST}]"
    echo "--port           The TLS port of the Hono device registry. [${HONO_REGISTRY_PORT}]"
    echo "--kafka-brokers  A comma separated list of host name/IP address:port tuples of the Kafka broker(s) used by Hono. [${HONO_HOST}:${HONO_KAFKA_SECURE_PORT}]"
    echo "--kafka-user     The username to use for authenticating to the Kafka broker(s) used by Hono. [${HONO_KAFKA_USER}]"
    echo "--kafka-pwd      The password to use for authenticating to the Kafka broker(s) used by Hono. [${HONO_KAFKA_PASSWORD}]"
    echo
}

for i in "$@"
do
  case $i in
    --tenant=*)
    HONO_TENANT_ID="${i#*=}"
    ;;
    --device-id=*)
    HONO_DEVICE_ID="${i#*=}"
    ;;
    --device-pwd=*)
    HONO_DEVICE_PASSWORD="${i#*=}"
    ;;
    --host=*)
    HONO_HOST="${i#*=}"
    ;;
    --port=*)
    HONO_REGISTRY_PORT="${i#*=}"
    ;;
    --kafka-brokers=*)
    HONO_KAFKA_BROKERS="${i#*=}"
    ;;
    --kafka-user=*)
    HONO_KAFKA_USER="${i#*=}"
    ;;
    --kafka-pwd=*)
    HONO_KAFKA_PASSWORD="${i#*=}"
    ;;
    --help)
      print_usage
      exit 1
    ;;
    -h)
      print_usage
      exit 1
    ;;
    *)
      echo "Ignoring unknown option: $i"
      echo "Run with flag -? for usage"
    ;;
  esac
done

if [[ -z "${HONO_TENANT_ID}" ]]; then
  echo "Missing required parameter: tenant"
  print_usage
  exit 1
fi
if [[ -z "${HONO_DEVICE_ID}" ]]; then
  echo "Missing required parameter: device-id"
  print_usage
  exit 1
fi
if [[ -z "${HONO_DEVICE_PASSWORD}" ]]; then
  echo "Missing required parameter: device-pwd"
  print_usage
  exit 1
fi
if [[ -z "${HONO_KAFKA_BROKERS}" ]]; then
  HONO_KAFKA_BROKERS=${HONO_HOST}:${HONO_KAFKA_SECURE_PORT}
fi

HONO_REGISTRY_URI="https://${HONO_HOST}:${HONO_REGISTRY_PORT}/v1"

curl --silent --show-error --fail -X POST -H "content-type: application/json" --data-binary '{
  "ext": {
    "messaging-type": "kafka"
  }
}' ${HONO_REGISTRY_URI}/tenants/${HONO_TENANT_ID}

curl --silent --show-error --fail -X POST ${HONO_REGISTRY_URI}/devices/${HONO_TENANT_ID}/${HONO_DEVICE_ID}

curl --silent --show-error --fail -X PUT -H "content-type: application/json" --data-binary '[{
  "type": "hashed-password",
  "auth-id": "'${HONO_DEVICE_ID}'",
  "secrets": [{
      "pwd-plain": "'${HONO_DEVICE_PASSWORD}'"
  }]
}]' ${HONO_REGISTRY_URI}/credentials/${HONO_TENANT_ID}/${HONO_DEVICE_ID}

# create file with environment variables for use with the FMS Forwarder running in the vehicle
cat <<EOF > hono-mqtt.env
MQTT_URI=mqtts://${HONO_HOST}:8883
MQTT_USERNAME=${HONO_DEVICE_ID}@${HONO_TENANT_ID}
MQTT_PASSWORD=${HONO_DEVICE_PASSWORD}
TRUST_STORE_PATH=/etc/ssl/certs/ca-certificates.crt
EOF

cat <<EOF > hono-kafka.env
KAFKA_TOPIC_NAME=hono.telemetry.${HONO_TENANT_ID}
EOF

# create properties file for use with the FMS Consumer running in the back end
cat <<EOF > hono-kafka.properties
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

echo "successfully provisioned vehicle device [${HONO_DEVICE_ID}] in tenant [${HONO_TENANT_ID}] for Hono instance [${HONO_HOST}]"

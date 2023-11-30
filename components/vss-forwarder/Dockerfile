# /********************************************************************************
# * Copyright (c) 2023 Contributors to the Eclipse Foundation
# *
# * See the NOTICE file(s) distributed with this work for additional
# * information regarding copyright ownership.
# *
# * This program and the accompanying materials are made available under the
# * terms of the Apache License 2.0 which is available at
# * https://www.apache.org/licenses/LICENSE-2.0
# *
# * SPDX-License-Identifier: Apache-2.0
# ********************************************************************************/


FROM python:3.10-alpine

# alpine-sdk and linux-headers needed for Alpine to build grpcio for aarch64, not needed for platforms with prebuilt grpcio
#RUN apk add --no-cache alpine-sdk linux-headers

WORKDIR /app

COPY requirements.txt /app/requirements.txt

ENV PIP_ROOT_USER_ACTION=ignore
ENV PYTHONUNBUFFERED=yes

RUN pip install --upgrade pip
RUN pip install --no-cache-dir -r requirements.txt

COPY entrypoint.sh /app/entrypoint.sh
COPY vss-forwarder.py /app/vss-forwarder.py

ENV INFLUXDB_TOKEN_FILE="/tmp/fms-demo.token"
ENV KUKSA_DATA_BROKER_ADDR="databroker"
ENV KUKSA_DATA_BROKER_PORT="55556"
ENV SIGNAL_FILTER="Vehicle.Speed,Vehicle.Powertrain.FuelSystem.Tank.First.RelativeLevel,Vehicle.Exterior.AirTemperature"

ENTRYPOINT [ "/app/entrypoint.sh" ]


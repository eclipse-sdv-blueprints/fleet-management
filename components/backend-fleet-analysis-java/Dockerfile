# /********************************************************************************
# * Copyright (c) 2026 Contributors to the Eclipse Foundation
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

FROM maven:3.9.6-eclipse-temurin-21 AS build

WORKDIR /workspace
COPY pom.xml .
COPY src ./src
RUN mvn -q -DskipTests package

FROM eclipse-temurin:21-jre

ARG PAYARA_MICRO_VERSION=6.2024.2
WORKDIR /opt/payara
ADD https://repo1.maven.org/maven2/fish/payara/extras/payara-micro/${PAYARA_MICRO_VERSION}/payara-micro-${PAYARA_MICRO_VERSION}.jar /opt/payara/payara-micro.jar

COPY --from=build /workspace/target/*.war /opt/payara/fleet-analysis-backend.war

ENV TMPDIR=/tmp

RUN mkdir -p /tmp && chmod 1777 /tmp

EXPOSE 8080
ENTRYPOINT ["java","-Djava.io.tmpdir=/tmp","-jar","/opt/payara/payara-micro.jar","--deploy","/opt/payara/fleet-analysis-backend.war","--contextRoot","/fleet-analysis"]

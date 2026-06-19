// ********************************************************************************
// * Copyright (c) 2026 Contributors to the Eclipse Foundation
// *
// * See the NOTICE file(s) distributed with this work for additional
// * information regarding copyright ownership.
// *
// * This program and the accompanying materials are made available under the
// * terms of the Apache License 2.0 which is available at
// * https://www.apache.org/licenses/LICENSE-2.0
// *
// * SPDX-License-Identifier: Apache-2.0
// ********************************************************************************

package org.eclipse.sdv.fleet.analysis;

import com.influxdb.client.InfluxDBClient;
import com.influxdb.client.InfluxDBClientFactory;
import com.influxdb.client.WriteApi;
import com.influxdb.client.domain.WritePrecision;
import com.influxdb.client.write.Point;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;
import jakarta.enterprise.context.ApplicationScoped;

@ApplicationScoped
public class InfluxDbWriter {
  private InfluxDBClient client;
  private WriteApi writeApi;

  @PostConstruct
  void init() {
    InfluxDbConfig config = InfluxDbConfig.fromEnv();
    client =
        InfluxDBClientFactory.create(
            config.getUri(), config.getToken().toCharArray(), config.getOrg(), config.getBucket());
    writeApi = client.getWriteApi();
  }

  @PreDestroy
  void shutdown() {
    if (writeApi != null) {
      writeApi.close();
    }
    if (client != null) {
      client.close();
    }
  }

  public void writeHeader(
      String vin, String trigger, long createdDateTime, InfluxTelemetryPayload.Header header) {
    Point point =
        Point.measurement("header")
            .addTag("vin", vin)
            .addTag("trigger", trigger)
            .addField("createdDateTime", createdDateTime)
            .time(createdDateTime, WritePrecision.MS);

    if (header != null) {
      addFieldIfPresent(point, "hrTotalVehicleDistance", header.getHrTotalVehicleDistance());
      addFieldIfPresent(point, "grossCombinationVehicleWeight", header.getGrossCombinationVehicleWeight());
      addFieldIfPresent(point, "totalEngineHours", header.getTotalEngineHours());
      addFieldIfPresent(point, "totalElectricMotorHours", header.getTotalElectricMotorHours());
      addFieldIfPresent(point, "engineTotalFuelUsed", header.getEngineTotalFuelUsed());
      addFieldIfPresent(point, "driver1Id", header.getDriver1Id());
      addFieldIfPresent(point, "driver1IdCardIssuer", header.getDriver1IdCardIssuer());
    }

    writeApi.writePoint(point);
  }

  public void writeSnapshot(
      String vin, String trigger, long createdDateTime, InfluxTelemetryPayload.Snapshot snapshot) {
    Point point =
        Point.measurement("snapshot")
            .addTag("vin", vin)
            .addTag("trigger", trigger)
            .addField("createdDateTime", createdDateTime)
            .time(createdDateTime, WritePrecision.MS);

    if (snapshot != null) {
      addFieldIfPresent(point, "latitude", snapshot.getLatitude());
      addFieldIfPresent(point, "longitude", snapshot.getLongitude());
      addFieldIfPresent(point, "heading", snapshot.getHeading());
      addFieldIfPresent(point, "altitude", snapshot.getAltitude());
      addFieldIfPresent(point, "speed", snapshot.getSpeed());
      addFieldIfPresent(point, "positionDateTime", snapshot.getPositionDateTime());
      addFieldIfPresent(point, "wheelBasedSpeed", snapshot.getWheelBasedSpeed());
      addFieldIfPresent(point, "tachographSpeed", snapshot.getTachographSpeed());
      addFieldIfPresent(point, "engineSpeed", snapshot.getEngineSpeed());
      addFieldIfPresent(point, "fuelType", snapshot.getFuelType());
      addFieldIfPresent(point, "catalystFuelLevel", snapshot.getCatalystFuelLevel());
      addFieldIfPresent(point, "fuelLevel1", snapshot.getFuelLevel1());
      addFieldIfPresent(point, "fuelLevel2", snapshot.getFuelLevel2());
      addFieldIfPresent(point, "driver1WorkingState", snapshot.getDriver1WorkingState());
      addFieldIfPresent(point, "driver2WorkingState", snapshot.getDriver2WorkingState());
      addFieldIfPresent(point, "ambientAirTemperature", snapshot.getAmbientAirTemperature());
      addFieldIfPresent(point, "parkingBrakeSwitch", snapshot.getParkingBrakeSwitch());
      addFieldIfPresent(point, "estimatedDistanceToEmptyFuel", snapshot.getEstimatedDistanceToEmptyFuel());
      addFieldIfPresent(point, "estimatedDistanceToEmptyTotal", snapshot.getEstimatedDistanceToEmptyTotal());
      addFieldIfPresent(point, "driver2Id", snapshot.getDriver2Id());
      addFieldIfPresent(point, "driver2IdCardIssuer", snapshot.getDriver2IdCardIssuer());
    }

    writeApi.writePoint(point);
  }

  private static void addFieldIfPresent(Point point, String name, Double value) {
    if (value != null) {
      point.addField(name, value);
    }
  }

  private static void addFieldIfPresent(Point point, String name, Long value) {
    if (value != null) {
      point.addField(name, value);
    }
  }

  private static void addFieldIfPresent(Point point, String name, Integer value) {
    if (value != null) {
      point.addField(name, value);
    }
  }

  private static void addFieldIfPresent(Point point, String name, Boolean value) {
    if (value != null) {
      point.addField(name, value);
    }
  }

  private static void addFieldIfPresent(Point point, String name, String value) {
    if (value != null && !value.isBlank()) {
      point.addField(name, value);
    }
  }
}

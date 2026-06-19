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

public class InfluxTelemetryPayload {
  private String vin;
  private String trigger;
  private Long createdDateTime;
  private Header header;
  private Snapshot snapshot;

  public String getVin() {
    return vin;
  }

  public void setVin(String vin) {
    this.vin = vin;
  }

  public String getTrigger() {
    return trigger;
  }

  public void setTrigger(String trigger) {
    this.trigger = trigger;
  }

  public Long getCreatedDateTime() {
    return createdDateTime;
  }

  public void setCreatedDateTime(Long createdDateTime) {
    this.createdDateTime = createdDateTime;
  }

  public Header getHeader() {
    return header;
  }

  public void setHeader(Header header) {
    this.header = header;
  }

  public Snapshot getSnapshot() {
    return snapshot;
  }

  public void setSnapshot(Snapshot snapshot) {
    this.snapshot = snapshot;
  }

  public static class Header {
    private Double hrTotalVehicleDistance;
    private Double grossCombinationVehicleWeight;
    private Double totalEngineHours;
    private Double totalElectricMotorHours;
    private Double engineTotalFuelUsed;
    private String driver1Id;
    private String driver1IdCardIssuer;

    public Double getHrTotalVehicleDistance() {
      return hrTotalVehicleDistance;
    }

    public void setHrTotalVehicleDistance(Double hrTotalVehicleDistance) {
      this.hrTotalVehicleDistance = hrTotalVehicleDistance;
    }

    public Double getGrossCombinationVehicleWeight() {
      return grossCombinationVehicleWeight;
    }

    public void setGrossCombinationVehicleWeight(Double grossCombinationVehicleWeight) {
      this.grossCombinationVehicleWeight = grossCombinationVehicleWeight;
    }

    public Double getTotalEngineHours() {
      return totalEngineHours;
    }

    public void setTotalEngineHours(Double totalEngineHours) {
      this.totalEngineHours = totalEngineHours;
    }

    public Double getTotalElectricMotorHours() {
      return totalElectricMotorHours;
    }

    public void setTotalElectricMotorHours(Double totalElectricMotorHours) {
      this.totalElectricMotorHours = totalElectricMotorHours;
    }

    public Double getEngineTotalFuelUsed() {
      return engineTotalFuelUsed;
    }

    public void setEngineTotalFuelUsed(Double engineTotalFuelUsed) {
      this.engineTotalFuelUsed = engineTotalFuelUsed;
    }

    public String getDriver1Id() {
      return driver1Id;
    }

    public void setDriver1Id(String driver1Id) {
      this.driver1Id = driver1Id;
    }

    public String getDriver1IdCardIssuer() {
      return driver1IdCardIssuer;
    }

    public void setDriver1IdCardIssuer(String driver1IdCardIssuer) {
      this.driver1IdCardIssuer = driver1IdCardIssuer;
    }
  }

  public static class Snapshot {
    private Double latitude;
    private Double longitude;
    private Double heading;
    private Double altitude;
    private Double speed;
    private Long positionDateTime;
    private Double wheelBasedSpeed;
    private Double tachographSpeed;
    private Double engineSpeed;
    private String fuelType;
    private Double catalystFuelLevel;
    private Double fuelLevel1;
    private Double fuelLevel2;
    private String driver1WorkingState;
    private String driver2WorkingState;
    private Double ambientAirTemperature;
    private Boolean parkingBrakeSwitch;
    private Double estimatedDistanceToEmptyFuel;
    private Double estimatedDistanceToEmptyTotal;
    private String driver2Id;
    private String driver2IdCardIssuer;

    public Double getLatitude() {
      return latitude;
    }

    public void setLatitude(Double latitude) {
      this.latitude = latitude;
    }

    public Double getLongitude() {
      return longitude;
    }

    public void setLongitude(Double longitude) {
      this.longitude = longitude;
    }

    public Double getHeading() {
      return heading;
    }

    public void setHeading(Double heading) {
      this.heading = heading;
    }

    public Double getAltitude() {
      return altitude;
    }

    public void setAltitude(Double altitude) {
      this.altitude = altitude;
    }

    public Double getSpeed() {
      return speed;
    }

    public void setSpeed(Double speed) {
      this.speed = speed;
    }

    public Long getPositionDateTime() {
      return positionDateTime;
    }

    public void setPositionDateTime(Long positionDateTime) {
      this.positionDateTime = positionDateTime;
    }

    public Double getWheelBasedSpeed() {
      return wheelBasedSpeed;
    }

    public void setWheelBasedSpeed(Double wheelBasedSpeed) {
      this.wheelBasedSpeed = wheelBasedSpeed;
    }

    public Double getTachographSpeed() {
      return tachographSpeed;
    }

    public void setTachographSpeed(Double tachographSpeed) {
      this.tachographSpeed = tachographSpeed;
    }

    public Double getEngineSpeed() {
      return engineSpeed;
    }

    public void setEngineSpeed(Double engineSpeed) {
      this.engineSpeed = engineSpeed;
    }

    public String getFuelType() {
      return fuelType;
    }

    public void setFuelType(String fuelType) {
      this.fuelType = fuelType;
    }

    public Double getCatalystFuelLevel() {
      return catalystFuelLevel;
    }

    public void setCatalystFuelLevel(Double catalystFuelLevel) {
      this.catalystFuelLevel = catalystFuelLevel;
    }

    public Double getFuelLevel1() {
      return fuelLevel1;
    }

    public void setFuelLevel1(Double fuelLevel1) {
      this.fuelLevel1 = fuelLevel1;
    }

    public Double getFuelLevel2() {
      return fuelLevel2;
    }

    public void setFuelLevel2(Double fuelLevel2) {
      this.fuelLevel2 = fuelLevel2;
    }

    public String getDriver1WorkingState() {
      return driver1WorkingState;
    }

    public void setDriver1WorkingState(String driver1WorkingState) {
      this.driver1WorkingState = driver1WorkingState;
    }

    public String getDriver2WorkingState() {
      return driver2WorkingState;
    }

    public void setDriver2WorkingState(String driver2WorkingState) {
      this.driver2WorkingState = driver2WorkingState;
    }

    public Double getAmbientAirTemperature() {
      return ambientAirTemperature;
    }

    public void setAmbientAirTemperature(Double ambientAirTemperature) {
      this.ambientAirTemperature = ambientAirTemperature;
    }

    public Boolean getParkingBrakeSwitch() {
      return parkingBrakeSwitch;
    }

    public void setParkingBrakeSwitch(Boolean parkingBrakeSwitch) {
      this.parkingBrakeSwitch = parkingBrakeSwitch;
    }

    public Double getEstimatedDistanceToEmptyFuel() {
      return estimatedDistanceToEmptyFuel;
    }

    public void setEstimatedDistanceToEmptyFuel(Double estimatedDistanceToEmptyFuel) {
      this.estimatedDistanceToEmptyFuel = estimatedDistanceToEmptyFuel;
    }

    public Double getEstimatedDistanceToEmptyTotal() {
      return estimatedDistanceToEmptyTotal;
    }

    public void setEstimatedDistanceToEmptyTotal(Double estimatedDistanceToEmptyTotal) {
      this.estimatedDistanceToEmptyTotal = estimatedDistanceToEmptyTotal;
    }

    public String getDriver2Id() {
      return driver2Id;
    }

    public void setDriver2Id(String driver2Id) {
      this.driver2Id = driver2Id;
    }

    public String getDriver2IdCardIssuer() {
      return driver2IdCardIssuer;
    }

    public void setDriver2IdCardIssuer(String driver2IdCardIssuer) {
      this.driver2IdCardIssuer = driver2IdCardIssuer;
    }
  }
}

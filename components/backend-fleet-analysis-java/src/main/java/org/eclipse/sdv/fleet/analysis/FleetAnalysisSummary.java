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

public class FleetAnalysisSummary {
  private int vehicleCount;
  private double averageSpeedKph;
  private double minBatterySoc;
  private double maxBatterySoc;
  private long brakingVehicles;

  public int getVehicleCount() {
    return vehicleCount;
  }

  public void setVehicleCount(int vehicleCount) {
    this.vehicleCount = vehicleCount;
  }

  public double getAverageSpeedKph() {
    return averageSpeedKph;
  }

  public void setAverageSpeedKph(double averageSpeedKph) {
    this.averageSpeedKph = averageSpeedKph;
  }

  public double getMinBatterySoc() {
    return minBatterySoc;
  }

  public void setMinBatterySoc(double minBatterySoc) {
    this.minBatterySoc = minBatterySoc;
  }

  public double getMaxBatterySoc() {
    return maxBatterySoc;
  }

  public void setMaxBatterySoc(double maxBatterySoc) {
    this.maxBatterySoc = maxBatterySoc;
  }

  public long getBrakingVehicles() {
    return brakingVehicles;
  }

  public void setBrakingVehicles(long brakingVehicles) {
    this.brakingVehicles = brakingVehicles;
  }
}

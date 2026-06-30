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

import java.time.Instant;

public class FleetTelemetry {
  private String vehicleId;
  private double speedKph;
  private double batterySoc;
  private boolean brakeActive;
  private Instant updatedAt;

  public String getVehicleId() {
    return vehicleId;
  }

  public void setVehicleId(String vehicleId) {
    this.vehicleId = vehicleId;
  }

  public double getSpeedKph() {
    return speedKph;
  }

  public void setSpeedKph(double speedKph) {
    this.speedKph = speedKph;
  }

  public double getBatterySoc() {
    return batterySoc;
  }

  public void setBatterySoc(double batterySoc) {
    this.batterySoc = batterySoc;
  }

  public boolean isBrakeActive() {
    return brakeActive;
  }

  public void setBrakeActive(boolean brakeActive) {
    this.brakeActive = brakeActive;
  }

  public Instant getUpdatedAt() {
    return updatedAt;
  }

  public void setUpdatedAt(Instant updatedAt) {
    this.updatedAt = updatedAt;
  }
}

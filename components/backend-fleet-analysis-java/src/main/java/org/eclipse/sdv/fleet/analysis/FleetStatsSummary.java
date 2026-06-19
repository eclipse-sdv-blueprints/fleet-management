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

public class FleetStatsSummary {
  private long vehicleCount;
  private long headerCount;
  private long snapshotCount;
  private long totalCount;
  private long generatedAt;

  public long getVehicleCount() {
    return vehicleCount;
  }

  public void setVehicleCount(long vehicleCount) {
    this.vehicleCount = vehicleCount;
  }

  public long getHeaderCount() {
    return headerCount;
  }

  public void setHeaderCount(long headerCount) {
    this.headerCount = headerCount;
  }

  public long getSnapshotCount() {
    return snapshotCount;
  }

  public void setSnapshotCount(long snapshotCount) {
    this.snapshotCount = snapshotCount;
  }

  public long getTotalCount() {
    return totalCount;
  }

  public void setTotalCount(long totalCount) {
    this.totalCount = totalCount;
  }

  public long getGeneratedAt() {
    return generatedAt;
  }

  public void setGeneratedAt(long generatedAt) {
    this.generatedAt = generatedAt;
  }
}

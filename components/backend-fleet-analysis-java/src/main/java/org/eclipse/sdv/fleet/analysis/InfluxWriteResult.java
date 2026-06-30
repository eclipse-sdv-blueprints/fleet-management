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

public class InfluxWriteResult {
  private boolean headerWritten;
  private boolean snapshotWritten;

  public InfluxWriteResult() {
  }

  public InfluxWriteResult(boolean headerWritten, boolean snapshotWritten) {
    this.headerWritten = headerWritten;
    this.snapshotWritten = snapshotWritten;
  }

  public boolean isHeaderWritten() {
    return headerWritten;
  }

  public void setHeaderWritten(boolean headerWritten) {
    this.headerWritten = headerWritten;
  }

  public boolean isSnapshotWritten() {
    return snapshotWritten;
  }

  public void setSnapshotWritten(boolean snapshotWritten) {
    this.snapshotWritten = snapshotWritten;
  }
}

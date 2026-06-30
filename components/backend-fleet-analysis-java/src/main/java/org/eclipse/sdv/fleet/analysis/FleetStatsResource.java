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

import jakarta.inject.Inject;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;

@Path("/analysis")
@Produces(MediaType.APPLICATION_JSON)
public class FleetStatsResource {

  @Inject
  private InfluxStatsService statsService;

  @GET
  @Path("/stats")
  public FleetStatsSummary getStats() {
    FleetStatsSummary stats = statsService.getLatestStats(true);
    return stats == null ? new FleetStatsSummary() : stats;
  }
}

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
import com.influxdb.client.QueryApi;
import com.influxdb.client.WriteApi;
import com.influxdb.client.domain.WritePrecision;
import com.influxdb.client.write.Point;
import com.influxdb.query.FluxRecord;
import com.influxdb.query.FluxTable;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;
import jakarta.enterprise.context.ApplicationScoped;

import java.util.List;
import java.util.Objects;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;
import java.util.logging.Logger;

@ApplicationScoped
public class InfluxStatsService {
  private static final Logger LOGGER = Logger.getLogger(InfluxStatsService.class.getName());

  private InfluxDbConfig influxConfig;
  private InfluxStatsConfig statsConfig;
  private InfluxDBClient client;
  private WriteApi writeApi;
  private ScheduledExecutorService scheduler;

  @PostConstruct
  void init() {
    influxConfig = InfluxDbConfig.fromEnv();
    statsConfig = InfluxStatsConfig.fromEnv();
    client =
        InfluxDBClientFactory.create(
            influxConfig.getUri(),
            influxConfig.getToken().toCharArray(),
            influxConfig.getOrg(),
            influxConfig.getBucket());
    writeApi = client.getWriteApi();

    scheduler =
        Executors.newSingleThreadScheduledExecutor(
            runnable -> {
              Thread thread = new Thread(runnable, "fleet-stats-scheduler");
              thread.setDaemon(true);
              return thread;
            });
    scheduler.scheduleAtFixedRate(
        this::refreshAndPersistStats,
        statsConfig.getInitialDelaySeconds(),
        statsConfig.getIntervalSeconds(),
        TimeUnit.SECONDS);
  }

  @PreDestroy
  void shutdown() {
    if (scheduler != null) {
      scheduler.shutdownNow();
    }
    if (writeApi != null) {
      writeApi.close();
    }
    if (client != null) {
      client.close();
    }
  }

  public FleetStatsSummary getLatestStats(boolean computeIfMissing) {
    FleetStatsSummary latest = queryLatestStats();
    if (latest == null && computeIfMissing) {
      latest = computeStats();
      writeStats(latest);
    }
    return latest;
  }

  private void refreshAndPersistStats() {
    try {
      FleetStatsSummary stats = computeStats();
      writeStats(stats);
    } catch (Exception ex) {
      LOGGER.log(Level.WARNING, "Failed to refresh fleet stats.", ex);
    }
  }

  private FleetStatsSummary computeStats() {
    long vehicleCount = queryVehicleCount();
    long headerCount = queryMeasurementCount("header");
    long snapshotCount = queryMeasurementCount("snapshot");

    FleetStatsSummary summary = new FleetStatsSummary();
    summary.setVehicleCount(vehicleCount);
    summary.setHeaderCount(headerCount);
    summary.setSnapshotCount(snapshotCount);
    summary.setTotalCount(headerCount + snapshotCount);
    summary.setGeneratedAt(System.currentTimeMillis());
    return summary;
  }

  private void writeStats(FleetStatsSummary summary) {
    Point point =
        Point.measurement("fleet_stats")
            .addField("vehicleCount", summary.getVehicleCount())
            .addField("headerCount", summary.getHeaderCount())
            .addField("snapshotCount", summary.getSnapshotCount())
            .addField("totalCount", summary.getTotalCount())
            .addField("generatedAt", summary.getGeneratedAt())
            .time(summary.getGeneratedAt(), WritePrecision.MS);
    writeApi.writePoint(point);
  }

  private FleetStatsSummary queryLatestStats() {
    String flux =
        "from(bucket: \""
            + influxConfig.getBucket()
            + "\")"
            + " |> range(start: 0)"
            + " |> filter(fn: (r) => r._measurement == \"fleet_stats\")"
            + " |> last()";

    List<FluxTable> tables = client.getQueryApi().query(flux);
    if (tables == null || tables.isEmpty()) {
      return null;
    }

    FleetStatsSummary summary = new FleetStatsSummary();
    boolean hasData = false;
    for (FluxTable table : tables) {
      for (FluxRecord record : table.getRecords()) {
        Object field = record.getField();
        Object value = record.getValue();
        if (field == null || value == null) {
          continue;
        }
        String name = field.toString();
        long numeric = toLong(value);
        switch (name) {
          case "vehicleCount" -> summary.setVehicleCount(numeric);
          case "headerCount" -> summary.setHeaderCount(numeric);
          case "snapshotCount" -> summary.setSnapshotCount(numeric);
          case "totalCount" -> summary.setTotalCount(numeric);
          case "generatedAt" -> summary.setGeneratedAt(numeric);
          default -> {
          }
        }
        hasData = true;
      }
    }

    return hasData ? summary : null;
  }

  private long queryVehicleCount() {
    String flux =
        "import \"influxdata/influxdb/schema\"\n"
            + "schema.tagValues(bucket: \""
            + influxConfig.getBucket()
            + "\", tag: \"vin\")"
            + " |> group(columns: [])"
            + " |> count(column: \"_value\")";
    return querySingleLong(flux);
  }

  private long queryMeasurementCount(String measurement) {
    String flux =
        "from(bucket: \""
            + influxConfig.getBucket()
            + "\")"
            + " |> range(start: 0)"
            + " |> filter(fn: (r) => r._measurement == \""
            + measurement
            + "\")"
            + " |> keep(columns: [\"_time\", \"vin\"])"
            + " |> map(fn: (r) => ({_value: string(v: r._time) + \":\" + r.vin}))"
            + " |> distinct(column: \"_value\")"
            + " |> group(columns: [])"
            + " |> count(column: \"_value\")";
    return querySingleLong(flux);
  }

  private long querySingleLong(String flux) {
    QueryApi queryApi = client.getQueryApi();
    List<FluxTable> tables = queryApi.query(flux);
    if (tables == null || tables.isEmpty()) {
      return 0L;
    }
    for (FluxTable table : tables) {
      for (FluxRecord record : table.getRecords()) {
        Object value = record.getValue();
        if (value == null) {
          value = record.getValueByKey("count");
        }
        if (value != null) {
          return toLong(value);
        }
      }
    }
    return 0L;
  }

  private long querySumLong(String flux) {
    QueryApi queryApi = client.getQueryApi();
    List<FluxTable> tables = queryApi.query(flux);
    if (tables == null || tables.isEmpty()) {
      return 0L;
    }
    long sum = 0L;
    for (FluxTable table : tables) {
      for (FluxRecord record : table.getRecords()) {
        Object value = record.getValue();
        if (value != null) {
          sum += toLong(value);
        }
      }
    }
    return sum;
  }

  private long toLong(Object value) {
    if (value instanceof Number number) {
      return number.longValue();
    }
    return Long.parseLong(Objects.toString(value, "0"));
  }
}

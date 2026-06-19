---
sidebar_position: 2
title: Fleet Analysis Backend
---

## Fleet Analysis Backend

The Fleet Management Blueprint includes a Jakarta EE backend service that provides fleet-level analytics on top of telemetry stored in InfluxDB.

## Overview

The backend runs in the same Docker Compose stack as the Fleet Management services and exposes REST APIs for:

- Computing summary statistics from telemetry snapshots
- Ingesting telemetry payloads into InfluxDB
- Reading periodically refreshed fleet statistics

## Endpoints

### POST /api/analysis/summary

Accepts a JSON array of vehicle snapshots and returns aggregate values such as fleet size, average speed and battery range.

### POST /api/telemetry/ingest

Writes header and snapshot telemetry to InfluxDB.

### GET /api/analysis/stats

Returns the latest precomputed fleet statistics snapshot.

## Configuration

The component supports these environment variables:

- INFLUXDB_URI (default: `http://influxdb:8086`)
- INFLUXDB_ORG (default: sdv)
- INFLUXDB_BUCKET (default: demo)
- INFLUXDB_TOKEN or INFLUXDB_TOKEN_FILE
- INFLUXDB_STATS_INTERVAL_SECONDS (default: 30)
- INFLUXDB_STATS_INITIAL_DELAY_SECONDS (default: 10)

## Build and Run

Build and run from the Fleet Management repository root:

```bash
docker compose -f ./fms-blueprint-compose.yaml -f ./fms-blueprint-compose-zenoh.yaml up --detach
```

The service is exposed at:

- `http://127.0.0.1:8082/fleet-analysis/api`

The source code is located in components/backend-fleet-analysis-java.

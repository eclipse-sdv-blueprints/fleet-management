# Fleet Analysis Backend (Jakarta EE 11)

This service provides a small Jakarta EE 11 backend to analyze Fleet Management telemetry and
return summary statistics for dashboards or alerts.

## Responsibilities

- Accepts fleet telemetry snapshots as JSON.
- Returns computed summary metrics (fleet size, average speed, battery SOC range, braking count).
- Writes header/snapshot telemetry to InfluxDB when requested.
- Reads InfluxDB telemetry and writes fleet statistics periodically.

## Build

```bash
mvn package
```

## Run (example with Payara Micro)

1. Download [Payara Micro 6](https://www.payara.fish/downloads/payara-platform-community-edition/).
2. Deploy the WAR:

```bash
java -jar payara-micro.jar --deploy target/fleet-analysis-backend.war --contextRoot /fleet-analysis
```

The API will be available at `http://localhost:8080/fleet-analysis/api`.

## Build Docker image (without Docker Compose)

From the repository root:

```bash
docker build -t fleet-analysis-backend:local components/backend-fleet-analysis-java
```

Run it manually (example):

```bash
docker run --rm -p 8082:8080 --name fleet-analysis-backend \
  -e INFLUXDB_TOKEN_FILE=/tmp/fms-demo.token \
  -e INFLUXDB_STATS_INTERVAL_SECONDS=30 \
  -v /path/to/fms-demo.token:/tmp/fms-demo.token:ro \
  fleet-analysis-backend:local
```

## Accessing output

Variant 1: Docker container logs

```bash
docker logs -f fleet-analysis-backend
```

Variant 2: API

```bash
curl -s http://localhost:8082/fleet-analysis/api/analysis/stats | jq
```

## API

See [openapi.yaml](openapi.yaml) for the complete API specification.

The API provides the following endpoints:

- `POST /api/analysis/summary` - Compute fleet summary statistics
- `POST /api/telemetry/ingest` - Ingest vehicle telemetry into InfluxDB
- `GET /api/analysis/stats` - Get latest fleet statistics

### Fleet stats (InfluxDB)

The service periodically computes fleet statistics from the `header`/`snapshot` measurements and
writes them into a `fleet_stats` measurement.

Default schedule: every 30 seconds (configure with `INFLUXDB_STATS_INTERVAL_SECONDS` as an env var
or Java system property).
Initial delay before the first stats run defaults to 10 seconds (configure with
`INFLUXDB_STATS_INITIAL_DELAY_SECONDS`).

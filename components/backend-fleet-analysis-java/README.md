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

### `POST /api/analysis/summary`

Request body (example):

```json
[
  {
    "vehicleId": "bike-001",
    "speedKph": 42.3,
    "batterySoc": 0.78,
    "brakeActive": false,
    "updatedAt": "2024-06-10T10:15:30Z"
  },
  {
    "vehicleId": "bike-002",
    "speedKph": 12.4,
    "batterySoc": 0.52,
    "brakeActive": true,
    "updatedAt": "2024-06-10T10:15:32Z"
  }
]
```

Response:

```json
{
  "vehicleCount": 2,
  "averageSpeedKph": 27.35,
  "minBatterySoc": 0.52,
  "maxBatterySoc": 0.78,
  "brakingVehicles": 1
}
```

### `POST /api/telemetry/ingest`

Writes header and/or snapshot measurements into InfluxDB. Configure the client using:

- `INFLUXDB_URI` (default: `http://influxdb:8086`)
- `INFLUXDB_ORG` (default: `sdv`)
- `INFLUXDB_BUCKET` (default: `demo`)
- `INFLUXDB_TOKEN` or `INFLUXDB_TOKEN_FILE`

Request body (example):

```json
{
  "vin": "truck-001",
  "trigger": "periodic",
  "createdDateTime": 1737940602000,
  "header": {
    "hrTotalVehicleDistance": 12345.6,
    "grossCombinationVehicleWeight": 18100.2,
    "totalEngineHours": 82.5,
    "engineTotalFuelUsed": 221.9,
    "driver1Id": "driver-01",
    "driver1IdCardIssuer": "fleet"
  },
  "snapshot": {
    "latitude": 37.7749,
    "longitude": -122.4194,
    "speed": 54.2,
    "positionDateTime": 1737940602,
    "wheelBasedSpeed": 53.7,
    "fuelLevel1": 0.42,
    "parkingBrakeSwitch": false
  }
}
```

### Fleet stats (InfluxDB)

The service periodically computes fleet statistics from the `header`/`snapshot` measurements and
writes them into a `fleet_stats` measurement.

Default schedule: every 30 seconds (configure with `INFLUXDB_STATS_INTERVAL_SECONDS` as an env var
or Java system property).
Initial delay before the first stats run defaults to 10 seconds (configure with
`INFLUXDB_STATS_INITIAL_DELAY_SECONDS`).

**Measurement: `fleet_stats`**

Fields:

- `vehicleCount` (unique `vin` across all time)
- `headerCount` (total number of header points)
- `snapshotCount` (total number of snapshot points)
- `totalCount` (header + snapshot points)
- `generatedAt` (ms since Unix epoch)

### `GET /api/analysis/stats`

Returns the latest stats snapshot (and triggers a refresh if none exist yet).

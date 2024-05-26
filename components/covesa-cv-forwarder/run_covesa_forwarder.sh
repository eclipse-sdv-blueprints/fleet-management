#!/bin/bash
# Copy the token file from the running influxdb container to local host:
docker exec -it influxDB cat /tmp/out/fms-demo.token > /tmp/influxdb.token
# Specify the path to that file when starting the covesa-cv-forwarder:
cargo run --bin covesa-cv-forwarder influx --influxdb-uri http://influxdb:8086 --influxdb-token-file /tmp/influxdb.token --influxdb-bucket covesa-demo
#!/bin/bash

if [ -z "$TOKEN_FILE" ]; then
    TOKEN_FILE="/tmp/fms-demo.token"
fi

if [ -f "$TOKEN_FILE" ]; then
    INFLUXDB_TOKEN="$(cat $TOKEN_FILE)"
fi

trap stop SIGINT SIGTERM

function stop() {
        kill $CHILD_PID
        wait $CHILD_PID
}

/usr/local/bin/node $NODE_OPTIONS node_modules/node-red/red.js --userDir /data $FLOWS "${@}" &

CHILD_PID="$!"

wait "${CHILD_PID}"

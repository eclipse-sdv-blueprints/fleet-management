#!/bin/sh

RUNNING=1

trap stop SIGINT SIGTERM

function stop() {
    RUNNING=0
    kill $PID
    wait $PID
}

SIGNAL_FILTER_FILE="/etc/vss/signal_filter.cfg"
FILTER_STATE=""

if [ -f "$SIGNAL_FILTER_FILE" ]; then
    FILTER_STATE="$(stat -t ${SIGNAL_FILTER_FILE})"
    SIGNAL_FILTER="$(cat ${SIGNAL_FILTER_FILE})"
    export SIGNAL_FILTER
fi

/app/vss-forwarder.py & PID=$!

while true
do
    if [ "$RUNNING" -ne 1 ]; then
        break
    fi

    if [ -f "$SIGNAL_FILTER_FILE" ]; then
        CURRENT_FILTER_STATE="$(stat -t ${SIGNAL_FILTER_FILE})"
        if [ "${CURRENT_FILTER_STATE}" != "${FILTER_STATE}" ]; then
            FILTER_STATE="${CURRENT_FILTER_STATE}"
            SIGNAL_FILTER="$(cat ${SIGNAL_FILTER_FILE})"
            export SIGNAL_FILTER

            echo "Updating Signal Filter:"
            echo "  ${FILTER_STATE}"

            echo "Restarting vss-forwarder."
            kill -9 $PID
            /app/vss-forwarder.py & PID=$!
        fi
    fi
    sleep 5
done

wait "${PID}"


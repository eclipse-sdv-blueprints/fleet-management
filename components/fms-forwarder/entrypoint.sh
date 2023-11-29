#!/bin/sh

SIGNAL_FILTER_FILE="/etc/vss/signal_filter.cfg"
FILTER_STATE=""

if [ -f "$SIGNAL_FILTER_FILE" ]; then
    FILTER_STATE="$(stat -t ${SIGNAL_FILTER_FILE})"
    SIGNAL_FILTER="$(cat ${SIGNAL_FILTER_FILE})"
    export SIGNAL_FILTER
fi

/app/fms-forwarder & PID=$!

while true
do
    if [ -f "$SIGNAL_FILTER_FILE" ]; then
        CURRENT_FILTER_STATE="$(stat -t ${SIGNAL_FILTER_FILE})"
        if [ "${CURRENT_FILTER_STATE}" != "${FILTER_STATE}" ]; then
            FILTER_STATE="${CURRENT_FILTER_STATE}"
            SIGNAL_FILTER="$(cat ${SIGNAL_FILTER_FILE})"
            export SIGNAL_FILTER

            echo "Updating Signal Filter:"
            echo "  ${FILTER_STATE}"

            echo "Restarting fms-forwarder."
            kill -9 $PID
            /app/fms-forwarder & PID=$!
        fi
    fi
    sleep 5
done

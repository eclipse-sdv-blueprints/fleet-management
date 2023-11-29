FROM nodered/node-red

LABEL org.opencontainers.image.title="MIA Node-RED application"
LABEL org.opencontainers.image.description="SDV Hackathon 2023 App from the Team MIA."

COPY package.json .
RUN npm install --unsafe-perm --no-update-notifier --no-fund --omit=dev

ENV INFLUXDB_HOST="influxdb"
ENV INFLUXDB_USERNAME="fms-demo"
ENV INFLUXDB_PASSWORD="fms-demo-secret"
ENV INFLUXDB_TOKEN_FILE="/tmp/fms-demo.token"
ENV INFLUXDB_TOKEN=""

COPY settings.js /data/settings.js
COPY flows.json /data/flows.json
COPY entrypoint.sh /usr/src/node-red/entrypoint.sh

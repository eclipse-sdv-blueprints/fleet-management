#!/usr/bin/env python3
# /********************************************************************************
# * Copyright (c) 2023 Contributors to the Eclipse Foundation
# *
# * See the NOTICE file(s) distributed with this work for additional
# * information regarding copyright ownership.
# *
# * This program and the accompanying materials are made available under the
# * terms of the Apache License 2.0 which is available at
# * https://www.apache.org/licenses/LICENSE-2.0
# *
# * SPDX-License-Identifier: Apache-2.0
# ********************************************************************************/


import asyncio
import logging
import os
import signal
import time

from kuksa_client.grpc import Datapoint
from kuksa_client.grpc.aio import VSSClient


logging.basicConfig(level=logging.DEBUG)
log = logging.getLogger("vss_forwarder")


class VSS_Forwarder():
    def __init__(self):
        self.running=False
        self.signal_filter =os.environ.get("SIGNAL_FILTER", "").split(",") 
        self.databroker_addr = os.environ.get("KUKSA_DATA_BROKER_ADDR", "") 
        self.databroker_port = os.environ.get("KUKSA_DATA_BROKER_PORT", "") 
        self.influxdb_token_file = os.environ.get("INFLUXDB_TOKEN_FILE", "") 
            
        if not self.signal_filter:
            log.warning("SIGNAL_FILTER has not been set!")

        signal.signal(signal.SIGINT, self.stop)
        signal.signal(signal.SIGTERM, self.stop)

    def run(self):
        self.running = True
        print("Hello!")
        
        print(f"databroker uri      : {self.databroker_addr}i:{self.databroker_port}")
        print(f"influxdb token file : {self.influxdb_token_file}")
        print(f"signal filter       : {self.signal_filter}")
        
        asyncio.run(self.handle_data())
        
    def stop(self, *args):
        log.info("Shutting down")
        self.running = False

    async def handle_data(self):
        #while True:
        #    if not self.running:
        #        break
        #    await asyncio.sleep(1)

        async with VSSClient(self.databroker_addr, self.databroker_port) as client:
            async for data in client.subscribe_current_values(self.signal_filter):
                print(data)
                #if updates['Vehicle.Body.Windshield.Front.Wiping.System.TargetPosition'] is not None:
                #    current_position = updates['Vehicle.Body.Windshield.Front.Wiping.System.TargetPosition'].value
                #print(f"Current wiper position is: {current_position}")

                if not self.running:
                    break



if __name__ == "__main__":
    forwarder = VSS_Forwarder()
    forwarder.run()


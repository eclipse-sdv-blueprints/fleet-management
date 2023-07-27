<!--
SPDX-FileCopyrightText: 2023 Contributors to the Eclipse Foundation

See the NOTICE file(s) distributed with this work for additional
information regarding copyright ownership.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

SPDX-License-Identifier: Apache-2.0
-->
# Using the CSV-Provider

With the CSV-provider one can replay signals from a CSV-file to an instance of the Kuksa.val data broker. More details are available in the [upstream repository](https://github.com/eclipse/kuksa.val.feeders/tree/main/csv_provider). To execute the CSV-provider with the Docker Compose setup, add the argument '--profile csv':

```
docker compose -f ./fms-demo-compose.yaml --profile direct --profile csv up --detach
```

## Recording
The file in `csv-provider/signalsFmsRecording.csv` is a CSV representation of the beginning of the CAN-trace in `dbc-feeder/220421_MAN_Si_RIO_CAN_converted.log`. The CSV-representation of the full VSS-recording of the CAN-trace is available in the archive `csv-providers/signalsFmsRecording.zip`.

You can perform your own recording for other traces by using the [CSV recorder from the Kuksa.val.feeders repository](https://github.com/eclipse/kuksa.val.feeders/tree/main/csv_provider). The script in [run_recorderFMS.sh](run_recorderFMS.sh) contains the signals to record for this use case.
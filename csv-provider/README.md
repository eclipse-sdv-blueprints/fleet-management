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

With the CSV-Provider one can replay signals from a CSV-file to an instance of the Eclipse Kuksa Databroker. More details are
available in the [CSV-Provider repository](https://github.com/eclipse-kuksa/kuksa-csv-provider).
The CSV-Provider is started by default when using the Docker Compose setup.

## Generating a CSV File

The [generate_csv.py](./generate_csv.py) Python script can be used to generate a CSV file that will contain random (but mostly consistent) VSS data points:

```bash
python3 generate_csv.py -o truck1_signals.csv
```

The generated file can then be used with the CSV-Provider to apply the signal updates to the Kuksa Databroker.

## Recording

Alternatively, a CSV file can be recorded from a running Kuksa Databroker using Kuksa's
[CSV-Recorder](https://github.com/eclipse-kuksa/kuksa-csv-provider/blob/main/README.md#recorder).
The script in [run_recorderFMS.sh](run_recorderFMS.sh) contains the signals to record for this use case.

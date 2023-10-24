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

With the CSV-provider one can replay signals from a CSV-file to an instance of the Kuksa.val data broker. More details are
available in the [upstream repository](https://github.com/eclipse/kuksa.val.feeders/tree/main/csv_provider).
The CSV-provider is started by default when using the Docker Compose setup.

## Recording

You can perform your own recording for other traces by using the
[CSV recorder from the Kuksa.val.feeders repository](https://github.com/eclipse/kuksa.val.feeders/tree/main/csv_provider).
The script in [run_recorderFMS.sh](run_recorderFMS.sh) contains the signals to record for this use case.

# How to contribute

First of all, thanks for considering to contribute to the Eclipse SDV Fleet Management Blueprint.
We really appreciate the time and effort you want to spend helping to improve things around here.
And help we can use :-)

Here is a (non-exclusive, non-prioritized) list of things you might be able to help us with:

* bug reports
* bug fixes
* improvements regarding code quality e.g. improving readability, performance, modularity etc.
* documentation (Getting Started guide, Examples, Deployment instructions)
* features (both ideas and code are welcome)

In order to get you started as fast as possible we need to go through some organizational issues first,
though.

## Eclipse Contributor Agreement

In order to be able to contribute to Eclipse Foundation projects you must
electronically sign the Eclipse Contributor Agreement (ECA).

* https://www.eclipse.org/legal/ECA.php

The ECA provides the Eclipse Foundation with a permanent record that you agree
that each of your contributions will comply with the commitments documented in
the Developer Certificate of Origin (DCO). Having an ECA on file associated with
the email address matching the "Author" field of your contribution's Git commits
fulfills the DCO's requirement that you sign-off on your contributions.

For more information, please see the Eclipse Project Handbook:
https://www.eclipse.org/projects/handbook/#resources-commit

## Making your Changes

* Fork the repository on GitHub
* Create a new branch for your changes
* Make your changes
* When you create new files make sure you include a proper license header at the top of the file
  (see [License Header section](#license-header) below).
* Make sure you include test cases for non-trivial features
* Make sure the test suite passes after your changes
* Commit your changes into that branch
* Use descriptive and meaningful commit messages. In particular, start the first line of the commit message with the
  number of the issue that the commit addresses, e.g. `[#9865] Improve data sampling algorithm`
* Squash multiple commits that are related to each other semantically into a single one
* Push your changes to your branch in your forked repository

## Submitting the Changes

Submit a pull request via the normal GitHub UI.

## After Submitting

* Do not use your branch for any other development, otherwise further changes that you make will be visible in the PR.

## License Header

Please make sure any file you newly create contains a proper license header like this:

```
/**
 * Copyright (c) 2023 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * https://www.eclipse.org/legal/epl-2.0
 *
 * SPDX-License-Identifier: EPL-2.0
 */
```
You should, of course, adapt this header to use the specific mechanism for comments pertaining to the type of file you create, e.g. using something like

```
<!--
 Copyright (c) 2023 Contributors to the Eclipse Foundation

 See the NOTICE file(s) distributed with this work for additional
 information regarding copyright ownership.

 This program and the accompanying materials are made available under the
 terms of the Eclipse Public License 2.0 which is available at
 https://www.eclipse.org/legal/epl-2.0

 SPDX-License-Identifier: EPL-2.0
-->
```

when adding an XML file.

There are cases where the type of document does not allow the inclusion of a license header, e.g. in a binary file like
a PNG. In such a case, please create a separate text file of the same name as the binary artifact with a suffix of
`.license`. For example, if you add an image called `overview.png`, then the text file's name should be
`overview.png.license`. Put the text file into the same folder as the binary artifact.

The content of the file should be:
```
# SPDX-FileCopyrightText: 2023 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
# 
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0
```

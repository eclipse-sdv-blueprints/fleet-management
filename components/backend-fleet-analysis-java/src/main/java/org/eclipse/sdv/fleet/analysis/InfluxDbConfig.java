// ********************************************************************************
// * Copyright (c) 2026 Contributors to the Eclipse Foundation
// *
// * See the NOTICE file(s) distributed with this work for additional
// * information regarding copyright ownership.
// *
// * This program and the accompanying materials are made available under the
// * terms of the Apache License 2.0 which is available at
// * https://www.apache.org/licenses/LICENSE-2.0
// *
// * SPDX-License-Identifier: Apache-2.0
// ********************************************************************************

package org.eclipse.sdv.fleet.analysis;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

public class InfluxDbConfig {
  private static final String DEFAULT_URI = "http://influxdb:8086";
  private static final String DEFAULT_ORG = "sdv";
  private static final String DEFAULT_BUCKET = "demo";

  private final String uri;
  private final String org;
  private final String bucket;
  private final String token;

  private InfluxDbConfig(String uri, String org, String bucket, String token) {
    this.uri = uri;
    this.org = org;
    this.bucket = bucket;
    this.token = token;
  }

  public static InfluxDbConfig fromEnv() {
    String uri = envOrDefault("INFLUXDB_URI", DEFAULT_URI);
    String org = envOrDefault("INFLUXDB_ORG", DEFAULT_ORG);
    String bucket = envOrDefault("INFLUXDB_BUCKET", DEFAULT_BUCKET);
    String token = env("INFLUXDB_TOKEN");

    if (isBlank(token)) {
      String tokenFile = env("INFLUXDB_TOKEN_FILE");
      if (!isBlank(tokenFile)) {
        token = readTokenFile(tokenFile);
      }
    }

    if (isBlank(token)) {
      throw new IllegalStateException(
          "Missing InfluxDB token. Set INFLUXDB_TOKEN or INFLUXDB_TOKEN_FILE.");
    }

    return new InfluxDbConfig(uri, org, bucket, token);
  }

  public String getUri() {
    return uri;
  }

  public String getOrg() {
    return org;
  }

  public String getBucket() {
    return bucket;
  }

  public String getToken() {
    return token;
  }

  private static String env(String key) {
    return System.getenv(key);
  }

  private static String envOrDefault(String key, String fallback) {
    String value = env(key);
    return isBlank(value) ? fallback : value;
  }

  private static boolean isBlank(String value) {
    return value == null || value.trim().isEmpty();
  }

  private static String readTokenFile(String tokenFile) {
    try {
      return Files.readString(Path.of(tokenFile), StandardCharsets.UTF_8).trim();
    } catch (IOException ex) {
      throw new IllegalStateException("Failed to read InfluxDB token file: " + tokenFile, ex);
    }
  }
}

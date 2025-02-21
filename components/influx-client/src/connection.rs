// SPDX-FileCopyrightText: 2023 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use clap::Args;
use influxrs::InfluxClient;
use log::{error, info};

const PARAM_INFLUXDB_BUCKET: &str = "influxdb-bucket";
const PARAM_INFLUXDB_ORG: &str = "influxdb-org";
const PARAM_INFLUXDB_URI: &str = "influxdb-uri";
const PARAM_INFLUXDB_TOKEN: &str = "influxdb-token";
const PARAM_INFLUXDB_TOKEN_FILE: &str = "influxdb-token-file";

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Token {
    /// The API token to use for authenticating to the InfluxDB server.
    #[arg(long = PARAM_INFLUXDB_TOKEN, value_name = "NAME", env = "INFLUXDB_TOKEN", value_parser = clap::builder::NonEmptyStringValueParser::new() )]
    token: Option<String>,

    /// The path to a file that contains the API token to use for authenticating to the InfluxDB server.
    #[arg(long = PARAM_INFLUXDB_TOKEN_FILE, value_name = "FILE", env = "INFLUXDB_TOKEN_FILE", value_parser = clap::builder::PathBufValueParser::new() )]
    token_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct InfluxConnectionConfig {
    /// The HTTP(S) URI of the InfluxDB server.
    #[arg(
        long = PARAM_INFLUXDB_URI,
        value_name = "URI",
        required = true,
        env = "INFLUXDB_URI",
        value_parser = clap::builder::NonEmptyStringValueParser::new()
    )]
    uri: String,

    /// The name of the organization to connect to on the InfluxDB server.
    #[arg(
        long = PARAM_INFLUXDB_ORG,
        value_name = "URI",
        required = false,
        env = "INFLUXDB_ORG",
        default_value = "sdv",
        value_parser = clap::builder::NonEmptyStringValueParser::new()
    )]
    org: String,

    /// The name of the bucket to write data to on the InfluxDB server.
    #[arg(
        long = PARAM_INFLUXDB_BUCKET,
        value_name = "URI",
        required = false,
        env = "INFLUXDB_BUCKET",
        default_value = "demo",
        value_parser = clap::builder::NonEmptyStringValueParser::new()
    )]
    bucket: String,

    #[command(flatten)]
    token: Token,
}

impl InfluxConnectionConfig {
    fn token(&self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(token) = self.token.token.as_ref() {
            Ok(token.to_owned())
        } else if let Some(path) = self.token.token_file.as_ref() {
            info!("reading token from file {:?}", path);
            Ok(std::fs::read_to_string(path)
                .map(|s| s.trim().to_string())
                .map_err(|e| {
                    error!("failed to read token from file: {e}");
                    Box::new(e)
                })?)
        } else {
            Err(Box::from("test"))
        }
    }
}

/// A connection to an InfluxDB server.
pub struct InfluxConnection {
    pub client: InfluxClient,
    pub bucket: String,
}

impl InfluxConnection {
    /// Creates a new connection to an InfluxDB server.
    ///
    /// Determines the parameters necessary for creating the connection from values specified on
    /// the command line or via environment variables as defined by [`add_command_line_args`].
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::{Args, Command, FromArgMatches};
    /// use influx_client::connection::{InfluxConnection, InfluxConnectionConfig};
    ///
    /// let command = Command::new("influx_client");
    /// let command = InfluxConnectionConfig::augment_args(command);
    /// let matches = command.get_matches_from(vec![
    ///     "influx_client",
    ///     "--influxdb-uri", "http://my-influx.io",
    ///     "--influxdb-token", "some-token",
    ///     "--influxdb-bucket", "the-bucket",
    /// ]);
    /// let connection_params = InfluxConnectionConfig::from_arg_matches(&matches)?;
    /// let connection = InfluxConnection::new(&connection_params)?;
    /// assert_eq!(connection.bucket, "the-bucket".to_string());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        connection_params: &InfluxConnectionConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = InfluxClient::builder(
            connection_params.uri.to_owned(),
            connection_params.token()?,
            connection_params.org.to_owned(),
        )
        .build()?;
        Ok(InfluxConnection {
            client,
            bucket: connection_params.bucket.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::{Args, Command, FromArgMatches};

    use super::*;
    use crate::connection::InfluxConnectionConfig;

    #[test]
    fn test_command_line_uses_defaults() {
        let command = Command::new("influx_client");
        let command = InfluxConnectionConfig::augment_args(command);
        let matches = command.get_matches_from(vec![
            "influx_client",
            "--influxdb-uri",
            "http://influx.io",
            "--influxdb-token",
            "the-token",
        ]);
        let connection_params = InfluxConnectionConfig::from_arg_matches(&matches)
            .expect("failed to create params from command line");
        assert_eq!(connection_params.uri, *"http://influx.io");
        assert_eq!(connection_params.token.token, Some("the-token".to_string()));
        assert_eq!(connection_params.org, *"sdv");
        assert_eq!(connection_params.bucket, *"demo");
    }

    #[test]
    fn test_command_line_requires_uri() {
        let command = Command::new("influx_client");
        let command = InfluxConnectionConfig::augment_args(command);
        let matches =
            command.try_get_matches_from(vec!["influx_client", "--influxdb-token", "the-token"]);
        assert!(
            matches.is_err_and(|e| { e.kind() == clap::error::ErrorKind::MissingRequiredArgument })
        );
    }

    #[test]
    fn test_command_line_requires_token_or_token_file() {
        let command = Command::new("influx_client");
        let command = InfluxConnectionConfig::augment_args(command);
        let no_token_matches = command.try_get_matches_from(vec![
            "influx_client",
            "--influxdb-uri",
            "http://influx.io",
        ]);
        assert!(no_token_matches
            .is_err_and(|e| e.kind() == clap::error::ErrorKind::MissingRequiredArgument));

        let command = Command::new("influx_client");
        let command = InfluxConnectionConfig::augment_args(command);
        let with_token_matches = command.get_matches_from(vec![
            "influx_client",
            "--influxdb-uri",
            "http://influx.io",
            "--influxdb-token",
            "the-token",
        ]);

        let connection_params = InfluxConnectionConfig::from_arg_matches(&with_token_matches)
            .expect("failed to create params from command line");
        assert_eq!(connection_params.token.token, Some("the-token".to_string()));

        let command = Command::new("influx_client");
        let command = InfluxConnectionConfig::augment_args(command);
        let with_token_file_matches = command.get_matches_from(vec![
            "influx_client",
            "--influxdb-uri",
            "http://influx.io",
            "--influxdb-token-file",
            "/path/to/token-file",
        ]);
        let connection_params = InfluxConnectionConfig::from_arg_matches(&with_token_file_matches)
            .expect("failed to create params from command line");
        println!("params: {:?}", connection_params);
        assert_eq!(
            connection_params.token.token_file,
            Some(PathBuf::from("/path/to/token-file"))
        );
    }
}

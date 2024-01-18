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

use clap::{Arg, ArgGroup, ArgMatches, Command};
use influxrs::InfluxClient;
use log::{error, info};

const PARAM_INFLUXDB_BUCKET: &str = "influxdb-bucket";
const PARAM_INFLUXDB_ORG: &str = "influxdb-org";
const PARAM_INFLUXDB_URI: &str = "influxdb-uri";
const PARAM_INFLUXDB_TOKEN: &str = "influxdb-token";
const PARAM_INFLUXDB_TOKEN_FILE: &str = "influxdb-token-file";

/// Adds command line arguments to an existing command line which can be
/// used to configure the connection to an InfluxDB server.
///
/// The following arguments are being added:
///
/// | long name           | environment variable    | default value |
/// |---------------------|-------------------------|---------------|
/// | influxdb-bucket     | INFLUXDB_BUCKET         | `demo`        |
/// | influxdb-org        | INFLUXDB_ORG            | `sdv`         |
/// | influxdb-uri        | INFLUXDB_URI            | -             |
/// | influxdb-token      | INFLUXDB_TOKEN          | -             |
/// | influxdb-token-file | INFLUXDB_TOKEN_FILE     | -             |
///
pub fn add_command_line_args(command_line: Command) -> Command {
    command_line
        .arg(
            Arg::new(PARAM_INFLUXDB_URI)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_INFLUXDB_URI)
                .alias("ia")
                .help("The HTTP(S) URI of the InfluxDB server to write data to.")
                .value_name("URI")
                .required(true)
                .env("INFLUXDB_URI"),
        )
        .group(ArgGroup::new("influxdb-auth-token").required(true))
        .arg(
            Arg::new(PARAM_INFLUXDB_TOKEN)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_INFLUXDB_TOKEN)
                .alias("token")
                .help("The API token to use for authenticating to the InfluxDB server.")
                .group("influxdb-auth-token")
                .value_name("TOKEN")
                .env("INFLUXDB_TOKEN"),
        )
        .arg(
            Arg::new(PARAM_INFLUXDB_TOKEN_FILE)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_INFLUXDB_TOKEN_FILE)
                .alias("token-file")
                .help("The path to a file that contains the API token to use for authenticating to the InfluxDB server.")
                .group("influxdb-auth-token")
                .value_name("FILE")
                .conflicts_with("influxdb-token")
                .env("INFLUXDB_TOKEN_FILE"),
        )
        .arg(
            Arg::new(PARAM_INFLUXDB_ORG)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_INFLUXDB_ORG)
                .alias("org")
                .help("The name of the organization to connect to on the InfluxDB server.")
                .value_name("NAME")
                .required(false)
                .default_value("sdv")
                .env("INFLUXDB_ORG"),
        )
        .arg(
            Arg::new(PARAM_INFLUXDB_BUCKET)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_INFLUXDB_BUCKET)
                .alias("bucket")
                .help("The name of the bucket to write data to on the InfluxDB server.")
                .value_name("NAME")
                .required(false)
                .default_value("demo")
                .env("INFLUXDB_BUCKET"),
        )
}

fn read_token_from_file(filename: &str) -> std::io::Result<String> {
    info!("reading token from file {filename}");
    std::fs::read_to_string(filename)
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            error!("failed to read token from file [{filename}]: {e}");
            e
        })
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
    /// use clap::Command;
    /// use influx_client::connection::InfluxConnection;
    ///
    /// let command = influx_client::connection::add_command_line_args(Command::new("influx_client"));
    /// let matches = command.get_matches_from(vec![
    ///     "influx_client",
    ///     "--influxdb-uri", "http://my-influx.io",
    ///     "--influxdb-token", "some-token",
    ///     "--influxdb-bucket", "the-bucket",
    /// ]);
    /// let connection = InfluxConnection::new(&matches)?;
    /// assert_eq!(connection.bucket, "the-bucket".to_string());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(args: &ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {
        let influx_uri = args
            .get_one::<String>(PARAM_INFLUXDB_URI)
            .unwrap()
            .to_owned();
        let influx_token = match args.get_one::<String>(PARAM_INFLUXDB_TOKEN) {
            Some(token) => token.to_string(),
            None => {
                let file_name = args.get_one::<String>(PARAM_INFLUXDB_TOKEN_FILE).unwrap();
                match read_token_from_file(file_name) {
                    Ok(token) => token,
                    Err(e) => return Err(Box::new(e)),
                }
            }
        };
        let influx_org = args
            .get_one::<String>(PARAM_INFLUXDB_ORG)
            .unwrap()
            .to_owned();
        let influx_bucket = args
            .get_one::<String>(PARAM_INFLUXDB_BUCKET)
            .unwrap()
            .to_owned();
        let client = InfluxClient::builder(influx_uri, influx_token, influx_org)
            .build()
            .unwrap();
        Ok(InfluxConnection {
            client,
            bucket: influx_bucket,
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_command_line_uses_defaults() {
        let command = crate::connection::add_command_line_args(clap::Command::new("influx_client"));
        let matches = command.get_matches_from(vec![
            "influx_client",
            "--influxdb-uri",
            "http://influx.io",
            "--influxdb-token",
            "the-token",
        ]);
        assert_eq!(
            matches
                .get_one::<String>(super::PARAM_INFLUXDB_URI)
                .unwrap(),
            "http://influx.io"
        );
        assert_eq!(
            matches
                .get_one::<String>(super::PARAM_INFLUXDB_TOKEN)
                .unwrap(),
            "the-token"
        );
        assert_eq!(
            matches
                .get_one::<String>(super::PARAM_INFLUXDB_ORG)
                .unwrap(),
            "sdv"
        );
        assert_eq!(
            matches
                .get_one::<String>(super::PARAM_INFLUXDB_BUCKET)
                .unwrap(),
            "demo"
        );
    }

    #[test]
    fn test_command_line_requires_uri() {
        let command = crate::connection::add_command_line_args(clap::Command::new("influx_client"));
        let matches =
            command.try_get_matches_from(vec!["influx_client", "--influxdb-token", "the-token"]);
        assert!(matches.is_err_and(|e| e.kind() == clap::error::ErrorKind::MissingRequiredArgument));
    }

    #[test]
    fn test_command_line_requires_token_or_token_file() {
        let command = crate::connection::add_command_line_args(clap::Command::new("influx_client"));
        let no_token_matches = command.try_get_matches_from(vec![
            "influx_client",
            "--influxdb-uri",
            "http://influx.io",
        ]);
        assert!(no_token_matches
            .is_err_and(|e| e.kind() == clap::error::ErrorKind::MissingRequiredArgument));

        let command = crate::connection::add_command_line_args(clap::Command::new("influx_client"));
        let with_token_matches = command.get_matches_from(vec![
            "influx_client",
            "--influxdb-uri",
            "http://influx.io",
            "--influxdb-token",
            "the-token",
        ]);
        assert_eq!(
            with_token_matches
                .get_one::<String>(super::PARAM_INFLUXDB_TOKEN)
                .unwrap(),
            "the-token"
        );

        let command = crate::connection::add_command_line_args(clap::Command::new("influx_client"));
        let with_token_file_matches = command.get_matches_from(vec![
            "influx_client",
            "--influxdb-uri",
            "http://influx.io",
            "--influxdb-token-file",
            "/path/to/token-file",
        ]);
        assert_eq!(
            with_token_file_matches
                .get_one::<String>(super::PARAM_INFLUXDB_TOKEN_FILE)
                .unwrap(),
            "/path/to/token-file"
        );
    }
}

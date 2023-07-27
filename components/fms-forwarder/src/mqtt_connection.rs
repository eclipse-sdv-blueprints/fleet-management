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

use clap::{Arg, ArgMatches, Command};
use log::{error, info, warn};
use mqtt::{
    AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, SslOptionsBuilder,
};
use paho_mqtt as mqtt;
use std::{thread, time::Duration};


const PARAM_CA_PATH: &str = "ca-path";
const PARAM_DEVICE_CERT: &str = "device-cert";
const PARAM_DEVICE_KEY: &str = "device-key";
const PARAM_MQTT_CLIENT_ID: &str = "mqtt-client-id";
const PARAM_MQTT_URI: &str = "mqtt-uri";
const PARAM_MQTT_USERNAME: &str = "mqtt-username";
const PARAM_MQTT_PASSWORD: &str = "mqtt-password";
const PARAM_TRUST_STORE_PATH: &str = "trust-store-path";

/// Adds arguments to an existing command line which can be
/// used to configure the connection to an MQTT endpoint.
/// 
/// The following arguments are being added:
/// 
/// | long name           | environment variable | default value |
/// |---------------------|----------------------|---------------|
/// | mqtt-client-id      | MQTT_CLIENT_ID       | random ID     |
/// | mqtt-uri            | MQTT_URI             | -             |
/// | mqtt-username       | MQTT_USERNAME        | -             |
/// | mqtt-password       | MQTT_PASSWORD        | -             |
/// | device-cert         | DEVICE_CERT          | -             |
/// | device-key          | DEVICE_KEY           | -             |
/// | ca-path             | CA_PATH              | -             |
/// | trust-store-path    | TRUST_STORE_PATH     | -             |
/// 
pub fn add_command_line_args(command: Command) -> Command {
    command
        .arg(
            Arg::new(PARAM_MQTT_CLIENT_ID)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_MQTT_CLIENT_ID)
                .help("The client identifier to use in the MQTT Connect Packet.")
                .value_name("ID")
                .required(false)
                .env("MQTT_CLIENT_ID"),
        )
        .arg(
            Arg::new(PARAM_MQTT_URI)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_MQTT_URI)
                .help("The URI of the MQTT adapter to publish data to.")
                .value_name("URI")
                .required(true)
                .env("MQTT_URI"),
        )
        .arg(
            Arg::new(PARAM_MQTT_USERNAME)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_MQTT_USERNAME)
                .help("The username to use for authenticating to the MQTT endpoint.")
                .value_name("USERNAME")
                .required(false)
                .env("MQTT_USERNAME"),
        )
        .arg(
            Arg::new(PARAM_MQTT_PASSWORD)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_MQTT_PASSWORD)
                .help("The password to use for authenticating to the MQTT endpoint.")
                .value_name("PWD")
                .required(false)
                .env("MQTT_PASSWORD"),
        )
        .arg(
            Arg::new(PARAM_DEVICE_CERT)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_DEVICE_CERT)
                .help("The path to a PEM file containing the X.509 certificate that the device should use for authentication.")
                .value_name("PATH")
                .required(false)
                .env("DEVICE_CERT"),
        )
        .arg(
            Arg::new(PARAM_DEVICE_KEY)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_DEVICE_KEY)
                .help("The path to a PEM file containing the private key that the device should use for authentication.")
                .value_name("PATH")
                .required(false)
                .env("DEVICE_KEY"),
        )
        .arg(
            Arg::new(PARAM_CA_PATH)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_CA_PATH)
                .help("The path to a folder that contains PEM files for trusted certificate authorities.")
                .value_name("PATH")
                .required(false)
                .env("CA_PATH"),
        )
        .arg(
            Arg::new(PARAM_TRUST_STORE_PATH)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_TRUST_STORE_PATH)
                .help("The path to a file that contains PEM encoded trusted certificates.")
                .value_name("PATH")
                .required(false)
                .env("TRUST_STORE_PATH"),
        )
}

/// A connection to an MQTT endpoint.
/// 
pub struct MqttConnection {
    pub mqtt_client: AsyncClient,
    pub uri: String,
    pub client_id: String,
}

impl MqttConnection {

    /// Creates a new connection to an MQTT endpoint.
    /// 
    /// Expects to find parameters as defined by [`add_command_line_args`] in the passed
    /// in *args*.
    /// 
    /// The connection returned is configured to keep trying to (re-)connect to the configured
    /// MQTT endpoint.
    pub async fn new(args: &ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {
        let mqtt_uri = args
            .get_one::<String>(PARAM_MQTT_URI)
            .unwrap()
            .to_owned();
        let client_id = args
            .get_one::<String>(PARAM_MQTT_CLIENT_ID)
            .unwrap_or(&"".to_string())
            .to_owned();
        let mut ssl_options_builder = SslOptionsBuilder::new();
        match args.get_one::<String>(PARAM_CA_PATH) {
            Some(path) => match ssl_options_builder.ca_path(path) {
                Err(e) => {
                    error!("failed to set CA path on MQTT client: {e}");
                    return Err(Box::new(e));
                }
                Ok(_builder) => (),
            },
            None => (),
        };
        match args.get_one::<String>(PARAM_TRUST_STORE_PATH) {
            Some(path) => match ssl_options_builder.trust_store(path) {
                Err(e) => {
                    error!("failed to set trust store path on MQTT client: {e}");
                    return Err(Box::new(e));
                }
                Ok(_builder) => (),
            },
            None => (),
        };

        let mut connect_options_builder = ConnectOptionsBuilder::new_v3();
        connect_options_builder.connect_timeout(Duration::from_secs(10));
        connect_options_builder
            .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(16));
        connect_options_builder.clean_session(true);
        connect_options_builder.keep_alive_interval(Duration::from_secs(10));
        connect_options_builder.max_inflight(10);

        match (
            args.get_one::<String>(PARAM_MQTT_USERNAME),
            args.get_one::<String>(PARAM_MQTT_PASSWORD),
            args.get_one::<String>(PARAM_DEVICE_CERT),
            args.get_one::<String>(PARAM_DEVICE_KEY),
        ) {
            (_, _, Some(cert_path), Some(key_path)) => {
                match ssl_options_builder.key_store(cert_path) {
                    Ok(_builder) => (),
                    Err(e) => {
                        error!("failed to set client certificate for MQTT client: {e}");
                        return Err(Box::new(e));
                    }
                }
                match ssl_options_builder.private_key(key_path) {
                    Ok(_builder) => (),
                    Err(e) => {
                        error!("failed to set private key for MQTT client: {e}");
                        return Err(Box::new(e));
                    }
                }
                info!("using client certificate for authenticating to MQTT endpoint");
            }
            (Some(username), Some(password), _, _) => {
                connect_options_builder.user_name(username);
                connect_options_builder.password(password);
                info!("using username and password for authenticating to MQTT endpoint");
            }
            _ => {
                info!("no credentials specified, trying to connect anonymously to MQTT endpoint");
            }
        }

        connect_options_builder.ssl_options(ssl_options_builder.finalize());
        let connect_options = connect_options_builder.finalize();
        info!("connecting to MQTT endpoint at {}", mqtt_uri);
        match CreateOptionsBuilder::new()
            .server_uri(&mqtt_uri)
            .max_buffered_messages(50)
            .send_while_disconnected(true)
            .delete_oldest_messages(true)
            .client_id(&client_id)
            .create_client()
        {
            Err(e) => {
                error!("failed to create MQTT client: {}", e);
                Err(Box::new(e))
            }
            Ok(client) => {
                client.connect_with_callbacks(
                    connect_options,
                    MqttConnection::on_connect_success,
                    MqttConnection::on_connect_failure,
                );
                Ok(MqttConnection {
                    mqtt_client: client,
                    uri: mqtt_uri,
                    client_id,
                })
            }
        }
    }

    fn on_connect_success(_client: &AsyncClient, _msgid: u16) {
        info!("successfully connected to MQTT endpoint");
    }

    fn on_connect_failure(client: &AsyncClient, _msgid: u16, rc: i32) {
        warn!(
            "attempt to connect to MQTT endpoint failed with error code {}, retrying ...",
            rc
        );
        thread::sleep(Duration::from_secs(3));
        client.reconnect_with_callbacks(
            MqttConnection::on_connect_success,
            MqttConnection::on_connect_failure,
        );
    }
}

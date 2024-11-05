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

use clap::{Args, ValueEnum};
use log::{debug, error, info, warn};
use mqtt::{
    AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, SslOptions, SslOptionsBuilder,
};
use paho_mqtt as mqtt;
use std::{fmt::Display, path::PathBuf, thread, time::Duration};

const PARAM_CA_PATH: &str = "ca-path";
const PARAM_DEVICE_CERT: &str = "device-cert";
const PARAM_DEVICE_KEY: &str = "device-key";
const PARAM_ENABLE_HOSTNAME_VERIFICATION: &str = "enable-hostname-verification";
const PARAM_MQTT_CLIENT_ID: &str = "mqtt-client-id";
const PARAM_MQTT_URI: &str = "mqtt-uri";
const PARAM_MQTT_USERNAME: &str = "mqtt-username";
const PARAM_MQTT_PASSWORD: &str = "mqtt-password";
const PARAM_MQTT_PROTOCOL_VERSION: &str = "mqtt-protocol-version";
const PARAM_TRUST_STORE_PATH: &str = "trust-store-path";

pub fn on_connect_success(_client: &AsyncClient, _msgid: u16) {
    info!("successfully connected to MQTT endpoint");
}

pub fn on_connect_failure(client: &AsyncClient, _msgid: u16, rc: i32) {
    warn!(
        "attempt to connect to MQTT endpoint failed with error code {}, retrying ...",
        rc
    );
    thread::sleep(Duration::from_secs(3));
    client.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}

#[derive(Clone, ValueEnum)]
pub enum MqttVersion {
    V3,
    V5,
}

impl Display for MqttVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MqttVersion::V3 => f.write_str("3.1.1"),
            MqttVersion::V5 => f.write_str("5"),
        }
    }
}

#[derive(Args)]
pub struct MqttClientOptions {
    /// The client identifier to use in the MQTT Connect Packet.
    #[arg(long = PARAM_MQTT_CLIENT_ID, value_name = "ID", env = "MQTT_CLIENT_ID")]
    client_id: Option<String>,

    /// The URI of the MQTT adapter to publish data to.
    #[arg(long = PARAM_MQTT_URI, value_name = "URI", env = "MQTT_URI")]
    server_uri: String,

    /// The username to use for authenticating to the MQTT endpoint.
    #[arg(long = PARAM_MQTT_USERNAME, value_name = "USERNAME", env = "MQTT_USERNAME")]
    username: Option<String>,

    /// The password to use for authenticating to the MQTT endpoint.
    #[arg(long = PARAM_MQTT_PASSWORD, value_name = "PWD", env = "MQTT_PASSWORD")]
    password: Option<String>,

    /// The path to a PEM file containing the X.509 certificate that the device should use for authentication.
    #[arg(long = PARAM_DEVICE_CERT, value_name = "PATH", env = "DEVICE_CERT", value_parser = clap::builder::PathBufValueParser::new())]
    cert_path: Option<PathBuf>,

    /// The path to a PEM file containing the private key that the device should use for authentication.
    #[arg(long = PARAM_DEVICE_KEY, value_name = "PATH", env = "DEVICE_KEY", value_parser = clap::builder::PathBufValueParser::new())]
    key_path: Option<PathBuf>,

    /// The path to a folder that contains PEM files for trusted certificate authorities.
    #[arg(long = PARAM_CA_PATH, value_name = "PATH", env = "CA_PATH", value_parser = clap::builder::PathBufValueParser::new())]
    ca_path: Option<PathBuf>,

    /// The path to a file that contains PEM encoded trusted certificates.
    #[arg(long = PARAM_TRUST_STORE_PATH, value_name = "PATH", env = "TRUST_STORE_PATH", value_parser = clap::builder::PathBufValueParser::new())]
    trust_store_path: Option<PathBuf>,

    /// Indicates whether server certificates should be matched against the
    /// hostname/IP address used by a client to connect to the server.
    #[arg(long = PARAM_ENABLE_HOSTNAME_VERIFICATION, value_name = "FLAG", default_value = "true", env = "ENABLE_HOSTNAME_VERIFICATION")]
    enable_hostname_verification: bool,

    /// The version of the MQTT protocol to use for connecting to the broker.
    #[arg(
        value_enum,
        long = PARAM_MQTT_PROTOCOL_VERSION,
        default_value = "v3",
        env = "MQTT_PROTOCOL_VERSION"
    )]
    protocol_version: MqttVersion,
}

impl MqttClientOptions {
    pub fn ssl_options(&self) -> Result<SslOptions, Box<dyn std::error::Error>> {
        let mut ssl_options_builder = SslOptionsBuilder::new();
        match (self.cert_path.as_ref(), self.key_path.as_ref()) {
            (Some(cert_path), Some(key_path)) => {
                if let Err(e) = ssl_options_builder.key_store(cert_path) {
                    error!("failed to set client certificate for MQTT client: {}", e);
                    return Err(Box::from(e));
                }
                if let Err(e) = ssl_options_builder.private_key(key_path) {
                    error!("failed to set private key for MQTT client: {}", e);
                    return Err(Box::from(e));
                }
                info!("using client certificate for authenticating to MQTT endpoint");
            }
            _ => {
                debug!("no client key material specified");
            }
        }

        if let Some(path) = self.ca_path.as_ref() {
            if let Err(e) = ssl_options_builder.ca_path(path) {
                error!("failed to set CA path on MQTT client: {}", e);
                return Err(Box::from(e));
            }
        }
        if let Some(path) = self.trust_store_path.as_ref() {
            if let Err(e) = ssl_options_builder.trust_store(path) {
                error!("failed to set trust store path on MQTT client: {}", e);
                return Err(Box::from(e));
            }
        }
        ssl_options_builder.verify(self.enable_hostname_verification);
        Ok(ssl_options_builder.finalize())
    }

    fn mqtt3_connect_options(
        &self,
    ) -> Result<paho_mqtt::ConnectOptions, Box<dyn std::error::Error>> {
        let mut connect_options_builder = ConnectOptionsBuilder::new_v3();
        connect_options_builder.connect_timeout(Duration::from_secs(10));
        connect_options_builder
            .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(16));
        connect_options_builder.clean_session(true);
        connect_options_builder.keep_alive_interval(Duration::from_secs(10));
        connect_options_builder.max_inflight(10);
        connect_options_builder.ssl_options(self.ssl_options()?);

        match (self.username.as_ref(), self.password.as_ref()) {
            (Some(username), Some(password)) => {
                connect_options_builder.user_name(username);
                connect_options_builder.password(password);
                info!("using username and password for authenticating to MQTT endpoint");
            }
            _ => {
                debug!("no credentials specified");
            }
        }
        Ok(connect_options_builder.finalize())
    }

    fn mqtt5_connect_options(
        &self,
    ) -> Result<paho_mqtt::ConnectOptions, Box<dyn std::error::Error>> {
        let options = ConnectOptionsBuilder::new_v5()
            .connect_timeout(Duration::from_secs(5))
            .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(16))
            .clean_start(true)
            .keep_alive_interval(Duration::from_secs(10))
            .max_inflight(10)
            .finalize();
        Ok(options)
    }

    pub fn create_async_client(&self) -> Result<AsyncClient, Box<dyn std::error::Error>> {
        CreateOptionsBuilder::new()
            .server_uri(self.server_uri.as_str())
            .max_buffered_messages(50)
            .send_while_disconnected(true)
            .delete_oldest_messages(true)
            .client_id(self.client_id.clone().unwrap_or_default())
            .create_client()
            .map_err(Box::from)
    }

    pub async fn connect(&self) -> Result<AsyncClient, Box<dyn std::error::Error>> {
        let connect_options = match self.protocol_version {
            MqttVersion::V3 => self.mqtt3_connect_options()?,
            MqttVersion::V5 => self.mqtt5_connect_options()?,
        };
        match self.create_async_client() {
            Err(e) => {
                error!("failed to create MQTT client: {}", e);
                Err(e)
            }
            Ok(client) => {
                info!(
                    "connecting to MQTT endpoint [broker URI: {}, protocol version: {}]",
                    client.server_uri(),
                    self.protocol_version
                );
                client
                    .connect_with_callbacks(
                        connect_options,
                        crate::mqtt_connection::on_connect_success,
                        crate::mqtt_connection::on_connect_failure,
                    )
                    .await
                    .map(|_response| client)
                    .map_err(Box::from)
            }
        }
    }
}

impl Display for MqttClientOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "MQTT Client Options [server-uri: {}]",
            self.server_uri
        ))
    }
}

#[cfg(test)]
mod tests {
    use clap::{Args, FromArgMatches};

    use super::*;

    #[test]
    fn test_command_line_uses_defaults() {
        let command = clap::Command::new("mqtt");
        let command = MqttClientOptions::augment_args(command);
        let matches =
            command.get_matches_from(vec!["mqtt", "--mqtt-uri", "mqtts://non-existing.host.io"]);
        let options = MqttClientOptions::from_arg_matches(&matches)
            .expect("failed to create options from command line");

        assert_eq!(options.server_uri, "mqtts://non-existing.host.io");
        assert!(options.client_id.is_none());
        assert!(options.username.is_none());
        assert!(options.password.is_none());
        assert!(options.cert_path.is_none());
        assert!(options.key_path.is_none());
        assert!(options.ca_path.is_none());
        assert!(options.enable_hostname_verification);
    }
}

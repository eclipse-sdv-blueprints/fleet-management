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

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;
use std::sync::Arc;

use clap::{Arg, ArgAction, ArgMatches, Command};
use fms_proto::fms::VehicleStatus;
use futures::TryStreamExt;
use influx_client::writer::InfluxWriter;
use log::{debug, error, info, trace};

use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::{BorrowedHeaders, BorrowedMessage, Headers};
use rdkafka::{ClientConfig, Message};

use zenoh::Config;

const CONTENT_TYPE_PROTOBUF: &str = "application/vnd.google.protobuf";

const HEADER_NAME_ORIG_ADDRESS: &str = "orig_address";

const PARAM_KAFKA_PROPERTIES_FILE: &str = "kafka-properties-file";
const PARAM_KAFKA_TOPIC_NAME: &str = "kafka-topic";

const SUBCOMMAND_HONO: &str = "hono";
const SUBCOMMAND_ZENOH: &str = "zenoh";

const KEY_EXPR: &str = "fms/vehicleStatus";

fn parse_zenoh_args(args: &ArgMatches) -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(conf_file) = args.get_one::<String>("config") {
        Config::from_file(conf_file)
    } else {
        Ok(Config::default())
    }
}

fn add_property_bag_to_map(property_bag: String, headers: &mut HashMap<String, String>) {
    property_bag.split('&').for_each(|p| {
        trace!("processing property: {p}");
        if let Some((key, value)) = p.split_once('=') {
            if headers.contains_key(key) {
                trace!("skipping property [{key}] from property bag because header with same name exists");
            } else {
                trace!("adding propery [key: {key}, value: {value}] to header map");
                headers.insert(key.to_string(), value.to_string());
            }
        }
    });
}

fn get_headers_as_map(headers: &BorrowedHeaders) -> HashMap<String, String> {
    let mut result = HashMap::new();
    headers.iter().for_each(|header| {
        match (
            header.key,
            header
                .value
                .and_then(|v| String::from_utf8(v.to_vec()).ok()),
        ) {
            (HEADER_NAME_ORIG_ADDRESS, Some(value)) => {
                if let Some((_topic, props)) = value.rsplit_once("/?") {
                    debug!("found property bag in {HEADER_NAME_ORIG_ADDRESS} header: {props}");
                    add_property_bag_to_map(props.to_string(), &mut result);
                }
            }
            (_, Some(value)) => {
                result.insert(header.key.to_string(), value);
            }
            (_, None) => {
                debug!("message contains empty header [{}]", header.key);
            }
        };
    });

    result
}

fn read_lines(filename: &String) -> Result<io::Lines<BufReader<File>>, Box<dyn std::error::Error>> {
    // Open the file in read-only mode.
    match File::open(filename) {
        Ok(file) => {
            // Read the file line by line, and return an iterator of the lines of the file.
            Ok(io::BufReader::new(file).lines())
        }
        Err(e) => Err(Box::new(e)),
    }
}

fn get_kafka_client_config(filename: &String) -> Result<ClientConfig, Box<dyn std::error::Error>> {
    read_lines(filename).map(|lines| {
        let mut client_config = ClientConfig::new();
        for line in lines {
            match line {
                Ok(property) => match property.split_once('=') {
                    Some((key, value)) => {
                        client_config.set(key, value);
                    }
                    None => {
                        debug!("cannot parse line into property: {}", property);
                    }
                },
                Err(e) => {
                    debug!("cannot read line from file: {e}");
                }
            }
        }
        client_config
    })
}

fn deserialize_vehicle_status(protobuf: &[u8]) -> Option<VehicleStatus> {
    use protobuf::Message;

    match VehicleStatus::parse_from_bytes(protobuf) {
        Ok(vehicle_status) => {
            trace!("successfully deserialized VehicleStatus from protobuf");
            Some(vehicle_status)
        }
        Err(e) => {
            debug!("failed to deserialize VehicleStatus from protobuf: {}", e);
            None
        }
    }
}

async fn process_protobuf_message(
    message_properties: HashMap<String, String>,
    payload: &[u8],
    influx_writer: Arc<InfluxWriter>,
) {
    match message_properties.get("device_id") {
        Some(device_id) => {
            debug!("received message from vehicle {}", device_id);
            if let Some(vehicle_status) = deserialize_vehicle_status(payload) {
                influx_writer.write_vehicle_status(&vehicle_status).await;
            }
        }
        None => debug!("discarding message from unknown device"),
    }
}

async fn process_zenoh_message(payload: &[u8], influx_writer: Arc<InfluxWriter>) {
    if let Some(vehicle_status) = deserialize_vehicle_status(payload) {
        influx_writer.write_vehicle_status(&vehicle_status).await;
    } else {
        debug!("ignoring message without payload");
    }
}

async fn process_hono_message(m: &BorrowedMessage<'_>, influx_writer: Arc<InfluxWriter>) {
    if let Some(headers) = m.headers() {
        let message_properties = get_headers_as_map(headers);
        match (
            message_properties.get("content-type").map(String::as_str),
            m.payload(),
        ) {
            (Some(CONTENT_TYPE_PROTOBUF), Some(payload)) => {
                debug!("received protobuf message");
                process_protobuf_message(message_properties, payload, influx_writer).await
            }
            (_, None) => debug!("ignoring message without payload"),
            _ => {}
        }
    } else {
        debug!("ignoring message without headers");
    }
}

async fn run_async_processor_hono(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let influx_writer = InfluxWriter::new(args).map(Arc::new)?;

    let hono_args = args.subcommand_matches(SUBCOMMAND_HONO).unwrap();
    let mut client_config = get_kafka_client_config(
        hono_args
            .get_one::<String>(PARAM_KAFKA_PROPERTIES_FILE)
            .unwrap(),
    )?;

    // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
    let consumer: StreamConsumer = client_config
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()?;

    let topic_name = hono_args.get_one::<String>(PARAM_KAFKA_TOPIC_NAME).unwrap();

    consumer.subscribe(&[topic_name.as_str()])?;
    info!("successfully subscribed to topic {topic_name}");
    info!("starting message consumer");
    consumer
        .stream()
        .try_for_each(|borrowed_message| {
            let cloned_writer = influx_writer.clone();
            async move {
                process_hono_message(&borrowed_message, cloned_writer).await;
                Ok(())
            }
        })
        .await?;
    Ok(())
}

async fn run_async_processor_zenoh(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let influx_writer = InfluxWriter::new(args).map(Arc::new)?;
    let zenoh_args = args.subcommand_matches(SUBCOMMAND_ZENOH).unwrap();
    let config = parse_zenoh_args(zenoh_args)
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

    info!("Opening session...");
    let session = zenoh::open(config)
        .await
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

    info!("Declaring Subscriber on '{}'...", &KEY_EXPR);
    let subscriber = session
        .declare_subscriber(KEY_EXPR)
        .await
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;
    while let Ok(sample) = subscriber.recv_async().await {
        let cloned_writer = influx_writer.clone();
        let payload = sample.payload().to_bytes();
        process_zenoh_message(&payload, cloned_writer).await;
    }
    Ok(())
}

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let version = option_env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT")
        .unwrap_or(option_env!("VERGEN_GIT_SHA").unwrap_or("unknown"));

    let mut parser = Command::new("FMS data consumer")
        .arg_required_else_help(true)
        .version(version)
        .about("Receives FMS related VSS data points via Hono's Kafka based Telemetry API or Eclipse Zenoh instance and writes them to an InfluxDB server");

    parser = influx_client::connection::add_command_line_args(parser);

    parser = parser
        .subcommand_required(true)
        .subcommand(
            Command::new(SUBCOMMAND_HONO)
                .about("Forwards VSS data to an Influx DB server from Hono's north bound Kafka API")
                .arg(
            Arg::new(PARAM_KAFKA_PROPERTIES_FILE)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_KAFKA_PROPERTIES_FILE)
                .help("The path to a file containing Kafka client properties for connecting to the Kafka broker(s).")
		.action(ArgAction::Set)
                .value_name("PATH")
                .env("KAFKA_PROPERTIES_FILE")
                .required(true),
        )
        .arg(
            Arg::new(PARAM_KAFKA_TOPIC_NAME)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_KAFKA_TOPIC_NAME)
                .alias("topic")
                .help("The name of the Kafka topic to consume VSS data from.")
                .value_name("TOPIC")
                .required(true)
                .env("KAFKA_TOPIC_NAME"),
        ),
        )
        .subcommand(
            Command::new(SUBCOMMAND_ZENOH)
                .about("Forwards VSS data to an Influx DB server from Eclipse Zenoh")
            .arg(
            Arg::new("config")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long("config")
                .short('c')
                .help("A configuration file.")
                .required(false),
        ),
        );

    let args = parser.get_matches();

    match args.subcommand_name() {
        Some(SUBCOMMAND_HONO) => {
            info!("starting FMS data consumer for Hono");
            if let Err(e) = run_async_processor_hono(&args).await {
                error!("failed to start Hono processor: {e}");
                process::exit(1);
            }
        }
        Some(SUBCOMMAND_ZENOH) => {
            info!("starting FMS data consumer for Zenoh");
            if let Err(e) = run_async_processor_zenoh(&args).await {
                error!("failed to start Zenoh processor: {e}");
                process::exit(1);
            }
        }
        Some(_) => {
            // cannot happen because subcommand is required
            process::exit(1);
        }
        None => {
            // cannot happen because subcommand is required
            process::exit(1);
        }
    };
}

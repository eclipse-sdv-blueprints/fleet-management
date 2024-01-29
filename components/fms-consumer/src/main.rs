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
use std::time::Duration;

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


use async_std::task::sleep;
use futures::prelude::*;
use futures::select;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;


const CONTENT_TYPE_PROTOBUF: &str = "application/vnd.google.protobuf";

const HEADER_NAME_ORIG_ADDRESS: &str = "orig_address";

const PARAM_KAFKA_PROPERTIES_FILE: &str = "kafka-properties-file";
const PARAM_KAFKA_TOPIC_NAME: &str = "kafka-topic";

const SUBCOMMAND_KAFKA: &str = "kafka";
const SUBCOMMAND_ZENOH: &str = "zenoh";

const KEY_EXPR: &str = "fms/vehicleStatus";

pub fn parse_args(args: &ArgMatches) -> Config {
    let mut config: Config = if let Some(conf_file) = args.get_one::<String>("config") {
        Config::from_file(conf_file).unwrap()
    } else {
        Config::default()
    };

    if let Some(mode) = args.get_one::<WhatAmI>("mode") {
        config.set_mode(Some(*mode)).unwrap();
    }

    if let Some(values) = args.get_many::<String>("connect") {
        config
            .connect
            .endpoints
            .extend(values.map(|v| v.parse().unwrap()))
    }
    if let Some(values) = args.get_many::<String>("listen") {
        config
            .listen
            .endpoints
            .extend(values.map(|v| v.parse().unwrap()))
    }
    if let Some(values) = args.get_one::<bool>("no-multicast-scouting") {
        config
            .scouting
            .multicast
            .set_enabled(Some(*values))
            .unwrap();
    }

    config
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

async fn process_zenoh_message(
    payload: &[u8],
    influx_writer: Arc<InfluxWriter>,
){
              if let Some(vehicle_status) = deserialize_vehicle_status(payload) {
                influx_writer.write_vehicle_status(&vehicle_status).await;
            } else {
        	debug!("ignoring message without payload");
    	    }
}

async fn process_message(m: &BorrowedMessage<'_>, influx_writer: Arc<InfluxWriter>) {
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

async fn run_async_processor(args: &ArgMatches) {
    let influx_writer = InfluxWriter::new(args).map_or_else(
        |e| {
            error!("failed to create InfluxDB writer: {e}");
            process::exit(1);
        },
        Arc::new,
    );

    let kafka_args = args.subcommand_matches(SUBCOMMAND_KAFKA).unwrap();
    let mut client_config =
        get_kafka_client_config(kafka_args.get_one::<String>(PARAM_KAFKA_PROPERTIES_FILE).unwrap())
            .unwrap_or_else(|e| {
                error!("failed to create Kafka client: {e}");
                process::exit(1);
            });

    // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
    let consumer: StreamConsumer = client_config
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .unwrap_or_else(|e| {
            error!("failed to create Kafka client: {e}");
            process::exit(1);
        });

    let topic_name = kafka_args.get_one::<String>(PARAM_KAFKA_TOPIC_NAME).unwrap();

    match consumer.fetch_metadata(Some(topic_name), Duration::from_secs(10)) {
        Err(e) => {
            error!("could not retrieve meta data for topic [{topic_name}] from broker: {e}");
            process::exit(1);
        }
        Ok(metadata) => match metadata
            .topics()
            .iter()
            .find(|topic| topic.name() == topic_name)
        {
            Some(topic) => {
                if topic.partitions().is_empty() {
                    error!("topic [{topic_name}] does not exist (yet)");
                    process::exit(1);
                }
            }
            None => {
                error!("broker did not return meta data for topic [{topic_name}]");
                process::exit(1);
            }
        },
    }

    match consumer.subscribe(&[topic_name.as_str()]) {
        Err(e) => {
            error!("failed to subscribe to topic: {e}");
            process::exit(1);
        }
        Ok(_) => {
            info!("successfully subscribed to topic {topic_name}");
            info!("starting message consumer");
            consumer
                .stream()
                .try_for_each(|borrowed_message| {
                    let cloned_writer = influx_writer.clone();
                    async move {
                        process_message(&borrowed_message, cloned_writer).await;
                        Ok(())
                    }
                })
                .await
                .unwrap_or_else(|e| {
                    error!("could not start consumer for topic [{topic_name}]: {e}");
                    process::exit(1);
                });
        }
    }
}

async fn run_async_processor_zenoh(args: &ArgMatches) {
 	               let influx_writer = InfluxWriter::new(&args).map_or_else(
                |e| {
                    error!("failed to create InfluxDB writer: {e}");
                    process::exit(1);
                },
                Arc::new,
            );
           let zenoh_args = args.subcommand_matches(SUBCOMMAND_ZENOH).unwrap();
            let config = parse_args(zenoh_args);

            println!("Opening session...");
            let session = Arc::new(zenoh::open(config).res().await.unwrap());

            println!("Declaring Subscriber on '{}'...", &KEY_EXPR);

            let subscriber = session.declare_subscriber(KEY_EXPR).res().await.unwrap();

            println!("Enter 'q' to quit...");
            let mut stdin = async_std::io::stdin();
            let mut input = [0_u8];
            loop {
                select!(
                    sample = subscriber.recv_async() => {
                        let sample = sample.unwrap();
                        let cloned_writer = influx_writer.clone();
                        process_zenoh_message(&*sample.value.payload.contiguous(), cloned_writer).await

                    },

                    _ = stdin.read_exact(&mut input).fuse() => {

                        match input[0] {
                            b'q' => break,
                            0 => sleep(Duration::from_secs(1)).await,
                            _ => (),
                        }

                    }
                );
            }
}
#[tokio::main]
pub async fn main() {
    env_logger::init();

    let version = option_env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT")
        .unwrap_or(option_env!("VERGEN_GIT_SHA").unwrap_or("unknown"));

    let mut parser = Command::new("FMS data consumer")
        .arg_required_else_help(true)
        .version(version)
        .about("Receives FMS related VSS data points via Hono's Kafka based Telemetry API and writes them to an InfluxDB server");

    parser = influx_client::connection::add_command_line_args(parser);

    parser = parser
        .subcommand_required(true)
        .subcommand(influx_client::connection::add_command_line_args(
            Command::new(SUBCOMMAND_KAFKA)
                .about("Forwards VSS data to an Influx DB server from Kafka").arg(
            Arg::new(PARAM_KAFKA_PROPERTIES_FILE)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long(PARAM_KAFKA_PROPERTIES_FILE)
                .help("The path to a file containing Kafka client properties for connecting to the Kafka broker(s).")                .action(ArgAction::Set)
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
        ))
        .subcommand(influx_client::connection::add_command_line_args(
            Command::new(SUBCOMMAND_ZENOH)
                .about("Forwards VSS data to an Influx DB server from zenoh")
            .arg(
            Arg::new("mode")
		.value_parser(clap::value_parser!(WhatAmI))
                .long("mode")
                .short('m')
                .help("The zenoh session mode (peer by default).")
                .required(false),
        )
        .arg(
            Arg::new("connect")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long("connect")
                .short('e')
                .help("Endpoints to connect to.")
                .required(false),
        )
        .arg(
            Arg::new("listen")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long("listen")
                .short('l')
                .help("Endpoints to listen on.")
                .required(false),
        )
        .arg(
            Arg::new("no-multicast-scouting")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long("no-multicast-scouting")
                .help("Disable the multicast-based scouting mechanism.")
                .required(false),
        )        
        .arg(
            Arg::new("config")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long("config")
                .short('c')
                .help("A configuration file.")
                .required(false),
        ),
        ));

    let args = parser.get_matches();

    match args.subcommand_name() {
        Some(SUBCOMMAND_KAFKA) => {
            info!("starting FMS data consumer  Kafka");
            run_async_processor(&args).await
        }
        Some(SUBCOMMAND_ZENOH) => {
            info!("starting FMS data consumer for Zenoh");
	    run_async_processor_zenoh(&args).await
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

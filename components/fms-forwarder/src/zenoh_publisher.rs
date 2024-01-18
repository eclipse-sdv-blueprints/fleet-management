use crate::status_publishing::StatusPublisher;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use fms_proto::fms::VehicleStatus;
use log::{debug, warn};
use protobuf::Message;
use std::sync::Arc;
use zenoh::config::Config;
use zenoh::prelude::sync::*;
use zenoh::publication::Publisher;

const KEY_EXPR: &str = "fms/vehicleStatus";

pub fn add_command_line_args(command: Command) -> Command {
    command
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
}

pub fn parse_args(args: &ArgMatches) -> Config {
    let mut config: Config = Config::default();

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

pub struct ZenohPublisher<'a> {
 // publisher
    publisher: Publisher<'a>,
}

impl<'a> ZenohPublisher<'a> {
    pub async fn new(args: &ArgMatches) -> Result<ZenohPublisher<'a>, Box<dyn std::error::Error>> {
        let config = parse_args(args);
        let session = Arc::new(zenoh::open(config).res().unwrap());
        let publisher = session.declare_publisher(KEY_EXPR).res().unwrap();
        Ok(ZenohPublisher {
            // publisher
            publisher,
        })
    }
}

#[async_trait]
impl<'a> StatusPublisher for ZenohPublisher<'a> {
    async fn publish_vehicle_status(&self, vehicle_status: &VehicleStatus) {
        match vehicle_status.write_to_bytes() {
            Ok(payload) => {
                match self.publisher.put(payload).res() {
                    Ok(_t) => debug!("successfully published vehicle status to Zenoh",),
                    Err(e) => {
                        warn!("error publishing vehicle status to Zenoh: {}", e);
                    }
                };
                return;
            }
            Err(e) => warn!(
                "error serializing vehicle status to protobuf message: {}",
                e
            ),
        }
    }
}

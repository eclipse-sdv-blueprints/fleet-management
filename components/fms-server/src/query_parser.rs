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

use axum::http::StatusCode;
use log::debug;

use std::collections::HashMap;
use std::str::FromStr;

use chrono::{DateTime, Utc};

const LATEST_ONLY_QUERY: &str = "latestOnly";
const START_TIME_QUERY: &str = "starttime";
const STOP_TIME_QUERY: &str = "stoptime";
const VIN_QUERY: &str = "vin";
const TRIGGER_FILTER_QUERY: &str = "triggerFiler";

pub struct QueryParameters {
    pub start_time: i64,
    pub stop_time: i64,
    pub vin: Option<String>,
    pub trigger_filter: Option<String>,
    pub latest_only: Option<bool>,
}

pub fn parse_query_parameters(
    params: &HashMap<String, String>,
) -> Result<QueryParameters, StatusCode> {
    let start_parameter = parse_time(params, START_TIME_QUERY)?;
    let stop_parameter = parse_time(params, STOP_TIME_QUERY)?;
    let latest_only = parse_latest_only(params)?;

    if start_parameter.is_none() && latest_only.is_none() {
        // rFMS makes it mandatory to either supply the starttime or latestOnly
        return Err(StatusCode::BAD_REQUEST);
    }

    if latest_only.is_some() && (start_parameter.is_some() || stop_parameter.is_some()) {
        // rFMS does not allow to set latestOnly and and time at the same time
        return Err(StatusCode::BAD_REQUEST);
    }

    let vin = params.get(VIN_QUERY).cloned();
    let trigger_filter = params.get(TRIGGER_FILTER_QUERY).cloned();
    let start_time = start_parameter.unwrap_or(0);
    let stop_time = stop_parameter.unwrap_or(Utc::now().timestamp());

    let parameters = QueryParameters {
        start_time,
        stop_time,
        vin,
        trigger_filter,
        latest_only,
    };
    Ok(parameters)
}

fn parse_latest_only(params: &HashMap<String, String>) -> Result<Option<bool>, StatusCode> {
    let latest_parameter = params.get(LATEST_ONLY_QUERY);
    if let Some(latest_string) = latest_parameter {
        let latest_result = latest_string.parse();
        if latest_result.is_err() {
            return Err(StatusCode::BAD_REQUEST);
        }
        return Ok(latest_result.ok());
    }
    Ok(None)
}

fn parse_time(params: &HashMap<String, String>, key: &str) -> Result<Option<i64>, StatusCode> {
    let text = params.get(key);
    if let Some(latest_string) = text {
        let latest_result = DateTime::<Utc>::from_str(latest_string);
        if latest_result.is_err() {
            debug!(
                "Error parsing date: {:?} and input: {}",
                latest_result, latest_string
            );
            return Err(StatusCode::BAD_REQUEST);
        }
        return Ok(Some(latest_result.unwrap().timestamp()));
    }
    Ok(None)
}

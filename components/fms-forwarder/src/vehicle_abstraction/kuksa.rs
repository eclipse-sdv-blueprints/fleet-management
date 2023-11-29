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

tonic::include_proto!("kuksa.val.v1");

use protobuf::{MessageField, well_known_types::timestamp::Timestamp};

use std::collections::HashMap;

use self::datapoint::Value;
use fms_proto::fms::VehicleStatus;
use fms_proto::fms::KeyValue;
use fms_proto::fms::SnapshotData;
use std::env;

#[derive(Debug)]
pub struct UnsupportedValueTypeError{}

impl TryFrom<Value> for u32 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<u32> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as u64),
            Value::Uint64(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<u64> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as u64)),
            Value::Uint64(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as i32),
            Value::Int32(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<i32> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as i32)),
            Value::Int32(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as i64),
            Value::Uint64(v) => Ok(v as i64),
            Value::Int32(v) => Ok(v as i64),
            Value::Int64(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<i64> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as i64)),
            Value::Uint64(v) => Ok(Some(v as i64)),
            Value::Int32(v) => Ok(Some(v as i64)),
            Value::Int64(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for f32 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as f32),
            Value::Int32(v) => Ok(v as f32),
            Value::Float(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<f32> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as f32)),
            Value::Int32(v) => Ok(Some(v as f32)),
            Value::Float(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as f64),
            Value::Uint64(v) => Ok(v as f64),
            Value::Int32(v) => Ok(v as f64),
            Value::Int64(v) => Ok(v as f64),
            Value::Double(v) => Ok(v),
            Value::Float(v) => Ok(v as f64),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<f64> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as f64)),
            Value::Uint64(v) => Ok(Some(v as f64)),
            Value::Int32(v) => Ok(Some(v as f64)),
            Value::Int64(v) => Ok(Some(v as f64)),
            Value::Double(v) => Ok(Some(v)),
            Value::Float(v) => Ok(Some(v as f64)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<String> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

impl TryFrom<Value> for Option<bool> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError{}),
        }
    }
}

pub fn new_vehicle_status(data: HashMap<String, Value>, _default_vin: &String) -> VehicleStatus {
    let mut vehicle_status = VehicleStatus::new();
    vehicle_status.created = MessageField::some(Timestamp::now());

    match env::var("SIGNAL_FILTER") {
        Ok(value) => {
            let mut snapshot_data_vec = SnapshotData::new();
            for field in value.split(',') {
                if let Some(measurement) = data.get(field) {
                    let mut entry = KeyValue::new();
                    entry.key = field.to_string();
                    // entry.value = measurement.clone().try_into().unwrap().to_string();
                    entry.value = <datapoint::Value as TryInto<String>>::try_into(measurement.clone()).unwrap().to_string();
                    //entry.value = "123".to_string();
                    snapshot_data_vec.entries.push(entry);
                }
            }
            vehicle_status.snapshot_data = MessageField::some(snapshot_data_vec);
        }
        Err(_) => {
            println!("No filter selected, so no values are forwarded. Configure SIGNAL_FILTER, if you want to get signal");
        }
    }

    vehicle_status
}

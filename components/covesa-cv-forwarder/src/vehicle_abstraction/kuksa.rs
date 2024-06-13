// SPDX-FileCopyrightText: 2024 Contributors to the Eclipse Foundation
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

use self::datapoint::Value;

#[derive(Debug)]
pub struct UnsupportedValueTypeError {}

impl TryFrom<Value> for u32 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for Option<u32> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as u64),
            Value::Uint64(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for Option<u64> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as u64)),
            Value::Uint64(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(v as i32),
            Value::Int32(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for Option<i32> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint32(v) => Ok(Some(v as i32)),
            Value::Int32(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError {}),
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
            _ => Err(UnsupportedValueTypeError {}),
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
            _ => Err(UnsupportedValueTypeError {}),
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
            _ => Err(UnsupportedValueTypeError {}),
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
            _ => Err(UnsupportedValueTypeError {}),
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
            _ => Err(UnsupportedValueTypeError {}),
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
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for Option<String> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

impl TryFrom<Value> for Option<bool> {
    type Error = UnsupportedValueTypeError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(Some(v)),
            _ => Err(UnsupportedValueTypeError {}),
        }
    }
}

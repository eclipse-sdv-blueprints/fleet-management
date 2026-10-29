// SPDX-FileCopyrightText: 2026 Contributors to the Eclipse Foundation
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

use axum::response::IntoResponse;
use axum_extra::extract::QueryRejection;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FmsError {
    #[error("{0}")]
    ParameterValidationError(String),
    #[error("Error retrieving {0} from database")]
    DataAccessError(String),
}

impl From<QueryRejection> for FmsError {
    fn from(rejection: QueryRejection) -> Self {
        FmsError::ParameterValidationError(rejection.body_text())
    }
}

impl IntoResponse for FmsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            FmsError::ParameterValidationError(_) => {
                let error_object = ErrorObject {
                    error: Some("ParameterValidationError".to_string()),
                    error_description: Some(self.to_string()),
                    error_uri: None,
                };
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    axum::Json(error_object),
                )
                    .into_response()
            }
            FmsError::DataAccessError(_) => {
                let error_object = ErrorObject {
                    error: Some("DataAccessError".to_string()),
                    error_description: Some(self.to_string()),
                    error_uri: None,
                };
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(error_object),
                )
                    .into_response()
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_uri: Option<String>,
}

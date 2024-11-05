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

use bytes::Bytes;
use protobuf::{well_known_types::any::Any, Enum, EnumOrUnknown, MessageField};
use up_rust::{
    UAttributes, UAttributesError, UAttributesValidators, UCode, UMessage, UMessageError,
    UMessageType, UPayloadFormat, UPriority, UUri, UUID,
};

pub use cloudevents::{cloud_event::CloudEventAttributeValue, CloudEvent};

include!(concat!(env!("OUT_DIR"), "/cloudevents/mod.rs"));

pub const CONTENT_TYPE_CLOUDEVENTS_PROTOBUF: &str = "application/cloudevents+protobuf";

const CLOUDEVENTS_SPEC_VERSION: &str = "1.0";

const EXTENSION_NAME_COMMSTATUS: &str = "commstatus";
const EXTENSION_NAME_DATA_CONTENT_TYPE: &str = "datacontenttype";
const EXTENSION_NAME_PERMISSION_LEVEL: &str = "plevel";
const EXTENSION_NAME_PRIORITY: &str = "priority";
const EXTENSION_NAME_REQUEST_ID: &str = "reqid";
const EXTENSION_NAME_SINK: &str = "sink";
const EXTENSION_NAME_TTL: &str = "ttl";
const EXTENSION_NAME_TOKEN: &str = "token";
const EXTENSION_NAME_TRACEPARENT: &str = "traceparent";

impl CloudEvent {
    pub fn get_id(&self) -> Result<UUID, UAttributesError> {
        self.id
            .parse::<UUID>()
            .map_err(|e| UAttributesError::parsing_error(e.to_string()))
    }

    pub fn set_id(&mut self, id: &UUID) {
        self.id = id.to_hyphenated_string();
    }

    pub fn get_type(&self) -> Result<UMessageType, UAttributesError> {
        UMessageType::try_from_cloudevent_type(self.type_.clone())
    }

    pub fn set_type(&mut self, type_: UMessageType) {
        self.type_ = type_.to_cloudevent_type();
    }

    pub fn get_source(&self) -> Result<UUri, UAttributesError> {
        self.source
            .parse::<UUri>()
            .map_err(|e| UAttributesError::parsing_error(e.to_string()))
    }

    pub fn set_source<T: Into<String>>(&mut self, uri: T) {
        self.source = uri.into();
    }

    pub fn get_sink(&self) -> Result<Option<UUri>, UAttributesError> {
        self.attributes
            .get(EXTENSION_NAME_SINK)
            .map(|v| v.ce_uri_ref())
            .map_or(Ok(None), |uri| {
                uri.parse::<UUri>()
                    .map(Option::Some)
                    .map_err(|e| UAttributesError::parsing_error(e.to_string()))
            })
    }

    pub fn set_sink<T: Into<String>>(&mut self, uri: T) {
        let mut val = CloudEventAttributeValue::new();
        val.set_ce_uri_ref(uri.into());
        self.attributes.insert(EXTENSION_NAME_SINK.to_string(), val);
    }

    pub fn get_priority(&self) -> Result<UPriority, UAttributesError> {
        self.attributes
            .get(EXTENSION_NAME_PRIORITY)
            .map(|v| v.ce_string())
            .map_or(Ok(UPriority::default()), |v| {
                UPriority::try_from_priority_code(v)
            })
    }

    pub fn set_priority(&mut self, priority: UPriority) {
        let mut val = CloudEventAttributeValue::new();
        val.set_ce_string(priority.to_priority_code());
        self.attributes
            .insert(EXTENSION_NAME_PRIORITY.to_string(), val);
    }

    pub fn get_ttl(&self) -> Option<u32> {
        self.attributes
            .get(EXTENSION_NAME_TTL)
            .map(|v| v.ce_integer() as u32)
    }

    pub fn set_ttl(&mut self, ttl: u32) -> Result<(), UAttributesError> {
        let v = i32::try_from(ttl).map_err(|e| UAttributesError::parsing_error(e.to_string()))?;
        let mut val = CloudEventAttributeValue::new();
        val.set_ce_integer(v);
        self.attributes.insert(EXTENSION_NAME_TTL.to_string(), val);
        Ok(())
    }

    pub fn get_token(&self) -> Option<String> {
        self.attributes
            .get(EXTENSION_NAME_TOKEN)
            .map(|val| val.ce_string().to_string())
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        let mut val = CloudEventAttributeValue::new();
        val.set_ce_string(token.into());
        self.attributes
            .insert(EXTENSION_NAME_TOKEN.to_string(), val);
    }

    pub fn get_permission_level(&self) -> Option<u32> {
        self.attributes
            .get(EXTENSION_NAME_PERMISSION_LEVEL)
            .map(|v| v.ce_integer() as u32)
    }

    pub fn set_permission_level(&mut self, level: u32) -> Result<(), UAttributesError> {
        let v = i32::try_from(level).map_err(|e| UAttributesError::parsing_error(e.to_string()))?;
        let mut val = CloudEventAttributeValue::new();
        val.set_ce_integer(v);
        self.attributes
            .insert(EXTENSION_NAME_PERMISSION_LEVEL.to_string(), val);
        Ok(())
    }

    pub fn get_request_id(&self) -> Result<Option<UUID>, UAttributesError> {
        self.attributes
            .get(EXTENSION_NAME_REQUEST_ID)
            .map(|v| v.ce_string().to_owned())
            .map_or(Ok(None), |v| {
                v.parse::<UUID>()
                    .map(Option::Some)
                    .map_err(|e| UAttributesError::parsing_error(e.to_string()))
            })
    }

    pub fn set_request_id(&mut self, id: &UUID) {
        let mut val = CloudEventAttributeValue::new();
        val.set_ce_string(id.to_hyphenated_string());
        self.attributes
            .insert(EXTENSION_NAME_REQUEST_ID.to_string(), val);
    }

    pub fn get_commstatus(&self) -> Option<UCode> {
        self.attributes
            .get(EXTENSION_NAME_COMMSTATUS)
            .map(|val| val.ce_integer())
            .and_then(UCode::from_i32)
    }

    pub fn set_commstatus(&mut self, status: UCode) {
        if status != UCode::OK {
            let mut val = CloudEventAttributeValue::new();
            val.set_ce_integer(status.value());
            self.attributes
                .insert(EXTENSION_NAME_COMMSTATUS.to_string(), val);
        }
    }

    pub fn get_traceparent(&self) -> Option<String> {
        self.attributes
            .get(EXTENSION_NAME_TRACEPARENT)
            .map(|val| val.ce_string().to_string())
    }

    pub fn set_traceparent<T: Into<String>>(&mut self, traceparent: T) {
        let mut val = CloudEventAttributeValue::new();
        val.set_ce_string(traceparent.into());
        self.attributes
            .insert(EXTENSION_NAME_TRACEPARENT.to_string(), val);
    }

    pub fn get_data_content_type(&self) -> Result<UPayloadFormat, UAttributesError> {
        self.attributes
            .get(EXTENSION_NAME_DATA_CONTENT_TYPE)
            .map(|val| val.ce_string())
            .map_or(Ok(UPayloadFormat::default()), |mt| {
                UPayloadFormat::from_media_type(mt)
                    .map_err(|e| UAttributesError::parsing_error(e.to_string()))
            })
    }

    pub fn set_data_content_type(
        &mut self,
        format: UPayloadFormat,
    ) -> Result<(), UAttributesError> {
        if format == UPayloadFormat::UPAYLOAD_FORMAT_PROTOBUF_WRAPPED_IN_ANY {
            // no need to set datacontenttype because this is the default payload format defined
            // by uProtocol
            return Ok(());
        }
        if let Some(media_type) = format.to_media_type() {
            let mut val = CloudEventAttributeValue::new();
            val.set_ce_string(media_type);
            self.attributes
                .insert(EXTENSION_NAME_DATA_CONTENT_TYPE.to_string(), val);
            Ok(())
        } else {
            Err(UAttributesError::parsing_error(
                "unsupported payload format",
            ))
        }
    }
}

impl TryFrom<UMessage> for CloudEvent {
    type Error = UMessageError;

    // Converts a uProtocol message into a CloudEvent using the
    // [Protobuf Event Format](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/formats/protobuf-format.md).
    //
    // # Arguments
    //
    // * `message` - The message to create the event from.
    //               Note that the message is not validated against the uProtocol specification before processing.
    //
    // # Returns
    //
    // Returns a CloudEvent protobuf with all information from the uProtocol message mapped as defined by the
    // [uProtocol specification]().
    //
    // # Errors
    //
    // Returns an error if the given message does not contain the necessary information for creating a CloudEvent.
    fn try_from(message: UMessage) -> Result<Self, Self::Error> {
        let Some(attributes) = message.attributes.as_ref() else {
            return Err(UMessageError::AttributesValidationError(
                UAttributesError::ValidationError("message has no attributes".to_string()),
            ));
        };
        let mut event = CloudEvent::new();
        event.spec_version = CLOUDEVENTS_SPEC_VERSION.into();
        if let Some(id) = attributes.id.as_ref() {
            event.set_id(id);
        } else {
            return Err(UMessageError::AttributesValidationError(
                UAttributesError::ValidationError("message has no id".to_string()),
            ));
        }
        if let Ok(message_type) = attributes.type_.enum_value() {
            event.set_type(message_type);
        } else {
            return Err(UMessageError::AttributesValidationError(
                UAttributesError::ValidationError("message has no type".to_string()),
            ));
        }
        if let Some(source) = attributes.source.as_ref() {
            event.set_source(source);
        } else {
            return Err(UMessageError::AttributesValidationError(
                UAttributesError::ValidationError("message has no source address".to_string()),
            ));
        }
        if let Some(sink) = attributes.sink.as_ref() {
            event.set_sink(sink);
        }
        if let Ok(priority) = attributes.priority.enum_value() {
            if priority != UPriority::UPRIORITY_UNSPECIFIED {
                event.set_priority(priority);
            }
        } else {
            return Err(UMessageError::AttributesValidationError(
                UAttributesError::ValidationError("message has unsupported priority".to_string()),
            ));
        }
        if let Some(ttl) = attributes.ttl {
            event.set_ttl(ttl)?;
        }
        if let Some(token) = attributes.token.as_ref() {
            event.set_token(token);
        }
        if let Some(plevel) = attributes.permission_level {
            event.set_permission_level(plevel)?;
        }
        if let Some(reqid) = attributes.reqid.as_ref() {
            event.set_request_id(reqid);
        }
        if let Some(commstatus) = attributes.commstatus.as_ref() {
            event.set_commstatus(commstatus.enum_value_or_default());
        }
        if let Some(traceparent) = attributes.traceparent.as_ref() {
            event.set_traceparent(traceparent);
        }
        if let Some(payload) = message.payload {
            let payload_format = attributes.payload_format.enum_value_or_default();
            event.set_data_content_type(payload_format)?;
            match payload_format {
                UPayloadFormat::UPAYLOAD_FORMAT_PROTOBUF
                | UPayloadFormat::UPAYLOAD_FORMAT_PROTOBUF_WRAPPED_IN_ANY => {
                    let data = Any {
                        value: payload.to_vec(),
                        ..Default::default()
                    };
                    event.set_proto_data(data);
                }
                UPayloadFormat::UPAYLOAD_FORMAT_TEXT | UPayloadFormat::UPAYLOAD_FORMAT_JSON => {
                    let data = String::from_utf8(payload.to_vec())
                        .map(|v| v.to_string())
                        .map_err(|_e| {
                            UMessageError::PayloadError(
                                "failed to transform payload to string".to_string(),
                            )
                        })?;
                    event.set_text_data(data);
                }
                _ => {
                    event.set_binary_data(payload.into());
                }
            }
        }
        Ok(event)
    }
}

impl TryFrom<CloudEvent> for UMessage {
    type Error = UMessageError;

    // Converts a CloudEvent to a uProtocol message.
    //
    // # Arguments
    //
    // * `event` - The CloudEvent to create the message from.
    //
    // # Errors
    //
    // Returns an error if the given event does not contain the necessary information for creating a uProtocol message.
    // Also returns an error if the resulting message is not a valid uProtocol message.
    fn try_from(event: CloudEvent) -> Result<Self, Self::Error> {
        if !CLOUDEVENTS_SPEC_VERSION.eq(&event.spec_version) {
            let msg = format!("expected spec version 1.0 but found {}", event.spec_version);
            return Err(UMessageError::AttributesValidationError(
                UAttributesError::ValidationError(msg),
            ));
        }

        let attributes = UAttributes {
            commstatus: event.get_commstatus().map(EnumOrUnknown::from),
            id: MessageField::from_option(Some(event.get_id()?)),
            type_: EnumOrUnknown::from(event.get_type()?),
            source: MessageField::from_option(Some(event.get_source()?)),
            sink: MessageField::from_option(event.get_sink()?),
            priority: EnumOrUnknown::from(event.get_priority()?),
            ttl: event.get_ttl(),
            permission_level: event.get_permission_level(),
            reqid: MessageField::from_option(event.get_request_id()?),
            token: event.get_token(),
            traceparent: event.get_traceparent(),
            payload_format: event.get_data_content_type().map(EnumOrUnknown::from)?,
            ..Default::default()
        };
        UAttributesValidators::get_validator_for_attributes(&attributes).validate(&attributes)?;

        let payload = if event.has_binary_data() {
            Some(Bytes::copy_from_slice(event.binary_data()))
        } else if event.has_text_data() {
            Some(event.text_data().to_owned().into())
        } else if event.has_proto_data() {
            Some(event.proto_data().value.to_vec().into())
        } else if event
            .attributes
            .contains_key(EXTENSION_NAME_DATA_CONTENT_TYPE)
        {
            return Err(UMessageError::PayloadError(
                "CloudEvent has datacontenttype attribute but no data of any type".to_string(),
            ));
        } else {
            None
        };

        Ok(UMessage {
            attributes: Some(attributes).into(),
            payload,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cloudevents::CloudEvent;
    use protobuf::{well_known_types::wrappers::StringValue, Message};

    use up_rust::UMessageBuilder;

    use super::*;

    const MESSAGE_ID: &str = "00000000-0001-7000-8010-101010101a1a";
    const TOPIC: &str = "//my-vehicle/A81B/1/A9BA";
    const METHOD: &str = "//my-vehicle/A000/2/1";
    const REPLY_TO: &str = "//my-vehicle/A81B/1/0";
    const DESTINATION: &str = "//my-vehicle/A000/2/0";
    const PERMISSION_LEVEL: u32 = 5;
    const PRIORITY: UPriority = UPriority::UPRIORITY_CS4;
    const TTL: u32 = 15_000;
    const TRACEPARENT: &str = "traceparent";
    const DATA: [u8; 4] = [0x00, 0x01, 0x02, 0x03];

    //
    // tests asserting conversion of UMessage -> CloudEvent
    //

    fn assert_standard_cloudevent_attributes(
        event: &CloudEvent,
        message_type: &str,
        source: &str,
        sink: Option<String>,
    ) {
        assert_eq!(event.spec_version, CLOUDEVENTS_SPEC_VERSION);
        assert_eq!(event.type_, message_type);
        assert_eq!(event.id, MESSAGE_ID);
        assert_eq!(event.source.as_str(), source);
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_SINK)
                .map(|v| v.ce_uri_ref().to_owned()),
            sink
        );
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_PRIORITY)
                .map(|v| v.ce_string().to_owned()),
            Some(PRIORITY.to_priority_code())
        );
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_TTL)
                .map(|v| v.ce_integer() as u32),
            Some(TTL),
            "unexpected TTL"
        );
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_TRACEPARENT)
                .map(|v| v.ce_string()),
            Some(TRACEPARENT)
        );
    }

    #[test]
    fn test_try_from_publish_message_succeeds() {
        let message_id = MESSAGE_ID
            .parse::<UUID>()
            .expect("failed to parse message ID");
        let message =
            UMessageBuilder::publish(UUri::from_str(TOPIC).expect("failed to create topic URI"))
                .with_message_id(message_id)
                .with_priority(PRIORITY)
                .with_ttl(TTL)
                .with_traceparent(TRACEPARENT)
                .build_with_payload("test".as_bytes(), UPayloadFormat::UPAYLOAD_FORMAT_TEXT)
                .expect("failed to create message");

        let event =
            CloudEvent::try_from(message).expect("failed to create CloudEvent from UMessage");
        assert_standard_cloudevent_attributes(&event, "pub.v1", TOPIC, None);
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_DATA_CONTENT_TYPE)
                .map(|v| v.ce_string().to_owned()),
            UPayloadFormat::UPAYLOAD_FORMAT_TEXT.to_media_type()
        );
        assert_eq!(event.text_data(), "test");
    }

    #[test]
    fn test_try_from_notification_message_succeeds() {
        let message_id = MESSAGE_ID
            .parse::<UUID>()
            .expect("failed to parse message ID");
        let message = UMessageBuilder::notification(
            UUri::from_str(TOPIC).expect("failed to create source URI"),
            UUri::from_str(DESTINATION).expect("failed to create sink URI"),
        )
        .with_message_id(message_id)
        .with_priority(PRIORITY)
        .with_ttl(TTL)
        .with_traceparent(TRACEPARENT)
        .build_with_payload(
            "{\"count\": 5}".as_bytes(),
            UPayloadFormat::UPAYLOAD_FORMAT_JSON,
        )
        .expect("failed to create message");

        let event =
            CloudEvent::try_from(message).expect("failed to create CloudEvent from UMessage");
        assert_standard_cloudevent_attributes(
            &event,
            "not.v1",
            TOPIC,
            Some(DESTINATION.to_string()),
        );
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_DATA_CONTENT_TYPE)
                .map(|v| v.ce_string().to_owned()),
            UPayloadFormat::UPAYLOAD_FORMAT_JSON.to_media_type()
        );
        assert_eq!(event.text_data(), "{\"count\": 5}");
    }

    #[test]
    fn test_try_from_request_message_succeeds() {
        let mut payload = StringValue::new();
        payload.value = "Hello".into();

        let message_id = MESSAGE_ID
            .parse::<UUID>()
            .expect("failed to parse message ID");
        let token = "my-token";
        let message = UMessageBuilder::request(
            UUri::from_str(METHOD).expect("failed to create sink URI"),
            UUri::from_str(REPLY_TO).expect("failed to create source URI"),
            TTL,
        )
        .with_message_id(message_id)
        .with_priority(PRIORITY)
        .with_permission_level(PERMISSION_LEVEL)
        .with_traceparent(TRACEPARENT)
        .with_token(token)
        .build_with_wrapped_protobuf_payload(&payload)
        .expect("failed to create message");
        let event =
            CloudEvent::try_from(message).expect("failed to create CloudEvent from UMessage");
        assert_standard_cloudevent_attributes(&event, "req.v1", REPLY_TO, Some(METHOD.to_string()));
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_TOKEN)
                .map(|v| v.ce_string()),
            Some(token)
        );
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_PERMISSION_LEVEL)
                .map(|v| v.ce_integer()),
            Some(PERMISSION_LEVEL as i32)
        );
        assert!(!event
            .attributes
            .contains_key(EXTENSION_NAME_DATA_CONTENT_TYPE));
        assert!(!event.has_binary_data());
        assert!(!event.has_text_data());
        let payload_wrapped_in_any = Any::pack(&payload).expect("failed to wrap payload in Any");
        assert_eq!(
            event.proto_data().value,
            Any::pack(&payload_wrapped_in_any)
                .expect("failed to pack payload into Any")
                .value
        );
    }

    #[test]
    fn test_try_from_response_message_succeeds() {
        let mut payload = StringValue::new();
        payload.value = "Hello".into();

        let message_id = MESSAGE_ID
            .parse::<UUID>()
            .expect("failed to parse message ID");
        let request_id = UUID::build();

        let message = UMessageBuilder::response(
            UUri::from_str(REPLY_TO).expect("failed to create sink URI"),
            request_id.clone(),
            UUri::from_str(METHOD).expect("failed to create source URI"),
        )
        .with_message_id(message_id)
        .with_ttl(TTL)
        .with_priority(PRIORITY)
        .with_comm_status(UCode::OK)
        .with_traceparent(TRACEPARENT)
        .build_with_protobuf_payload(&payload)
        .expect("failed to create message");

        let event =
            CloudEvent::try_from(message).expect("failed to create CloudEvent from UMessage");
        assert_standard_cloudevent_attributes(&event, "res.v1", METHOD, Some(REPLY_TO.to_string()));
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_COMMSTATUS)
                .map(|v| v.ce_integer()),
            None
        );
        assert_eq!(
            event
                .attributes
                .get(EXTENSION_NAME_DATA_CONTENT_TYPE)
                .map(|v| v.ce_string().to_owned()),
            UPayloadFormat::UPAYLOAD_FORMAT_PROTOBUF.to_media_type()
        );
        assert!(!event.has_binary_data());
        assert!(!event.has_text_data());
        assert_eq!(
            event.proto_data().value,
            Any::pack(&payload)
                .expect("failed to pack payload into Any")
                .value
        );
    }

    //
    // tests asserting conversion of CloudEvent -> UMessage
    //

    fn assert_standard_umessage_attributes(
        attribs: &UAttributes,
        message_type: UMessageType,
        source: &str,
        sink: Option<String>,
    ) {
        assert_eq!(attribs.type_.enum_value_or_default(), message_type);
        assert_eq!(
            attribs.id.get_or_default().to_hyphenated_string(),
            MESSAGE_ID
        );
        assert_eq!(attribs.source.get_or_default().to_uri(false), source);
        assert_eq!(attribs.sink.as_ref().map(|uuri| uuri.to_uri(false)), sink);
        assert_eq!(
            attribs.priority.enum_value_or_default(),
            UPriority::UPRIORITY_CS4
        );
        assert_eq!(attribs.ttl, Some(TTL));
        assert_eq!(attribs.traceparent, Some(TRACEPARENT.to_string()));
    }

    #[test]
    fn test_try_from_cloudevent_without_sink_fails() {
        let mut event = CloudEvent::new();
        event.spec_version = CLOUDEVENTS_SPEC_VERSION.into();
        event.type_ = UMessageType::UMESSAGE_TYPE_NOTIFICATION.to_cloudevent_type();
        event.id = MESSAGE_ID.into();
        event.source = TOPIC.into();

        assert!(UMessage::try_from(event).is_err());
    }

    #[test]
    fn test_try_from_publish_cloudevent_succeeds() {
        let mut event = CloudEvent::new();
        event.spec_version = CLOUDEVENTS_SPEC_VERSION.into();
        event.set_type(UMessageType::UMESSAGE_TYPE_PUBLISH);
        event.id = MESSAGE_ID.into();
        event.source = TOPIC.into();
        event.set_priority(UPriority::UPRIORITY_CS4);
        event.set_ttl(TTL).expect("failed to set TTL on message");
        event.set_traceparent(TRACEPARENT);
        event
            .set_data_content_type(UPayloadFormat::UPAYLOAD_FORMAT_TEXT)
            .expect("failed to set payload format on message");
        event.set_text_data("test".to_string());

        let umessage =
            UMessage::try_from(event).expect("failed to create UMessage from CloudEvent");
        let attribs = umessage.attributes.get_or_default();
        assert_standard_umessage_attributes(
            attribs,
            UMessageType::UMESSAGE_TYPE_PUBLISH,
            TOPIC,
            None,
        );
        assert_eq!(
            attribs.payload_format.enum_value_or_default(),
            UPayloadFormat::UPAYLOAD_FORMAT_TEXT
        );
        assert_eq!(umessage.payload, Some("test".as_bytes().to_vec().into()))
    }

    #[test]
    fn test_try_from_notification_cloudevent_succeeds() {
        let mut event = CloudEvent::new();
        event.spec_version = CLOUDEVENTS_SPEC_VERSION.into();
        event.set_type(UMessageType::UMESSAGE_TYPE_NOTIFICATION);
        event.id = MESSAGE_ID.into();
        event.source = TOPIC.into();
        event.set_sink(DESTINATION);
        event.set_priority(UPriority::UPRIORITY_CS4);
        event.set_ttl(TTL).expect("failed to set TTL on message");
        event.set_traceparent(TRACEPARENT);
        event
            .set_data_content_type(UPayloadFormat::UPAYLOAD_FORMAT_JSON)
            .expect("failed to set payload format on message");
        event.set_text_data("{\"count\": 5}".to_string());

        let umessage =
            UMessage::try_from(event).expect("failed to create UMessage from CloudEvent");
        let attribs = umessage.attributes.get_or_default();
        assert_standard_umessage_attributes(
            attribs,
            UMessageType::UMESSAGE_TYPE_NOTIFICATION,
            TOPIC,
            Some(DESTINATION.to_string()),
        );
        assert_eq!(
            attribs.payload_format.enum_value_or_default(),
            UPayloadFormat::UPAYLOAD_FORMAT_JSON
        );
        assert_eq!(
            umessage.payload,
            Some("{\"count\": 5}".as_bytes().to_vec().into())
        )
    }

    #[test]
    fn test_try_from_request_cloudevent_succeeds() {
        let mut event = CloudEvent::new();
        event.spec_version = CLOUDEVENTS_SPEC_VERSION.into();
        event.set_type(UMessageType::UMESSAGE_TYPE_REQUEST);
        event.id = MESSAGE_ID.into();
        event.source = REPLY_TO.into();
        event.set_sink(METHOD);
        event.set_priority(UPriority::UPRIORITY_CS4);
        event.set_ttl(TTL).expect("failed to set TTL on message");
        event.set_traceparent(TRACEPARENT);
        event
            .set_permission_level(PERMISSION_LEVEL)
            .expect("failed to set permission level on message");
        event.set_token("my-token");

        let mut payload = StringValue::new();
        payload.value = "Hello".into();
        let payload_wrapped_in_any = Any::pack(&payload).expect("failed to wrap payload in Any");
        let serialized_payload = payload_wrapped_in_any
            .write_to_bytes()
            .expect("failed to serialize payload");
        event.set_proto_data(Any {
            value: serialized_payload.clone(),
            ..Default::default()
        });

        let umessage =
            UMessage::try_from(event).expect("failed to create UMessage from CloudEvent");
        let attribs = umessage.attributes.get_or_default();
        assert_standard_umessage_attributes(
            attribs,
            UMessageType::UMESSAGE_TYPE_REQUEST,
            REPLY_TO,
            Some(METHOD.to_string()),
        );
        assert_eq!(attribs.permission_level, Some(PERMISSION_LEVEL));
        assert_eq!(attribs.token, Some("my-token".to_string()));
        assert_eq!(
            attribs.payload_format.enum_value_or_default(),
            UPayloadFormat::UPAYLOAD_FORMAT_UNSPECIFIED
        );
        assert_eq!(umessage.payload, Some(serialized_payload.into()));
    }

    #[test]
    fn test_try_from_response_cloudevent_succeeds() {
        let request_id = UUID::build();
        let mut event = CloudEvent::new();
        event.spec_version = CLOUDEVENTS_SPEC_VERSION.into();
        event.set_type(UMessageType::UMESSAGE_TYPE_RESPONSE);
        event.id = MESSAGE_ID.into();
        event.source = METHOD.into();
        event.set_sink(REPLY_TO);
        event.set_priority(UPriority::UPRIORITY_CS4);
        event.set_ttl(TTL).expect("failed to set TTL on message");
        event.set_traceparent(TRACEPARENT);
        event.set_request_id(&request_id);
        event.set_commstatus(UCode::OK);
        event
            .set_data_content_type(UPayloadFormat::UPAYLOAD_FORMAT_PROTOBUF)
            .expect("failed to set payload format on message");
        event.set_proto_data(Any {
            value: DATA.to_vec(),
            ..Default::default()
        });

        let umessage =
            UMessage::try_from(event).expect("failed to create UMessage from CloudEvent");
        let attribs = umessage.attributes.get_or_default();
        assert_standard_umessage_attributes(
            attribs,
            UMessageType::UMESSAGE_TYPE_RESPONSE,
            METHOD,
            Some(REPLY_TO.to_string()),
        );
        assert_eq!(attribs.commstatus, None);
        assert_eq!(attribs.reqid, Some(request_id).into());
        assert_eq!(
            attribs.payload_format.enum_value_or_default(),
            UPayloadFormat::UPAYLOAD_FORMAT_PROTOBUF
        );
        assert_eq!(umessage.payload, Some(DATA.to_vec().into()))
    }
}

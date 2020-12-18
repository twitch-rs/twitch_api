#![allow(missing_docs, dead_code)]
//! Holds serializable eventsub stuff
//!

use crate::types;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod user_update;

pub trait EventSubscription: DeserializeOwned + Serialize + PartialEq {
    type Payload: NotificationPayload;

    const VERSION: &'static str;
    const EVENT_TYPE: EventType;

    fn condition(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

pub trait NotificationPayload: Serialize + DeserializeOwned {}

#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct Transport {
    method: TransportMethod,
    callback: String,
    secret: String,
}

#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct TransportResponse {
    method: TransportMethod,
    callback: String,
}

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    Webhook,
}

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
pub enum EventType {
    #[serde(rename = "channel.update")]
    ChannelUpdate,
    #[serde(rename = "user.update")]
    UserUpdate,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// Designates that the subscription is in an operable state and is valid.
    Enabled,
    /// Webhook is pending verification of the callback specified in the subscription creation request.
    WebhookCallbackVerificationPending,
    /// Webhook failed verification of the callback specified in the subscription creation request.
    WebhookCallbackVerificationFailed,
    /// Notification delivery failure rate was too high.
    NotificationFailuresExceeded,
    /// Authorization for user(s) in the condition was revoked.
    AuthorizationRevoked,
    /// A user in the condition of the subscription was removed.
    UserRemoved,
}

#![allow(missing_docs, dead_code)]
//! Holds serializable eventsub stuff
//!

use crate::types;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

pub mod user_update;

pub trait EventSubscription: DeserializeOwned + Serialize + PartialEq {
    type Payload: PartialEq + std::fmt::Debug + DeserializeOwned + Serialize;

    const VERSION: &'static str;
    const EVENT_TYPE: EventType;

    fn condition(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)] // FIXME: Clone?
#[serde(remote = "Self")]
pub enum Response {
    UserUpdateV1(NotificationPayload<user_update::UserUpdateV1>),
}

impl Response {
    pub fn parse(source: &str) -> Result<Response, serde_json::Error> {
        serde_json::from_str(source)
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::convert::TryInto;
        macro_rules! match_event {
            ($response:expr; $($module:ident::$event:ident);* $(;)?) => {
                match (&*$response.s.version, &$response.s.type_) {
                    $(  (<$module::$event as EventSubscription>::VERSION, &<$module::$event as EventSubscription>::EVENT_TYPE) => {
                        Response::$event(NotificationPayload {
                            subscription: $response.s.try_into().map_err(serde::de::Error::custom)?,
                            event: serde_json::from_value($response.e).map_err(serde::de::Error::custom)?,
                        })
                    }  )*
                    (v, e) => return Err(serde::de::Error::custom(format!("could not find a match for version `{}` on event type `{}`", v, e)))
                }
            }
        }
        // match_event!{ &*r.s.version
        //     user_update::UserUpdateV1, EventType::UserUpdate;
        // }
        #[derive(Deserialize, Clone)]
        struct IEventSubscripionInformation {
            condition: serde_json::Value,
            created_at: types::Timestamp,
            id: String,
            transport: TransportResponse,
            #[serde(rename = "type")]
            type_: EventType,
            version: String,
        }
        #[derive(Deserialize)]
        struct IResponse {
            #[serde(rename = "subscription")]
            s: IEventSubscripionInformation,
            #[serde(rename = "event")]
            e: serde_json::Value,
        }

        impl<E: EventSubscription> std::convert::TryFrom<IEventSubscripionInformation>
            for EventSubscriptionInformation<E>
        {
            type Error = serde_json::Error;

            fn try_from(info: IEventSubscripionInformation) -> Result<Self, Self::Error> {
                debug_assert_eq!(info.version, E::VERSION);
                debug_assert_eq!(info.type_, E::EVENT_TYPE);
                Ok(EventSubscriptionInformation {
                    id: info.id,
                    condition: serde_json::from_value(info.condition)?,
                    created_at: info.created_at,
                    transport: info.transport,
                })
            }
        }

        let response = IResponse::deserialize(deserializer).map_err(|e| {
            serde::de::Error::custom(format!("could not deserialize response: {}", e))
        })?;
        Ok(match_event! { response;
            user_update::UserUpdateV1;
        })
    }
}

#[derive(PartialEq, Deserialize, Serialize, Debug)] // FIXME: Clone?
pub struct NotificationPayload<E: EventSubscription> {
    /// Subscription information.
    #[serde(bound = "E: EventSubscription")]
    subscription: EventSubscriptionInformation<E>,
    /// Event information.
    #[serde(bound = "E: EventSubscription")]
    event: <E as EventSubscription>::Payload,
}

#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct EventSubscriptionInformation<E: EventSubscription> {
    /// Your client ID.
    id: String,
    /// Subscription-specific parameters.
    #[serde(bound = "E: EventSubscription")]
    condition: E,
    /// The time the notification was created.
    created_at: types::Timestamp,
    transport: TransportResponse,
}

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
    /// The `user.update` subscription type sends a notification when user updates their account.
    #[serde(rename = "user.update")]
    UserUpdate,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
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

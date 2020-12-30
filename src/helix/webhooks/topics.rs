//! Various topics

use serde::{Deserialize, Serialize};

pub mod users;

/// Subscription payload. Received on events. Enumerates all possible [`NotificationPayload`s](NotificationPayload)
///
/// Use [`Payload::parse`] to construct
#[derive(PartialEq, Debug, Serialize, Deserialize)] // FIXME: Clone?
#[serde(remote = "Self")]
#[allow(clippy::large_enum_variant)]
pub enum Payload {}

impl Payload {
    /// Parse string slice as a [Payload]
    pub fn parse(source: &str) -> Result<Payload, PayloadParseError> {
        serde_json::from_str(source).map_err(Into::into)
    }

    // FIXME: Should not throwaway headers etc
    /// Parse http response as a [Payload].
    pub fn parse_response(source: &http::Response<Vec<u8>>) -> Result<Payload, PayloadParseError> {
        Payload::parse(std::str::from_utf8(source.body())?)
    }
}

/// Errors that can happen when parsing payload
#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum PayloadParseError {
    /// could not parse [`http::Request::body()`] as UTF8
    Utf8Error(#[from] std::str::Utf8Error),
    /// could not parse [`http::Request::body()`] as a [`Payload`]
    DeserializeError(#[from] serde_json::Error),
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::convert::TryInto;
        macro_rules! match_event {
            ($response:expr; $($module:ident::$event:ident);* $(;)?) => {
                #[deny(unreachable_patterns)]
                match (&*$response.s.version, &$response.s.type_) {
                    $(  (<$module::$event as EventSubscription>::VERSION, &<$module::$event as EventSubscription>::EVENT_TYPE) => {
                        Payload::$event(NotificationPayload {
                            subscription: $response.s.try_into().map_err(serde::de::Error::custom)?,
                            event: serde_json::from_value($response.e).map_err(serde::de::Error::custom)?,
                        })
                    }  )*
                    (v, e) => return Err(serde::de::Error::custom(format!("could not find a match for version `{}` on event type `{}`", v, e)))
                }
            }
        }

        #[derive(Deserialize, Clone)]
        #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
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
        #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
        struct IResponse {
            #[serde(rename = "subscription")]
            s: IEventSubscripionInformation,
            #[serde(rename = "event", alias = "challenge")]
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
        #[derive(Deserialize)]
        #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
        #[serde(untagged)]
        enum IIResponse {
            VerificationRequest(VerificationRequest),
            IResponse(IResponse),
        }

        let response = IIResponse::deserialize(deserializer).map_err(|e| {
            serde::de::Error::custom(format!("could not deserialize response: {}", e))
        })?;
        match response {
            IIResponse::VerificationRequest(verification) => {
                Ok(Payload::VerificationRequest(verification))
            }
            IIResponse::IResponse(response) => Ok(match_event! { response;
                channel::ChannelUpdateV1;
                channel::ChannelFollowV1;
                channel::ChannelSubscribeV1;
                channel::ChannelCheerV1;
                channel::ChannelBanV1;
                channel::ChannelUnbanV1;
                channel::ChannelPointsCustomRewardAddV1;
                channel::ChannelPointsCustomRewardUpdateV1;
                channel::ChannelPointsCustomRewardRemoveV1;
                channel::ChannelPointsCustomRewardRedemptionAddV1;
                channel::ChannelPointsCustomRewardRedemptionUpdateV1;
                channel::ChannelHypeTrainBeginV1;
                channel::ChannelHypeTrainProgressV1;
                channel::ChannelHypeTrainEndV1;
                stream::StreamOnlineV1;
                stream::StreamOfflineV1;
                user::UserUpdateV1;
                user::UserAuthorizationRevokeV1;
            }),
        }
    }
}

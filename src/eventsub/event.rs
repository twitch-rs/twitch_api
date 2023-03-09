//! EventSub events and their types
#![allow(deprecated)]
#[cfg(feature = "unsupported")]
pub mod websocket;

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::*;

macro_rules! fill_events {
    ($callback:ident( $($args:tt)* )) => {
        $callback!($($args)*
            channel::ChannelUpdateV1;
            channel::ChannelFollowV1;
            channel::ChannelFollowV2;
            channel::ChannelSubscribeV1;
            channel::ChannelCheerV1;
            channel::ChannelBanV1;
            channel::ChannelUnbanV1;
            channel::ChannelCharityCampaignDonateV1;
            channel::ChannelCharityCampaignProgressV1;
            channel::ChannelCharityCampaignStartV1;
            channel::ChannelCharityCampaignStopV1;
            channel::ChannelPointsCustomRewardAddV1;
            channel::ChannelPointsCustomRewardUpdateV1;
            channel::ChannelPointsCustomRewardRemoveV1;
            channel::ChannelPointsCustomRewardRedemptionAddV1;
            channel::ChannelPointsCustomRewardRedemptionUpdateV1;
            channel::ChannelPollBeginV1;
            channel::ChannelPollProgressV1;
            channel::ChannelPollEndV1;
            channel::ChannelPredictionBeginV1;
            channel::ChannelPredictionProgressV1;
            channel::ChannelPredictionLockV1;
            channel::ChannelPredictionEndV1;
            channel::ChannelShoutoutCreateV1;
            channel::ChannelShoutoutReceiveV1;
            channel::ChannelRaidV1;
            channel::ChannelSubscriptionEndV1;
            channel::ChannelSubscriptionGiftV1;
            channel::ChannelSubscriptionMessageV1;
            channel::ChannelGoalBeginV1;
            channel::ChannelGoalProgressV1;
            channel::ChannelGoalEndV1;
            channel::ChannelHypeTrainBeginV1;
            channel::ChannelHypeTrainProgressV1;
            channel::ChannelHypeTrainEndV1;
            stream::StreamOnlineV1;
            stream::StreamOfflineV1;
            user::UserUpdateV1;
            user::UserAuthorizationGrantV1;
            user::UserAuthorizationRevokeV1;
        )
    };
}

macro_rules! is_thing {
    (@inner $s:expr, $thing:ident; $( $(#[$meta:meta])* $module:ident::$event:ident);* $(;)?) => {
        match $s {
            $( $(#[$meta])* Event::$event(Payload { message : Message::$thing(..), ..}) => true,)*
            _ => false,
        }
    };
    ($s:expr, $thing:ident) => {
        fill_events!(is_thing(@inner $s, $thing;))
    };
}

macro_rules! make_event_type {
    ($enum_docs:literal: pub enum $enum_name:ident {
        $(
            $event_docs:literal:
            $variant_name:ident => $event_name:literal,
        )*
    },
        to_str: $to_str_docs:literal,
        from_str_error: $from_str_error:ident,
    ) => {
        #[doc = $enum_docs]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        #[non_exhaustive]
        pub enum $enum_name {
            $(
                #[doc = concat!("`", $event_name, "`: ", $event_docs)]
                #[serde(rename = $event_name)]
                $variant_name,
            )*
        }

        impl $enum_name {
            #[doc = $to_str_docs]
            pub const fn to_str(&self) -> &'static str {
                use $enum_name::*;
                match self {
                    $($variant_name => $event_name,)*
                }
            }
        }

        impl std::str::FromStr for $enum_name {
            type Err = $from_str_error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use $enum_name::*;
                match s {
                    $($event_name => Ok($variant_name),)*
                    _ => Err($from_str_error),
                }
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.to_str())
            }
        }
    };
}

/// Error when parsing an event-type string.
#[derive(thiserror::Error, Debug, Clone)]
#[error("Unknown event type")]
pub struct EventTypeParseError;

make_event_type!("Event Types": pub enum EventType {
    "a user donates to the broadcaster’s charity campaign.":
    ChannelCharityCampaignDonate => "channel.charity_campaign.donate",
    "progress is made towards the campaign’s goal or when the broadcaster changes the fundraising goal.":
    ChannelCharityCampaignProgress => "channel.charity_campaign.progress",
    "a broadcaster starts a charity campaign.":
    ChannelCharityCampaignStart => "channel.charity_campaign.start",
    "a broadcaster stops a charity campaign.":
    ChannelCharityCampaignStop => "channel.charity_campaign.stop",
    "subscription type sends notifications when a broadcaster updates the category, title, mature flag, or broadcast language for their channel.":
    ChannelUpdate => "channel.update",
    "a specified channel receives a follow.":
    ChannelFollow => "channel.follow",
    "a specified channel receives a subscriber. This does not include resubscribes.":
    ChannelSubscribe => "channel.subscribe",
    "a user cheers on the specified channel.":
    ChannelCheer => "channel.cheer",
    "a viewer is banned from the specified channel.":
    ChannelBan => "channel.ban",
    "a viewer is unbanned from the specified channel.":
    ChannelUnban => "channel.unban",
    "a custom channel points reward has been created for the specified channel.":
    ChannelPointsCustomRewardAdd => "channel.channel_points_custom_reward.add",
    "a custom channel points reward has been updated for the specified channel.":
    ChannelPointsCustomRewardUpdate => "channel.channel_points_custom_reward.update",
    "a custom channel points reward has been removed from the specified channel.":
    ChannelPointsCustomRewardRemove => "channel.channel_points_custom_reward.remove",
    "a viewer has redeemed a custom channel points reward on the specified channel.":
    ChannelPointsCustomRewardRedemptionAdd => "channel.channel_points_custom_reward_redemption.add",
    "a redemption of a channel points custom reward has been updated for the specified channel.":
    ChannelPointsCustomRewardRedemptionUpdate => "channel.channel_points_custom_reward_redemption.update",
    "a poll begins on the specified channel.":
    ChannelPollBegin => "channel.poll.begin",
    "a user responds to a poll on the specified channel.":
    ChannelPollProgress => "channel.poll.progress",
    "a poll ends on the specified channel.":
    ChannelPollEnd => "channel.poll.end",
    "a Prediction begins on the specified channel":
    ChannelPredictionBegin => "channel.prediction.begin",
    "a user participates in a Prediction on the specified channel.":
    ChannelPredictionProgress => "channel.prediction.progress",
    "a Prediction is locked on the specified channel.":
    ChannelPredictionLock => "channel.prediction.lock",
    "a Prediction ends on the specified channel.":
    ChannelPredictionEnd => "channel.prediction.end",
    "a specified broadcaster sends a Shoutout.":
    ChannelShoutoutCreate => "channel.shoutout.create",
    "a specified broadcaster receives a Shoutout.":
    ChannelShoutoutReceive => "channel.shoutout.receive",
    "a broadcaster raids another broadcaster’s channel.":
    ChannelRaid => "channel.raid",
    "a subscription to the specified channel expires.":
    ChannelSubscriptionEnd => "channel.subscription.end",
    "a user gives one or more gifted subscriptions in a channel.":
    ChannelSubscriptionGift => "channel.subscription.gift",
    "a user sends a resubscription chat message in a specific channel":
    ChannelSubscriptionMessage => "channel.subscription.message",
    "a goal begins on the specified channel.":
    ChannelGoalBegin => "channel.goal.begin",
    "a goal makes progress on the specified channel.":
    ChannelGoalProgress => "channel.goal.progress",
    "a goal ends on the specified channel.":
    ChannelGoalEnd => "channel.goal.end",
    "a hype train begins on the specified channel.":
    ChannelHypeTrainBegin => "channel.hype_train.begin",
    "a hype train makes progress on the specified channel.":
    ChannelHypeTrainProgress => "channel.hype_train.progress",
    "a hype train ends on the specified channel.":
    ChannelHypeTrainEnd => "channel.hype_train.end",
    "the specified broadcaster starts a stream.":
    StreamOnline => "stream.online",
    "the specified broadcaster stops a stream.":
    StreamOffline => "stream.offline",
    "user updates their account.":
    UserUpdate => "user.update",
    "a user has revoked authorization for your client id. Use this webhook to meet government requirements for handling user data, such as GDPR, LGPD, or CCPA.":
    UserAuthorizationRevoke => "user.authorization.revoke",
    "a user’s authorization has been granted to your client id.":
    UserAuthorizationGrant => "user.authorization.grant",
},
    to_str: r#"Get the event string of this event.
```
# use twitch_api::eventsub::EventType;
fn main() {
    assert_eq!(EventType::ChannelUpdate.to_str(), "channel.update");
    assert_eq!(EventType::ChannelUnban.to_str(), "channel.unban");
}
```"#,
    from_str_error: EventTypeParseError,
);

/// A notification with an event payload. Enumerates all possible [`Payload`s](Payload)
///
/// Parse with [`Event::parse`] or parse the whole http request your server receives with [`Payload::parse_http`]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum Event {
    /// Channel Charity Campaign Donate V1 Event
    ChannelCharityCampaignDonateV1(Payload<channel::ChannelCharityCampaignDonateV1>),
    /// Channel Charity Campaign Progress V1 Event
    ChannelCharityCampaignProgressV1(Payload<channel::ChannelCharityCampaignProgressV1>),
    /// Channel Charity Campaign Start V1 Event
    ChannelCharityCampaignStartV1(Payload<channel::ChannelCharityCampaignStartV1>),
    /// Channel Charity Campaign Stop V1 Event
    ChannelCharityCampaignStopV1(Payload<channel::ChannelCharityCampaignStopV1>),
    /// Channel Update V1 Event
    ChannelUpdateV1(Payload<channel::ChannelUpdateV1>),
    /// Channel Follow V1 Event
    #[deprecated(note = "use `Event::ChannelFollowV2` instead")]
    ChannelFollowV1(Payload<channel::ChannelFollowV1>),
    /// Channel Follow V2 Event
    ChannelFollowV2(Payload<channel::ChannelFollowV2>),
    /// Channel Subscribe V1 Event
    ChannelSubscribeV1(Payload<channel::ChannelSubscribeV1>),
    /// Channel Cheer V1 Event
    ChannelCheerV1(Payload<channel::ChannelCheerV1>),
    /// Channel Ban V1 Event
    ChannelBanV1(Payload<channel::ChannelBanV1>),
    /// Channel Unban V1 Event
    ChannelUnbanV1(Payload<channel::ChannelUnbanV1>),
    /// Channel Points Custom Reward Add V1 Event
    ChannelPointsCustomRewardAddV1(Payload<channel::ChannelPointsCustomRewardAddV1>),
    /// Channel Points Custom Reward Update V1 Event
    ChannelPointsCustomRewardUpdateV1(Payload<channel::ChannelPointsCustomRewardUpdateV1>),
    /// Channel Points Custom Reward Remove V1 Event
    ChannelPointsCustomRewardRemoveV1(Payload<channel::ChannelPointsCustomRewardRemoveV1>),
    /// Channel Points Custom Reward Redemption Add V1 Event
    ChannelPointsCustomRewardRedemptionAddV1(
        Payload<channel::ChannelPointsCustomRewardRedemptionAddV1>,
    ),
    /// Channel Points Custom Reward Redemption Update V1 Event
    ChannelPointsCustomRewardRedemptionUpdateV1(
        Payload<channel::ChannelPointsCustomRewardRedemptionUpdateV1>,
    ),
    /// Channel Poll Begin V1 Event
    ChannelPollBeginV1(Payload<channel::ChannelPollBeginV1>),
    /// Channel Poll Progress V1 Event
    ChannelPollProgressV1(Payload<channel::ChannelPollProgressV1>),
    /// Channel Poll End V1 Event
    ChannelPollEndV1(Payload<channel::ChannelPollEndV1>),
    /// Channel Prediction Begin V1 Event
    ChannelPredictionBeginV1(Payload<channel::ChannelPredictionBeginV1>),
    /// Channel Prediction Progress V1 Event
    ChannelPredictionProgressV1(Payload<channel::ChannelPredictionProgressV1>),
    /// Channel Prediction Lock V1 Event
    ChannelPredictionLockV1(Payload<channel::ChannelPredictionLockV1>),
    /// Channel Prediction End V1 Event
    ChannelPredictionEndV1(Payload<channel::ChannelPredictionEndV1>),
    /// Channel Shoutout Create V1 Event
    ChannelShoutoutCreateV1(Payload<channel::ChannelShoutoutCreateV1>),
    /// Channel Shoutout Receive V1 Event
    ChannelShoutoutReceiveV1(Payload<channel::ChannelShoutoutReceiveV1>),
    /// Channel Goal Begin V1 Event
    ChannelGoalBeginV1(Payload<channel::ChannelGoalBeginV1>),
    /// Channel Goal Progress V1 Event
    ChannelGoalProgressV1(Payload<channel::ChannelGoalProgressV1>),
    /// Channel Goal End V1 Event
    ChannelGoalEndV1(Payload<channel::ChannelGoalEndV1>),
    /// Channel Hype Train Begin V1 Event
    ChannelHypeTrainBeginV1(Payload<channel::ChannelHypeTrainBeginV1>),
    /// Channel Hype Train Progress V1 Event
    ChannelHypeTrainProgressV1(Payload<channel::ChannelHypeTrainProgressV1>),
    /// Channel Hype Train End V1 Event
    ChannelHypeTrainEndV1(Payload<channel::ChannelHypeTrainEndV1>),
    /// StreamOnline V1 Event
    StreamOnlineV1(Payload<stream::StreamOnlineV1>),
    /// StreamOffline V1 Event
    StreamOfflineV1(Payload<stream::StreamOfflineV1>),
    /// User Update V1 Event
    UserUpdateV1(Payload<user::UserUpdateV1>),
    /// User Authorization Grant V1 Event
    UserAuthorizationGrantV1(Payload<user::UserAuthorizationGrantV1>),
    /// User Authorization Revoke V1 Event
    UserAuthorizationRevokeV1(Payload<user::UserAuthorizationRevokeV1>),
    /// Channel Raid V1 Event
    ChannelRaidV1(Payload<channel::ChannelRaidV1>),
    /// Channel Subscription End V1 Event
    ChannelSubscriptionEndV1(Payload<channel::ChannelSubscriptionEndV1>),
    /// Channel Subscription Gift V1 Event
    ChannelSubscriptionGiftV1(Payload<channel::ChannelSubscriptionGiftV1>),
    /// Channel Subscription Message V1 Event
    ChannelSubscriptionMessageV1(Payload<channel::ChannelSubscriptionMessageV1>),
}

impl Event {
    /// Parse string slice as an [`Event`]. Consider using [`Event::parse_http`] instead.
    pub fn parse(source: &str) -> Result<Event, PayloadParseError> {
        let (version, ty, message_type) =
            get_version_event_type_and_message_type_from_text(source)?;
        Self::parse_request(version, &ty, message_type, source.as_bytes().into())
    }

    /// Returns `true` if the message in the [`Payload`] is [`Notification`].
    ///
    /// [`Notification`]: Message::Notification
    pub fn is_notification(&self) -> bool { is_thing!(self, Notification) }

    /// Returns `true` if the message in the [`Payload`] is [`Revocation`].
    ///
    /// [`Revocation`]: Message::Revocation
    pub fn is_revocation(&self) -> bool { is_thing!(self, Revocation) }

    /// Returns `true` if the message in the [`Payload`] is [`VerificationRequest`].
    ///
    /// [`VerificationRequest`]: Message::VerificationRequest
    pub fn is_verification_request(&self) -> bool { is_thing!(self, VerificationRequest) }

    /// If this event is a [`VerificationRequest`], return the [`VerificationRequest`] message, including the message.
    #[rustfmt::skip]
    pub fn get_verification_request(&self) -> Option<&VerificationRequest> {
        macro_rules! match_event {
            ($($(#[$meta:meta])* $module:ident::$event:ident);* $(;)?) => {{

                #[deny(unreachable_patterns)]
                match &self {
                    $(  $(#[$meta])* Event::$event(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),)*
                    _ => None,
                }
            }}
        }
        fill_events!(match_event())
    }

    /// Make a [`EventSubSubscription`] from this notification.
    pub fn subscription(&self) -> Result<EventSubSubscription, serde_json::Error> {
        macro_rules! match_event {
            ($($(#[$meta:meta])* $module:ident::$event:ident);* $(;)?) => {{
                match &self {
                    $(
                        $(#[$meta])*
                        Event::$event(notif) => Ok({
                            let self::Payload {subscription, ..} = notif; // FIXME: Use @ pattern-binding, currently stable

                            EventSubSubscription {
                            cost: subscription.cost,
                            condition: subscription.condition.condition()?,
                            created_at: subscription.created_at.clone(),
                            id: subscription.id.clone(),
                            status: subscription.status.clone(),
                            transport: subscription.transport.clone(),
                            type_: notif.get_event_type(),
                            version: notif.get_event_version().to_owned(),
                        }}),
                    )*
                }
            }}
        }

        fill_events!(match_event())
    }

    /// Verify that this event is authentic using `HMAC-SHA256`.
    ///
    /// HMAC key is `secret`, HMAC message is a concatenation of `Twitch-Eventsub-Message-Id` header, `Twitch-Eventsub-Message-Timestamp` header and the request body.
    /// HMAC signature is `Twitch-Eventsub-Message-Signature` header.
    #[cfg(feature = "hmac")]
    #[cfg_attr(nightly, doc(cfg(feature = "hmac")))]
    #[must_use]
    pub fn verify_payload<B>(request: &http::Request<B>, secret: &[u8]) -> bool
    where B: AsRef<[u8]> {
        use crypto_hmac::{Hmac, Mac};

        fn message_and_signature<B>(request: &http::Request<B>) -> Option<(Vec<u8>, Vec<u8>)>
        where B: AsRef<[u8]> {
            static SHA_HEADER: &str = "sha256=";

            let id = request
                .headers()
                .get("Twitch-Eventsub-Message-Id")?
                .as_bytes();
            let timestamp = request
                .headers()
                .get("Twitch-Eventsub-Message-Timestamp")?
                .as_bytes();
            let body = request.body().as_ref();

            let mut message = Vec::with_capacity(id.len() + timestamp.len() + body.len());
            message.extend_from_slice(id);
            message.extend_from_slice(timestamp);
            message.extend_from_slice(body);

            let signature = request
                .headers()
                .get("Twitch-Eventsub-Message-Signature")?
                .to_str()
                .ok()?;
            if !signature.starts_with(SHA_HEADER) {
                return None;
            }
            let signature = signature.split_at(SHA_HEADER.len()).1;
            if signature.len() % 2 == 0 {
                // Convert signature to [u8] from hex digits
                // Hex decode inspired by https://stackoverflow.com/a/52992629
                let signature = ((0..signature.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&signature[i..i + 2], 16))
                    .collect::<Result<Vec<u8>, _>>())
                .ok()?;

                Some((message, signature))
            } else {
                None
            }
        }

        if let Some((message, signature)) = message_and_signature(request) {
            let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret).expect("");
            mac.update(&message);
            mac.verify(crypto_hmac::digest::generic_array::GenericArray::from_slice(&signature))
                .is_ok()
        } else {
            false
        }
    }
}

/// Helper function to get version and type of event from text.
#[allow(clippy::type_complexity)]
fn get_version_event_type_and_message_type_from_text(
    source: &str,
) -> Result<(Cow<'_, str>, EventType, Cow<'_, [u8]>), PayloadParseError> {
    #[derive(Deserialize)]
    struct IEventSubscripionInformation {
        // condition: serde_json::Value,
        // created_at: types::Timestamp,
        // status: Status,
        // cost: usize,
        // id: types::EventSubId,
        // transport: TransportResponse,
        #[serde(rename = "type")]
        type_: EventType,
        version: String,
    }
    #[derive(Deserialize)]
    struct IEvent {
        subscription: IEventSubscripionInformation,
        challenge: Option<serde_json::Value>,
        event: Option<serde_json::Value>,
    }

    let IEvent {
        subscription,
        challenge,
        event,
    } = parse_json(source, false)?;
    // FIXME: A visitor is really what we want.
    if event.is_some() {
        Ok((
            subscription.version.into(),
            subscription.type_,
            Cow::Borrowed(b"notification"),
        ))
    } else if challenge.is_some() {
        Ok((
            subscription.version.into(),
            subscription.type_,
            Cow::Borrowed(b"webhook_callback_verification"),
        ))
    } else {
        Ok((
            subscription.version.into(),
            subscription.type_,
            Cow::Borrowed(b"revocation"),
        ))
    }
}

/// Helper function to get version and type of event from http.
#[allow(clippy::type_complexity)]
fn get_version_event_type_and_message_type_from_http<B>(
    request: &http::Request<B>,
) -> Result<(Cow<'_, str>, EventType, Cow<'_, [u8]>), PayloadParseError>
where B: AsRef<[u8]> {
    use serde::de::IntoDeserializer;
    match (
        request
            .headers()
            .get("Twitch-Eventsub-Subscription-Type")
            .map(|v| v.as_bytes())
            .map(std::str::from_utf8)
            .transpose()?,
        request
            .headers()
            .get("Twitch-Eventsub-Subscription-Version")
            .map(|v| v.as_bytes())
            .map(std::str::from_utf8)
            .transpose()?,
        request
            .headers()
            .get("Twitch-Eventsub-Message-Type")
            .map(|v| v.as_bytes()),
    ) {
        (Some(ty), Some(version), Some(message_type)) => Ok((
            version.into(),
            EventType::deserialize(ty.into_deserializer()).map_err(
                |_: serde::de::value::Error| PayloadParseError::UnknownEventType(ty.to_owned()),
            )?,
            message_type.into(),
        )),
        (..) => Err(PayloadParseError::MalformedEvent),
    }
}

impl Event {
    /// Parse a http payload as an [`Event`]
    ///
    /// Create the webhook via [`CreateEventSubSubscription`](crate::helix::eventsub::CreateEventSubSubscriptionRequest) according to the [Eventsub WebHooks guide](https://dev.twitch.tv/docs/eventsub/handling-webhook-events)
    pub fn parse_http<B>(request: &http::Request<B>) -> Result<Event, PayloadParseError>
    where B: AsRef<[u8]> {
        let (version, ty, message_type) =
            get_version_event_type_and_message_type_from_http(request)?;
        let source = request.body().as_ref().into();
        Self::parse_request(version, &ty, message_type, source)
    }

    /// Parse a string slice as an [`Event`]. You should not use this, instead, use [`Event::parse_http`] or [`Event::parse`].
    #[doc(hidden)]
    pub fn parse_request<'a>(
        version: Cow<'a, str>,
        event_type: &'a EventType,
        message_type: Cow<'a, [u8]>,
        source: Cow<'a, [u8]>,
    ) -> Result<Event, PayloadParseError> {
        /// Match on all defined eventsub types.
        ///
        /// If this is not done, we'd get a much worse error message.
        macro_rules! match_event {
            ($($(#[$meta:meta])* $module:ident::$event:ident);* $(;)?) => {{

                #[deny(unreachable_patterns)]
                match (version.as_ref(), event_type) {
                    $(  $(#[$meta])* (<$module::$event as EventSubscription>::VERSION, &<$module::$event as EventSubscription>::EVENT_TYPE) => {
                        Event::$event(Payload::parse_request(message_type, source)?)
                    }  )*
                    (v, e) => return Err(PayloadParseError::UnimplementedEvent{version: v.to_owned(), event_type: e.clone()})
                }
            }}
        }

        Ok(fill_events!(match_event()))
    }

    /// Parse a websocket frame as an [`EventsubWebsocketData`]
    ///
    /// Create the websocket via [`CreateEventSubSubscription`](crate::helix::eventsub::CreateEventSubSubscriptionRequest) according to the [Eventsub WebSocket guide](https://dev.twitch.tv/docs/eventsub/handling-websocket-events)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::eventsub::{Event, EventsubWebsocketData};
    /// let notification = r#"
    /// {
    ///     "metadata": {
    ///         "message_id": "befa7b53-d79d-478f-86b9-120f112b044e",
    ///         "message_type": "notification",
    ///         "message_timestamp": "2019-11-16T10:11:12.123Z",
    ///         "subscription_type": "channel.follow",
    ///         "subscription_version": "1"
    ///     },
    ///     "payload": {
    ///         "subscription": {
    ///             "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
    ///             "status": "enabled",
    ///             "type": "channel.follow",
    ///             "version": "1",
    ///             "cost": 1,
    ///             "condition": {
    ///                 "broadcaster_user_id": "12826"
    ///             },
    ///             "transport": {
    ///                 "method": "websocket",
    ///                 "session_id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB"
    ///             },
    ///             "created_at": "2019-11-16T10:11:12.123Z"
    ///         },
    ///         "event": {
    ///             "user_id": "1337",
    ///             "user_login": "awesome_user",
    ///             "user_name": "Awesome_User",
    ///             "broadcaster_user_id": "12826",
    ///             "broadcaster_user_login": "twitch",
    ///             "broadcaster_user_name": "Twitch",
    ///             "followed_at": "2020-07-15T18:16:11.17106713Z"
    ///         }
    ///     }
    /// }
    /// "#;
    /// let event: EventsubWebsocketData<'_> =
    ///     Event::parse_websocket(notification)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg(feature = "unsupported")]
    pub fn parse_websocket(frame: &str) -> Result<EventsubWebsocketData<'_>, PayloadParseError> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct EventsubWebsocketFrame<'a> {
            metadata: EventsubWebsocketMetadata<'a>,
            #[serde(borrow)]
            payload: &'a serde_json::value::RawValue,
        }

        let frame: EventsubWebsocketFrame = crate::parse_json(frame, true)?;

        macro_rules! match_event {
            ($metadata:expr, $message_type:literal, $($(#[$meta:meta])* $module:ident::$event:ident);* $(;)?) => {{

                #[deny(unreachable_patterns)]
                match ($metadata.subscription_version.as_ref(), &$metadata.subscription_type) {
                    $(  $(#[$meta])* (<$module::$event as EventSubscription>::VERSION, &<$module::$event as EventSubscription>::EVENT_TYPE) => {
                        Event::$event(Payload::parse_request_str($message_type.as_ref(), frame.payload.get())?)
                    }  )*
                    (v, e) => return Err(PayloadParseError::UnimplementedEvent{version: v.to_owned(), event_type: e.clone()})
                }
            }}
        }

        match frame.metadata {
            EventsubWebsocketMetadata::Notification(notification) => {
                let event = fill_events!(match_event(notification, "notification",));
                Ok(EventsubWebsocketData::Notification {
                    metadata: notification,
                    payload: event,
                })
            }
            EventsubWebsocketMetadata::Revocation(revocation) => {
                let event = fill_events!(match_event(revocation, "revocation",));
                Ok(EventsubWebsocketData::Revocation {
                    metadata: revocation,
                    payload: event,
                })
            }
            EventsubWebsocketMetadata::Welcome(welcome) => Ok(EventsubWebsocketData::Welcome {
                metadata: welcome,
                payload: crate::parse_json(frame.payload.get(), true)?,
            }),
            EventsubWebsocketMetadata::Keepalive(keepalive) => {
                Ok(EventsubWebsocketData::Keepalive {
                    metadata: keepalive,
                    payload: (),
                })
            }
            EventsubWebsocketMetadata::Reconnect(reconnect) => {
                Ok(EventsubWebsocketData::Reconnect {
                    metadata: reconnect,
                    payload: crate::parse_json(frame.payload.get(), true)?,
                })
            }
        }
    }
}

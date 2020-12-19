//! Holds serializable eventsub stuff
//!

use crate::types;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

pub mod channel_ban;
pub mod channel_cheer;
pub mod channel_follow;
pub mod channel_points_custom_reward;
pub mod channel_points_custom_reward_redemption;
pub mod channel_subscribe;
pub mod channel_unban;
pub mod channel_update;
pub mod user_update;

/// An EventSub subscription.
pub trait EventSubscription: DeserializeOwned + Serialize + PartialEq {
    /// Payload for given subscription
    type Payload: PartialEq + std::fmt::Debug + DeserializeOwned + Serialize;

    /// Scopes needed by this subscription
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope];
    /// Optional scopes needed by this subscription
    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    /// Subscription type version
    const VERSION: &'static str;
    /// Subscription type name.
    const EVENT_TYPE: EventType;

    /// Creates the [`condition`](https://dev.twitch.tv/docs/eventsub/eventsub-reference#conditions) for this EventSub subscription
    fn condition(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
/// Subscription payload. Received on events. Enumerates all possible [`NotificationPayload`s](NotificationPayload)
///
/// Use [Payload::parse] to construct
#[derive(PartialEq, Debug, Serialize, Deserialize)] // FIXME: Clone?
#[serde(remote = "Self")]
#[allow(clippy::large_enum_variant)]
pub enum Payload {
    /// User Update V1 Event
    UserUpdateV1(NotificationPayload<user_update::UserUpdateV1>),
    /// Channel Update V1 Event
    ChannelUpdateV1(NotificationPayload<channel_update::ChannelUpdateV1>),
    /// Channel Follow V1 Event
    ChannelFollowV1(NotificationPayload<channel_follow::ChannelFollowV1>),
    /// Channel Subscribe V1 Event
    ChannelSubscribeV1(NotificationPayload<channel_subscribe::ChannelSubscribeV1>),
    /// Channel Cheer V1 Event
    ChannelCheerV1(NotificationPayload<channel_cheer::ChannelCheerV1>),
    /// Channel Ban V1 Event
    ChannelBanV1(NotificationPayload<channel_ban::ChannelBanV1>),
    /// Channel Unban V1 Event
    ChannelUnbanV1(NotificationPayload<channel_unban::ChannelUnbanV1>),
    /// Channel Points Custom Reward Add V1 Event
    ChannelPointsCustomRewardAddV1(
        NotificationPayload<channel_points_custom_reward::ChannelPointsCustomRewardAddV1>,
    ),
    /// Channel Points Custom Reward Update V1 Event
    ChannelPointsCustomRewardUpdateV1(
        NotificationPayload<channel_points_custom_reward::ChannelPointsCustomRewardUpdateV1>,
    ),
    /// Channel Points Custom Reward Remove V1 Event
    ChannelPointsCustomRewardRemoveV1(
        NotificationPayload<channel_points_custom_reward::ChannelPointsCustomRewardRemoveV1>,
    ),
    /// Channel Points Custom Reward Redemption Add V1 Event
    ChannelPointsCustomRewardRedemptionAddV1(
        NotificationPayload<
            channel_points_custom_reward_redemption::ChannelPointsCustomRewardRedemptionAddV1,
        >,
    ),
    /// Channel Points Custom Reward Redemption Update V1 Event
    ChannelPointsCustomRewardRedemptionUpdateV1(
        NotificationPayload<
            channel_points_custom_reward_redemption::ChannelPointsCustomRewardRedemptionUpdateV1,
        >,
    ),
}

impl Payload {
    /// Parse string slice as a [Payload]
    pub fn parse(source: &str) -> Result<Payload, serde_json::Error> {
        serde_json::from_str(source)
    }
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
            channel_update::ChannelUpdateV1;
            channel_follow::ChannelFollowV1;
            channel_subscribe::ChannelSubscribeV1;
            channel_cheer::ChannelCheerV1;
            channel_ban::ChannelBanV1;
            channel_unban::ChannelUnbanV1;
            channel_points_custom_reward::ChannelPointsCustomRewardAddV1;
            channel_points_custom_reward::ChannelPointsCustomRewardUpdateV1;
            channel_points_custom_reward::ChannelPointsCustomRewardRemoveV1;
            channel_points_custom_reward_redemption::ChannelPointsCustomRewardRedemptionAddV1;
            channel_points_custom_reward_redemption::ChannelPointsCustomRewardRedemptionUpdateV1;
            user_update::UserUpdateV1;
        })
    }
}

/// Notification received
#[derive(PartialEq, Deserialize, Serialize, Debug)] // FIXME: Clone?
pub struct NotificationPayload<E: EventSubscription> {
    /// Subscription information.
    #[serde(bound = "E: EventSubscription")]
    subscription: EventSubscriptionInformation<E>,
    /// Event information.
    #[serde(bound = "E: EventSubscription")]
    event: <E as EventSubscription>::Payload,
}

/// Metadata about the subscription.
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

/// Transport setting for event notification
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct Transport {
    method: TransportMethod,
    callback: String,
    secret: String,
}

/// Transport response on event notification
///
/// Does not include secret.
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct TransportResponse {
    method: TransportMethod,
    callback: String,
}

/// Transport method
///
/// Currently, only webhooks are supported
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    /// Webhook
    Webhook,
}

/// Event name
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
pub enum EventType {
    /// The `channel.update` subscription type sends notifications when a broadcaster updates the category, title, mature flag, or broadcast language for their channel.
    #[serde(rename = "channel.update")]
    ChannelUpdate,
    /// The `channel.follow` subscription type sends a notification when a specified channel receives a follow.
    #[serde(rename = "channel.follow")]
    ChannelFollow,
    /// The `channel.subscribe` subscription type sends a notification when a specified channel receives a subscriber. This does not include resubscribes.
    #[serde(rename = "channel.subscribe")]
    ChannelSubscribe,
    /// The `channel.cheer` subscription type sends a notification when a user cheers on the specified channel.
    #[serde(rename = "channel.cheer")]
    ChannelCheer,
    /// The `channel.ban` subscription type sends a notification when a viewer is banned from the specified channel.
    #[serde(rename = "channel.ban")]
    ChannelBan,
    /// The `channel.unban` subscription type sends a notification when a viewer is unbanned from the specified channel.
    #[serde(rename = "channel.unban")]
    ChannelUnban,
    /// The `channel.channel_points_custom_reward.add` subscription type sends a notification when a custom channel points reward has been created for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.add")]
    ChannelPointsCustomRewardAdd,
    /// The `channel.channel_points_custom_reward.update` subscription type sends a notification when a custom channel points reward has been updated for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.update")]
    ChannelPointsCustomRewardUpdate,
    /// The `channel.channel_points_custom_reward.remove` subscription type sends a notification when a custom channel points reward has been removed from the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.remove")]
    ChannelPointsCustomRewardRemove,
    /// The `channel.channel_points_custom_reward_redemption.add` subscription type sends a notification when a viewer has redeemed a custom channel points reward on the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward_redemption.add")]
    ChannelPointsCustomRewardRedemptionAdd,
    /// The `channel.channel_points_custom_reward_redemption.update` subscription type sends a notification when a redemption of a channel points custom reward has been updated for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward_redemption.update")]
    ChannelPointsCustomRewardRedemptionUpdate,
    /// The `user.update` subscription type sends a notification when user updates their account.
    #[serde(rename = "user.update")]
    UserUpdate,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
}

///  Subscription request status
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

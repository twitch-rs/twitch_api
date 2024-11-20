#![doc(alias = "channel.subscription.end")]
//! A subscription to the specified channel expires.

use super::*;
/// [`channel.subscription.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelSubscriptionEnd): a subscription to the specified channel expires.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscriptionEndV1 {
    /// The broadcaster user ID for the channel you want to get subscription end notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelSubscriptionEndV1 {
    /// The broadcaster user ID for the channel you want to get subscription end notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelSubscriptionEndV1 {
    type Payload = ChannelSubscriptionEndV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelSubscriptionEnd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelReadSubscriptions];
    const VERSION: &'static str = "1";
}

/// [`channel.subscription.end`](ChannelSubscriptionEndV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscriptionEndV1Payload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The user ID for the user whose subscription ended.
    pub user_id: types::UserId,
    /// The user login for the user whose subscription ended.
    pub user_login: types::UserName,
    /// The user display name for the user whose subscription ended.
    pub user_name: types::DisplayName,
    /// The tier of the subscription that ended. Valid values are 1000, 2000, and 3000.
    pub tier: types::SubscriptionTier,
    /// Whether the subscription was a gift.
    pub is_gift: bool,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.subscription.end",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
               "broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "user_id": "1234",
            "user_login": "cool_user",
            "user_name": "Cool_User",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cooler_user",
            "broadcaster_user_name": "Cooler_User",
            "tier": "1000",
            "is_gift": false
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

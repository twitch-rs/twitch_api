#![doc(alias = "subscription")]
#![doc(alias = "subscriptions")]
#![doc(alias = "channel.subscribe")]
//! A specified channel receives a subscriber. This does not include resubscribes.
use super::*;

/// [`channel.subscribe`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsubscribe): a specified channel receives a subscriber. This does not include resubscribes.
#[derive(Clone, Debug, typed_builder::TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscribeV1 {
    /// The broadcaster user ID for the channel you want to get subscribe notifications for.
    #[builder(setter(into))]
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelSubscribeV1 {
    type Payload = ChannelSubscribeV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelSubscribe;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelReadSubscriptions];
    const VERSION: &'static str = "1";
}

/// [`channel.subscribe`](ChannelSubscribeV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscribeV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// Whether the subscription is a gift.
    pub is_gift: bool,
    /// The tier of the subscription. Valid values are 1000, 2000, and 3000.
    pub tier: types::SubscriptionTier,
    /// The user ID for the user who subscribed to the specified channel.
    pub user_id: types::UserId,
    /// The user login for the user who subscribed to the specified channel.
    pub user_login: types::UserName,
    /// The user display name for the user who subscribed to the specified channel.
    pub user_name: types::DisplayName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.subscribe",
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
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

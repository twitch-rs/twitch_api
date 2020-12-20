//! A specified channel receives a subscriber. This does not include resubscribes.
use super::*;

/// [`channel.subscribe`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsubscribe): a specified channel receives a subscriber. This does not include resubscribes.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelSubscribeV1 {
    /// The broadcaster user ID for the channel you want to get subscribe notifications for.
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

/// [`channel.update`](ChannelSubscribeV1) response payload.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelSubscribeV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster name.
    pub broadcaster_user_name: types::UserName,
    /// Whether the subscription is a gift.
    pub is_gift: bool,
    /// The tier of the subscription. Valid values are 1000, 2000, and 3000.
    pub tier: types::SubscriptionTier,
    /// The user ID for the user who subscribed to the specified channel.
    pub user_id: types::UserId,
    /// The user name for the user who subscribed to the specified channel.
    pub user_name: types::UserName,
}

#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.subscribe",
            "version": "1",
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
            "user_name": "cool_user",
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "cooler_user",
            "tier": "1000",
            "is_gift": false
        }
    }
    "#;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

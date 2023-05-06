#![doc(alias = "channel.subscription.gift")]
//! A user gives one or more gifted subscriptions in a channel.

use super::*;
/// [`channel.subscription.gift`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsubscriptiongift): a subscription to the specified channel expires.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscriptionGiftV1 {
    /// The broadcaster user ID for the channel you want to get subscription gift notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelSubscriptionGiftV1 {
    /// The broadcaster user ID for the channel you want to get subscription gift notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelSubscriptionGiftV1 {
    type Payload = ChannelSubscriptionGiftV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelSubscriptionGift;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelReadSubscriptions];
    const VERSION: &'static str = "1";
}

/// [`channel.subscription.gift`](ChannelSubscriptionGiftV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscriptionGiftV1Payload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster login.
    pub broadcaster_user_login: types::DisplayName,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::UserName,
    /// The number of subscriptions gifted by this user in the channel. This value is null for anonymous gifts or if the gifter has opted out of sharing this information.
    pub cumulative_total: Option<i64>,
    /// Whether the subscription gift was anonymous.
    pub is_anonymous: bool,
    /// The tier of subscriptions in the subscription gift.
    pub tier: types::SubscriptionTier,
    /// The number of subscriptions in the subscription gift.
    pub total: i64,
    /// The user ID of the user who sent the subscription gift. Set to null if it was an anonymous subscription gift.
    pub user_id: Option<types::UserId>,
    /// The user login of the user who sent the gift. Set to null if it was an anonymous subscription gift.
    pub user_login: Option<types::DisplayName>,
    /// The user display name of the user who sent the gift. Set to null if it was an anonymous subscription gift.
    pub user_name: Option<types::UserName>,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.subscription.gift",
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
            "total": 2,
            "tier": "1000",
            "cumulative_total": 284,
            "is_anonymous": false
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

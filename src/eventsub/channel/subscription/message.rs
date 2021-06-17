#![doc(alias = "channel.subscription.message")]
//! A user sends a resubscription chat message in a specific channel

use super::*;
/// [`channel.subscription.message`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsubscriptionmessage): a subscription to the specified channel expires.
#[derive(Clone, Debug, typed_builder::TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscriptionMessageV1 {
    /// The broadcaster user ID for the channel you want to get resubscription chat message notifications for.
    #[builder(setter(into))]
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelSubscriptionMessageV1 {
    type Payload = ChannelSubscriptionMessageV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelSubscriptionMessage;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelReadSubscriptions];
    const VERSION: &'static str = "1";
}

/// [`channel.subscription.message`](ChannelSubscriptionMessageV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscriptionMessageV1Payload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    // FIXME: Twitch says this is `cumulative_total`, see https://github.com/twitchdev/issues/issues/415
    /// The total number of months the user has been subscribed to the channel.
    pub cumulative_months: i64,
    /// The month duration of the subscription.
    pub duration_months: i64,
    /// An object that contains the resubscription message and emote information needed to recreate the message.
    pub message: SubscriptionMessage,
    /// The number of consecutive months the user’s current subscription has been active. This value is null if the user has opted out of sharing this information.
    pub streak_months: Option<i64>,
    /// The tier of the user’s subscription.
    pub tier: String,
    /// The user ID of the user who sent a resubscription chat message.
    pub user_id: types::UserId,
    /// The user login of the user who sent a resubscription chat message.
    pub user_login: types::UserName,
    /// The user display name of the user who a resubscription chat message.
    pub user_name: types::DisplayName,
}

/// A message attached to a [`ChannelSubscriptionMessageV1Payload`]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SubscriptionMessage {
    /// The text of the resubscription chat message.
    pub text: String,
    /// An array that includes the emote ID and start and end positions for where the emote appears in the text.
    pub emotes: Vec<types::ResubscriptionEmote>,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.subscription.message",
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
            "message": {
                "text": "Love the stream! FevziGG",
                "emotes": [
                    {
                        "begin": 23,
                        "end": 30,
                        "id": "302976485"
                    }
                ]
            },
            "cumulative_months": 15,
            "streak_months": 1,
            "duration_months": 6
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Payload::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

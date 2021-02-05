#![doc(alias = "channel.cheer")]
//! A user cheers on the specified channel.
use super::*;

/// [`channel.cheer`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcheer): a user cheers on the specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelCheerV1 {
    /// The broadcaster user ID for the channel you want to get cheer notifications for.
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelCheerV1 {
    type Payload = ChannelCheerV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelCheer;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::BitsRead];
    const VERSION: &'static str = "1";
}

/// [`channel.cheer`](ChannelCheerV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelCheerV1Payload {
    /// The number of bits cheered.
    pub bits: i64,
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// Whether the user cheered anonymously or not.
    pub is_anonymous: bool,
    /// The message sent with the cheer.
    pub message: String,
    /// The user ID for the user who cheered on the specified channel. This is null/empty if is_anonymous is true.
    pub user_id: Option<types::UserId>,
    /// The user login for the user who cheered on the specified channel. This is null/empty if is_anonymous is true.
    pub user_login: Option<types::UserName>,
    /// The user display name for the user who cheered on the specified channel. This is null/empty if is_anonymous is true.
    pub user_name: Option<types::DisplayName>,
}

#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.cheer",
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
            "is_anonymous": false,
            "user_id": "1234",
            "user_login": "cool_user",
            "user_name": "Cool_User",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cooler_user",
            "broadcaster_user_name": "Cooler_User",
            "message": "pogchamp",
            "bits": 1000
        }
    }
    "#;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

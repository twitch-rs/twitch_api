//! Subscription that sends a notification when a user cheers on the specified channel.
use super::*;

/// The `channel.cheer` subscription type sends a notification when a user cheers on the specified channel.
/// [`channelcheer`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcheer)
#[derive(PartialEq, Deserialize, Serialize, Debug)]
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

/// Response payload for [`channel.cheer` version `1`](ChannelCheerV1) subscription.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelCheerV1Payload {
    /// The number of bits cheered.
    pub bits: i64,
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster name.
    pub broadcaster_user_name: types::UserName,
    /// Whether the user cheered anonymously or not.
    pub is_anonymous: bool,
    /// The message sent with the cheer.
    pub message: String,
    /// The user ID for the user who cheered on the specified channel. This is null/empty if is_anonymous is true.
    pub user_id: Option<types::UserId>,
    /// The user name for the user who cheered on the specified channel. This is null/empty if is_anonymous is true.
    pub user_name: Option<types::UserId>,
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
            "user_name": "cool_user",
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "cooler_user",
            "message": "pogchamp",
            "bits": 1000
        }
    }
    "#;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

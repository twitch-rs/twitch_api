//! A specified channel receives a follow.
use super::*;

/// [`channel.follow`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelfollow): a specified channel receives a follow.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelFollowV1 {
    /// The broadcaster user ID for the channel you want to get follow notifications for.
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelFollowV1 {
    type Payload = ChannelFollowV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelFollow;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const VERSION: &'static str = "1";
}

/// [`channel.follow`](ChannelFollowV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelFollowV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster name.
    pub broadcaster_user_name: types::UserName,
    /// The user ID for the user now following the specified channel.
    pub user_id: types::UserId,
    /// The user name for the user now following the specified channel.
    pub user_name: types::UserName,
}

#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.follow",
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
            "broadcaster_user_name": "cooler_user"
        }
    }
    "#;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

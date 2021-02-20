#![doc(alias = "channel.unban")]
//! A viewer is unbanned from the specified channel.
use super::*;

/// [`channel.unban`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelunban): a viewer is unbanned from the specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelUnbanV1 {
    /// The broadcaster user ID for the channel you want to get unban notifications for.
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelUnbanV1 {
    type Payload = ChannelUnbanV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelUnban;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelModerate];
    const VERSION: &'static str = "1";
}

/// [`channel.unban`](ChannelUnbanV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelUnbanV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The user id for the user who was unbanned on the specified channel.
    pub user_id: types::UserId,
    /// The user login for the user who was unbanned on the specified channel.
    pub user_login: types::UserName,
    /// The user display name for the user who was unbanned on the specified channel.
    pub user_name: types::DisplayName,
    /// The user ID of the issuer of the unban.
    pub moderator_user_id: types::UserId,
    /// The user login of the issuer of the unban.
    pub moderator_user_login: types::UserName,
    /// The user name of the issuer of the unban.
    pub moderator_user_name: types::DisplayName,
}

#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.unban",
            "version": "1",
            "status": "enabled",
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
            "moderator_user_id": "1339",
            "moderator_user_login": "mod_user",
            "moderator_user_name": "Mod_User"
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Payload::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

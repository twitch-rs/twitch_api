#![doc(alias = "channel.ban")]
//! A viewer is banned from the specified channel.
use super::*;

/// [`channel.ban`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelban): a viewer is banned from the specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelBanV1 {
    /// The broadcaster user ID for the channel you want to get ban notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelBanV1 {
    type Payload = ChannelBanV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelBan;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelModerate];
    const VERSION: &'static str = "1";
}

/// [`channel.ban`](ChannelBanV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelBanV1Payload {
    /// The user ID for the user who was banned on the specified channel.
    pub user_id: types::UserId,
    /// The user login for the user who was banned on the specified channel.
    pub user_login: types::UserName,
    /// The requested broadcaster display name.
    pub user_name: types::DisplayName,
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The user ID of the issuer of the ban.
    pub moderator_user_id: types::UserId,
    /// The user login of the issuer of the ban.
    pub moderator_user_login: types::UserName,
    /// The user name of the issuer of the ban.
    pub moderator_user_name: types::DisplayName,
    /// The reason behind the ban.
    pub reason: String,
    /// Will be null if permanent ban. If it is a timeout, this field shows when the timeout will end.
    pub ends_at: Option<types::Timestamp>,
    /// Indicates whether the ban is permanent (true) or a timeout (false). If true, ends_at will be null.
    pub is_permanent: bool,
}
#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.ban",
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
            "moderator_user_id": "1339",
            "moderator_user_login": "mod_user",
            "moderator_user_name": "Mod_User",
            "reason": "Offensive language",
            "ends_at": "2020-07-15T18:16:11.17106713Z",
            "is_permanent": false
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

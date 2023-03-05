#![allow(deprecated)]
#![doc(alias = "channel.follow")]
//! A specified channel receives a follow.
use super::*;

/// [`channel.follow` v1](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelfollow): a specified channel receives a follow.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[deprecated(note = "use `ChannelFollowV2` instead")]
#[non_exhaustive]
pub struct ChannelFollowV1 {
    /// The broadcaster user ID for the channel you want to get follow notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelFollowV1 {
    /// The broadcaster user ID for the channel you want to get follow notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
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
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelFollowV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The user ID for the user now following the specified channel.
    pub user_id: types::UserId,
    /// The user display name for the user now following the specified channel.
    pub user_name: types::DisplayName,
    /// The user login for the user now following the specified channel.
    pub user_login: types::UserName,
    /// RFC3339 timestamp of when the follow occurred.
    pub followed_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload_v1() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.follow",
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
            "followed_at": "2020-07-15T18:16:11.17106713Z"
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

/// [`channel.follow` v2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelfollow): a specified channel receives a follow.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelFollowV2 {
    /// The broadcaster user ID for the channel you want to get follow notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The ID of the moderator of the channel you want to get follow notifications for. If you have authorization from the broadcaster rather than a moderator, specify the broadcaster’s user ID here.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelFollowV2 {
    /// Create a new [ChannelFollowV2] subscription
    pub fn new(
        broadcaster_user_id: impl Into<types::UserId>,
        moderator_user_id: impl Into<types::UserId>,
    ) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
            moderator_user_id: moderator_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelFollowV2 {
    type Payload = ChannelFollowV2Payload;

    const EVENT_TYPE: EventType = EventType::ChannelFollow;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModeratorReadFollowers];
    const VERSION: &'static str = "2";
}

/// [`channel.follow`](ChannelFollowV2) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelFollowV2Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The user ID for the user now following the specified channel.
    pub user_id: types::UserId,
    /// The user display name for the user now following the specified channel.
    pub user_name: types::DisplayName,
    /// The user login for the user now following the specified channel.
    pub user_login: types::UserName,
    /// RFC3339 timestamp of when the follow occurred.
    pub followed_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload_v2() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.follow",
            "version": "2",
            "status": "enabled",
            "cost": 0,
            "condition": {
               "broadcaster_user_id": "1337",
               "moderator_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.634234626Z"
        },
        "event": {
            "user_id": "1234",
            "user_login": "cool_user",
            "user_name": "Cool_User",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cooler_user",
            "broadcaster_user_name": "Cooler_User",
            "followed_at": "2020-07-15T18:16:11.17106713Z"
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

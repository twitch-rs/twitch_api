#![doc(alias = "channel.moderator.remove")]
//! a user has moderator privileges removed on a specified channel.

use super::*;
/// [`channel.moderator.remove`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatorremove): a user has moderator privileges removed on a specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelModeratorRemoveV1 {
    /// Get notifications when a moderator is removed in this channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelModeratorRemoveV1 {
    /// Get notifications for removed moderators in this channel
    pub fn new(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelModeratorRemoveV1 {
    type Payload = ChannelModeratorRemoveV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelModeratorRemove;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModerationRead];
    const VERSION: &'static str = "1";
}

/// [`channel.moderator.remove`](ChannelModeratorRemoveV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelModeratorRemoveV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,

    /// The user ID of the removed moderator.
    pub user_id: types::UserId,
    /// The user login of the removed moderator.
    pub user_login: types::UserName,
    /// The display name of the removed moderator.
    pub user_name: types::DisplayName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.moderator.remove",
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
            "created_at": "2019-11-16T10:11:12.634234626Z"
        },
        "event": {
            "user_id": "1234",
            "user_login": "not_mod_user",
            "user_name": "Not_Mod_User",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cooler_user",
            "broadcaster_user_name": "Cooler_User"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModeratorRemoveV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert_eq!(notif.broadcaster_user_login.as_str(), "cooler_user");
    assert_eq!(notif.broadcaster_user_name.as_str(), "Cooler_User");
    assert_eq!(notif.user_id.as_str(), "1234");
    assert_eq!(notif.user_login.as_str(), "not_mod_user");
    assert_eq!(notif.user_name.as_str(), "Not_Mod_User");
}

#![doc(alias = "channel.moderator.add")]
//! a user is given moderator privileges on a specified channel.

use super::*;
/// [`channel.moderator.add`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatoradd): a user is given moderator privileges on a specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelModeratorAddV1 {
    /// Get notifications when a moderator is added in this channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelModeratorAddV1 {
    /// Get notifications for added moderators in this channel
    pub fn new(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelModeratorAddV1 {
    type Payload = ChannelModeratorAddV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelModeratorAdd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModerationRead];
    const VERSION: &'static str = "1";
}

/// [`channel.moderator.add`](ChannelModeratorAddV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelModeratorAddV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,

    /// The user ID of the new moderator.
    pub user_id: types::UserId,
    /// The user login of the new moderator.
    pub user_login: types::UserName,
    /// The display name of the new moderator.
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
            "type": "channel.moderator.add",
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
            "user_login": "mod_user",
            "user_name": "Mod_User",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cooler_user",
            "broadcaster_user_name": "Cooler_User"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModeratorAddV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert_eq!(notif.broadcaster_user_login.as_str(), "cooler_user");
    assert_eq!(notif.broadcaster_user_name.as_str(), "Cooler_User");
    assert_eq!(notif.user_id.as_str(), "1234");
    assert_eq!(notif.user_login.as_str(), "mod_user");
    assert_eq!(notif.user_name.as_str(), "Mod_User");
}

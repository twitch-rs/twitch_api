#![doc(alias = "channel.warning.acknowledge")]
//! a warning is acknowledged by a user.

use super::*;
/// [`channel.warning.acknowledge`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningacknowledge): a warning is acknowledged by a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelWarningAcknowledgeV1 {
    /// The User ID of the broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The User ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelWarningAcknowledgeV1 {
    /// Get notifications for acknowledged warnings in this channel as a moderator
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

impl EventSubscription for ChannelWarningAcknowledgeV1 {
    type Payload = ChannelWarningAcknowledgeV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelWarningAcknowledge;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ModeratorReadWarnings,
        twitch_oauth2::Scope::ModeratorManageWarnings
    )];
    const VERSION: &'static str = "1";
}

/// [`channel.warning.acknowledge`](ChannelWarningAcknowledgeV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelWarningAcknowledgeV1Payload {
    /// The user ID of the broadcaster.
    pub broadcaster_user_id: types::UserId,
    /// The login of the broadcaster.
    pub broadcaster_user_login: types::UserName,
    /// The user name of the broadcaster.
    pub broadcaster_user_name: types::DisplayName,

    /// The ID of the user that has acknowledged their warning.
    pub user_id: types::UserId,
    /// The login of the user that has acknowledged their warning.
    pub user_login: types::UserName,
    /// The user name of the user that has acknowledged their warning.
    pub user_name: types::DisplayName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "channel.warning.acknowledge",
            "version": "1",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "423374343",
                "moderator_user_id": "424596340"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z"
        },
        "event": {
            "broadcaster_user_id": "423374343",
            "broadcaster_user_login": "glowillig",
            "broadcaster_user_name": "glowillig",
            "user_id": "141981764",
            "user_login": "twitchdev",
            "user_name": "TwitchDev"
        }
}
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelWarningAcknowledgeV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "423374343");
    assert_eq!(notif.broadcaster_user_login.as_str(), "glowillig");
    assert_eq!(notif.broadcaster_user_name.as_str(), "glowillig");
    assert_eq!(notif.user_id.as_str(), "141981764");
    assert_eq!(notif.user_login.as_str(), "twitchdev");
    assert_eq!(notif.user_name.as_str(), "TwitchDev");
}

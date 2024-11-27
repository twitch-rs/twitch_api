#![doc(alias = "channel.unban_request.create")]
//! a user creates an unban request.

use super::*;
/// [`channel.unban_request.create`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestcreate): a user creates an unban request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelUnbanRequestCreateV1 {
    /// The ID of the broadcaster you want to get chat unban request notifications for. Maximum: 1.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The ID of the user that has permission to moderate the broadcaster’s channel and has granted your app permission to subscribe to this subscription type.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelUnbanRequestCreateV1 {
    /// Get notifications when unban requests are created in this channel as a moderator
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

impl EventSubscription for ChannelUnbanRequestCreateV1 {
    type Payload = ChannelUnbanRequestCreateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelUnbanRequestCreate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorReadSuspiciousUsers];
    const VERSION: &'static str = "1";
}

/// [`channel.unban_request.create`](ChannelUnbanRequestCreateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelUnbanRequestCreateV1Payload {
    /// The ID of the unban request.
    pub id: types::UnbanRequestId,

    /// The broadcaster’s user ID for the channel the unban request was created for.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_user_name: types::DisplayName,

    /// User ID of user that is requesting to be unbanned.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,

    /// Message sent in the unban request.
    pub text: String,

    /// The UTC timestamp (in RFC3339 format) of when the unban request was created.
    pub created_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.unban_request.create",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337",
                "moderator_user_id": "1338"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.634234626Z"
        },
        "event": {
            "id": "60",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "user_id": "1339",
            "user_login": "not_cool_user",
            "user_name": "Not_Cool_User",
            "text": "unban me",
            "created_at": "2023-11-16T10:11:12.634234626Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelUnbanRequestCreateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.id.as_str(), "60");
    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert_eq!(notif.user_id.as_str(), "1339");
    assert_eq!(notif.text, "unban me");
    assert_eq!(notif.created_at.as_str(), "2023-11-16T10:11:12.634234626Z");
}

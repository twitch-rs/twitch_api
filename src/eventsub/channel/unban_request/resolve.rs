#![doc(alias = "channel.unban_request.resolve")]
//! an unban request has been resolved.

use super::*;
/// [`channel.unban_request.resolve`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestresolve): an unban request has been resolved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelUnbanRequestResolveV1 {
    /// The ID of the broadcaster you want to get unban request resolution notifications for. Maximum: 1.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The ID of the user that has permission to moderate the broadcaster’s channel and has granted your app permission to subscribe to this subscription type.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelUnbanRequestResolveV1 {
    /// Get notifications when unban requests are resolved in this channel as a moderator
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

impl EventSubscription for ChannelUnbanRequestResolveV1 {
    type Payload = ChannelUnbanRequestResolveV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelUnbanRequestResolve;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ModeratorReadUnbanRequests,
        twitch_oauth2::Scope::ModeratorManageUnbanRequests,
    )];
    const VERSION: &'static str = "1";
}

/// [`channel.unban_request.resolve`](ChannelUnbanRequestResolveV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelUnbanRequestResolveV1Payload {
    /// The ID of the unban request.
    pub id: types::UnbanRequestId,

    /// The broadcaster’s user ID for the channel the unban request was updated for.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_user_name: types::DisplayName,

    /// User ID of moderator who approved/denied the request.
    pub moderator_user_id: Option<types::UserId>,
    /// The moderator’s login name
    pub moderator_user_login: Option<types::UserName>,
    /// The moderator’s display name
    pub moderator_user_name: Option<types::DisplayName>,

    /// User ID of user that requested to be unbanned.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,

    /// Resolution text supplied by the mod/broadcaster upon approval/denial of the request.
    pub resolution_text: Option<String>,
    /// Dictates whether the unban request was approved or denied.
    pub status: UnbanRequestStatus,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.unban_request.resolve",
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
            "moderator_user_id": "1337",
            "moderator_user_login": "cool_user",
            "moderator_user_name": "Cool_User",
            "user_id": "1339",
            "user_login": "not_cool_user",
            "user_name": "Not_Cool_User",
            "resolution_text": "no",
            "status": "denied"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelUnbanRequestResolveV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.id.as_str(), "60");
    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert_eq!(notif.moderator_user_id.unwrap().as_str(), "1337");
    assert_eq!(notif.user_id.as_str(), "1339");
    assert_eq!(notif.resolution_text.as_deref(), Some("no"));
    assert_eq!(notif.status, UnbanRequestStatus::Denied);
}

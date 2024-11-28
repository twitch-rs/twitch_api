#![doc(alias = "channel.guest_star_session.begin")]
//! the host begins a new Guest Star session.

use super::*;
/// [`channel.guest_star_session.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin): the host begins a new Guest Star session.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGuestStarSessionBeginBeta {
    /// The broadcaster user ID for the channel you want to receive Guest Star session begin notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID of the moderator or broadcaster of the specified channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelGuestStarSessionBeginBeta {
    /// Get notifications for guest star sessions in this channel as a moderator
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

impl EventSubscription for ChannelGuestStarSessionBeginBeta {
    type Payload = ChannelGuestStarSessionBeginBetaPayload;

    const EVENT_TYPE: EventType = EventType::ChannelGuestStarSessionBegin;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadGuestStar,
        twitch_oauth2::Scope::ChannelManageGuestStar,
        twitch_oauth2::Scope::ModeratorReadGuestStar,
        twitch_oauth2::Scope::ModeratorManageGuestStar,
    )];
    const VERSION: &'static str = "beta";
}

/// [`channel.guest_star_session.begin`](ChannelGuestStarSessionBeginBeta) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGuestStarSessionBeginBetaPayload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster display name.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster login.
    pub broadcaster_user_name: types::DisplayName,

    /// ID representing the unique session that was started.
    pub session_id: types::GuestStarSessionId,
    /// RFC3339 timestamp indicating the time the session began.
    pub started_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.guest_star_session.begin",
            "version": "beta",
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
            "created_at": "2023-04-11T10:11:12.123Z"
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "Cool_User",
            "broadcaster_user_login": "cool_user",
            "session_id": "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "started_at": "2023-04-11T16:20:03.17106713Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelGuestStarSessionBeginBeta(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert_eq!(notif.session_id.as_str(), "2KFRQbFtpmfyD3IevNRnCzOPRJI");
    assert_eq!(notif.started_at.as_str(), "2023-04-11T16:20:03.17106713Z");
}

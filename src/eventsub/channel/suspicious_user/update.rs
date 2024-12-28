#![doc(alias = "channel.suspicious_user.update")]
//! a suspicious user has been updated.

use super::*;
/// [`channel.suspicious_user.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_userupdate): a suspicious user has been updated.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSuspiciousUserUpdateV1 {
    /// The broadcaster you want to get suspicious user update notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The ID of a user that has permission to moderate the broadcaster’s channel and has granted your app permission to subscribe to this subscription type.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelSuspiciousUserUpdateV1 {
    /// Get notifications for updates from suspicious users in this channel as a moderator
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

impl EventSubscription for ChannelSuspiciousUserUpdateV1 {
    type Payload = ChannelSuspiciousUserUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelSuspiciousUserUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorReadSuspiciousUsers];
    const VERSION: &'static str = "1";
}

/// [`channel.suspicious_user.update`](ChannelSuspiciousUserUpdateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSuspiciousUserUpdateV1Payload {
    /// The ID of the channel where the treatment for a suspicious user was updated.
    pub broadcaster_user_id: types::UserId,
    /// The display name of the channel where the treatment for a suspicious user was updated.
    pub broadcaster_user_login: types::UserName,
    /// The Login of the channel where the treatment for a suspicious user was updated.
    pub broadcaster_user_name: types::DisplayName,

    /// The ID of the moderator that updated the treatment for a suspicious user.
    pub moderator_user_id: types::UserId,
    /// The display name of the moderator that updated the treatment for a suspicious user.
    pub moderator_user_login: types::UserName,
    /// The login of the moderator that updated the treatment for a suspicious user.
    pub moderator_user_name: types::DisplayName,

    /// The ID of the suspicious user whose treatment was updated.
    pub user_id: types::UserId,
    /// The display name of the suspicious user whose treatment was updated.
    pub user_login: types::UserName,
    /// The login of the suspicious user whose treatment was updated.
    pub user_name: types::DisplayName,

    /// The status set for the suspicious user.
    pub low_trust_status: LowTrustStatus,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.suspicious_user.update",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1050263435",
                "moderator_user_id": "1050263436"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2023-04-11T10:11:12.123Z"
        },
        "event": {
            "broadcaster_user_id": "1050263435",
            "broadcaster_user_name": "77f111cbb75341449f5",
            "broadcaster_user_login": "77f111cbb75341449f5",
            "moderator_user_id": "1050263436",
            "moderator_user_name": "29087e59dfc441968f6",
            "moderator_user_login": "29087e59dfc441968f6",
            "user_id": "1050263437",
            "user_name": "06fbcc75952245c5a87",
            "user_login": "06fbcc75952245c5a87",
            "low_trust_status": "restricted"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelSuspiciousUserUpdateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1050263435");
    assert_eq!(notif.moderator_user_id.as_str(), "1050263436");
    assert_eq!(notif.user_id.as_str(), "1050263437");
    assert_eq!(notif.low_trust_status, LowTrustStatus::Restricted);
}

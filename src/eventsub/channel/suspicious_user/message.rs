#![doc(alias = "channel.suspicious_user.message")]
//! a chat message has been sent from a suspicious user.

use super::*;
/// [`channel.suspicious_user.message`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_usermessage): a chat message has been sent from a suspicious user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSuspiciousUserMessageV1 {
    /// User ID of the channel to receive chat message events for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The ID of a user that has permission to moderate the broadcaster’s channel and has granted your app permission to subscribe to this subscription type.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelSuspiciousUserMessageV1 {
    /// Get notifications for messages from suspicious users in this channel as a moderator
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

impl EventSubscription for ChannelSuspiciousUserMessageV1 {
    type Payload = ChannelSuspiciousUserMessageV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelSuspiciousUserMessage;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorReadSuspiciousUsers];
    const VERSION: &'static str = "1";
}

/// [`channel.suspicious_user.message`](ChannelSuspiciousUserMessageV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSuspiciousUserMessageV1Payload {
    /// The ID of the channel where the treatment for a suspicious user was updated.
    pub broadcaster_user_id: types::UserId,
    /// The display name of the channel where the treatment for a suspicious user was updated.
    pub broadcaster_user_login: types::UserName,
    /// The login of the channel where the treatment for a suspicious user was updated.
    pub broadcaster_user_name: types::DisplayName,

    /// The user ID of the user that sent the message.
    pub user_id: types::UserId,
    /// The user name of the user that sent the message.
    pub user_login: types::UserName,
    /// The user login of the user that sent the message.
    pub user_name: types::DisplayName,

    /// The status set for the suspicious user.
    pub low_trust_status: LowTrustStatus,
    /// A list of channel IDs where the suspicious user is also banned.
    #[serde(deserialize_with = "crate::deserialize_default_from_null")]
    pub shared_ban_channel_ids: Vec<types::UserId>,
    /// User types (if any) that apply to the suspicious user.
    #[serde(deserialize_with = "crate::deserialize_default_from_null")]
    pub types: Vec<SuspiciousUserType>,
    /// A ban evasion likelihood value (if any) that as been applied to the user automatically by Twitch.
    pub ban_evasion_evaluation: BanEvasionEvaluation,
    /// The structured chat message.
    pub message: crate::eventsub::automod::message::AutomodMessage,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.suspicious_user.message",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "moderator_user_id": "9001",
                "broadcaster_user_id": "1050263432"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2023-04-11T10:11:12.123Z"
        },
        "event": {
            "broadcaster_user_id": "1050263432",
            "broadcaster_user_name": "dcf9dd9336034d23b65",
            "broadcaster_user_login": "dcf9dd9336034d23b65",
            "user_id": "1050263434",
            "user_name": "4a46e2cf2e2f4d6a9e6",
            "user_login": "4a46e2cf2e2f4d6a9e6",
            "low_trust_status": "active_monitoring",
            "shared_ban_channel_ids": [
                "100",
                "200"
            ],
            "types": [
                "ban_evader"
            ],
            "ban_evasion_evaluation": "likely",
            "message": {
                "message_id": "101010",
                "text": "bad stuff pogchamp",
                "fragments": [
                    {
                        "type": "emote",
                        "text": "bad stuff",
                        "cheermote": null,
                        "emote": {
                            "id": "899",
                            "emote_set_id": "1"
                        }
                    },
                    {
                        "type": "cheermote",
                        "text": "pogchamp",
                        "cheermote": {
                            "prefix": "pogchamp",
                            "bits": 100,
                            "tier": 1
                        },
                        "emote": null
                    }
                ]
            }
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelSuspiciousUserMessageV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1050263432");
    assert_eq!(notif.user_id.as_str(), "1050263434");
    assert_eq!(notif.low_trust_status, LowTrustStatus::ActiveMonitoring);
    assert_eq!(notif.shared_ban_channel_ids.len(), 2);
    assert_eq!(notif.shared_ban_channel_ids[0].as_str(), "100");
    assert_eq!(notif.shared_ban_channel_ids[1].as_str(), "200");
    assert_eq!(notif.types.len(), 1);
    assert_eq!(notif.types[0], SuspiciousUserType::BanEvader);
    assert_eq!(notif.ban_evasion_evaluation, BanEvasionEvaluation::Likely);
    assert_eq!(notif.message.fragments.len(), 2);
}

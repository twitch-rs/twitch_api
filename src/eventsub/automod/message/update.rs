#![doc(alias = "automod.message.update")]
//! a message in the automod queue had its status changed

use super::*;
/// [`automod.message.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate): a message in the automod queue had its status changed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageUpdateV1 {
    /// User ID of the broadcaster (channel). Maximum: 1
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// User ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl AutomodMessageUpdateV1 {
    /// Get automod update notifications for messages in this channel as a moderator
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

impl EventSubscription for AutomodMessageUpdateV1 {
    type Payload = AutomodMessageUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::AutomodMessageUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageAutoMod];
    const VERSION: &'static str = "1";
}

/// [`automod.message.update`](AutomodMessageUpdateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageUpdateV1Payload {
    /// The ID of the broadcaster specified in the request.
    pub broadcaster_user_id: types::UserId,
    /// The login of the broadcaster specified in the request.
    pub broadcaster_user_login: types::UserName,
    /// The user name of the broadcaster specified in the request.
    pub broadcaster_user_name: types::DisplayName,

    /// The message sender’s user ID.
    pub user_id: types::UserId,
    /// The message sender’s login name.
    pub user_login: types::UserName,
    /// The message sender’s display name.
    pub user_name: types::DisplayName,

    /// The ID of the moderator.
    pub moderator_user_id: types::UserId,
    /// The login of the moderator.
    pub moderator_user_login: types::UserName,
    /// The moderator’s user name.
    pub moderator_user_name: types::DisplayName,

    /// The ID of the message that was flagged by automod.
    pub message_id: types::MsgId,
    /// The body of the message.
    pub message: AutomodMessage,
    /// The category of the message.
    pub category: super::AutomodCategory,
    /// The level of severity. Measured between 1 to 4.
    pub level: u8,
    /// The message’s status.
    pub status: AutomodMessageStatus,
    /// The timestamp of when automod saved the message.
    pub held_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload_v1() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "79cc58a2-1c34-48e0-97fe-126d5d77bf10",
            "status": "enabled",
            "type": "automod.message.update",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQZ12VWLotRG6u3pudLlbhvhIGY2VsbC1j"
            },
            "created_at": "2024-11-03T11:52:04.695680375Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "user_id": "489584266",
            "user_login": "uint128",
            "user_name": "uint128",
            "moderator_user_id": "129546453",
            "moderator_user_login": "nerixyz",
            "moderator_user_name": "nerixyz",
            "message_id": "8b722958-741f-4013-8a8b-c7793d3aef9f",
            "message": {
                "text": "boobs",
                "fragments": [
                    {
                        "type": "text",
                        "text": "boobs",
                        "cheermote": null,
                        "emote": null
                    }
                ]
            },
            "category": "sexwords",
            "level": 4,
            "status": "approved",
            "held_at": "2024-11-03T11:53:45.331308397Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodMessageUpdateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.category, AutomodCategory::Sexwords);
    assert_eq!(notif.level, 4);
    assert_eq!(notif.status, AutomodMessageStatus::Approved);
    assert_eq!(notif.message.fragments.len(), 1);
}

/// [`automod.message.update`](dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate-v2): a message in the automod queue had its status changed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
#[cfg(feature = "beta")]
pub struct AutomodMessageUpdateBeta {
    /// User ID of the broadcaster (channel).
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// User ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

#[cfg(feature = "beta")]
impl AutomodMessageUpdateBeta {
    /// Get automod update notifications for this channel as a moderator
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

#[cfg(feature = "beta")]
impl EventSubscription for AutomodMessageUpdateBeta {
    type Payload = AutomodMessageUpdateBetaPayload;

    const EVENT_TYPE: EventType = EventType::AutomodMessageUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageAutoMod];
    const VERSION: &'static str = "beta";
}

/// [`automod.message.hold`](AutomodMessageUpdateBeta) response payload.
// XXX: this struct can't be deny-unknown-fields because of the flattened reason
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[cfg(feature = "beta")]
pub struct AutomodMessageUpdateBetaPayload {
    /// The ID of the broadcaster specified in the request.
    pub broadcaster_user_id: types::UserId,
    /// The login of the broadcaster specified in the request.
    pub broadcaster_user_login: types::UserName,
    /// The user name of the broadcaster specified in the request.
    pub broadcaster_user_name: types::DisplayName,

    /// The message sender’s user ID.
    pub user_id: types::UserId,
    /// The message sender’s login name.
    pub user_login: types::UserName,
    /// The message sender’s display name.
    pub user_name: types::DisplayName,

    /// The ID of the moderator.
    pub moderator_user_id: types::UserId,
    /// The login of the moderator.
    pub moderator_user_login: types::UserName,
    /// The moderator’s user name.
    pub moderator_user_name: types::DisplayName,

    /// The ID of the message that was flagged by automod.
    pub message_id: types::MsgId,
    /// The body of the message.
    pub message: AutomodMessage,
    /// The message’s status.
    pub status: AutomodMessageStatus,
    /// The timestamp of when automod saved the message.
    pub held_at: types::Timestamp,
    /// The reason why a message was held
    #[serde(flatten)]
    pub reason: AutomodHeldReason,
}

#[cfg(all(test, feature = "beta"))]
#[test]
fn parse_payload_v2_automod() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "5d64b907-001e-4cf1-9227-37871c7ce1b0",
            "status": "enabled",
            "type": "automod.message.update",
            "version": "beta",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQSrkrRHHrQsS-i4xbndeC0hIGY2VsbC1j"
            },
            "created_at": "2024-11-18T19:25:05.666970955Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "user_id": "489584266",
            "user_login": "uint128",
            "user_name": "uint128",
            "moderator_user_id": "129546453",
            "moderator_user_login": "nerixyz",
            "moderator_user_name": "nerixyz",
            "message_id": "2a867e45-a4d3-4e7e-a5cc-a9a00ee98bf7",
            "message": {
                "text": "Kappa ass",
                "fragments": [
                    {
                        "type": "emote",
                        "text": "Kappa",
                        "cheermote": null,
                        "emote": {
                            "id": "25",
                            "emote_set_id": "0"
                        }
                    },
                    {
                        "type": "text",
                        "text": " ",
                        "cheermote": null,
                        "emote": null
                    },
                    {
                        "type": "text",
                        "text": "ass",
                        "cheermote": null,
                        "emote": null
                    }
                ]
            },
            "reason": "automod",
            "automod": {
                "category": "swearing",
                "level": 4,
                "boundaries": [
                    {
                        "start_pos": 6,
                        "end_pos": 8
                    }
                ]
            },
            "blocked_term": null,
            "status": "denied",
            "held_at": "2024-11-18T19:26:37.707305502Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodMessageUpdateBeta(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.message.fragments.len(), 3);
    assert_eq!(notif.status, AutomodMessageStatus::Denied);

    let AutomodHeldReason::Automod { automod } = &notif.reason else {
        panic!("invalid held reason");
    };
    assert_eq!(
        automod.category,
        AutomodCategory::Unknown("swearing".to_string())
    );
    assert_eq!(automod.level, 4);
    assert_eq!(
        automod.boundaries,
        &[AutomodMessageBoundary {
            start_pos: 6,
            end_pos: 8
        }]
    );
}

#[cfg(all(test, feature = "beta"))]
#[test]
fn parse_payload_v2_blocked_term() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "5d64b907-001e-4cf1-9227-37871c7ce1b0",
            "status": "enabled",
            "type": "automod.message.update",
            "version": "beta",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQSrkrRHHrQsS-i4xbndeC0hIGY2VsbC1j"
            },
            "created_at": "2024-11-18T19:25:05.666970955Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "user_id": "489584266",
            "user_login": "uint128",
            "user_name": "uint128",
            "moderator_user_id": "129546453",
            "moderator_user_login": "nerixyz",
            "moderator_user_name": "nerixyz",
            "message_id": "8c2b43ed-88a0-4b3e-8c02-266c323e1d95",
            "message": {
                "text": "foo",
                "fragments": [
                    {
                        "type": "text",
                        "text": "foo",
                        "cheermote": null,
                        "emote": null
                    }
                ]
            },
            "reason": "blocked_term",
            "automod": null,
            "blocked_term": {
                "terms_found": [
                    {
                        "term_id": "e4d4f1ba-99bf-4b19-9875-cd4eda98ead9",
                        "owner_broadcaster_user_id": "129546453",
                        "owner_broadcaster_user_login": "nerixyz",
                        "owner_broadcaster_user_name": "nerixyz",
                        "boundary": {
                            "start_pos": 0,
                            "end_pos": 2
                        }
                    }
                ]
            },
            "status": "approved",
            "held_at": "2024-11-18T19:25:52.991756968Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodMessageUpdateBeta(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.message.fragments.len(), 1);
    assert_eq!(notif.status, AutomodMessageStatus::Approved);

    let AutomodHeldReason::BlockedTerm { blocked_term } = &notif.reason else {
        panic!("invalid held reason");
    };
    assert_eq!(blocked_term.terms_found.len(), 1);
    assert_eq!(
        blocked_term.terms_found[0].term_id.as_str(),
        "e4d4f1ba-99bf-4b19-9875-cd4eda98ead9"
    );
    assert_eq!(
        blocked_term.terms_found[0].boundary,
        AutomodMessageBoundary {
            start_pos: 0,
            end_pos: 2
        }
    );
}

#![doc(alias = "automod.message.hold")]
//! A message was caught by automod for review

use super::*;
/// [`automod.message.hold`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold): a message was caught by automod for review.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageHoldV1 {
    /// User ID of the broadcaster (channel).
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// User ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl AutomodMessageHoldV1 {
    /// Get automod hold notifications for this channel as a moderator
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

impl EventSubscription for AutomodMessageHoldV1 {
    type Payload = AutomodMessageHoldV1Payload;

    const EVENT_TYPE: EventType = EventType::AutomodMessageHold;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageAutoMod];
    const VERSION: &'static str = "1";
}

/// [`automod.message.hold`](AutomodMessageHoldV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageHoldV1Payload {
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
    /// The ID of the message that was flagged by automod.
    pub message_id: types::MsgId,
    /// The body of the message.
    pub message: AutomodMessage,
    /// The category of the message.
    pub category: super::AutomodCategory,
    /// The level of severity. Measured between 1 to 4.
    pub level: u8,
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
            "id": "e523fda0-01b6-4b0e-9024-a5a80c5ad680",
            "status": "enabled",
            "type": "automod.message.hold",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQRniKAQ1ITYSESh4ku8anEBIGY2VsbC1j"
            },
            "created_at": "2024-10-19T20:11:13.917500523Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "user_id": "489584266",
            "user_login": "uint128",
            "user_name": "uint128",
            "message_id": "332e99ac-e19c-4368-a15b-793e8266b51f",
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
            "held_at": "2024-10-19T20:11:16.799750627Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodMessageHoldV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.category, AutomodCategory::Sexwords);
    assert_eq!(notif.level, 4);
    assert_eq!(notif.message.fragments.len(), 1);
}

/// [`automod.message.hold`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold-v2): a message was caught by automod for review.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
#[cfg(feature = "beta")]
pub struct AutomodMessageHoldBeta {
    /// User ID of the broadcaster (channel).
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// User ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

#[cfg(feature = "beta")]
impl AutomodMessageHoldBeta {
    /// Get automod hold notifications for this channel as a moderator
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
impl EventSubscription for AutomodMessageHoldBeta {
    type Payload = AutomodMessageHoldBetaPayload;

    const EVENT_TYPE: EventType = EventType::AutomodMessageHold;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageAutoMod];
    const VERSION: &'static str = "beta";
}

/// [`automod.message.hold`](AutomodMessageHoldBeta) response payload.
// XXX: this struct can't be deny-unknown-fields because of the flattened reason
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[cfg(feature = "beta")]
pub struct AutomodMessageHoldBetaPayload {
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
    /// The ID of the message that was flagged by automod.
    pub message_id: types::MsgId,
    /// The body of the message.
    pub message: AutomodMessage,
    /// The reason why a message was held
    #[serde(flatten)]
    pub reason: AutomodHeldReason,
    /// The timestamp of when automod saved the message.
    pub held_at: types::Timestamp,
}

#[cfg(all(test, feature = "beta"))]
#[test]
fn parse_payload_v2_automod() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "85c8dcb0-7af4-4581-b684-32087d386384",
            "status": "enabled",
            "type": "automod.message.hold",
            "version": "beta",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQREw4FYBWQ5quz4J-S4VYkRIGY2VsbC1j"
            },
            "created_at": "2024-11-18T16:36:08.691979783Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "user_id": "489584266",
            "user_login": "uint128",
            "user_name": "uint128",
            "message_id": "78ccd959-3e7e-4f8d-bd8b-f92c359b0a7d",
            "message": {
                "text": "😂 ass",
                "fragments": [
                    {
                        "type": "text",
                        "text": "😂 ",
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
                        "start_pos": 2,
                        "end_pos": 4
                    }
                ]
            },
            "blocked_term": null,
            "held_at": "2024-11-18T16:59:46.323937273Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodMessageHoldBeta(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.message.fragments.len(), 2);

    let AutomodHeldReason::Automod { automod } = &notif.reason else {
        panic!("invalid held reason");
    };
    assert_eq!(automod.category, AutomodCategory::Swearing);
    assert_eq!(automod.level, 4);
    assert_eq!(
        automod.boundaries,
        &[AutomodMessageBoundary {
            start_pos: 2,
            end_pos: 4
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
            "id": "85c8dcb0-7af4-4581-b684-32087d386384",
            "status": "enabled",
            "type": "automod.message.hold",
            "version": "beta",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQREw4FYBWQ5quz4J-S4VYkRIGY2VsbC1j"
            },
            "created_at": "2024-11-18T16:36:08.691979783Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "user_id": "489584266",
            "user_login": "uint128",
            "user_name": "uint128",
            "message_id": "dcfc6b48-0fd1-446c-8cf5-d1810bb55b73",
            "message": {
                "text": "boobs Kappa 😂😂 foo private",
                "fragments": [
                    {
                        "type": "text",
                        "text": "boobs",
                        "cheermote": null,
                        "emote": null
                    },
                    {
                        "type": "text",
                        "text": " ",
                        "cheermote": null,
                        "emote": null
                    },
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
                        "text": " 😂😂 ",
                        "cheermote": null,
                        "emote": null
                    },
                    {
                        "type": "text",
                        "text": "foo",
                        "cheermote": null,
                        "emote": null
                    },
                    {
                        "type": "text",
                        "text": " ",
                        "cheermote": null,
                        "emote": null
                    },
                    {
                        "type": "text",
                        "text": "private",
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
                            "start_pos": 15,
                            "end_pos": 17
                        }
                    },
                    {
                        "term_id": "e60a94ea-e5d9-444e-a114-4cfd2f86c6ad",
                        "owner_broadcaster_user_id": "129546453",
                        "owner_broadcaster_user_login": "nerixyz",
                        "owner_broadcaster_user_name": "nerixyz",
                        "boundary": {
                            "start_pos": 19,
                            "end_pos": 25
                        }
                    }
                ]
            },
            "held_at": "2024-11-18T16:58:41.476117057Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodMessageHoldBeta(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.message.fragments.len(), 7);

    let AutomodHeldReason::BlockedTerm { blocked_term } = &notif.reason else {
        panic!("invalid held reason");
    };
    assert_eq!(blocked_term.terms_found.len(), 2);
    assert_eq!(
        blocked_term.terms_found[0].term_id.as_str(),
        "e4d4f1ba-99bf-4b19-9875-cd4eda98ead9"
    );
    assert_eq!(
        blocked_term.terms_found[0].boundary,
        AutomodMessageBoundary {
            start_pos: 15,
            end_pos: 17
        }
    );
}

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
fn parse_payload() {
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

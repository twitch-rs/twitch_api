#![doc(alias = "channel.chat.user_message_hold")]
//! A user's message is caught by automod.

use super::*;
/// [`channel.chat.user_message_hold`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatuser_message_hold): a user's message is caught by automod.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatUserMessageHoldV1 {
    /// User ID of the channel to receive chat message events for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID to read chat as.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub user_id: types::UserId,
}

impl ChannelChatUserMessageHoldV1 {
    /// Get user message hold events on a broadcasters channel reading chat as a specific user.
    pub fn new(
        broadcaster_user_id: impl Into<types::UserId>,
        user_id: impl Into<types::UserId>,
    ) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
            user_id: user_id.into(),
        }
    }
}

impl EventSubscription for ChannelChatUserMessageHoldV1 {
    type Payload = ChannelChatUserMessageHoldV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelChatUserMessageHold;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadChat];
    const VERSION: &'static str = "1";
}

/// [`channel.chat.user_message_hold`](ChannelChatUserMessageHoldV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatUserMessageHoldV1Payload {
    /// The ID of the broadcaster specified in the request.
    pub broadcaster_user_id: types::UserId,
    /// The login of the broadcaster specified in the request.
    pub broadcaster_user_login: types::UserName,
    /// The user name of the broadcaster specified in the request.
    pub broadcaster_user_name: types::DisplayName,
    /// The User ID of the message sender.
    pub user_id: types::UserId,
    /// The message sender’s login.
    pub user_login: types::UserName,
    /// The message sender’s display name.
    pub user_name: types::DisplayName,
    /// The ID of the message that was flagged by automod.
    pub message_id: types::MsgId,
    /// The body of the message.
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
            "type": "channel.chat.user_message_hold",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337",
                "user_id": "9001"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2023-04-11T10:11:12.123Z"
        },
        "event": {
            "broadcaster_user_id": "123",
            "broadcaster_user_login": "bob",
            "broadcaster_user_name": "Bob",
            "user_id": "456",
            "user_login": "tom",
            "user_name": "Tommy",
            "message_id": "789",
            "message": {
                "text": "hey world",
                "fragments": [
                    {
                        "type": "emote",
                        "text": "hey world",
                        "cheermote": null,
                        "emote": {
                            "id": "foo",
                            "emote_set_id": "7"
                        }
                    },
                    {
                        "type": "cheermote",
                        "text": "bye world",
                        "cheermote": {
                            "prefix": "prefix",
                            "bits": 100,
                            "tier": 1
                        },
                        "emote": null
                    },
                    {
                        "type": "text",
                        "text": "surprise",
                        "cheermote": null,
                        "emote": null
                    }
                ]
            }
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelChatUserMessageHoldV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "123");
    assert_eq!(notif.user_id.as_str(), "456");
    assert_eq!(notif.message_id.as_str(), "789");
    assert_eq!(notif.message.text, "hey world");
}

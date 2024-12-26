#![doc(alias = "channel.chat.message")]
//! Any user sends a message to a specific chat room.

use super::*;

/// [`channel.chat.message`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage): a user sends a message to a specific chat room.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatMessageV1 {
    /// User ID of the channel to receive chat message events for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID to read chat as.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub user_id: types::UserId,
}

impl ChannelChatMessageV1 {
    /// Create a new [ChannelChatMessageV1]
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

impl EventSubscription for ChannelChatMessageV1 {
    type Payload = ChannelChatMessageV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelChatMessage;
    #[cfg(feature = "twitch_oauth2")]
    /// Additionally, if an app access token is used,
    /// [user:bot][twitch_oauth2::Scope::UserBot] is requried from the chatting user, i.e. the user specified by [user_id][ChannelChatMessageV1::user_id],
    /// and either [channel:bot][twitch_oauth2::Scope::ChannelBot] from the broadcaster or moderator status in chat.
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadChat];
    const VERSION: &'static str = "1";
}

/// [`channel.chat.message`](ChannelChatMessageV1Payload) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatMessageV1Payload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The user ID of the user that sent the message.
    pub chatter_user_id: types::UserId,
    /// The user name of the user that sent the message.
    pub chatter_user_name: types::DisplayName,
    /// The user login of the user that sent the message.
    pub chatter_user_login: types::UserName,
    /// A UUID that identifies the message.
    pub message_id: types::MsgId,
    /// The structured chat message.
    pub message: Message,
    /// The type of message.
    pub message_type: MessageType,
    /// List of chat badges.
    pub badges: Vec<Badge>,
    /// Metadata if this message is a cheer.
    pub cheer: Option<Cheer>,
    /// The color of the user's name in the chat room.
    /// This is a hexadecimal RGB color code in the form, `#<RGB>`.
    /// This may be empty if it is never set.
    pub color: types::HexColor,
    /// Metadata if this message is a reply.
    pub reply: Option<Reply>,
    /// The ID of a channel points custom reward that was redeemed.
    pub channel_points_custom_reward_id: Option<types::RewardId>,
    /// An ID for the type of animation selected as part of an “animate my message” redemption.
    pub channel_points_animation_id: Option<String>,
    /// Only present when in a shared chat session. The broadcaster user ID of the channel the message was sent from.
    pub source_broadcaster_user_id: Option<types::UserId>,
    /// Only present when in a shared chat session. The user name of the broadcaster of the channel the message was sent from.
    pub source_broadcaster_user_name: Option<types::DisplayName>,
    /// Only present when in a shared chat session. The login of the broadcaster of the channel the message was sent from.
    pub source_broadcaster_user_login: Option<types::UserName>,
    /// Only present when in a shared chat session. The UUID that identifies the source message from the channel the message was sent from.
    pub source_message_id: Option<types::MsgId>,
    /// Only present when in a shared chat session. The list of chat badges for the chatter in the channel the message was sent from.
    #[serde(deserialize_with = "crate::deserialize_default_from_null")]
    pub source_badges: Vec<Badge>,
}

/// The type a message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// A regular text message
    Text,
    /// A highlighted message with channel points
    ChannelPointsHighlighted,
    /// A message sent with channel points during sub-only mode
    ChannelPointsSubOnly,
    /// A first message from a user
    UserIntro,
    /// A gigantified emote
    PowerUpsGigantifiedEmote,
    /// A message sent with effects
    PowerUpsMessageEffect,
}

/// Chat badge
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Badge {
    /// An ID that identifies this set of chat badges. For example, Bits or Subscriber.
    pub set_id: types::BadgeSetId,
    /// An ID that identifies this version of the badge. The ID can be any value.
    /// For example, for Bits, the ID is the Bits tier level, but for World of Warcraft, it could be Alliance or Horde.
    pub id: types::ChatBadgeId,
    /// Contains metadata related to the chat badges in the badges tag.
    /// Currently, this tag contains metadata only for subscriber badges, to indicate the number of months the user has been a subscriber.
    pub info: String,
}

/// Metadata for cheer messages
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Cheer {
    /// The amount of Bits the user cheered.
    pub bits: usize,
}

/// Metadata for reply messages
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Reply {
    /// An ID that uniquely identifies the parent message that this message is replying to.
    pub parent_message_id: types::MsgId,
    /// The message body of the parent message.
    pub parent_message_body: String,
    /// User ID of the sender of the parent message.
    pub parent_user_id: types::UserId,
    /// User name of the sender of the parent message.
    pub parent_user_name: types::DisplayName,
    /// User login of the sender of the parent message.
    pub parent_user_login: types::UserName,
    /// An ID that identifies the parent message of the reply thread.
    pub thread_message_id: types::MsgId,
    /// User ID of the sender of the thread's parent message.
    pub thread_user_id: types::UserId,
    /// User name of the sender of the thread's parent message.
    pub thread_user_name: types::DisplayName,
    /// User login of the sender of the thread's parent message.
    pub thread_user_login: types::UserName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
        {
        "subscription": {
            "id": "0b7f3361-672b-4d39-b307-dd5b576c9b27",
            "status": "enabled",
            "type": "channel.chat.message",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "1971641",
                "user_id": "2914196"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQHR3s6Mb4T8GFB1l3DlPfiRIGY2VsbC1h"
            },
            "created_at": "2023-11-06T18:11:47.492253549Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "1971641",
            "broadcaster_user_login": "streamer",
            "broadcaster_user_name": "streamer",
            "chatter_user_id": "4145994",
            "chatter_user_login": "viewer32",
            "chatter_user_name": "viewer32",
            "message_id": "cc106a89-1814-919d-454c-f4f2f970aae7",
            "message": {
                "text": "Hi chat",
                "fragments": [
                    {
                        "type": "text",
                        "text": "Hi chat",
                        "cheermote": null,
                        "emote": null,
                        "mention": null
                    }
                ]
            },
            "color": "#00FF7F",
            "badges": [
                {
                    "set_id": "moderator",
                    "id": "1",
                    "info": ""
                },
                {
                    "set_id": "subscriber",
                    "id": "12",
                    "info": "16"
                },
                {
                    "set_id": "sub-gifter",
                    "id": "1",
                    "info": ""
                }
            ],
            "message_type": "text",
            "cheer": null,
            "reply": null,
            "channel_points_custom_reward_id": null,
            "source_broadcaster_user_id": null,
            "source_broadcaster_user_login": null,
            "source_broadcaster_user_name": null,
            "source_message_id": null,
            "source_badges": null
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#[cfg(test)]
#[test]
fn parse_payload_shared() {
    let payload = r##"
    {
        "subscription": {
            "id": "0b7f3361-672b-4d39-b307-dd5b576c9b27",
            "status": "enabled",
            "type": "channel.chat.message",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "1971641",
                "user_id": "2914196"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQHR3s6Mb4T8GFB1l3DlPfiRIGY2VsbC1h"
            },
            "created_at": "2023-11-06T18:11:47.492253549Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "1971641",
            "broadcaster_user_login": "streamer",
            "broadcaster_user_name": "streamer",
            "chatter_user_id": "4145994",
            "chatter_user_login": "viewer32",
            "chatter_user_name": "viewer32",
            "message_id": "cc106a89-1814-919d-454c-f4f2f970aae7",
            "message": {
                "text": "Hi chat",
                "fragments": [
                    {
                        "type": "text",
                        "text": "Hi chat",
                        "cheermote": null,
                        "emote": null,
                        "mention": null
                    }
                ]
            },
            "color": "#00FF7F",
            "badges": [
                {
                    "set_id": "moderator",
                    "id": "1",
                    "info": ""
                },
                {
                    "set_id": "subscriber",
                    "id": "12",
                    "info": "16"
                },
                {
                    "set_id": "sub-gifter",
                    "id": "1",
                    "info": ""
                }
            ],
            "message_type": "text",
            "cheer": null,
            "reply": null,
            "channel_points_custom_reward_id": null,
            "source_broadcaster_user_id": "112233",
            "source_broadcaster_user_login": "streamer33",
            "source_broadcaster_user_name": "streamer33",
            "source_message_id": "e03f6d5d-8ec8-4c63-b473-9e5fe61e289b",
            "source_badges": [
                {
                    "set_id": "subscriber",
                    "id": "3",
                    "info": "3"
                }
            ]
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

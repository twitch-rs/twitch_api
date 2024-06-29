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
    pub broadcaster_user_name: types::UserName,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The user ID of the user that sent the message.
    pub chatter_user_id: types::UserId,
    /// The user name of the user that sent the message.
    pub chatter_user_name: types::UserName,
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
    pub parent_user_name: types::UserName,
    /// User login of the sender of the parent message.
    pub parent_user_login: types::UserName,
    /// An ID that identifies the parent message of the reply thread.
    pub thread_message_id: types::MsgId,
    /// User ID of the sender of the thread's parent message.
    pub thread_user_id: types::UserId,
    /// User name of the sender of the thread's parent message.
    pub thread_user_name: types::UserName,
    /// User login of the sender of the thread's parent message.
    pub thread_user_login: types::UserName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "47faedb0-b918-4d79-a974-fe799c9b1f6b",
            "status": "enabled",
            "type": "channel.chat.message",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "141981764",
                "user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQL5tbQXjKS4SBPvF0F-Qz0hIGY2VsbC1j"
            },
            "created_at": "2024-02-24T17:17:49.772726224Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "141981764",
            "broadcaster_user_login": "twitchdev",
            "broadcaster_user_name": "TwitchDev",
            "chatter_user_id": "129546453",
            "chatter_user_login": "nerixyz",
            "chatter_user_name": "nerixyz",
            "message_id": "9d0bcb5e-ee31-4b09-b72f-66eb94ce061e",
            "message": {
                "text": "Hello, World! DinoDance",
                "fragments": [
                    {
                        "type": "text",
                        "text": "Hello, World! ",
                        "cheermote": null,
                        "emote": null,
                        "mention": null
                    },
                    {
                        "type": "emote",
                        "text": "DinoDance",
                        "cheermote": null,
                        "emote": {
                            "id": "emotesv2_dcd06b30a5c24f6eb871e8f5edbd44f7",
                            "emote_set_id": "0",
                            "owner_id": "0",
                            "format": [
                                "static",
                                "animated"
                            ]
                        },
                        "mention": null
                    }
                ]
            },
            "color": "#FF0000",
            "badges": [
                {
                    "set_id": "no_video",
                    "id": "1",
                    "info": ""
                }
            ],
            "message_type": "text",
            "cheer": null,
            "reply": null,
            "channel_points_custom_reward_id": null
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

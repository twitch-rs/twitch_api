#![doc(alias = "channel.chat.clear_user_messages")]
//! A moderator or bot clears all messages for a specific user.

use super::*;
/// [`channel.chat.clear_user_messages`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatclear_user_messages): a moderator or bot clears all messages for a specific user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatClearUserMessagesV1 {
    /// User ID of the channel to receive chat clear user messages events for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID to read chat as.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub user_id: types::UserId,
}

impl ChannelChatClearUserMessagesV1 {
    /// Get chat clear user messages on broadcasters channel reading chat as a specific user.
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

impl EventSubscription for ChannelChatClearUserMessagesV1 {
    type Payload = ChannelChatClearUserMessagesV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelChatClearUserMessages;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadChat];
    const VERSION: &'static str = "1";
}

/// [`channel.chat.clear_user_messages`](ChannelChatClearUserMessagesV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatClearUserMessagesV1Payload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The ID of the user that was banned or put in a timeout.
    ///
    /// All of their messages are deleted.
    pub target_user_id: types::UserId,
    /// The user name of the user that was banned or put in a timeout.
    pub target_user_name: types::DisplayName,
    /// The user login of the user that was banned or put in a timeout.
    pub target_user_login: types::UserName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.chat.clear_user_messages",
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
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "Cool_User",
            "broadcaster_user_login": "cool_user",
            "target_user_id": "7734",
            "target_user_name": "Uncool_viewer",
            "target_user_login": "uncool_viewer"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

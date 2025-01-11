#![doc(alias = "channel.chat_settings.update")]
//! a broadcaster’s chat settings are updated.

use super::*;
/// [`channel.chat_settings.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate): a broadcaster’s chat settings are updated.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatSettingsUpdateV1 {
    /// User ID of the channel to receive chat settings update events for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID to read chat as.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub user_id: types::UserId,
}

impl ChannelChatSettingsUpdateV1 {
    /// Get notifications for updates on chat settings in this channel as a user
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

impl EventSubscription for ChannelChatSettingsUpdateV1 {
    type Payload = ChannelChatSettingsUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelChatSettingsUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadChat];
    const VERSION: &'static str = "1";
}

/// [`channel.chat_settings.update`](ChannelChatSettingsUpdateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatSettingsUpdateV1Payload {
    /// The ID of the broadcaster specified in the request.
    pub broadcaster_user_id: types::UserId,
    /// The login of the broadcaster specified in the request.
    pub broadcaster_user_login: types::UserName,
    /// The user name of the broadcaster specified in the request.
    pub broadcaster_user_name: types::DisplayName,

    /// A Boolean value that determines whether chat messages must contain only emotes.
    ///
    /// True if only messages that are 100% emotes are allowed; otherwise false.
    pub emote_mode: bool,
    /// A Boolean value that determines whether the broadcaster restricts the chat room to followers only, based on how long they’ve followed.
    ///
    /// True if the broadcaster restricts the chat room to followers only; otherwise false.
    ///
    /// See [follower_mode_duration_minutes][Self::follower_mode_duration_minutes] for how long the followers must have followed the broadcaster to participate in the chat room.
    pub follower_mode: bool,
    /// The length of time, in minutes, that the followers must have followed the broadcaster to participate in the chat room. See [follower_mode][Self::follower_mode].
    ///
    /// [None] if [follower_mode][Self::follower_mode] is false.
    pub follower_mode_duration_minutes: Option<usize>,
    /// A Boolean value that determines whether the broadcaster limits how often users in the chat room are allowed to send messages.
    ///
    /// Is true, if the broadcaster applies a delay; otherwise, false.
    ///
    /// See [slow_mode_wait_time_seconds][Self::slow_mode_wait_time_seconds] for the delay.
    pub slow_mode: bool,
    /// The amount of time, in seconds, that users need to wait between sending messages. See [slow_mode][Self::slow_mode].
    ///
    /// [None] if [slow_mode][Self::slow_mode] is false.
    pub slow_mode_wait_time_seconds: Option<usize>,
    /// A Boolean value that determines whether only users that subscribe to the broadcaster’s channel can talk in the chat room.
    ///
    /// True if the broadcaster restricts the chat room to subscribers only; otherwise false.
    pub subscriber_mode: bool,
    /// A Boolean value that determines whether the broadcaster requires users to post only unique messages in the chat room.
    ///
    /// True if the broadcaster requires unique messages only; otherwise false.
    pub unique_chat_mode: bool,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.chat_settings.update",
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
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "emote_mode": true,
            "follower_mode": false,
            "follower_mode_duration_minutes": null,
            "slow_mode": true,
            "slow_mode_wait_time_seconds": 10,
            "subscriber_mode": false,
            "unique_chat_mode": false
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelChatSettingsUpdateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert!(notif.emote_mode);
    assert!(!notif.follower_mode);
    assert!(notif.follower_mode_duration_minutes.is_none());
    assert!(notif.slow_mode);
    assert_eq!(notif.slow_mode_wait_time_seconds, Some(10));
    assert!(!notif.subscriber_mode);
    assert!(!notif.unique_chat_mode);
}

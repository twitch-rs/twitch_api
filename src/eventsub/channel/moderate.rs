#![doc(alias = "channel.moderate")]
//! a moderator performs a moderation action in a channel.

use super::*;
/// [`channel.moderate`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate): a moderator performs a moderation action in a channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelModerateV1 {
    /// The user ID of the broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelModerateV1 {
    /// Get moderation notifications in the specified channel as a moderator.
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

impl EventSubscription for ChannelModerateV1 {
    type Payload = ChannelModerateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelModerate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![all(
        any(
            twitch_oauth2::Scope::ModeratorReadBlockedTerms,
            twitch_oauth2::Scope::ModeratorManageBlockedTerms
        ),
        any(
            twitch_oauth2::Scope::ModeratorReadChatSettings,
            twitch_oauth2::Scope::ModeratorManageChatSettings
        ),
        any(
            twitch_oauth2::Scope::ModeratorReadUnbanRequests,
            twitch_oauth2::Scope::ModeratorManageUnbanRequests
        ),
        any(
            twitch_oauth2::Scope::ModeratorReadBannedUsers,
            twitch_oauth2::Scope::ModeratorManageBannedUsers
        ),
        any(
            twitch_oauth2::Scope::ModeratorReadChatMessages,
            twitch_oauth2::Scope::ModeratorManageChatMessages
        ),
        twitch_oauth2::Scope::ModeratorReadModerators,
        twitch_oauth2::Scope::ModeratorReadVips,
    )];
    const VERSION: &'static str = "1";
}

// XXX: this struct can never be deny_unknown_fields as it has flattened fields
/// [`channel.moderate`](ChannelModerateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChannelModerateV1Payload {
    /// The ID of the broadcaster.
    pub broadcaster_user_id: types::UserId,
    /// The login of the broadcaster.
    pub broadcaster_user_login: types::UserName,
    /// The user name of the broadcaster.
    pub broadcaster_user_name: types::DisplayName,

    /// The channel in which the action originally occurred. Is the same as the `broadcaster_user_id` if not in shared chat.
    pub source_broadcaster_user_id: Option<types::UserId>,
    /// The channel in which the action originally occurred. Is the same as the `broadcaster_user_login` if not in shared chat.
    pub source_broadcaster_user_login: Option<types::UserName>,
    /// The channel in which the action originally occurred.
    ///
    /// Is [None] when the moderator action happens in the same channel as the broadcaster.
    /// Is [Some] when in a shared chat session, and the action happens in the channel of a participant other than the broadcaster.
    pub source_broadcaster_user_name: Option<types::DisplayName>,

    /// The broadcaster user ID.
    pub moderator_user_id: types::UserId,
    /// The broadcaster display name.
    pub moderator_user_name: types::DisplayName,
    /// The broadcaster login.
    pub moderator_user_login: types::UserName,
    /// The action being taken
    #[serde(flatten)]
    pub action: ActionV1,
}

/// All possible actions in [`ChannelModerateV1Payload`]
// XXX: this struct can never be deny_unknown_fields as it is flattened
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ActionV1 {
    /// Chat is now in followers only mode.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Followers(Followers),
    /// Chat is now in slow mode.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Slow(Slow),
    /// A VIP was added.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Vip(Vip),
    /// A VIP was removed.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unvip(Unvip),
    /// A moderator was added.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Mod(Mod),
    /// A moderator was removed.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unmod(Unmod),
    /// A user was banned.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Ban(Ban),
    /// A user was unbanned.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unban(Unban),
    /// A user was timed out.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Timeout(Timeout),
    /// A user was untimed out.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Untimeout(Untimeout),
    /// A channel is being raided.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Raid(Raid),
    /// A raid was cancelled.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unraid(Unraid),
    /// A message was deleted.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Delete(Delete),
    /// An unban request was approved
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    ApproveUnbanRequest(UnbanRequest),
    /// An unban request was denied
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    DenyUnbanRequest(UnbanRequest),
    /// A ban in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatBan(SharedChatBan),
    /// An unban in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatUnban(SharedChatUnban),
    /// A timeout in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatTimeout(SharedChatTimeout),
    /// An untimeout in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatUntimeout(SharedChatUntimeout),
    /// A message deletion in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatDelete(SharedChatDelete),
    /// Chat is now in emote only mode
    #[serde(rename = "emoteonly")]
    EmoteOnly,
    /// Chat is no longer in emote only mode
    #[serde(rename = "emoteonlyoff")]
    EmoteOnlyOff,
    /// Chat is no longer in followers only mode
    #[serde(rename = "followersoff")]
    FollowersOff,
    /// Chat is now in uniquechat mode
    Uniquechat,
    /// Chat is no longer in uniquechat mode
    #[serde(rename = "uniquechatoff")]
    UniquechatOff,
    /// Chat is no longer in slow mode
    #[serde(rename = "slowoff")]
    SlowOff,
    /// Chat is now in subscribers only mode
    Subscribers,
    /// Chat is now longer in subscribers only mode
    SubscribersOff,
    /// A blocked term was added
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    AddBlockedTerm(AutomodTerms),
    /// A permitted term was added
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    AddPermittedTerm(AutomodTerms),
    /// A blocked term was removed
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    RemoveBlockedTerm(AutomodTerms),
    /// A permitted term was removed
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    RemovePermittedTerm(AutomodTerms),
}

// MARK: V1 Actions

/// Metadata associated with the followers command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Followers {
    /// The length of time, in minutes, that the followers must have followed the broadcaster to participate in the chat room.
    pub follow_duration_minutes: usize,
}

/// Metadata associated with the slow command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Slow {
    /// The amount of time, in seconds, that users need to wait between sending messages.
    pub wait_time_seconds: usize,
}

/// Metadata associated with the vip command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Vip {
    /// The ID of the user gaining VIP status.
    pub user_id: types::UserId,
    /// The login of the user gaining VIP status.
    pub user_login: types::UserName,
    /// The user name of the user gaining VIP status.
    pub user_name: types::DisplayName,
}

/// Metadata associated with the unvip command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Unvip {
    /// The ID of the user losing VIP status.
    pub user_id: types::UserId,
    /// The login of the user losing VIP status.
    pub user_login: types::UserName,
    /// The user name of the user losing VIP status.
    pub user_name: types::DisplayName,
}

/// Metadata associated with the mod command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Mod {
    /// The ID of the user gaining mod status.
    pub user_id: types::UserId,
    /// The login of the user gaining mod status.
    pub user_login: types::UserName,
    /// The user name of the user gaining mod status.
    pub user_name: types::DisplayName,
}

/// Metadata associated with the unmod command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Unmod {
    /// The ID of the user losing mod status.
    pub user_id: types::UserId,
    /// The login of the user losing mod status.
    pub user_login: types::UserName,
    /// The user name of the user losing mod status.
    pub user_name: types::DisplayName,
}

/// Metadata associated with the ban command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Ban {
    /// The ID of the user being banned.
    pub user_id: types::UserId,
    /// The login of the user being banned.
    pub user_login: types::UserName,
    /// The user name of the user being banned.
    pub user_name: types::DisplayName,
    /// Reason given for the ban.
    #[serde(
        default,
        deserialize_with = "crate::deserialize_none_from_empty_string"
    )]
    pub reason: Option<String>,
}

/// Metadata associated with the unban command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Unban {
    /// The ID of the user being unbanned.
    pub user_id: types::UserId,
    /// The login of the user being unbanned.
    pub user_login: types::UserName,
    /// The user name of the user being unbanned.
    pub user_name: types::DisplayName,
}

/// Metadata associated with the timeout command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Timeout {
    /// The ID of the user being timed out.
    pub user_id: types::UserId,
    /// The login of the user being timed out.
    pub user_login: types::UserName,
    /// The user name of the user being timed out.
    pub user_name: types::DisplayName,
    /// The reason given for the timeout.
    #[serde(
        default,
        deserialize_with = "crate::deserialize_none_from_empty_string"
    )]
    pub reason: Option<String>,
    /// The time at which the timeout ends.
    pub expires_at: types::Timestamp,
}

/// Metadata associated with the untimeout command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Untimeout {
    /// The ID of the user being untimed out.
    pub user_id: types::UserId,
    /// The login of the user being untimed out.
    pub user_login: types::UserName,
    /// The user name of the user untimed out.
    pub user_name: types::DisplayName,
}

/// Metadata associated with the raid command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Raid {
    /// The ID of the user being raided.
    pub user_id: types::UserId,
    /// The login of the user being raided.
    pub user_login: types::UserName,
    /// The user name of the user raided.
    pub user_name: types::DisplayName,
    /// The viewer count.
    pub viewer_count: usize,
}

/// Metadata associated with the unraid command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Unraid {
    /// The ID of the user no longer being raided.
    pub user_id: types::UserId,
    /// The login of the user no longer being raided.
    pub user_login: types::UserName,
    /// The user name of the no longer user raided.
    pub user_name: types::DisplayName,
}

/// Metadata associated with the delete command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Delete {
    /// The ID of the user whose message is being deleted.
    pub user_id: types::UserId,
    /// The login of the user.
    pub user_login: types::UserName,
    /// The user name of the user.
    pub user_name: types::DisplayName,
    /// The ID of the message being deleted.
    pub message_id: types::MsgId,
    /// The message body of the message being deleted.
    pub message_body: String,
}

/// Metadata associated with the automod terms changes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodTerms {
    /// The action being taken on the list
    pub action: AutomodTermAction,
    /// The affected list
    pub list: AutomodTermList,
    /// Terms being added or removed.
    pub terms: Vec<String>,
    /// Whether the terms were added due to an Automod message approve/deny action.
    pub from_automod: bool,
}

/// An action on an Automod term
// XXX: this is similar to the one from automod::terms::update, but it doesn't combine the action and list
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AutomodTermAction {
    /// A term was added to a list
    Add,
    /// A term was removed from a list
    Remove,
    /// An unknown term action, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

/// A list with Automod terms
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AutomodTermList {
    /// The list of blocked terms
    Blocked,
    /// The list of permitted terms
    Permitted,
    /// An unknown term list, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

/// Metadata associated with an unban request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UnbanRequest {
    /// Whether or not the unban request was approved or denied.
    pub is_approved: bool,
    /// The ID of the banned user.
    pub user_id: types::UserId,
    /// The login of the user.
    pub user_login: types::UserName,
    /// The user name of the user.
    pub user_name: types::DisplayName,
    /// The message included by the moderator explaining their approval or denial.
    pub moderator_message: String,
}

/// Information about the `shared_chat_ban` event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(transparent)]
#[repr(transparent)]
pub struct SharedChatBan(pub Ban);

/// Information about the `shared_chat_unban` event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(transparent)]
#[repr(transparent)]
pub struct SharedChatUnban(pub Unban);

/// Information about the `shared_chat_timeout` event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(transparent)]
#[repr(transparent)]
pub struct SharedChatTimeout(pub Timeout);

/// Information about the `shared_chat_untimeout` event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(transparent)]
#[repr(transparent)]
pub struct SharedChatUntimeout(pub Untimeout);

/// Information about the `shared_chat_delete` event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(transparent)]
#[repr(transparent)]
pub struct SharedChatDelete(pub Delete);

macro_rules! named_fields {
    ($($typ:ty = $name:literal),*) => {
        $(
            impl crate::eventsub::NamedField for $typ {
                const NAME: &'static str = $name;
            }
        )*
    };
}

named_fields! {
    Followers = "followers",
    Slow = "slow",
    Vip = "vip",
    Unvip = "unvip",
    Mod = "mod",
    Unmod = "unmod",
    Ban = "ban",
    Unban = "unban",
    Timeout = "timeout",
    Untimeout = "untimeout",
    Raid = "raid",
    Unraid = "unraid",
    Delete = "delete",
    AutomodTerms = "automod_terms",
    UnbanRequest = "unban_request",
    SharedChatBan = "shared_chat_ban",
    SharedChatUnban = "shared_chat_unban",
    SharedChatTimeout = "shared_chat_timeout",
    SharedChatUntimeout = "shared_chat_untimeout",
    SharedChatDelete = "shared_chat_delete"
}

// MARK: V1 Tests

#[cfg(test)]
#[test]
fn parse_payload_v1_timeout() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "53be57fd-baa0-470c-af30-f8f0958f7f0b",
            "status": "enabled",
            "type": "channel.moderate",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQ2sjLsebwRk2kuZ5zFe2CFRIGY2VsbC1j"
            },
            "created_at": "2024-11-27T18:12:03.396116773Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "source_broadcaster_user_id": null,
            "source_broadcaster_user_login": null,
            "source_broadcaster_user_name": null,
            "moderator_user_id": "129546453",
            "moderator_user_login": "nerixyz",
            "moderator_user_name": "nerixyz",
            "action": "timeout",
            "followers": null,
            "slow": null,
            "vip": null,
            "unvip": null,
            "mod": null,
            "unmod": null,
            "ban": null,
            "unban": null,
            "timeout": {
                "user_id": "141981764",
                "user_login": "twitchdev",
                "user_name": "TwitchDev",
                "reason": "test Kappa",
                "expires_at": "2024-11-27T18:12:43.640505703Z"
            },
            "untimeout": null,
            "raid": null,
            "unraid": null,
            "delete": null,
            "automod_terms": null,
            "unban_request": null,
            "shared_chat_ban": null,
            "shared_chat_unban": null,
            "shared_chat_timeout": null,
            "shared_chat_untimeout": null,
            "shared_chat_delete": null
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModerateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.source_broadcaster_user_id, None);
    assert_eq!(notif.moderator_user_id.as_str(), "129546453");
    let ActionV1::Timeout(action) = notif.action else {
        panic!("invalid action");
    };
    assert_eq!(action.user_id.as_str(), "141981764");
}

#[cfg(test)]
#[test]
fn parse_payload_v1_emoteonly() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "channel.moderate",
            "version": "1",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337",
                "moderator_user_id": "1337"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z"
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "glowillig",
            "broadcaster_user_name": "glowillig",
            "moderator_user_id": "424596340",
            "moderator_user_login": "quotrok",
            "moderator_user_name": "quotrok",
            "action": "emoteonly",
            "followers": null,
            "slow": null,
            "vip": null,
            "unvip": null,
            "mod": null,
            "unmod": null,
            "ban": null,
            "unban": null,
            "timeout": null,
            "untimeout": null,
            "raid": null,
            "unraid": null,
            "delete": null,
            "automod_terms": null,
            "unban_request": null,
            "shared_chat_ban": null,
            "shared_chat_unban": null,
            "shared_chat_timeout": null,
            "shared_chat_untimeout": null,
            "shared_chat_delete": null
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModerateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert_eq!(notif.source_broadcaster_user_id, None);
    assert_eq!(notif.moderator_user_id.as_str(), "424596340");
    assert_eq!(notif.action, ActionV1::EmoteOnly);
}

// MARK: V2

/// [`channel.moderate`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate): a moderator performs a moderation action in a channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelModerateV2 {
    /// The user ID of the broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelModerateV2 {
    /// Get moderation notifications in the specified channel as a moderator.
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

impl EventSubscription for ChannelModerateV2 {
    type Payload = ChannelModerateV2Payload;

    const EVENT_TYPE: EventType = EventType::ChannelModerate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![all(
        any(
            twitch_oauth2::Scope::ModeratorReadBlockedTerms,
            twitch_oauth2::Scope::ModeratorManageBlockedTerms
        ),
        any(
            twitch_oauth2::Scope::ModeratorReadChatSettings,
            twitch_oauth2::Scope::ModeratorManageChatSettings
        ),
        any(
            twitch_oauth2::Scope::ModeratorReadUnbanRequests,
            twitch_oauth2::Scope::ModeratorManageUnbanRequests
        ),
        any(
            twitch_oauth2::Scope::ModeratorReadBannedUsers,
            twitch_oauth2::Scope::ModeratorManageBannedUsers
        ),
        any(
            twitch_oauth2::Scope::ModeratorReadChatMessages,
            twitch_oauth2::Scope::ModeratorManageChatMessages
        ),
        twitch_oauth2::Scope::ModeratorReadModerators,
        twitch_oauth2::Scope::ModeratorReadVips,
        // new:
        any(
            twitch_oauth2::Scope::ModeratorReadWarnings,
            twitch_oauth2::Scope::ModeratorManageWarnings
        ),
    )];
    const VERSION: &'static str = "2";
}

// XXX: this struct can never be deny_unknown_fields as it has flattened fields
/// [`channel.moderate`](ChannelModerateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChannelModerateV2Payload {
    /// The ID of the broadcaster.
    pub broadcaster_user_id: types::UserId,
    /// The login of the broadcaster.
    pub broadcaster_user_login: types::UserName,
    /// The user name of the broadcaster.
    pub broadcaster_user_name: types::DisplayName,

    /// The channel in which the action originally occurred. Is the same as the `broadcaster_user_id`` if not in shared chat.
    pub source_broadcaster_user_id: Option<types::UserId>,
    /// The channel in which the action originally occurred. Is the same as the `broadcaster_user_login`` if not in shared chat.
    pub source_broadcaster_user_login: Option<types::UserName>,
    /// The channel in which the action originally occurred.
    ///
    /// Is [None] when the moderator action happens in the same channel as the broadcaster.
    /// Is [Some] when in a shared chat session, and the action happens in the channel of a participant other than the broadcaster.
    pub source_broadcaster_user_name: Option<types::DisplayName>,

    /// The broadcaster user ID.
    pub moderator_user_id: types::UserId,
    /// The broadcaster display name.
    pub moderator_user_name: types::DisplayName,
    /// The broadcaster login.
    pub moderator_user_login: types::UserName,
    /// The action being taken
    #[serde(flatten)]
    pub action: ActionV2,
}

/// All possible actions in [`ChannelModerateV1Payload`]
// XXX: this struct can never be deny_unknown_fields as it is flattened
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ActionV2 {
    /// Chat is now in followers only mode.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Followers(Followers),
    /// Chat is now in slow mode.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Slow(Slow),
    /// A VIP was added.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Vip(Vip),
    /// A VIP was removed.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unvip(Unvip),
    /// A moderator was added.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Mod(Mod),
    /// A moderator was removed.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unmod(Unmod),
    /// A user was banned.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Ban(Ban),
    /// A user was unbanned.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unban(Unban),
    /// A user was timed out.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Timeout(Timeout),
    /// A user was untimed out.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Untimeout(Untimeout),
    /// A channel is being raided.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Raid(Raid),
    /// A raid was cancelled.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unraid(Unraid),
    /// A message was deleted.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Delete(Delete),
    /// An unban request was approved
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    ApproveUnbanRequest(UnbanRequest),
    /// An unban request was denied
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    DenyUnbanRequest(UnbanRequest),
    /// A ban in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatBan(SharedChatBan),
    /// An unban in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatUnban(SharedChatUnban),
    /// A timeout in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatTimeout(SharedChatTimeout),
    /// An untimeout in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatUntimeout(SharedChatUntimeout),
    /// A message deletion in a shared chat session
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SharedChatDelete(SharedChatDelete),
    /// Chat is now in emote only mode
    #[serde(rename = "emoteonly")]
    EmoteOnly,
    /// Chat is no longer in emote only mode
    #[serde(rename = "emoteonlyoff")]
    EmoteOnlyOff,
    /// Chat is no longer in followers only mode
    #[serde(rename = "followersoff")]
    FollowersOff,
    /// Chat is now in uniquechat mode
    Uniquechat,
    /// Chat is no longer in uniquechat mode
    #[serde(rename = "uniquechatoff")]
    UniquechatOff,
    /// Chat is no longer in slow mode
    #[serde(rename = "slowoff")]
    SlowOff,
    /// Chat is now in subscribers only mode
    Subscribers,
    /// Chat is now longer in subscribers only mode
    SubscribersOff,
    /// A blocked term was added
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    AddBlockedTerm(AutomodTerms),
    /// A permitted term was added
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    AddPermittedTerm(AutomodTerms),
    /// A blocked term was removed
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    RemoveBlockedTerm(AutomodTerms),
    /// A permitted term was removed
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    RemovePermittedTerm(AutomodTerms),
    /// A user was warned
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Warn(Warn),
}

/// Metadata associated with the warn command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Warn {
    /// The ID of the user being warned.
    pub user_id: types::UserId,
    /// The login of the user being warned.
    pub user_login: types::UserName,
    /// The user name of the user being warned.
    pub user_name: types::DisplayName,
    /// Reason given for the warning.
    #[serde(
        default,
        deserialize_with = "crate::deserialize_none_from_empty_string"
    )]
    pub reason: Option<String>,
    /// Chat rules cited for the warning.
    pub chat_rules_cited: Option<Vec<String>>,
}

named_fields! { Warn = "warn" }

// MARK: V2 Tests

#[cfg(test)]
#[test]
fn parse_payload_v2_timeout() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "5a893cd8-c10e-4198-a620-e9f35fd6ccd6",
            "status": "enabled",
            "type": "channel.moderate",
            "version": "2",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQMmDnfVEhRI6iLiBniEWHkxIGY2VsbC1j"
            },
            "created_at": "2024-11-27T20:01:50.171283653Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "source_broadcaster_user_id": null,
            "source_broadcaster_user_login": null,
            "source_broadcaster_user_name": null,
            "moderator_user_id": "129546453",
            "moderator_user_login": "nerixyz",
            "moderator_user_name": "nerixyz",
            "action": "timeout",
            "followers": null,
            "slow": null,
            "vip": null,
            "unvip": null,
            "mod": null,
            "unmod": null,
            "ban": null,
            "unban": null,
            "timeout": {
                "user_id": "141981764",
                "user_login": "twitchdev",
                "user_name": "TwitchDev",
                "reason": "test Kappa",
                "expires_at": "2024-11-27T20:01:55.358149527Z"
            },
            "untimeout": null,
            "raid": null,
            "unraid": null,
            "delete": null,
            "automod_terms": null,
            "unban_request": null,
            "warn": null,
            "shared_chat_ban": null,
            "shared_chat_unban": null,
            "shared_chat_timeout": null,
            "shared_chat_untimeout": null,
            "shared_chat_delete": null
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModerateV2(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.source_broadcaster_user_id, None);
    assert_eq!(notif.moderator_user_id.as_str(), "129546453");
    let ActionV2::Timeout(action) = notif.action else {
        panic!("invalid action");
    };
    assert_eq!(action.user_id.as_str(), "141981764");
}

#[cfg(test)]
#[test]
fn parse_payload_v2_warn() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "channel.moderate",
            "version": "2",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "423374343",
                "moderator_user_id": "424596340"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z"
        },
        "event": {
            "broadcaster_user_id": "423374343",
            "broadcaster_user_login": "glowillig",
            "broadcaster_user_name": "glowillig",
            "source_broadcaster_user_id": "41292030",
            "source_broadcaster_user_login": "adflynn404",
            "source_broadcaster_user_name": "adflynn404",
            "moderator_user_id": "424596340",
            "moderator_user_login": "quotrok",
            "moderator_user_name": "quotrok",
            "action": "warn",
            "followers": null,
            "slow": null,
            "vip": null,
            "unvip": null,
            "warn": {
                "user_id": "141981764",
                "user_login": "twitchdev",
                "user_name": "TwitchDev",
                "reason": "cut it out",
                "chat_rules_cited": null
            },
            "unmod": null,
            "ban": null,
            "unban": null,
            "timeout": null,
            "untimeout": null,
            "raid": null,
            "unraid": null,
            "delete": null,
            "automod_terms": null,
            "unban_request": null,
            "shared_chat_ban": null,
            "shared_chat_unban": null,
            "shared_chat_timeout": null,
            "shared_chat_untimeout": null,
            "shared_chat_delete": null
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModerateV2(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "423374343");
    assert_eq!(
        notif.source_broadcaster_user_id.unwrap().as_str(),
        "41292030"
    );
    assert_eq!(notif.moderator_user_id.as_str(), "424596340");
    let ActionV2::Warn(action) = notif.action else {
        panic!("invalid action");
    };
    assert_eq!(action.user_id.as_str(), "141981764");
    assert_eq!(action.reason.unwrap(), "cut it out");
    assert_eq!(action.chat_rules_cited, None);
}

#[cfg(test)]
#[test]
fn parse_payload_v2_shared_chat_timeout() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "channel.moderate",
            "version": "2",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "423374343",
                "moderator_user_id": "424596340"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z"
        },
        "event": {
            "broadcaster_user_id": "423374343",
            "broadcaster_user_login": "glowillig",
            "broadcaster_user_name": "glowillig",
            "source_broadcaster_user_id": "41292030",
            "source_broadcaster_user_login": "adflynn404",
            "source_broadcaster_user_name": "adflynn404",
            "moderator_user_id": "424596340",
            "moderator_user_login": "quotrok",
            "moderator_user_name": "quotrok",
            "action": "shared_chat_timeout",
            "followers": null,
            "slow": null,
            "vip": null,
            "unvip": null,
            "warn": null,
            "unmod": null,
            "ban": null,
            "unban": null,
            "timeout": null,
            "untimeout": null,
            "raid": null,
            "unraid": null,
            "delete": null,
            "automod_terms": null,
            "unban_request": null,
            "shared_chat_ban": null,
            "shared_chat_unban": null,
            "shared_chat_timeout": {
                "user_id": "141981764",
                "user_login": "twitchdev",
                "user_name": "TwitchDev",
                "reason": "Has never seen the Harry Potter films.",
                "expires_at": "2022-03-15T02:00:28Z"
            },
            "shared_chat_untimeout": null,
            "shared_chat_delete": null
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModerateV2(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "423374343");
    assert_eq!(
        notif.source_broadcaster_user_id.unwrap().as_str(),
        "41292030"
    );
    assert_eq!(notif.moderator_user_id.as_str(), "424596340");
    let ActionV2::SharedChatTimeout(SharedChatTimeout(action)) = notif.action else {
        panic!("invalid action");
    };
    assert_eq!(action.user_id.as_str(), "141981764");
    assert_eq!(
        action.reason.unwrap(),
        "Has never seen the Harry Potter films."
    );
}

#[cfg(test)]
#[test]
fn parse_payload_v2_mod() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "channel.moderate",
            "version": "2",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "423374343",
                "moderator_user_id": "423374343"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z"
        },
        "event": {
            "broadcaster_user_id": "423374343",
            "broadcaster_user_login": "glowillig",
            "broadcaster_user_name": "glowillig",
            "moderator_user_id": "424596340",
            "moderator_user_login": "quotrok",
            "moderator_user_name": "quotrok",
            "action": "mod",
            "followers": null,
            "slow": null,
            "vip": null,
            "unvip": null,
            "warn": null,
            "mod": {
                "user_id": "141981764",
                "user_login": "twitchdev",
                "user_name": "TwitchDev"
            },
            "unmod": null,
            "ban": null,
            "unban": null,
            "timeout": null,
            "untimeout": null,
            "raid": null,
            "unraid": null,
            "delete": null,
            "automod_terms": null,
            "unban_request": null,
            "shared_chat_ban": null,
            "shared_chat_unban": null,
            "shared_chat_timeout": null,
            "shared_chat_untimeout": null,
            "shared_chat_delete": null
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModerateV2(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "423374343");
    assert_eq!(notif.source_broadcaster_user_id, None);
    assert_eq!(notif.moderator_user_id.as_str(), "424596340");
    let ActionV2::Mod(action) = notif.action else {
        panic!("invalid action");
    };
    assert_eq!(action.user_id.as_str(), "141981764");
}

#[cfg(test)]
#[test]
fn parse_payload_v2_emoteonly() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "channel.moderate",
            "version": "2",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "423374343",
                "moderator_user_id": "423374343"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z"
        },
        "event": {
            "broadcaster_user_id": "423374343",
            "broadcaster_user_login": "glowillig",
            "broadcaster_user_name": "glowillig",
            "moderator_user_id": "424596340",
            "moderator_user_login": "quotrok",
            "moderator_user_name": "quotrok",
            "action": "emoteonly",
            "followers": null,
            "slow": null,
            "vip": null,
            "unvip": null,
            "mod": null,
            "unmod": null,
            "ban": null,
            "unban": null,
            "warn": null,
            "timeout": null,
            "untimeout": null,
            "raid": null,
            "unraid": null,
            "delete": null,
            "automod_terms": null,
            "unban_request": null,
            "shared_chat_ban": null,
            "shared_chat_unban": null,
            "shared_chat_timeout": null,
            "shared_chat_untimeout": null,
            "shared_chat_delete": null
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelModerateV2(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "423374343");
    assert_eq!(notif.source_broadcaster_user_id, None);
    assert_eq!(notif.moderator_user_id.as_str(), "424596340");
    assert_eq!(notif.action, ActionV2::EmoteOnly);
}

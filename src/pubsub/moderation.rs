//! PubSub messages for moderator actions
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A moderator performs an action in the channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct ChatModeratorActions {
    /// The user_id to listen as. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub user_id: u32,
    /// The channel_id to listen to. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    ChatModeratorActions,
    "chat_moderator_actions",
    user_id,
    channel_id
);

impl pubsub::Topic for ChatModeratorActions {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelModerate];
}

/// A moderation action. `moderation_action`
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ModerationAction {
    /// Arguments for moderation_action
    #[serde(deserialize_with = "pubsub::deserialize_default_from_null")]
    pub args: Vec<String>,
    // FIXME: Should be option::none if empty
    /// User that did moderation action
    pub created_by: types::UserName,
    // FIXME: Should be option::none if empty
    /// ID of user that did moderation action
    pub created_by_user_id: types::UserId,
    /// Moderation action is triggered from automod
    pub from_automod: bool,
    /// Type of action
    pub moderation_action: ModerationActionCommand,
    /// ID of message associated with moderation action
    pub msg_id: Option<String>,
    /// Target of moderation action
    pub target_user_id: types::UserId,
    /// Type of moderation
    #[serde(rename = "type")]
    pub type_: ModerationType,
    // Never filled
    #[doc(hidden)]
    pub target_user_login: Option<String>,
}

/// A moderator was added. `moderator_added`
///
/// # Notes
///
/// There is no `moderator_removed` message
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ModeratorAdded {
    /// ID of channel where moderator was added
    pub channel_id: types::UserId,
    /// ID of added moderator
    pub target_user_id: types::UserId,
    /// Moderation actiom. Should be [`mod`](ModerationActionCommand::Mod)
    pub moderation_action: ModerationActionCommand,
    /// Username of added moderator
    pub target_user_login: types::UserName,
    /// ID of user that added moderator
    pub created_by_user_id: types::UserId,
    /// Username of user that added moderator
    pub created_by: types::UserName,
}

/// Reply from [ChatModeratorActions]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(tag = "type", content = "data")]
#[non_exhaustive]
pub enum ChatModeratorActionsReply {
    /// A moderation action. `moderation_action`
    #[serde(rename = "moderation_action")]
    ModerationAction(ModerationAction),
    /// A moderator was added. `moderator_added`
    #[serde(rename = "moderator_added")]
    ModeratorAdded(ModeratorAdded),
    /// Unban request denied
    #[serde(rename = "deny_unban_request")]
    DenyUnbanRequest(UnbanRequest),
    /// Unban request approved
    #[serde(rename = "approve_unban_request")]
    ApproveUnbanRequest(UnbanRequest),
}

/// Unban request
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UnbanRequest {
    /// Unban response created by user with id
    pub created_by_id: types::UserId,
    /// Unban response created by user with login
    pub created_by_login: types::UserName,
    /// Action taken, should be [ModerationActionCommand::ApproveUnbanRequest] or [ModerationActionCommand::DenyUnbanRequest]
    pub moderation_action: ModerationActionCommand,
    /// Message attached to unban request response
    pub moderator_message: String,
    /// Target user ID of unban request, e.g the user that was banned
    pub target_user_id: types::UserId,
    /// Target login of unban request, e.g the user that was banned
    pub target_user_login: types::UserName,
}

/// A command
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ModerationActionCommand {
    /// Delete
    ///
    /// Given when a message is deleted with `/delete <msg-id>`
    Delete,
    /// Timeout
    ///
    /// Given when a user is timed-out with `/timeout <user> <time> <reason>`
    Timeout,
    /// Untimeout
    ///
    /// Given when a user is unbanned while under a timeout `/untimeout <user>` or `/unban <user>`
    Untimeout,
    /// Mod
    ///
    /// Given when a user is added as a moderator. `/mod <user>`.
    ///
    /// See [ChatModeratorActionsReply::ModeratorAdded] where this is given
    Mod,
    /// Unmod
    ///
    /// Given when a user is removed as a moderator, `/unmod <user>`
    Unmod,
    /// Modified automod properties
    ///
    /// Given when automod config is changed. I.e filtering changed etc
    ModifiedAutomodProperties,
    /// Ban
    ///
    /// Given when a user is banned with `/timeout <user> <reason>`
    Ban,
    /// Unban
    ///
    /// Given when a user is unbanned with `/unban <user>` or `/untimeout <user>`
    Unban,
    /// Automod message rejected
    AutomodRejected,
    /// Automod permitted term added
    AddPermittedTerm,
    /// Automod permitted term removed
    DeletePermittedTerm,
    /// Automod blocked term added
    AddBlockedTerm,
    /// Automod blocked term removed
    DeleteBlockedTerm,
    /// Automod message approved
    ApproveAutomodMessage,
    /// Automod message denied
    DeniedAutomodMessage,
    /// Raid
    ///
    /// Given when editor/broadcaster does `/raid <channel>`
    Raid,
    /// Slow-mode chat enabled
    Slow,
    #[serde(rename = "slowoff")]
    /// Slow-mode chat disabled
    SlowOff,
    /// Followers-only chat enabled
    Followers,
    /// Followers-only chat disabled
    #[serde(rename = "followersoff")]
    FollowersOff,
    /// Subscriber-only chat enabled
    Subscribers,
    /// Subscriber-only chat disabled
    #[serde(rename = "subscribersoff")]
    SubscribersOff,
    /// Emote-only chat enabled
    #[serde(rename = "emoteonly")]
    EmoteOnly,
    /// Emote-only chat disabled
    #[serde(rename = "emoteonlyoff")]
    EmoteOnlyOff,
    /// Chat cleared for all viewers
    Clear,
    /// Unique chat enabled
    #[serde(rename = "r9kbeta")]
    R9KBeta,
    /// Unique chat disabled
    #[serde(rename = "r9kbetaoff")]
    R9KBetaOff,
    /// User added as VIP
    Vip,
    /// User removed as VIP
    Unvip,
    /// Channel host started
    Host,
    /// Channel host removed
    Unhost,
    /// Unban Request Approved
    #[serde(rename = "APPROVE_UNBAN_REQUEST")]
    ApproveUnbanRequest,
    /// Unban Request Denied
    #[serde(rename = "DENY_UNBAN_REQUEST")]
    DenyUnbanRequest,
}

impl std::fmt::Display for ModerationActionCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
}

/// Moderation type
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ModerationType {
    /// Chat moderated
    ChatLoginModeration,
    /// Channel moderated
    ChatChannelModeration,
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::super::{Response, TopicData};
    use super::*;

    #[test]
    #[cfg(feature = "unsupported")]
    fn mod_action_delete() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "chat_moderator_actions.27620241.27620241",
        "message": "{\"type\":\"moderation_action\",\"data\":{\"type\":\"chat_login_moderation\",\"moderation_action\":\"delete\",\"args\":[\"tmo\",\"bop\",\"e513c02d-dca5-4480-9af5-e6078d954e42\"],\"created_by\":\"emilgardis\",\"created_by_user_id\":\"27620241\",\"msg_id\":\"\",\"target_user_id\":\"1234\",\"target_user_login\":\"\",\"from_automod\":false}}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }
    #[test]
    #[cfg(feature = "unsupported")]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "chat_moderator_actions.1337.1234";
        assert_eq!(
            ChatModeratorActions {
                user_id: 1337,
                channel_id: 1234,
            },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "unsupported")]
    fn check_ser() {
        let s = "chat_moderator_actions.1337.1234";
        let right: String = ChatModeratorActions {
            user_id: 1337,
            channel_id: 1234,
        }
        .into();
        assert_eq!(s.to_string(), right);
    }

    #[test]
    #[cfg(feature = "unsupported")]
    fn mod_action_timeout() {
        let source = r#"{"type":"MESSAGE","data":{"topic":"chat_moderator_actions.27620241.27620241","message":"{\"type\":\"moderation_action\",\"data\":{\"type\":\"chat_login_moderation\",\"moderation_action\":\"timeout\",\"args\":[\"tmo\",\"1\",\"\"],\"created_by\":\"emilgardis\",\"created_by_user_id\":\"27620241\",\"msg_id\":\"\",\"target_user_id\":\"1234\",\"target_user_login\":\"\",\"from_automod\":false}}"}}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }
    #[test]
    #[cfg(feature = "unsupported")]
    fn mod_add_moderator() {
        let source = r#"{"type":"MESSAGE","data":{"topic":"chat_moderator_actions.27620241.27620241","message":"{\"type\":\"moderator_added\",  \"data\":{\"channel_id\":\"27620241\",\"target_user_id\":\"19264788\",\"moderation_action\":\"mod\",\"target_user_login\":\"nightbot\",\"created_by_user_id\":\"27620241\",\"created_by\":\"emilgardis\"}}"}}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }

    #[test]
    #[cfg(feature = "unsupported")]
    fn mod_add_moderator_no_user_id() {
        let source = r#"{"type":"MESSAGE","data":{"topic":"chat_moderator_actions.27620241.27620241","message":"{\"type\":\"moderator_added\",  \"data\":{\"channel_id\":\"27620241\",\"target_user_id\":\"19264788\",\"moderation_action\":\"mod\",\"target_user_login\":\"nightbot\",\"created_by_user_id\":\"27620241\",\"created_by\":\"emilgardis\"}}"}}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }
    #[test]
    #[cfg(feature = "unsupported")]
    fn mod_automod() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "chat_moderator_actions.27620241.27620241",
        "message": "{\"type\":\"moderation_action\",\"data\":{\"type\":\"chat_channel_moderation\",\"moderation_action\":\"modified_automod_properties\",\"args\":null,\"created_by\":\"emilgardis\",\"created_by_user_id\":\"27620241\",\"msg_id\":\"\",\"target_user_id\":\"\",\"target_user_login\":\"\",\"from_automod\":false}}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }

    #[test]
    #[cfg(feature = "unsupported")]
    fn mod_automod_delete_blocked_term() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "chat_moderator_actions.27620241.27620241",
        "message": "{\"type\":\"moderation_action\",\"data\":{\"type\":\"chat_channel_moderation\",\"moderation_action\":\"delete_blocked_term\",\"args\":[\"cunt dick pussy vagina\"],\"created_by\":\"emilgardis\",\"created_by_user_id\":\"27620241\",\"msg_id\":\"\",\"target_user_id\":\"\",\"target_user_login\":\"\",\"from_automod\":false}}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }

    #[test]
    #[cfg(feature = "unsupported")]
    fn mod_slowmode() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "chat_moderator_actions.27620241.27620241",
        "message": "{\"type\":\"moderation_action\",\"data\":{\"type\":\"chat_channel_moderation\",\"moderation_action\":\"slow\",\"args\":[\"5\"],\"created_by\":\"tmo\",\"created_by_user_id\":\"1234\",\"msg_id\":\"\",\"target_user_id\":\"\",\"target_user_login\":\"\",\"from_automod\":false}}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }

    #[test]
    #[cfg(feature = "allow_unknown_fields")]
    fn allow_unknown() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "chat_moderator_actions.27620241.27620241",
        "message": "{\"type\":\"moderation_action\",\"data\":{\"type\":\"chat_channel_moderation\",\"moderation_action\":\"slow\",\"unknownfield\": 1,\"args\":[\"5\"],\"created_by\":\"tmo\",\"created_by_user_id\":\"1234\",\"msg_id\":\"\",\"target_user_id\":\"\",\"target_user_login\":\"\",\"from_automod\":false}}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }

    #[test]
    fn deny_unban_request() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "chat_moderator_actions.80525799.80525799",
        "message": "{\"type\":\"deny_unban_request\",\"data\":{\"moderation_action\":\"DENY_UNBAN_REQUEST\",\"created_by_id\":\"27620241\",\"created_by_login\":\"emilgardis\",\"moderator_message\":\"ok\",\"target_user_id\":\"465894629\",\"target_user_login\":\"emil_the_impostor\"}}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }

    #[test]
    fn approve_unban_request() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "chat_moderator_actions.80525799.80525799",
        "message": "{\"type\":\"approve_unban_request\",\"data\":{\"moderation_action\":\"APPROVE_UNBAN_REQUEST\",\"created_by_id\":\"27620241\",\"created_by_login\":\"emilgardis\",\"moderator_message\":\"ok\",\"target_user_id\":\"465894629\",\"target_user_login\":\"emil_the_impostor\"}}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChatModeratorActions { .. },
            }
        ));
    }
}

#![doc(alias = "mod")]
//! Helix endpoints regarding moderation

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod add_blocked_term;
pub mod add_channel_moderator;
pub mod ban_user;
pub mod check_automod_status;
pub mod delete_chat_messages;
pub mod get_banned_users;
pub mod get_blocked_terms;
pub mod get_moderators;
pub mod manage_held_automod_messages;
pub mod remove_blocked_term;
pub mod remove_channel_moderator;
pub mod unban_user;

#[doc(inline)]
pub use add_blocked_term::{AddBlockedTermBody, AddBlockedTermRequest};
#[doc(inline)]
pub use add_channel_moderator::{AddChannelModeratorRequest, AddChannelModeratorResponse};
#[doc(inline)]
pub use ban_user::{BanUser, BanUserBody, BanUserRequest};
#[doc(inline)]
pub use check_automod_status::{
    CheckAutoModStatus, CheckAutoModStatusBody, CheckAutoModStatusRequest,
};
#[doc(inline)]
pub use delete_chat_messages::{DeleteChatMessagesRequest, DeleteChatMessagesResponse};
#[doc(inline)]
pub use get_banned_users::{BannedUser, GetBannedUsersRequest};
#[doc(inline)]
pub use get_moderators::{GetModeratorsRequest, Moderator};
#[doc(inline)]
pub use manage_held_automod_messages::{
    AutoModAction, ManageHeldAutoModMessages, ManageHeldAutoModMessagesBody,
    ManageHeldAutoModMessagesRequest,
};
#[doc(inline)]
pub use remove_blocked_term::{RemoveBlockedTerm, RemoveBlockedTermRequest};
#[doc(inline)]
pub use remove_channel_moderator::{RemoveChannelModeratorRequest, RemoveChannelModeratorResponse};
#[doc(inline)]
pub use unban_user::{UnbanUserRequest, UnbanUserResponse};

/// A blocked term in automod
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BlockedTerm {
    /// The broadcaster that owns the list of blocked terms.
    pub broadcaster_id: types::UserId,
    /// The UTC date and time (in RFC3339 format) of when the term was blocked.
    pub created_at: types::Timestamp,
    /// The UTC date and time (in RFC3339 format) of when the blocked term is set to expire. After the block expires, user’s will be able to use the term in the broadcaster’s chat room.
    ///
    /// This field is null if the term was added manually or was permanently blocked by AutoMod.
    pub expires_at: Option<types::Timestamp>,
    /// An ID that uniquely identifies this blocked term.
    pub id: types::BlockedTermId,
    /// The moderator that blocked the word or phrase from being used in the broadcaster’s chat room.
    pub moderator_id: types::UserId,
    /// The blocked word or phrase.
    pub text: String,
    /// The UTC date and time (in RFC3339 format) of when the term was updated.
    ///
    /// When the term is added, this timestamp is the same as created_at. The timestamp changes as AutoMod continues to deny the term.
    pub updated_at: types::Timestamp,
}

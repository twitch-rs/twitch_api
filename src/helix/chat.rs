//! Helix endpoints regarding chat

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};
pub mod get_channel_chat_badges;
pub mod get_global_chat_badges;

#[doc(inline)]
pub use get_channel_chat_badges::GetChannelChatBadgesRequest;

#[doc(inline)]
pub use get_global_chat_badges::GetGlobalChatBadgesRequest;

/// A set of badges
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BadgeSet {
    /// ID for the chat badge set.
    pub set_id: types::BadgeSetId,
    /// Contains chat badge objects for the set.
    pub versions: Vec<ChatBadge>,
}
/// A chat Badge
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChatBadge {
    /// ID of the chat badge version.
    pub id: types::ChatBadgeId,
    // FIXME: Use types::Image, see https://github.com/serde-rs/serde/issues/1504
    /// URL to png of size 28x28
    pub image_url_1x: String,
    /// URL to png of size 56x56
    pub image_url_2x: String,
    /// URL to png of size 112x112
    pub image_url_4x: String,
}

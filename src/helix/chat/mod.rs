//! Helix endpoints regarding chat

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod get_channel_chat_badges;
pub mod get_channel_emotes;
pub mod get_emote_sets;
pub mod get_global_chat_badges;
pub mod get_global_emotes;

#[doc(inline)]
pub use get_channel_chat_badges::GetChannelChatBadgesRequest;

#[doc(inline)]
pub use get_global_chat_badges::GetGlobalChatBadgesRequest;

#[doc(inline)]
pub use get_channel_emotes::GetChannelEmotesRequest;

#[doc(inline)]
pub use get_global_emotes::GetGlobalEmotesRequest;

#[doc(inline)]
pub use get_emote_sets::GetEmoteSetsRequest;

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

/// A chat emote
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelEmote {
    /// ID of the emote.
    id: types::EmoteId,
    /// Name of the emote a viewer types into Twitch chat for the image to appear.
    name: String,
    /// Object of image URLs for the emote.
    images: types::Image,
    /// If the emote_type is "subscriptions", this indicates the subscriber tier at which the emote is unlocked. Set to an empty string otherwise.
    #[serde(
        default,
        deserialize_with = "helix::deserialize_none_from_empty_string"
    )]
    tier: Option<types::SubscriptionTier>,
    // FIXME: Enumify?
    /// The type of emote.
    ///
    /// The most common values for custom channel emotes are
    ///
    /// `subscriptions`: Indicates a custom subscriber emote.
    ///
    /// `bitstier`: Indicates a custom Bits tier emote.
    ///
    /// `follower`: Indicates a custom follower emote.
    emote_type: String,
    /// ID of the emote set the emote belongs to.
    emote_set_id: types::EmoteSetId,
}

/// A chat emote
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GlobalEmote {
    /// ID of the emote.
    id: types::EmoteId,
    /// Name of the emote a viewer types into Twitch chat for the image to appear.
    name: String,
    /// Object of image URLs for the emote.
    images: types::Image,
}

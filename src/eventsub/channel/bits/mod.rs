#![doc(alias = "channel.bits")]
//! Bits are used in a channel
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod r#use;

#[doc(inline)]
pub use r#use::{ChannelBitsUseV1, ChannelBitsUseV1Payload};

/// The type of Bits used.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum BitsType {
    /// Bits send via Cheer
    Cheer,
    /// Bits send via Power-Up
    PowerUp,
}

/// Data about Power-up
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitsPowerUp {
    /// Possible values: message_effect | celebration | gigantify_an_emote
    #[serde(rename = "type")]
    pub _type: BitsPowerUpType,
    /// Optional. Emote associated with the reward.
    pub emote: Option<crate::eventsub::channel::chat::Emote>,
    /// Optional. The ID of the message effect.
    pub message_effect_id: Option<String>,
}

/// The type of Power Up used.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum BitsPowerUpType {
    /// Message Effect Power-Up used
    MessageEffect,
    /// Celebration Effect Power-Up used
    Celebration,
    /// Gigantify an Emote Effect Power-Up used
    GigantifyAnEmote,
}

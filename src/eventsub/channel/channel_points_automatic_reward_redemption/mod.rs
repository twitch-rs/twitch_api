#![doc(alias = "channel.channel_points_automatic_reward_redemption")]
//! A viewer has redeemed an automatic channel points reward in a specified channel.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod add;

#[doc(inline)]
pub use add::{
    ChannelPointsAutomaticRewardRedemptionAddV1, ChannelPointsAutomaticRewardRedemptionAddV1Payload,
};

/// Basic information about the automatic reward that was redeemed, at the time it was redeemed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomaticReward {
    /// The type of reward.
    #[serde(rename = "type")]
    pub type_: AutomaticRewardType,
    /// The reward cost.
    pub cost: i64,
    /// Emote that was unlocked.
    pub unlocked_emote: Option<UnlockedEmote>,
}

/// The type of reward
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AutomaticRewardType {
    /// Send a message in sub mode as a non-sub
    SingleMessageBypassSubMode,
    /// Send a message with a highlight
    SendHighlightedMessage,
    /// Unlock a random sub emote
    RandomSubEmoteUnlock,
    /// Unlock a specific sub emote
    ChosenSubEmoteUnlock,
    /// Unlock a specific sub emote with a modifier
    ChosenModifiedSubEmoteUnlock,
    /// Add a special effect to the message
    MessageEffect,
    /// Send a giant emote in chat
    GigantifyAnEmote,
    /// On-screen celebration
    Celebration,
    /// An unknown type was redeemed
    #[serde(untagged)]
    Unknown(String),
}

/// An emote that was unlocked as part of a reward
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UnlockedEmote {
    /// The emote ID.
    pub id: types::EmoteId,
    /// The human readable emote token.
    pub name: String,
}

/// The user message and emote information from a reward redemption
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct RedemptionMessage {
    /// The text of the chat message.
    pub text: String,
    /// An array that includes the emote ID and start and end positions for where the emote appears in the text.
    pub emotes: Vec<types::EmoteOccurrence>,
}

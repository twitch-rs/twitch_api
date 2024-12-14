#![doc(alias = "channel.guest_star_settings")]
//! Events regarding settings of guest star sessions
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod update;

#[doc(inline)]
pub use update::{ChannelGuestStarSettingsUpdateBeta, ChannelGuestStarSettingsUpdateBetaPayload};

/// How guests are laid out in a group browser source
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum GroupLayout {
    /// All live guests are tiled within the browser source with the same size.
    Tiled,
    /// All live guests are tiled within the browser source with the same size. If there is an active screen share, it is sized larger than the other guests.
    Screenshare,
    /// Indicates the group layout will contain all participants in a top-aligned horizontal stack.
    HorizontalTop,
    /// Indicates the group layout will contain all participants in a bottom-aligned horizontal stack.
    HorizontalBottom,
    /// Indicates the group layout will contain all participants in a left-aligned vertical stack.
    VerticalLeft,
    /// Indicates the group layout will contain all participants in a right-aligned vertical stack.
    VerticalRight,
    /// An unknown group layout, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

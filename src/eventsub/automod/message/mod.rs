#![doc(alias = "automod.message")]
//! Events for Automod actions on messages
use super::{AutomodCategory, EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod hold;
pub mod update;

#[doc(inline)]
pub use hold::{AutomodMessageHoldV1, AutomodMessageHoldV1Payload};
#[doc(inline)]
pub use update::{AutomodMessageUpdateV1, AutomodMessageUpdateV1Payload};

/// A message's Automod status
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AutomodMessageStatus {
    /// The message was approved (shown in chat)
    Approved,
    /// The message was denied (not shown in chat)
    Denied,
    /// The message is too old, it can't be acted upon anymore.
    Expired,
    /// An unknown Automod message status, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

#![doc(alias = "automod.message")]
//! Events for Automod actions on messages
use super::{AutomodCategory, EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod hold;

#[doc(inline)]
pub use hold::{AutomodMessageHoldV1, AutomodMessageHoldV1Payload};

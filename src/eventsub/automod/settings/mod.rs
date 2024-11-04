#![doc(alias = "automod.settings")]
//! Events for Automod settings
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod update;

#[doc(inline)]
pub use update::{AutomodSettingsUpdateV1, AutomodSettingsUpdateV1Payload};

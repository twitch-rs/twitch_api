#![doc(alias = "automod.terms")]
//! Events for Automod terms (allowed/denied words)
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod update;

#[doc(inline)]
pub use update::{AutomodTermAction, AutomodTermsUpdateV1, AutomodTermsUpdateV1Payload};

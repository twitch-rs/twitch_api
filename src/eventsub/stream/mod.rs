#![doc(alias = "streams")]
//! Subscription types regarding streams

use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod offline;
pub mod online;

#[doc(inline)]
pub use offline::{StreamOfflineV1, StreamOfflineV1Payload};
#[doc(inline)]
pub use online::{StreamOnlineV1, StreamOnlineV1Payload};

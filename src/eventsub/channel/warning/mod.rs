#![doc(alias = "channel.warning")]
//! Notifications for warnings in a channel.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod acknowledge;

#[doc(inline)]
pub use acknowledge::{ChannelWarningAcknowledgeV1, ChannelWarningAcknowledgeV1Payload};

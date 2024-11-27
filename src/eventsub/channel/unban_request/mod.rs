#![doc(alias = "channel.unban_request")]
//! An unban request in a specified channel is changed.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod create;

#[doc(inline)]
pub use create::{ChannelUnbanRequestCreateV1, ChannelUnbanRequestCreateV1Payload};

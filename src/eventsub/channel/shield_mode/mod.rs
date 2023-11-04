#![doc(alias = "channel.shield_mode")]
//! Shield mode on the specified channel begins or ends.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod begin;
pub mod end;

#[doc(inline)]
pub use begin::{ChannelShieldModeBeginV1, ChannelShieldModeBeginV1Payload};
#[doc(inline)]
pub use end::{ChannelShieldModeEndV1, ChannelShieldModeEndV1Payload};

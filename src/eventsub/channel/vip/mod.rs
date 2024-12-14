#![doc(alias = "channel.vip")]
//! A user's VIP status on a specified channel is changed.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod add;
pub mod remove;

#[doc(inline)]
pub use add::{ChannelVipAddV1, ChannelVipAddV1Payload};
#[doc(inline)]
pub use remove::{ChannelVipRemoveV1, ChannelVipRemoveV1Payload};

#![doc(alias = "channel.poll")]
//! Subscription on a specified channel has changed
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod end;

#[doc(inline)]
pub use end::{ChannelSubscriptionEndV1, ChannelSubscriptionEndV1Payload};

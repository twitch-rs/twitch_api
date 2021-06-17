#![doc(alias = "channel.poll")]
//! Subscription on a specified channel has changed
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod end;
pub mod gift;

#[doc(inline)]
pub use end::{ChannelSubscriptionEndV1, ChannelSubscriptionEndV1Payload};
#[doc(inline)]
pub use gift::{ChannelSubscriptionGiftV1, ChannelSubscriptionGiftV1Payload};

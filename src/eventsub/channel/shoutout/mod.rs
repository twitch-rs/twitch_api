#![doc(alias = "channel.shoutout")]
//! Subscription for when a Shoutout has happened
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod create;
pub mod receive;

#[doc(inline)]
pub use create::{ChannelShoutoutCreateBeta, ChannelShoutoutCreateBetaPayload};
#[doc(inline)]
pub use receive::{ChannelShoutoutReceiveBeta, ChannelShoutoutReceiveBetaPayload};

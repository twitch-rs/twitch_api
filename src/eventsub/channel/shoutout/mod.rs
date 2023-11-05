#![doc(alias = "channel.shoutout")]
//! Subscription for when a Shoutout has happened
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod create;
pub mod receive;

#[doc(inline)]
pub use create::{ChannelShoutoutCreateV1, ChannelShoutoutCreateV1Payload};
#[doc(inline)]
pub use receive::{ChannelShoutoutReceiveV1, ChannelShoutoutReceiveV1Payload};

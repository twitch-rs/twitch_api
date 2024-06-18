//! Subscription types regarding conduits.
use super::{EventSubscription, EventType};

pub mod shard;

#[doc(inline)]
pub use shard::disabled::{ConduitShardDisabledV1, ConduitShardDisabledV1Payload};

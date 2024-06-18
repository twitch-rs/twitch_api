#![doc(alias = "conduit.shard")]
//! Subscription types regarding conduit shards.
use super::{EventSubscription, EventType};
use crate::eventsub;
use serde_derive::{Deserialize, Serialize};

pub mod disabled;

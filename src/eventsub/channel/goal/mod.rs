#![doc(alias = "channel.goal")]
//! A broadcaster has started, progressed or ended a goal.
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod begin;
pub mod end;
pub mod progress;

#[doc(inline)]
pub use begin::{ChannelGoalBeginV1, ChannelGoalBeginV1Payload};
#[doc(inline)]
pub use end::{ChannelGoalEndV1, ChannelGoalEndV1Payload};
#[doc(inline)]
pub use progress::{ChannelGoalProgressV1, ChannelGoalProgressV1Payload};

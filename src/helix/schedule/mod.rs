//! Helix endpoints regarding stream schedules
use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod get_channel_stream_schedule;

#[doc(inline)]
pub use get_channel_stream_schedule::{GetChannelStreamScheduleRequest, Segment, ScheduledBroadcasts};

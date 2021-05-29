#![doc(alias = "channel.poll")]
//! Poll on a specific channel has been begun, ended or progressed.
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod begin;
pub mod end;
pub mod progress;

#[doc(inline)]
pub use begin::{ChannelPollBeginV1, ChannelPollBeginV1Payload};
#[doc(inline)]
pub use end::{ChannelPollEndV1, ChannelPollEndV1Payload};
#[doc(inline)]
pub use progress::{ChannelPollProgressV1, ChannelPollProgressV1Payload};

/// Bits voting settings for a poll
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BitsVoting {
    // FIXME: Is this null or 0 when not enabled?
    /// Number of Bits required to vote once with Bits.
    pub amount_per_vote: i64,
    /// Indicates if Bits can be used for voting.
    pub is_enabled: bool,
}

/// Channel Points voting settings
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPointsVoting {
    // FIXME: Is this null or 0 when not enabled?
    /// Number of Channel Points required to vote once with Channel Points.
    pub amount_per_vote: i64,
    /// Indicates if Channel Points can be used for voting.
    pub is_enabled: bool,
}

#![doc(alias = "channel.prediction")]
//!  Prediction on the specified channel begins, progresses, locks or ends.
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod begin;
pub mod end;
pub mod lock;
pub mod progress;

#[doc(inline)]
pub use begin::{ChannelPredictionBeginV1, ChannelPredictionBeginV1Payload};
#[doc(inline)]
pub use end::{ChannelPredictionEndV1, ChannelPredictionEndV1Payload};
#[doc(inline)]
pub use lock::{ChannelPredictionLockV1, ChannelPredictionLockV1Payload};
#[doc(inline)]
pub use progress::{ChannelPredictionProgressV1, ChannelPredictionProgressV1Payload};

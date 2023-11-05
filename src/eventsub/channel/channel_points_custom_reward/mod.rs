#![doc(alias = "points")]
#![doc(alias = "channel.channel_points_custom_reward")]
//! Custom channel points rewards on specific channel has been changed, removed or updated.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod add;
pub mod remove;
pub mod update;

#[doc(inline)]
pub use add::{ChannelPointsCustomRewardAddV1, ChannelPointsCustomRewardAddV1Payload};
#[doc(inline)]
pub use remove::{ChannelPointsCustomRewardRemoveV1, ChannelPointsCustomRewardRemoveV1Payload};
#[doc(inline)]
pub use update::{ChannelPointsCustomRewardUpdateV1, ChannelPointsCustomRewardUpdateV1Payload};

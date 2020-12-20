//! Subscriptions that sends a notification on changes to custom channel points rewards on specific channel.
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod add;
pub mod remove;
pub mod update;

#[doc(inline)]
pub use add::{ChannelPointsCustomRewardAddV1, ChannelPointsCustomRewardAddV1Payload};
#[doc(inline)]
pub use remove::{ChannelPointsCustomRewardRemoveV1, ChannelPointsCustomRewardRemoveV1Payload};
#[doc(inline)]
pub use update::{ChannelPointsCustomRewardUpdateV1, ChannelPointsCustomRewardUpdateV1Payload};

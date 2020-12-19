#![allow(missing_docs)]
//! Subscriptions that sends a notification on changes to custom channel points rewards on specific channel.
use super::*;

pub mod add;

#[doc(inline)]
pub use add::{ChannelPointsCustomRewardAddV1, ChannelPointsCustomRewardAddV1Payload};

#![doc(alias = "points")]
#![doc(alias = "channel.channel_points_custom_reward_redemption")]
//! A viewer has redeemed a custom channel points reward or a redemption of a channel points custom reward has been updated for the specified channel.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod add;
pub mod update;

#[doc(inline)]
pub use add::{
    ChannelPointsCustomRewardRedemptionAddV1, ChannelPointsCustomRewardRedemptionAddV1Payload,
};
#[doc(inline)]
pub use update::{
    ChannelPointsCustomRewardRedemptionUpdateV1, ChannelPointsCustomRewardRedemptionUpdateV1Payload,
};

/// Basic information about the reward that was redeemed, at the time it was redeemed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Reward {
    /// The reward cost.
    pub cost: i64,
    /// The reward identifier.
    pub id: types::RewardId,
    /// The reward description.
    pub prompt: String,
    /// The reward name.
    pub title: String,
}

/// Custom reward redemption statuses: UNFULFILLED, FULFILLED or CANCELED
///
/// See also [`CustomRewardRedemptionStatus`](crate::helix::points::CustomRewardRedemptionStatus)
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum RedemptionStatus {
    /// Unfulfilled reward - the user has claimed it but it is still pending.
    Unfulfilled,
    /// Fulfilled reward - the user has claimed it and the reward has been granted.
    Fulfilled,
    /// Cancelled reward - the reward has been cancelled before fulfillment, and any spent points have been refunded.
    Canceled,
    /// Unknown reward status
    Unknown,
}

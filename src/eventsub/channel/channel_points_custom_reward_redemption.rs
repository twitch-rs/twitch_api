//! Subscriptions that sends a notification when a viewer has redeemed a custom channel points reward or when a redemption of a channel points custom reward has been updated for the specified channel.
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

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
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
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

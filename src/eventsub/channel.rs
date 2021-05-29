#![doc(alias = "channels")]
//! Subscription types regarding channels
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod ban;
pub mod channel_points_custom_reward;
pub mod channel_points_custom_reward_redemption;
pub mod cheer;
pub mod follow;
pub mod hypetrain;
pub mod poll;
pub mod prediction;
pub mod raid;
pub mod subscribe;
pub mod subscription;
pub mod unban;
pub mod update;

#[doc(inline)]
pub use ban::{ChannelBanV1, ChannelBanV1Payload};
#[doc(inline)]
pub use channel_points_custom_reward::{
    ChannelPointsCustomRewardAddV1, ChannelPointsCustomRewardAddV1Payload,
};
#[doc(inline)]
pub use channel_points_custom_reward::{
    ChannelPointsCustomRewardRemoveV1, ChannelPointsCustomRewardRemoveV1Payload,
};
#[doc(inline)]
pub use channel_points_custom_reward::{
    ChannelPointsCustomRewardUpdateV1, ChannelPointsCustomRewardUpdateV1Payload,
};
#[doc(inline)]
pub use channel_points_custom_reward_redemption::{
    ChannelPointsCustomRewardRedemptionAddV1, ChannelPointsCustomRewardRedemptionAddV1Payload,
};
#[doc(inline)]
pub use channel_points_custom_reward_redemption::{
    ChannelPointsCustomRewardRedemptionUpdateV1, ChannelPointsCustomRewardRedemptionUpdateV1Payload,
};
#[doc(inline)]
pub use cheer::{ChannelCheerV1, ChannelCheerV1Payload};
#[doc(inline)]
pub use follow::{ChannelFollowV1, ChannelFollowV1Payload};
#[doc(inline)]
pub use hypetrain::{ChannelHypeTrainBeginV1, ChannelHypeTrainBeginV1Payload};
#[doc(inline)]
pub use hypetrain::{ChannelHypeTrainEndV1, ChannelHypeTrainEndV1Payload};
#[doc(inline)]
pub use hypetrain::{ChannelHypeTrainProgressV1, ChannelHypeTrainProgressV1Payload};
#[doc(inline)]
pub use poll::{ChannelPollBeginV1, ChannelPollBeginV1Payload};
#[doc(inline)]
pub use poll::{ChannelPollEndV1, ChannelPollEndV1Payload};
#[doc(inline)]
pub use poll::{ChannelPollProgressV1, ChannelPollProgressV1Payload};
#[doc(inline)]
pub use prediction::{ChannelPredictionBeginV1, ChannelPredictionBeginV1Payload};
#[doc(inline)]
pub use prediction::{ChannelPredictionEndV1, ChannelPredictionEndV1Payload};
#[doc(inline)]
pub use prediction::{ChannelPredictionLockV1, ChannelPredictionLockV1Payload};
#[doc(inline)]
pub use prediction::{ChannelPredictionProgressV1, ChannelPredictionProgressV1Payload};
#[doc(inline)]
pub use raid::{ChannelRaidV1, ChannelRaidV1Payload};
#[doc(inline)]
pub use subscribe::{ChannelSubscribeV1, ChannelSubscribeV1Payload};
#[doc(inline)]
pub use subscription::{ChannelSubscriptionEndV1, ChannelSubscriptionEndV1Payload};
#[doc(inline)]
pub use unban::{ChannelUnbanV1, ChannelUnbanV1Payload};
#[doc(inline)]
pub use update::{ChannelUpdateV1, ChannelUpdateV1Payload};

#![doc(alias = "channels")]
#![allow(deprecated)]
//! Subscription types regarding channels
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod ad_break;
pub mod ban;
pub mod channel_points_custom_reward;
pub mod channel_points_custom_reward_redemption;
pub mod charity_campaign;
pub mod chat;
pub mod cheer;
pub mod follow;
pub mod goal;
pub mod hypetrain;
pub mod poll;
pub mod prediction;
pub mod raid;
pub mod shared_chat;
pub mod shield_mode;
pub mod shoutout;
pub mod subscribe;
pub mod subscription;
pub mod unban;
pub mod update;

#[doc(inline)]
pub use ad_break::{ChannelAdBreakBeginV1, ChannelAdBreakBeginV1Payload};
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
pub use charity_campaign::{ChannelCharityCampaignDonateV1, ChannelCharityCampaignDonateV1Payload};
#[doc(inline)]
pub use charity_campaign::{
    ChannelCharityCampaignProgressV1, ChannelCharityCampaignProgressV1Payload,
};
#[doc(inline)]
pub use charity_campaign::{ChannelCharityCampaignStartV1, ChannelCharityCampaignStartV1Payload};
#[doc(inline)]
pub use charity_campaign::{ChannelCharityCampaignStopV1, ChannelCharityCampaignStopV1Payload};
#[doc(inline)]
pub use chat::{ChannelChatClearUserMessagesV1, ChannelChatClearUserMessagesV1Payload};
#[doc(inline)]
pub use chat::{ChannelChatClearV1, ChannelChatClearV1Payload};
#[doc(inline)]
pub use chat::{ChannelChatMessageDeleteV1, ChannelChatMessageDeleteV1Payload};
#[doc(inline)]
pub use chat::{ChannelChatMessageV1, ChannelChatMessageV1Payload};
#[doc(inline)]
pub use chat::{ChannelChatNotificationV1, ChannelChatNotificationV1Payload};
#[doc(inline)]
pub use cheer::{ChannelCheerV1, ChannelCheerV1Payload};
#[doc(inline)]
pub use follow::{ChannelFollowV1, ChannelFollowV1Payload};
#[doc(inline)]
pub use follow::{ChannelFollowV2, ChannelFollowV2Payload};
#[doc(inline)]
pub use goal::{ChannelGoalBeginV1, ChannelGoalBeginV1Payload};
#[doc(inline)]
pub use goal::{ChannelGoalEndV1, ChannelGoalEndV1Payload};
#[doc(inline)]
pub use goal::{ChannelGoalProgressV1, ChannelGoalProgressV1Payload};
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
pub use shared_chat::{ChannelSharedChatBeginV1, ChannelSharedChatBeginV1Payload};
#[doc(inline)]
pub use shared_chat::{ChannelSharedChatUpdateV1, ChannelSharedChatUpdateV1Payload};
#[doc(inline)]
pub use shield_mode::{ChannelShieldModeBeginV1, ChannelShieldModeBeginV1Payload};
#[doc(inline)]
pub use shield_mode::{ChannelShieldModeEndV1, ChannelShieldModeEndV1Payload};
#[doc(inline)]
pub use shoutout::{ChannelShoutoutCreateV1, ChannelShoutoutCreateV1Payload};
#[doc(inline)]
pub use shoutout::{ChannelShoutoutReceiveV1, ChannelShoutoutReceiveV1Payload};
#[doc(inline)]
pub use subscribe::{ChannelSubscribeV1, ChannelSubscribeV1Payload};
#[doc(inline)]
pub use subscription::{ChannelSubscriptionEndV1, ChannelSubscriptionEndV1Payload};
#[doc(inline)]
pub use subscription::{ChannelSubscriptionGiftV1, ChannelSubscriptionGiftV1Payload};
#[doc(inline)]
pub use subscription::{ChannelSubscriptionMessageV1, ChannelSubscriptionMessageV1Payload};
#[doc(inline)]
pub use unban::{ChannelUnbanV1, ChannelUnbanV1Payload};
#[doc(inline)]
pub use update::{ChannelUpdateV1, ChannelUpdateV1Payload};
#[doc(inline)]
pub use update::{ChannelUpdateV2, ChannelUpdateV2Payload};

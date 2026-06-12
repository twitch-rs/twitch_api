#![doc(alias = "channel.hype_train")]
//! A hype train has started, progressed or ended.
use super::{EventSubscription, EventType};
use crate::common::hypetrain::{HypeTrainType, SharedTrainParticipant};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod begin;
pub mod end;
pub mod progress;

#[doc(inline)]
pub use begin::{
    ChannelHypeTrainBeginV1, ChannelHypeTrainBeginV1Payload, ChannelHypeTrainBeginV2,
    ChannelHypeTrainBeginV2Payload,
};
#[doc(inline)]
pub use end::{
    ChannelHypeTrainEndV1, ChannelHypeTrainEndV1Payload, ChannelHypeTrainEndV2,
    ChannelHypeTrainEndV2Payload,
};
#[doc(inline)]
pub use progress::{
    ChannelHypeTrainProgressV1, ChannelHypeTrainProgressV1Payload, ChannelHypeTrainProgressV2,
    ChannelHypeTrainProgressV2Payload,
};

// Re-exported from common for backwards compatability
#[doc(inline)]
pub use crate::common::hypetrain::HypeTrainContributionType as ContributionType;

/// A contribution to hype train
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Contribution {
    /// The total contributed.
    pub total: i64,
    #[serde(rename = "type")]
    /// Type of contribution. Valid values include bits, subscription.
    pub type_: ContributionType,
    /// The ID of the user.
    pub user_id: types::UserId,
    /// The login of the user.
    pub user_login: types::UserName,
    /// The display name of the user.
    pub user_name: types::DisplayName,
}

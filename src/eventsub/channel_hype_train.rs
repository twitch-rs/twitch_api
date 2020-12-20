//! Subscriptions that sends a notification related to hype trains
use super::*;

pub mod begin;
pub mod end;
pub mod progress;

#[doc(inline)]
pub use begin::{ChannelHypeTrainBeginV1, ChannelHypeTrainBeginV1Payload};
#[doc(inline)]
pub use end::{ChannelHypeTrainEndV1, ChannelHypeTrainEndV1Payload};
#[doc(inline)]
pub use progress::{ChannelHypeTrainProgressV1, ChannelHypeTrainProgressV1Payload};

/// Type of contribution
#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ContributionType {
    /// Bits
    Bits,
    /// Channel Subscriptions. Either gifted or not.
    Subscription,
}

/// A contribution to hype train
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct Contribution {
    /// The total contributed.
    pub total: i64,
    #[serde(rename = "type")]
    /// Type of contribution. Valid values include bits, subscription.
    pub type_: ContributionType,
    /// The ID of the user.
    pub user_id: types::UserId,
    /// The name of the user.
    pub user_name: types::UserName,
}

//! Helix endpoints regarding channel points/redeems
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::helix::{HelixClient};
//! # use twitch_api::helix::points::{GetCustomRewardRedemptionRequest, CustomRewardRedemptionStatus};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = GetCustomRewardRedemptionRequest::broadcaster_id("274637212")
//!     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01")
//!     .status(CustomRewardRedemptionStatus::Canceled);
//!
//! println!("{:?}", &client.req_get(request, &token).await?.data.first());
//! # Ok(())
//! # }
//! ```

use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod create_custom_rewards;
pub mod delete_custom_reward;
pub mod get_custom_reward;
pub mod get_custom_reward_redemption;
pub mod update_custom_reward;
pub mod update_redemption_status;

#[doc(inline)]
pub use create_custom_rewards::{
    CreateCustomRewardBody, CreateCustomRewardRequest, CreateCustomRewardResponse,
};
#[doc(inline)]
pub use delete_custom_reward::{DeleteCustomReward, DeleteCustomRewardRequest};
#[doc(inline)]
pub use get_custom_reward::{CustomReward, GetCustomRewardRequest};
#[doc(inline)]
pub use get_custom_reward_redemption::{CustomRewardRedemption, GetCustomRewardRedemptionRequest};
#[doc(inline)]
pub use update_custom_reward::{UpdateCustomRewardBody, UpdateCustomRewardRequest};
#[doc(inline)]
pub use update_redemption_status::{
    UpdateRedemptionStatusBody, UpdateRedemptionStatusInformation, UpdateRedemptionStatusRequest,
};
/// Custom reward redemption statuses: UNFULFILLED, FULFILLED or CANCELED
#[derive(PartialEq, Eq, Serialize, Deserialize, Copy, Clone, Debug)]
#[non_exhaustive]
pub enum CustomRewardRedemptionStatus {
    /// Unfulfilled reward - the user has claimed it but it is still pending.
    #[serde(rename = "UNFULFILLED")]
    Unfulfilled,
    /// Fulfilled reward - the user has claimed it and the reward has been granted.
    #[serde(rename = "FULFILLED")]
    Fulfilled,
    /// Cancelled reward - the reward has been cancelled before fulfillment, and any spent points have been refunded.
    #[serde(rename = "CANCELED")]
    Canceled,
}

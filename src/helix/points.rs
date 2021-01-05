#![doc(alias = "channel points")]
#![doc(alias = "channel redeems")]
//! Endpoints regarding channel points/redeems
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient};
//! # use twitch_api2::helix::points::{GetCustomRewardRedemptionRequest, CustomRewardRedemptionStatus};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let request = GetCustomRewardRedemptionRequest::builder()
//!     .broadcaster_id("274637212".to_string())
//!     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
//!     .status(CustomRewardRedemptionStatus::Canceled)
//!     .build();
//!
//!
//! println!("{:?}", &client.req_get(request, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```

use crate::{helix, types};
use serde::{Deserialize, Serialize};

pub mod get_custom_reward_redemption;
pub mod update_redemption_status;

#[doc(inline)]
pub use get_custom_reward_redemption::{CustomRewardRedemption, GetCustomRewardRedemptionRequest};
#[doc(inline)]
pub use update_redemption_status::{
    UpdateRedemptionStatusBody, UpdateRedemptionStatusInformation, UpdateRedemptionStatusRequest,
};

/// Custom reward redemption statuses: UNFULFILLED, FULFILLED or CANCELED
#[derive(PartialEq, serde::Serialize, serde::Deserialize, Clone, Debug)]
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

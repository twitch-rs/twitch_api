#![doc(alias = "subscription")]
//! Helix endpoints regarding subscriptions
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, subscriptions::GetBroadcasterSubscriptionsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetBroadcasterSubscriptionsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//!
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod check_user_subscription;
pub mod get_broadcaster_subscriptions;
pub mod get_broadcaster_subscriptions_events;

#[doc(inline)]
pub use get_broadcaster_subscriptions::{
    BroadcasterSubscription, GetBroadcasterSubscriptionsRequest,
};
#[doc(inline)]
pub use get_broadcaster_subscriptions_events::{
    BroadcasterSubscriptionEvent, GetBroadcasterSubscriptionsEventsRequest,
};

#[doc(inline)]
pub use check_user_subscription::{CheckUserSubscriptionRequest, UserSubscription};

//! Endpoints regarding subscriptions
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, subscriptions::GetBroadcasterSubscriptionsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetBroadcasterSubscriptionsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//!
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```

use crate::{helix, types};
use serde::{Deserialize, Serialize};

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

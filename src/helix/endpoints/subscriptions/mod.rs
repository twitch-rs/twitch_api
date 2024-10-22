#![doc(alias = "subscription")]
//! Helix endpoints regarding subscriptions
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::helix::{HelixClient, subscriptions::GetBroadcasterSubscriptionsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetBroadcasterSubscriptionsRequest::broadcaster_id("1234");
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Subscriptions 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Broadcaster Subscriptions](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions) | [`HelixClient::get_broadcaster_subscriptions`](crate::helix::HelixClient::get_broadcaster_subscriptions) | [`get_broadcaster_subscriptions`] |
//! | [Check User Subscription](https://dev.twitch.tv/docs/api/reference#check-user-subscription) | - | [`check_user_subscription`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->

use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

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

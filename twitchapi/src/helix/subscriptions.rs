//! Endpoints regarding subscriptions
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, subscriptions::GetBroadcasterSubscriptionsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(token, None).await?;
//! let client = HelixClient::new();
//! let req = GetBroadcasterSubscriptionsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//!
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use get_broadcaster_subscriptions::{
    GetBroadcasterSubscriptions, GetBroadcasterSubscriptionsRequest,
};

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Get all of a broadcaster’s subscriptions.
/// [`get-broadcaster-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions)
pub mod get_broadcaster_subscriptions {
    use super::*;
    /// Query Parameters for [Get Broadcaster Subscriptions](super::get_broadcaster_subscriptions)
    ///
    /// [`get-broadcaster-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetBroadcasterSubscriptionsRequest {
        /// User ID of the broadcaster. Must match the User ID in the Bearer token.
        #[builder(setter(into))]
        pub broadcaster_id: String,
        /// Unique identifier of account to get subscription status of. Accepts up to 100 values.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<String>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
    }

    /// Return Values for [Get Broadcaster Subscriptions](super::get_broadcaster_subscriptions)
    ///
    /// [`get-broadcaster-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions)
    #[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct GetBroadcasterSubscriptions {
        /// User ID of the broadcaster.
        pub broadcaster_id: String,
        /// Display name of the broadcaster.
        pub broadcaster_name: String,
        /// Determines if the subscription is a gift subscription.
        pub is_gift: bool,
        /// Type of subscription (Tier 1, Tier 2, Tier 3). 1000 = Tier 1, 2000 = Tier 2, 3000 = Tier 3 subscriptions.
        pub tier: String,
        /// Name of the subscription.
        pub plan_name: String,
        /// ID of the subscribed user.
        pub user_id: String,
        /// Display name of the subscribed user.
        pub user_name: String,
    }

    impl helix::Request for GetBroadcasterSubscriptionsRequest {
        type Response = GetBroadcasterSubscriptions;

        const PATH: &'static str = "subscriptions";
        const SCOPE: &'static [twitch_oauth2::Scope] =
            &[twitch_oauth2::Scope::ChannelReadSubscriptions];
    }

    impl helix::RequestGet for GetBroadcasterSubscriptionsRequest {}

    impl helix::Paginated for GetBroadcasterSubscriptionsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }
}

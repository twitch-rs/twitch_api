//! Endpoints regarding subscriptions
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, subscriptions::GetBroadcasterSubscriptionsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
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
#[doc(inline)]
pub use get_broadcaster_subscriptions::{
    BroadcasterSubscriptions, GetBroadcasterSubscriptionsRequest,
};

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Get all of a broadcaster’s subscriptions.
/// [`get-broadcaster-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions)
pub mod get_broadcaster_subscriptions {
    use super::*;
    /// Query Parameters for [Get Broadcaster Subscriptions](super::get_broadcaster_subscriptions)
    ///
    /// [`get-broadcaster-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetBroadcasterSubscriptionsRequest {
        /// User ID of the broadcaster. Must match the User ID in the Bearer token.
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
        /// Unique identifier of account to get subscription status of. Accepts up to 100 values.
        #[builder(default)]
        pub user_id: Vec<types::UserId>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub after: Option<helix::Cursor>,
    }

    /// Return Values for [Get Broadcaster Subscriptions](super::get_broadcaster_subscriptions)
    ///
    /// [`get-broadcaster-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions)
    #[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct BroadcasterSubscriptions {
        /// User ID of the broadcaster.
        pub broadcaster_id: types::UserId,
        /// Display name of the broadcaster.
        pub broadcaster_name: types::DisplayName,
        /// Determines if the subscription is a gift subscription.
        pub is_gift: bool,
        /// Type of subscription (Tier 1, Tier 2, Tier 3). 1000 = Tier 1, 2000 = Tier 2, 3000 = Tier 3 subscriptions.
        pub tier: types::SubscriptionTier,
        /// Name of the subscription.
        pub plan_name: String,
        /// ID of the subscribed user.
        pub user_id: types::UserId,
        /// Display name of the subscribed user.
        pub user_name: types::DisplayName,
    }

    impl helix::Request for GetBroadcasterSubscriptionsRequest {
        type Response = Vec<BroadcasterSubscriptions>;

        const PATH: &'static str = "subscriptions";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] =
            &[twitch_oauth2::Scope::ChannelReadSubscriptions];
    }

    impl helix::RequestGet for GetBroadcasterSubscriptionsRequest {}

    impl helix::Paginated for GetBroadcasterSubscriptionsRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetBroadcasterSubscriptionsRequest::builder()
            .broadcaster_id("123".to_string())
            .build();

        // From twitch docs. Malformed example on https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions
        let data = br#"
{
    "data": [
        {
        "broadcaster_id": "123",
        "broadcaster_name": "test_user",
        "is_gift": true,
        "tier": "1000",
        "plan_name": "The Ninjas",
        "user_id": "123",
        "user_name": "snoirf"
        }
    ],
    "pagination": {
        "cursor": "xxxx"
    }
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/subscriptions?broadcaster_id=123"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

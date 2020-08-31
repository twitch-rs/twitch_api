//! Endpoints regarding users
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, users::GetUsersRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetUsersRequest::builder()
//!     .login(vec!["justinfan1337".to_string()])
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use get_users::{GetUsersRequest, User};

#[doc(inline)]
pub use get_users_follows::{GetUsersFollowsRequest, UsersFollows};

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Gets information about one or more specified Twitch users.
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
pub mod get_users {
    use super::*;
    /// Query Parameters for [Get Users](super::get_users)
    ///
    /// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetUsersRequest {
        /// User ID. Multiple user IDs can be specified. Limit: 100.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub id: Vec<String>,
        /// User login name. Multiple login names can be specified. Limit: 100.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub login: Vec<String>,
    }

    /// Return Values for [Get Users](super::get_users)
    ///
    /// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct User {
        /// User’s broadcaster type: "partner", "affiliate", or "".
        pub broadcaster_type: Option<String>,
        /// User’s channel description.
        pub description: Option<String>,
        /// User’s display name.
        pub display_name: String,
        /// User’s email address. Returned if the request includes the [user:read:email scope](twitch_oauth2::Scope::UserReadEmail).
        pub email: Option<String>,
        /// User’s ID.
        pub id: String,
        /// User’s login name.
        pub login: String,
        /// URL of the user’s offline image.
        pub offline_image_url: Option<String>,
        /// URL of the user’s profile image.
        pub profile_image_url: Option<String>,
        /// User’s type: "staff", "admin", "global_mod", or "".
        #[serde(rename = "type")]
        pub type_: Option<String>,
        /// Total number of views of the user’s channel.
        pub view_count: usize,
    }

    impl helix::Request for GetUsersRequest {
        type Response = User;

        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
        const PATH: &'static str = "users";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetUsersRequest {}
}

/// Gets information on follow relationships between two Twitch users.
/// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
pub mod get_users_follows {
    use super::*;
    /// Query Parameters for [Get Users Follows](super::get_users_follows)
    ///
    /// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetUsersFollowsRequest {
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first: Option<usize>,
        /// User ID. The request returns information about users who are being followed by the from_id user.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<String>,
        /// User ID. The request returns information about users who are following the to_id user.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to_id: Option<String>,
    }

    /// Return Values for [Get Users Follows](super::get_users_follows)
    ///
    /// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct UsersFollows {
        ///Date and time when the from_id user followed the to_id user.
        pub followed_at: String,
        ///ID of the user following the to_id user.
        pub from_id: String,
        ///Display name corresponding to from_id.
        pub from_name: String,
        ///ID of the user being followed by the from_id user.
        pub to_id: String,
        ///Display name corresponding to to_id.
        pub to_name: String,
        // FIXME: This never seems to be returned.
        /// Total number of items returned.
        ///
        /// * If only `from_id` was in the request, this is the total number of followed users.
        /// * If only `to_id` was in the request, this is the total number of followers.
        /// * If both `from_id` and to_id were in the request, this is 1 (if the "from" user follows the "to" user) or 0.
        pub total: Option<usize>,
    }

    impl helix::Request for GetUsersFollowsRequest {
        type Response = UsersFollows;

        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
        const PATH: &'static str = "users/follows";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetUsersFollowsRequest {}
    impl helix::Paginated for GetUsersFollowsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor); }
    }
}

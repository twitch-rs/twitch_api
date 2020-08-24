//! Endpoints regarding users
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, users::GetUsersRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
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
pub use get_users::{GetUsers, GetUsersRequest};

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
    pub struct GetUsers {
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
        type Response = GetUsers;

        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
        const PATH: &'static str = "users";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetUsersRequest {}
}

//! Gets information about one or more specified Twitch users.
//! [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
//!
//! ## Request: [GetUsersRequest]
//!
//! To use this endpoint, construct a [`GetUsersRequest`] with the [`GetUsersRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::users::get_users;
//! let request = get_users::GetUsersRequest::builder()
//!     .id(vec!["1234".to_string()])
//!     .login(vec!["justintvfan".to_string()])
//!     .build();
//! ```
//!
//! ## Response: [User]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, users::get_users};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_users::GetUsersRequest::builder()
//!     .id(vec!["1234".to_string()])
//!     .login(vec!["justintvfan".to_string()])
//!     .build();
//! let response: Vec<get_users::User> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())

use super::*;
/// Query Parameters for [Get Users](super::get_users)
///
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetUsersRequest {
    /// User ID. Multiple user IDs can be specified. Limit: 100.
    #[builder(default)]
    pub id: Vec<types::UserId>,
    /// User login name. Multiple login names can be specified. Limit: 100.
    #[builder(default)]
    pub login: Vec<types::UserName>,
}

/// Return Values for [Get Users](super::get_users)
///
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct User {
    /// User’s broadcaster type: "partner", "affiliate", or "".
    pub broadcaster_type: Option<types::BroadcasterType>,
    /// User’s channel description.
    pub description: Option<String>,
    /// User’s display name.
    pub display_name: types::DisplayName,
    /// User’s email address. Returned if the request includes the [`user:read:email` scope](twitch_oauth2::Scope::UserReadEmail).
    pub email: Option<String>,
    /// User’s ID.
    pub id: types::UserId,
    /// User’s login name.
    pub login: types::UserName,
    /// URL of the user’s offline image.
    pub offline_image_url: Option<String>,
    /// URL of the user’s profile image.
    pub profile_image_url: Option<String>,
    /// User’s type: "staff", "admin", "global_mod", or "".
    #[serde(rename = "type")]
    pub type_: Option<types::UserType>,
    /// Total number of views of the user’s channel.
    pub view_count: usize,
}

impl helix::Request for GetUsersRequest {
    type Response = Vec<User>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
    const PATH: &'static str = "users";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl helix::RequestGet for GetUsersRequest {}

#[test]
fn test_request() {
    use helix::*;
    let req = GetUsersRequest::builder()
        .id(vec!["44322889".to_string()])
        .build();

    // From twitch docs
    let data = br#"
{
    "data": [{
        "id": "44322889",
        "login": "dallas",
        "display_name": "dallas",
        "type": "staff",
        "broadcaster_type": "",
        "description": "Just a gamer playing games and chatting. :)",
        "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-profile_image-1a2c906ee2c35f12-300x300.png",
        "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-channel_offline_image-1a2c906ee2c35f12-1920x1080.png",
        "view_count": 191836881,
        "email": "login@provider.com"
    }]
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users?id=44322889"
    );

    dbg!(req.parse_response(&uri, http_response).unwrap());
}

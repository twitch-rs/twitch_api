//! Gets information about one or more specified Twitch users.
//! [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
//!
//! ## Request: [GetUsersRequest]
//!
//! To use this endpoint, construct a [`GetUsersRequest`] with the [`GetUsersRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::get_users;
//! let request = get_users::GetUsersRequest::builder()
//!     .id(vec!["1234".into()])
//!     .login(vec!["justintvfan".into()])
//!     .build();
//! ```
//!
//! ## Response: [User]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::get_users};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_users::GetUsersRequest::builder()
//!     .id(vec!["1234".into()])
//!     .login(vec!["justintvfan".into()])
//!     .build();
//! let response: Vec<get_users::User> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUsersRequest::parse_response(None, &request.get_uri(), response)`](GetUsersRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Users](super::get_users)
///
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetUsersRequest {
    /// User ID. Multiple user IDs can be specified. Limit: 100.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub id: Vec<types::UserId>,
    /// User login name. Multiple login names can be specified. Limit: 100.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub login: Vec<types::UserName>,
}

impl GetUsersRequest {
    /// Get a single user by their [`UserName`](types::UserName)
    pub fn login(login: impl Into<types::UserName>) -> Self {
        Self {
            id: Vec::default(),
            login: vec![login.into()],
        }
    }

    /// Get multiple user by their [`UserName`](types::UserName)
    ///
    /// ```rust
    /// use twitch_api2::helix::users::get_users::GetUsersRequest;
    /// GetUsersRequest::logins(&["twitch".into(), "justintv".into()])
    /// ```
    pub fn logins(login: impl IntoIterator<Item = types::UserName>) -> Self {
        Self {
            id: Vec::default(),
            login: login.into_iter().collect(),
        }
    }

    /// Get a user by their [`UserId`](types::UserId)
    pub fn id(id: impl Into<types::UserId>) -> Self {
        Self {
            id: vec![id.into()],
            login: Vec::default(),
        }
    }

    /// Get multiple user by their [`UserId`](types::UserId)
    pub fn ids(ids: impl IntoIterator<Item = types::UserId>) -> Self {
        Self {
            id: ids.into_iter().collect(),
            login: Vec::default(),
        }
    }

    /// Returns an empty [`GetUsersRequest`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api2::helix::users::GetUsersRequest;
    /// GetUsersRequest {
    ///     id: vec!["1234".into()],
    ///     ..GetUsersRequest::empty()
    /// }
    /// ```
    pub fn empty() -> Self {
        Self {
            id: Vec::default(),
            login: Vec::default(),
        }
    }
}

/// Return Values for [Get Users](super::get_users)
///
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct User {
    /// User’s broadcaster type: "partner", "affiliate", or "".
    pub broadcaster_type: Option<types::BroadcasterType>,
    /// Date when the user was created.
    pub created_at: types::Timestamp,
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
    #[deprecated(
        since = "0.7.0",
        note = "removed, see https://discuss.dev.twitch.tv/t/get-users-api-endpoint-view-count-deprecation/37777"
    )]
    #[serde(default)]
    /// Total number of views of the user’s channel.
    pub view_count: usize,
}

impl Request for GetUsersRequest {
    type Response = Vec<User>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
    const PATH: &'static str = "users";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetUsersRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetUsersRequest::id("44322889");

    // From twitch docs
    // FIXME: This is not valid anymore. Twitch....
    let data = br#"
{
    "data": [
        {
        "id": "141981764",
        "login": "twitchdev",
        "display_name": "TwitchDev",
        "type": "",
        "broadcaster_type": "partner",
        "description": "Supporting third-party developers building Twitch integrations from chatbots to game integrations.",
        "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png",
        "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png",
        "view_count": 5980557,
        "email": "not-real@email.com",
        "created_at": "2016-12-14T20:32:28.894263Z"
        }
    ]
    }
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users?id=44322889"
    );

    dbg!(GetUsersRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

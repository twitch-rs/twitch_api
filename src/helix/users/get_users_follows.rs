//! Gets information on follow relationships between two Twitch users.
//! [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
//!
//! ## Request: [GetUsersFollowsRequest]
//!
//! To use this endpoint, construct a [`GetUsersFollowsRequest`] with the [`GetUsersFollowsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::users::get_users_follows;
//! let request = get_users_follows::GetUsersFollowsRequest::builder()
//!     .to_id("1234".to_string())
//!     .build();
//! ```
//!
//! ## Response: [UsersFollow]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, users::get_users_follows};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_users_follows::GetUsersFollowsRequest::builder()
//!     .to_id("1234".to_string())
//!     .build();
//! let response: Vec<get_users_follows::UsersFollow> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())

use super::*;
/// Query Parameters for [Get Users Follows](super::get_users_follows)
///
/// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetUsersFollowsRequest {
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[builder(default)]
    pub first: Option<usize>,
    /// User ID. The request returns information about users who are being followed by the from_id user.
    #[builder(default, setter(into))]
    pub from_id: Option<String>,
    /// User ID. The request returns information about users who are following the to_id user.
    #[builder(default, setter(into))]
    pub to_id: Option<String>,
}

/// Return Values for [Get Users Follows](super::get_users_follows)
///
/// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UsersFollow {
    ///Date and time when the from_id user followed the to_id user.
    pub followed_at: types::Timestamp,
    ///ID of the user following the to_id user.
    pub from_id: types::UserId,
    ///Display name corresponding to from_id.
    pub from_name: types::DisplayName,
    ///ID of the user being followed by the from_id user.
    pub to_id: types::UserId,
    ///Display name corresponding to to_id.
    pub to_name: types::DisplayName,
    // FIXME: This never seems to be returned.
    /// Total number of items returned.
    ///
    /// * If only `from_id` was in the request, this is the total number of followed users.
    /// * If only `to_id` was in the request, this is the total number of followers.
    /// * If both `from_id` and to_id were in the request, this is 1 (if the "from" user follows the "to" user) or 0.
    pub total: Option<usize>,
}

impl helix::Request for GetUsersFollowsRequest {
    type Response = Vec<UsersFollow>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "users/follows";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl helix::RequestGet for GetUsersFollowsRequest {}

impl helix::Paginated for GetUsersFollowsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetUsersFollowsRequest::builder()
        .to_id("23161357".to_string())
        .build();

    // From twitch docs
    let data = br#"
{
    "total": 12345,
    "data":
    [
        {
            "from_id": "171003792",
            "from_name": "IIIsutha067III",
            "to_id": "23161357",
            "to_name": "LIRIK",
            "followed_at": "2017-08-22T22:55:24Z"
        },
        {
            "from_id": "113627897",
            "from_name": "Birdman616",
            "to_id": "23161357",
            "to_name": "LIRIK",
            "followed_at": "2017-08-22T22:55:04Z"
        }
    ],
    "pagination":{
        "cursor": "eyJiIjpudWxsLCJhIjoiMTUwMzQ0MTc3NjQyNDQyMjAwMCJ9"
    }
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/follows?to_id=23161357"
    );

    dbg!(req.parse_response(&uri, http_response).unwrap());
}

//! Returns all banned and timed-out users in a channel.
//! [`get-banned-users`](https://dev.twitch.tv/docs/api/reference#get-banned-users)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetBannedUsersRequest]
//!
//! To use this endpoint, construct a [`GetBannedUsersRequest`] with the [`GetBannedUsersRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::get_banned_users;
//! let request = get_banned_users::GetBannedUsersRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [BannedUser]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::get_banned_users};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_banned_users::GetBannedUsersRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_banned_users::BannedUser> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetBannedUsersRequest::parse_response(None, &request.get_uri(), response)`](GetBannedUsersRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Banned Users](super::get_banned_users)
///
/// [`get-banned-users`](https://dev.twitch.tv/docs/api/reference#get-banned-users)
#[derive(PartialEq, Eq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetBannedUsersRequest {
    /// Must match the User ID in the Bearer token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
    /// Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
    /// Maximum: 100
    #[builder(default)]
    pub user_id: Vec<types::UserId>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub before: Option<helix::Cursor>,
    /// Number of values to be returned per page. Limit: 100. Default: 20.
    #[builder(setter(into), default)]
    pub first: Option<String>,
}

/// Return Values for [Get Banned Users](super::get_banned_users)
///
/// [`get-banned-users`](https://dev.twitch.tv/docs/api/reference#get-banned-users)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BannedUser {
    /// User ID of a user who has been banned.
    pub user_id: types::UserId,
    /// Display name of a user who has been banned.
    pub user_name: types::DisplayName,
    /// Login of a user who has been banned.
    pub user_login: types::UserName,
    #[serde(deserialize_with = "helix::deserialize_none_from_empty_string")]
    /// RFC3339 formatted timestamp for timeouts; empty string for bans.
    pub expires_at: Option<types::Timestamp>,
    /// The reason for the ban if provided by the moderator.
    #[serde(deserialize_with = "helix::deserialize_none_from_empty_string")]
    pub reason: Option<String>,
    /// User ID of the moderator who initiated the ban.
    pub moderator_id: types::UserId,
    /// Login of the moderator who initiated the ban.
    pub moderator_login: types::UserName,
    /// Display name of the moderator who initiated the ban.
    pub moderator_name: types::DisplayName,
}

impl Request for GetBannedUsersRequest {
    type Response = Vec<BannedUser>;

    const PATH: &'static str = "moderation/banned";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
}

impl RequestGet for GetBannedUsersRequest {}

impl helix::Paginated for GetBannedUsersRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetBannedUsersRequest::builder()
        .broadcaster_id("198704263".to_string())
        .build();

    // From twitch docs
    let data = br#"
{
  "data": [
    {
      "user_id": "423374343",
      "user_login": "glowillig",
      "user_name": "glowillig",
      "expires_at": "2022-03-15T02:00:28Z",
      "reason": "Does not like pineapple on pizza.",
      "moderator_id": "141981764",
      "moderator_login": "twitchdev",
      "moderator_name": "TwitchDev"
    },
    {
      "user_id": "424596340",
      "user_login": "quotrok",
      "user_name": "quotrok",
      "expires_at": "2022-08-07T02:07:55Z",
      "reason": "Inappropriate words.",
      "moderator_id": "141981764",
      "moderator_login": "twitchdev",
      "moderator_name": "TwitchDev"
    },
    {
      "user_id": "424596340",
      "user_login": "quotrok",
      "user_name": "quotrok",
      "expires_at": "",
      "reason": "",
      "moderator_id": "141981764",
      "moderator_login": "twitchdev",
      "moderator_name": "TwitchDev"
    }
  ],
  "pagination": {
    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
  }
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/banned?broadcaster_id=198704263"
    );

    dbg!(GetBannedUsersRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

//! Gets a list of users that follow the specified broadcaster. You can also use this endpoint to see whether a specific user follows the broadcaster.
//! [`get-channel-followers`](https://dev.twitch.tv/docs/api/reference#get-channel-followers)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetChannelFollowersRequest]
//!
//! To use this endpoint, construct a [`GetChannelFollowersRequest`] with the [`GetChannelFollowersRequest::broadcaster_id()`] or [`GetChannelFollowersRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::get_channel_followers;
//! let request =
//!     get_channel_followers::GetChannelFollowersRequest::broadcaster_id(
//!         "1234",
//!     );
//! ```
//!
//! ## Response: [Follower]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::get_channel_followers};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_channel_followers::GetChannelFollowersRequest::broadcaster_id("1234");
//! let response: Vec<get_channel_followers::Follower> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelFollowersRequest::parse_response(None, &request.get_uri(), response)`](GetChannelFollowersRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel Followers](super::get_channel_followers)
///
/// [`get-channel-followers`](https://dev.twitch.tv/docs/api/reference#get-channel-followers)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetChannelFollowersRequest<'a> {
    /// A user’s ID. Use this parameter to see whether the user follows this broadcaster.
    /// If specified, the response contains this user if they follow the broadcaster.
    /// If not specified, the response contains all users that follow the broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Option<Cow<'a, types::UserIdRef>>,
    /// Broadcaster’s user ID associated with the channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
}

impl<'a> GetChannelFollowersRequest<'a> {
    /// Get specified broadcasters channel editors
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            user_id: None,
            after: None,
            first: None,
        }
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }

    /// Check if this user id is following the broadcaster
    pub fn user_id(self, user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: Some(user_id.into_cow()),
            ..self
        }
    }
}

/// Return Values for [Get Channel Followers](super::get_channel_followers)
///
/// [`get-channel-followers`](https://dev.twitch.tv/docs/api/reference#get-channel-followers)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Follower {
    /// The UTC timestamp when the user started following the broadcaster.
    pub followed_at: types::Timestamp,
    /// An ID that uniquely identifies the user that’s following the broadcaster.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,
}

impl Request for GetChannelFollowersRequest<'_> {
    type Response = Vec<Follower>;

    const PATH: &'static str = "channels/followers";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModeratorReadFollowers];
}

impl RequestGet for GetChannelFollowersRequest<'_> {}

impl helix::Paginated for GetChannelFollowersRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelFollowersRequest::broadcaster_id("123456");

    // From twitch docs
    // FIXME: example has trailing `,`, is missing a `,` and has ...
    let data = br#"
    {
        "total": 8,
        "data": [
          {
            "user_id": "w",
            "user_name": "UserDisplayName",
            "user_login": "userloginname",
            "followed_at": "2022-05-24T22:22:08Z"
          },
          {
            "user_id": "11111",
            "user_name": "UserDisplayName",
            "user_login": "userloginname",
            "followed_at": "2022-05-24T22:22:08Z"
          }
        ],
        "pagination": {
          "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
        }
      }
        "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/followers?broadcaster_id=123456"
    );

    dbg!(GetChannelFollowersRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

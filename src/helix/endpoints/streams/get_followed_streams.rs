//! Gets information about active streams belonging to channels that the authenticated user follows.
//! [`get-followed-streams`](https://dev.twitch.tv/docs/api/reference#get-followed-streams)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetFollowedStreamsRequest]
//!
//! To use this endpoint, construct a [`GetFollowedStreamsRequest`] with the [`GetFollowedStreamsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::streams::get_followed_streams;
//! let request = get_followed_streams::GetFollowedStreamsRequest::builder()
//!     .user_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [Stream]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, streams::get_followed_streams};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_followed_streams::GetFollowedStreamsRequest::builder()
//!     .user_id("1234")
//!     .build();
//! let response: Vec<get_followed_streams::GetFollowedStreamsResponse> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetFollowedStreamsRequest::parse_response(None, &request.get_uri(), response)`](GetFollowedStreamsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Followed Streams](super::get_followed_streams)
///
/// [`get-followed-streams`](https://dev.twitch.tv/docs/api/reference#get-followed-streams)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetFollowedStreamsRequest<'a> {
    /// Returns streams broadcast by one or more specified user IDs. You can specify up to 100 IDs.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub after: Option<helix::Cursor>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(borrow)]
    pub before: Option<&'a helix::CursorRef>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
}

impl<'a> GetFollowedStreamsRequest<'a> {
    /// Get a users followed streams.
    ///
    /// Requires token with scope [`user:read:follows`](twitch_oauth2::Scope::UserReadFollows).
    ///
    /// See also [`HelixClient::get_followed_streams`](crate::helix::HelixClient::get_followed_streams).
    pub fn user_id(user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: user_id.to_cow(),
            after: Default::default(),
            before: Default::default(),
            first: Default::default(),
        }
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Followed Streams](super::get_followed_streams)
///
/// [`get-followed-streams`](https://dev.twitch.tv/docs/api/reference#get-followed-streams)
pub type GetFollowedStreamsResponse = Stream;

impl Request for GetFollowedStreamsRequest<'_> {
    type Response = Vec<GetFollowedStreamsResponse>;

    const PATH: &'static str = "streams/followed";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadFollows];
}

impl RequestGet for GetFollowedStreamsRequest<'_> {}

impl helix::Paginated for GetFollowedStreamsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetFollowedStreamsRequest::user_id("141981764");

    // From twitch docs.
    let data = br#"
{
    "data": [
        {
            "id": "26007494656",
            "user_id": "23161357",
            "user_name": "LIRIK",
            "user_login": "lirik",
            "game_id": "417752",
            "game_name": "Talk Shows & Podcasts",
            "type": "live",
            "title": "Hey Guys, It's Monday - Twitter: @Lirik",
            "viewer_count": 32575,
            "started_at": "2017-08-14T16:08:32Z",
            "language": "en",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_lirik-{width}x{height}.jpg",
            "tag_ids":  [
                "6ea6bca4-4712-4ab9-a906-e3336a9d8039"
            ],
            "is_mature": false
        },
        {
            "id": "26007494656",
            "user_id": "23161357",
            "user_name": "LIRIK",
            "user_login": "lirik",
            "game_id": "417752",
            "game_name": "Talk Shows & Podcasts",
            "type": "live",
            "title": "Hey Guys, It's Monday - Twitter: @Lirik",
            "viewer_count": 32575,
            "started_at": "2017-08-14T16:08:32Z",
            "language": "en",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_lirik-{width}x{height}.jpg",
            "tag_ids":  [
                "6ea6bca4-4712-4ab9-a906-e3336a9d8039"
            ],
            "is_mature": false
        }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6MjB9fQ=="
    }
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/streams/followed?user_id=141981764"
    );

    dbg!(GetFollowedStreamsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

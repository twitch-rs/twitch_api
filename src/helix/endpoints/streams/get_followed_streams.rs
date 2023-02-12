//! Gets information about active streams belonging to channels that the authenticated user follows.
//! [`get-followed-streams`](https://dev.twitch.tv/docs/api/reference#get-followed-streams)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetFollowedStreamsRequest]
//!
//! To use this endpoint, construct a [`GetFollowedStreamsRequest`] with the [`GetFollowedStreamsRequest::user_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::streams::get_followed_streams;
//! let request =
//!     get_followed_streams::GetFollowedStreamsRequest::user_id("1234");
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
//! let request = get_followed_streams::GetFollowedStreamsRequest::user_id("1234");
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
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub before: Option<Cow<'a, helix::CursorRef>>,
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
            user_id: user_id.into_cow(),
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
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
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
            "id": "42170724654",
            "user_id": "132954738",
            "user_login": "aws",
            "user_name": "AWS",
            "game_id": "417752",
            "game_name": "Talk Shows & Podcasts",
            "type": "live",
            "title": "AWS Howdy Partner! Y'all welcome ExtraHop to the show!",
            "viewer_count": 20,
            "started_at": "2021-03-31T20:57:26Z",
            "language": "en",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_aws-{width}x{height}.jpg",
            "tag_ids": ["6ea6bca4-4712-4ab9-a906-e3336a9d8039"],
            "tags": ["English"],
            "is_mature": false
          },
          {
            "id": "42170724654",
            "user_id": "132954738",
            "user_login": "aws",
            "user_name": "AWS",
            "game_id": "417752",
            "game_name": "Talk Shows & Podcasts",
            "type": "live",
            "title": "AWS Howdy Partner! Y'all welcome ExtraHop to the show!",
            "viewer_count": 20,
            "started_at": "2021-03-31T20:57:26Z",
            "language": "en",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_aws-{width}x{height}.jpg",
            "tag_ids": ["6ea6bca4-4712-4ab9-a906-e3336a9d8039"],
            "tags": ["English"],
            "is_mature": false
          }
    ],
    "pagination": {
        "cursor": "eyJiIjp7IkN1cnNvciI6ImV5SnpJam8zT0RNMk5TNDBORFF4TlRjMU1UY3hOU3dpWkNJNlptRnNjMlVzSW5RaU9uUnlkV1Y5In0sImEiOnsiQ3Vyc29yIjoiZXlKeklqb3hOVGs0TkM0MU56RXhNekExTVRZNU1ESXNJbVFpT21aaGJITmxMQ0owSWpwMGNuVmxmUT09In19=="
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

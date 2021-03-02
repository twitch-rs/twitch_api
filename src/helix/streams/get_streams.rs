//! Gets information about active streams.
//! [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetStreamsRequest]
//!
//! To use this endpoint, construct a [`GetStreamsRequest`] with the [`GetStreamsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::streams::get_streams;
//! let request = get_streams::GetStreamsRequest::builder()
//!     .user_login(vec!["justintvfan".to_string()])
//!     .build();
//! ```
//!
//! ## Response: [Stream]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, streams::get_streams};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_streams::GetStreamsRequest::builder()
//!     .user_login(vec!["justintvfan".to_string()])
//!     .build();
//! let response: Vec<get_streams::Stream> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())

use super::*;

/// Query Parameters for [Get Streams](super::get_streams)
///
/// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetStreamsRequest {
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub before: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub first: Option<usize>,
    /// Returns streams broadcasting a specified game ID. You can specify up to 10 IDs.
    #[builder(default)]
    pub game_id: Vec<types::CategoryId>,
    /// Stream language. You can specify up to 100 languages.
    #[builder(default)]
    pub language: Option<String>,
    /// Returns streams broadcast by one or more specified user IDs. You can specify up to 100 IDs.
    #[builder(default, setter(into))]
    pub user_id: Vec<types::UserId>,
    /// Returns streams broadcast by one or more specified user login names. You can specify up to 100 names.
    #[builder(default)]
    pub user_login: Vec<types::UserName>,
}

/// Return Values for [Get Streams](super::get_streams)
///
/// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Stream {
    /// ID of the game being played on the stream.
    pub game_id: types::CategoryId,
    /// Stream ID.
    pub id: String,
    /// Stream language.
    pub language: String,
    /// UTC timestamp.
    pub started_at: types::Timestamp,
    /// Shows tag IDs that apply to the stream.
    pub tag_ids: Vec<types::TagId>,
    /// Thumbnail URL of the stream. All image URLs have variable width and height. You can replace {width} and {height} with any values to get that size image
    pub thumbnail_url: String,
    /// Stream title.
    pub title: String,
    /// Stream type: "live" or "" (in case of error).
    #[serde(rename = "type")]
    pub type_: StreamType,
    /// ID of the user who is streaming.
    pub user_id: types::UserId,
    /// Display name corresponding to user_id.
    pub user_name: types::DisplayName,
    /// Login of the user who is streaming.
    pub user_login: types::UserName,
    /// Number of viewers watching the stream at the time of the query.
    pub viewer_count: usize,
}

impl helix::Request for GetStreamsRequest {
    type Response = Vec<Stream>;

    const PATH: &'static str = "streams";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl helix::RequestGet for GetStreamsRequest {}

impl helix::Paginated for GetStreamsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetStreamsRequest::builder().build();

    // From twitch docs. example 1 in https://dev.twitch.tv/docs/api/reference#get-streams is malformed
    let data = br#"
{
    "data": [
        {
            "id": "26007494656",
            "user_id": "23161357",
            "user_name": "LIRIK",
            "user_login": "lirik",
            "game_id": "417752",
            "type": "live",
            "title": "Hey Guys, It's Monday - Twitter: @Lirik",
            "viewer_count": 32575,
            "started_at": "2017-08-14T16:08:32Z",
            "language": "en",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_lirik-{width}x{height}.jpg",
            "tag_ids":  [
                "6ea6bca4-4712-4ab9-a906-e3336a9d8039"
            ]
        },
        {
            "id": "26007494656",
            "user_id": "23161357",
            "user_name": "LIRIK",
            "user_login": "lirik",
            "game_id": "417752",
            "type": "live",
            "title": "Hey Guys, It's Monday - Twitter: @Lirik",
            "viewer_count": 32575,
            "started_at": "2017-08-14T16:08:32Z",
            "language": "en",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_lirik-{width}x{height}.jpg",
            "tag_ids":  [
                "6ea6bca4-4712-4ab9-a906-e3336a9d8039"
            ]
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
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/streams?");

    dbg!(GetStreamsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

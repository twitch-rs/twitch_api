//! Gets video information by video ID (one or more), user ID (one only), or game ID (one only).
//! [`get-videos`](https://dev.twitch.tv/docs/api/reference#get-videos)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetVideosRequest]
//!
//! To use this endpoint, construct a [`GetVideosRequest`]
//!
//! ```rust
//! use twitch_api::helix::videos::get_videos;
//! let request = get_videos::GetVideosRequest::user_id("1234");
//! ```
//!
//! ## Response: [Video]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, videos::get_videos};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_videos::GetVideosRequest::user_id("1234");
//! let response: Vec<get_videos::Video> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetVideosRequest::parse_response(None, &request.get_uri(), response)`](GetVideosRequest::parse_response)

use super::*;
use helix::RequestGet;

// FIXME: One of id, user_id or game_id needs to be specified. typed_builder should have enums. id can not be used with other params
/// Query Parameters for [Get Videos](super::get_videos)
///
/// [`get-videos`](https://dev.twitch.tv/docs/api/reference#get-videos)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetVideosRequest<'a> {
    /// ID of the video being queried. Limit: 100. If this is specified, you cannot use any of the optional query parameters below.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "Cow::Borrowed(&[])", setter(into))
    )]
    #[serde(borrow="'a")]
    pub id: Cow<'a, [&'a types::VideoIdRef]>,
    /// ID of the user who owns the video.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(borrow="'a")]
    pub user_id: Option<Cow<'a, types::UserIdRef>>,
    /// ID of the game the video is of.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(borrow="'a")]
    pub game_id: Option<Cow<'a, types::CategoryIdRef>>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(borrow="'a")]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(borrow="'a")]
    pub before: Option<Cow<'a, helix::CursorRef>>,
    /// Number of values to be returned when getting videos by user or game ID. Limit: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// Language of the video being queried. Limit: 1.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(borrow="'a")]
    pub language: Option<Cow<'a, str>>,
    /// Period during which the video was created. Valid values: "all", "day", "week", "month". Default: "all".
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub period: Option<VideoPeriod>,
    /// Sort order of the videos. Valid values: "time", "trending", "views". Default: "time".
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub sort: Option<Sort>,
    /// Type of video. Valid values: "all", "upload", "archive", "highlight". Default: "all".
    #[serde(rename = "type")]
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub type_: Option<VideoTypeFilter>,
}

impl<'a> GetVideosRequest<'a> {
    /// IDs of the videos being queried.
    pub fn ids(ids: impl Into<Cow<'a, [&'a types::VideoIdRef]>>) -> Self {
        Self {
            id: ids.into(),
            ..Self::default()
        }
    }

    /// ID of the user who owns the video.
    pub fn user_id(user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: Some(user_id.to_cow()),
            ..Self::default()
        }
    }

    /// ID of the game the video is of.
    pub fn game_id(game_id: impl types::IntoCow<'a, types::CategoryIdRef> + 'a) -> Self {
        Self {
            game_id: Some(game_id.to_cow()),
            ..Self::default()
        }
    }
}

/// Return Values for [Get Videos](super::get_videos)
///
/// [`get-videos`](https://dev.twitch.tv/docs/api/reference#get-videos)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Video {
    /// Date when the video was created.
    pub created_at: types::Timestamp,
    /// Description of the video.
    pub description: String,
    /// Length of the video.
    pub duration: String,
    /// ID of the video.
    pub id: types::VideoId,
    /// Language of the video.
    pub language: String,
    /// Muted segments in the video.
    #[serde(deserialize_with = "helix::deserialize_default_from_null")]
    pub muted_segments: Vec<MutedSegment>,
    /// Date when the video was published.
    pub published_at: types::Timestamp,
    /// ID of the stream that the video originated from if the type is "archive". Otherwise set to null.
    pub stream_id: Option<types::StreamId>,
    /// Template URL for the thumbnail of the video.
    pub thumbnail_url: String,
    /// Title of the video.
    pub title: String,
    /// Type of video. Valid values: "upload", "archive", "highlight".
    #[serde(rename = "type")]
    pub type_: types::VideoType,
    /// URL of the video.
    pub url: String,
    /// ID of the user who owns the video.
    pub user_id: types::UserId,
    /// Display name corresponding to user_id.
    pub user_name: types::DisplayName,
    /// Login of the user who owns the video.
    pub user_login: types::UserName,
    /// Number of times the video has been viewed.
    pub view_count: i64,
    /// Indicates whether the video is publicly viewable. Valid values: "public", "private".
    pub viewable: types::VideoPrivacy,
}

/// muted segment in a video.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct MutedSegment {
    /// Duration of the muted segment.
    pub duration: i64,
    /// Offset in the video at which the muted segment begins.
    pub offset: i64,
}

impl Request for GetVideosRequest<'_> {
    type Response = Vec<Video>;

    const PATH: &'static str = "videos";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetVideosRequest<'_> {}

impl helix::Paginated for GetVideosRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetVideosRequest::ids(vec!["234482848".into()]);

    // From twitch docs
    let data = br#"
{
  "data": [
    {
      "id": "335921245",
      "stream_id": null,
      "user_id": "141981764",
      "user_login": "twitchdev",
      "user_name": "TwitchDev",
      "title": "Twitch Developers 101",
      "description": "Welcome to Twitch development! Here is a quick overview of our products and information to help you get started.",
      "created_at": "2018-11-14T21:30:18Z",
      "published_at": "2018-11-14T22:04:30Z",
      "url": "https://www.twitch.tv/videos/335921245",
      "thumbnail_url": "https://static-cdn.jtvnw.net/cf_vods/d2nvs31859zcd8/twitchdev/335921245/ce0f3a7f-57a3-4152-bc06-0c6610189fb3/thumb/index-0000000000-%{width}x%{height}.jpg",
      "viewable": "public",
      "view_count": 1863062,
      "language": "en",
      "type": "upload",
      "duration": "3m21s",
      "muted_segments": [
        {
          "duration": 30,
          "offset": 120
        }
      ]
    }
  ],
  "pagination": {}
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/videos?id=234482848"
    );

    dbg!(GetVideosRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

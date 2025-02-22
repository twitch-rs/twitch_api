//! Gets a list of markers from the user’s most recent stream or from the specified VOD/video.
//! [`get-stream-markers`](https://dev.twitch.tv/docs/api/reference#get-stream-markers)
//!
//! A marker is an arbitrary point in a live stream that the broadcaster or editor marked, so they can return to that spot later to create video highlights (see Video Producer, Highlights in the Twitch UX).
//!
//! # Accessing the endpoint
//!
//! ## Notes
//!
//! See also [`HelixClient::get_stream_markers_from_video`](crate::helix::HelixClient::get_stream_markers_from_video).
//!
//! ## Request: [GetStreamMarkersRequest]
//!
//! To use this endpoint, construct a [`GetStreamMarkersRequest`] with the [`GetStreamMarkersRequest::video_id()`], or [`GetStreamMarkersRequest::user_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::streams::get_stream_markers;
//! let request = get_stream_markers::GetStreamMarkersRequest::video_id("1234");
//! ```
//!
//! ## Response: [StreamMarkerGroup]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, streams::get_stream_markers};
//! # use twitch_api::{client, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client: helix::HelixClient<'static, client::DummyHttpClient> =
//!     helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_stream_markers::GetStreamMarkersRequest::video_id("1234");
//! let response: Vec<helix::streams::StreamMarkerGroup> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetStreamMarkersRequest::parse_response(None, &request.get_uri(), response)`](GetStreamMarkersRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Stream Markers](super::get_stream_markers)
///
/// [`get-stream-markers`](https://dev.twitch.tv/docs/api/reference#get-stream-markers)
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetStreamMarkersRequest<'a> {
    /// A user ID. The request returns the markers from this user’s most recent video. This ID must match the user ID in the access token or the user in the access token must be one of the broadcaster’s editors.
    ///
    /// This parameter and the `video_id`` query parameter are mutually exclusive.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Option<Cow<'a, types::UserIdRef>>,
    /// A video on demand (VOD)/video ID. The request returns the markers from this VOD/video. The user in the access token must own the video or the user must be one of the broadcaster’s editors.
    ///
    /// This parameter and the user_id query parameter are mutually exclusive.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub video_id: Option<Cow<'a, types::VideoIdRef>>,
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

impl<'a> GetStreamMarkersRequest<'a> {
    /// Return stream markers from the most recent video of the specified user.
    pub fn user_id(user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: Some(user_id.into_cow()),
            video_id: None,
            after: None,
            before: None,
            first: None,
        }
    }

    /// Return stream markers for a specific video.
    pub fn video_id(video_id: impl types::IntoCow<'a, types::VideoIdRef> + 'a) -> Self {
        Self {
            user_id: None,
            video_id: Some(video_id.into_cow()),
            after: None,
            before: None,
            first: None,
        }
    }

    /// Set amount of results returned per page.
    pub const fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Stream Markers](super::get_stream_markers)
///
/// [`get-stream-markers`](https://dev.twitch.tv/docs/api/reference#get-stream-markers)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StreamMarkerGroup {
    /// The ID of the user that created the marker.
    pub user_id: types::UserId,
    /// The user’s display name.
    pub user_name: types::DisplayName,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// A list of videos that contain markers. The list contains a single video.
    pub videos: Vec<StreamMarkerVideo>,
}

/// A video with markers
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StreamMarkerVideo {
    /// An ID that identifies this video.
    pub video_id: types::VideoId,
    /// The list of markers in this video. The list in ascending order by when the marker was created.
    pub markers: Vec<StreamMarker>,
}

/// A marker on a video
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StreamMarker {
    /// An ID that identifies this marker.
    pub id: types::StreamMarkerId,
    /// The UTC date and time (in RFC3339 format) of when the user created the marker.
    pub created_at: types::Timestamp,
    /// The relative offset (in seconds) of the marker from the beginning of the stream.
    pub position_seconds: u64,
    /// The description that the user gave the marker to help them remember why they marked the location. Is an empty string if the user didn’t provide one.
    pub description: String,
    /// A URL that opens the video in Twitch Highlighter.
    pub url: String,
}

impl Request for GetStreamMarkersRequest<'_> {
    type Response = Vec<StreamMarkerGroup>;

    const PATH: &'static str = "streams/markers";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::UserReadBroadcast,
        twitch_oauth2::Scope::ChannelManageBroadcast,
    )];
}

impl RequestGet for GetStreamMarkersRequest<'_> {}

impl helix::Paginated for GetStreamMarkersRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetStreamMarkersRequest::user_id("123").first(5);

    // From twitch docs (except 'URL' is 'url' here)
    let data = r#"
    {
      "data": [
        {
          "user_id": "123",
          "user_name": "TwitchName",
          "user_login": "twitchname",
          "videos": [
            {
              "video_id": "456",
              "markers": [
                {
                  "id": "106b8d6243a4f883d25ad75e6cdffdc4",
                  "created_at": "2018-08-20T20:10:03Z",
                  "description": "hello, this is a marker!",
                  "position_seconds": 244,
                  "url": "https://twitch.tv/twitchname/manager/highlighter/456?t=0h4m06s"
                }
              ]
            }
          ]
        }
      ],
      "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjoiMjk1MjA0Mzk3OjI1Mzpib29rbWFyazoxMDZiOGQ1Y"
      }
    }
    "#
    .as_bytes()
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/streams/markers?user_id=123&first=5"
    );

    let res = GetStreamMarkersRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].user_name.as_str(), "TwitchName");
    assert_eq!(res[0].videos.len(), 1);
    assert_eq!(res[0].videos[0].video_id.as_str(), "456");
    assert_eq!(res[0].videos[0].markers.len(), 1);
    assert_eq!(res[0].videos[0].markers[0].position_seconds, 244);
    assert_eq!(
        res[0].videos[0].markers[0].url,
        "https://twitch.tv/twitchname/manager/highlighter/456?t=0h4m06s"
    );
}

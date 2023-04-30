//! Gets clip information by clip ID (one or more), broadcaster ID (one only), or game ID (one only).
//! [`get-clips`](https://dev.twitch.tv/docs/api/reference#get-clips)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetClipsRequest]
//!
//! To use this endpoint, construct a [`GetClipsRequest`] with the [`GetClipsRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::clips::get_clips;
//! let request = get_clips::GetClipsRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [Clip]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, clips::get_clips};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_clips::GetClipsRequest::broadcaster_id("1234");
//! let response: Vec<get_clips::Clip> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetClipsRequest::parse_response(None, &request.get_uri(), response)`](GetClipsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Clips](super::get_clips)
///
/// [`get-clips`](https://dev.twitch.tv/docs/api/reference#get-clips)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetClipsRequest<'a> {
    /// ID of the broadcaster for whom clips are returned. The number of clips returned is determined by the first query-string parameter (default: 20). Results are ordered by view count.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Option<Cow<'a, types::UserIdRef>>,
    /// ID of the game for which clips are returned. The number of clips returned is determined by the first query-string parameter (default: 20). Results are ordered by view count.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub game_id: Option<Cow<'a, types::CategoryIdRef>>,
    // FIXME: add types::ClipId
    /// ID of the clip being queried. Limit: 100.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub id: Cow<'a, [&'a types::ClipIdRef]>,
    // one of above is needed.
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries specifying broadcaster_id or game_id. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries specifying broadcaster_id or game_id. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub before: Option<Cow<'a, helix::CursorRef>>,
    /// Ending date/time for returned clips, in RFC3339 format. (Note that the seconds value is ignored.) If this is specified, started_at also must be specified; otherwise, the time period is ignored.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub ended_at: Option<Cow<'a, types::TimestampRef>>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// Starting date/time for returned clips, in RFC3339 format. (Note that the seconds value is ignored.) If this is specified, ended_at also should be specified; otherwise, the ended_at date/time will be 1 week after the started_at value.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub started_at: Option<Cow<'a, types::TimestampRef>>,
}

impl<'a> GetClipsRequest<'a> {
    /// An empty request
    ///
    /// # Notes
    ///
    /// This is not a valid request and needs to be filled out with other fields
    pub fn empty() -> Self {
        Self {
            broadcaster_id: Default::default(),
            game_id: Default::default(),
            id: Cow::Borrowed(&[]),
            after: Default::default(),
            before: Default::default(),
            ended_at: Default::default(),
            first: Default::default(),
            started_at: Default::default(),
        }
    }

    /// Broadcaster for whom clips are returned.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: Some(broadcaster_id.into_cow()),
            ..Self::empty()
        }
    }

    /// Game for which clips are returned.
    pub fn game_id(game_id: impl types::IntoCow<'a, types::CategoryIdRef> + 'a) -> Self {
        Self {
            game_id: Some(game_id.into_cow()),
            ..Self::empty()
        }
    }

    /// IDs of clips being queried
    pub fn clip_ids(clip_ids: impl Into<Cow<'a, [&'a types::ClipIdRef]>>) -> Self {
        Self {
            id: clip_ids.into(),
            ..Self::empty()
        }
    }

    /// Ending date/time for the returned clips
    pub fn started_at(
        &mut self,
        started_at: impl types::IntoCow<'a, types::TimestampRef> + 'a,
    ) -> &mut Self {
        self.started_at = Some(started_at.into_cow());
        self
    }

    /// Ending date/time for the returned clips
    pub fn ended_at(
        &mut self,
        ended_at: impl types::IntoCow<'a, types::TimestampRef> + 'a,
    ) -> &mut Self {
        self.ended_at = Some(ended_at.into_cow());
        self
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Clips](super::get_clips)
///
/// [`get-clips`](https://dev.twitch.tv/docs/api/reference#get-clips)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Clip {
    /// User ID of the stream from which the clip was created.
    pub broadcaster_id: types::UserId,
    /// Display name corresponding to broadcaster_id.
    pub broadcaster_name: types::DisplayName,
    /// Date when the clip was created.
    pub created_at: types::Timestamp,
    /// ID of the user who created the clip.
    pub creator_id: types::UserId,
    /// Display name corresponding to creator_id.
    pub creator_name: types::DisplayName,
    /// Duration of the Clip in seconds (up to 0.1 precision).
    pub duration: f64,
    /// URL to embed the clip.
    pub embed_url: String,
    /// ID of the game assigned to the stream when the clip was created.
    pub game_id: types::CategoryId,
    /// ID of the clip being queried.
    pub id: String,
    /// Language of the stream from which the clip was created.
    pub language: String,
    /// URL of the clip thumbnail.
    pub thumbnail_url: String,
    /// Title of the clip.
    pub title: String,
    /// URL where the clip can be viewed.
    pub url: String,
    /// ID of the video from which the clip was created.
    pub video_id: types::VideoId,
    /// Number of times the clip has been viewed.
    pub view_count: i64,
    /// The zero-based offset, in seconds, to where the clip starts in the video (VOD).
    ///
    /// Is none if the video is not available or hasn’t been created yet from the live stream. See [video_id](Clip::video_id).
    pub vod_offset: Option<i64>,
}

impl Request for GetClipsRequest<'_> {
    type Response = Vec<Clip>;

    const PATH: &'static str = "clips";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetClipsRequest<'_> {}

impl helix::Paginated for GetClipsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;

    let req = GetClipsRequest::clip_ids(vec!["AwkwardHelplessSalamanderSwiftRage".into()]);

    // From twitch docs
    let data = br#"
{
    "data": [
      {
        "id": "RandomClip1",
        "url": "https://clips.twitch.tv/AwkwardHelplessSalamanderSwiftRage",
        "embed_url": "https://clips.twitch.tv/embed?clip=RandomClip1",
        "broadcaster_id": "1234",
        "broadcaster_name": "JJ",
        "creator_id": "123456",
        "creator_name": "MrMarshall",
        "video_id": "",
        "game_id": "33103",
        "language": "en",
        "title": "random1",
        "view_count": 10,
        "created_at": "2017-11-30T22:34:18Z",
        "thumbnail_url": "https://clips-media-assets.twitch.tv/157589949-preview-480x272.jpg",
        "duration": 12.9,
        "vod_offset": 1957
      }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjoiIn0"
    }
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/clips?id=AwkwardHelplessSalamanderSwiftRage"
    );

    dbg!(GetClipsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

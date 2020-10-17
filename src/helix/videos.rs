//! Endpoints regarding videos
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, videos::GetVideosRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetVideosRequest::builder()
//!     .id(vec!["1337".to_string()])
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use get_videos::{GetVideosRequest, Video};

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Sort order of the videos
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    /// Sort by time
    Time,
    /// Sort by trending
    Trending,
    /// Sort by views
    Views,
}

/// Period during which the video was created
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPeriod {
    /// Filter by all. Effectively a no-op
    All,
    /// Filter by from this day only
    Day,
    /// Filter by this week
    Week,
    /// Filter by this month
    Month,
}

/// Type of video
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoType {
    /// All video types
    All,
    /// A uploaded video
    Upload,
    /// An archived video
    Archive,
    /// A highlight
    Highlight,
}

/// Type of video
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPrivacy {
    /// Video is public
    Public,
    /// Video is private
    Private,
}

/// Gets video information by video ID (one or more), user ID (one only), or game ID (one only).
/// [`get-videos`](https://dev.twitch.tv/docs/api/reference#get-videos)
pub mod get_videos {
    use super::*;

    // FIXME: One of id, user_id or game_id needs to be specified. typed_builder should have enums. id can not be used with other params
    /// Query Parameters for [Get Videos](super::get_videos)
    ///
    /// [`get-videos`](https://dev.twitch.tv/docs/api/reference#get-videos)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetVideosRequest {
        /// ID of the video being queried. Limit: 100. If this is specified, you cannot use any of the optional query parameters below.
        #[builder(default)]
        pub id: Vec<types::VideoId>,
        /// ID of the user who owns the video.
        #[builder(default, setter(into))]
        pub user_id: Option<types::UserId>,
        /// ID of the game the video is of.
        #[builder(default, setter(into))]
        pub game_id: Option<types::CategoryId>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub after: Option<helix::Cursor>,
        /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub before: Option<helix::Cursor>,
        /// Number of values to be returned when getting videos by user or game ID. Limit: 100. Default: 20.
        #[builder(default)]
        #[builder(setter(strip_option))]
        pub first: Option<usize>,
        /// Language of the video being queried. Limit: 1.
        #[builder(default, setter(into))]
        pub language: Option<String>,
        /// Period during which the video was created. Valid values: "all", "day", "week", "month". Default: "all".
        #[builder(default, setter(into))]
        pub period: Option<VideoPeriod>,
        /// Sort order of the videos. Valid values: "time", "trending", "views". Default: "time".
        #[builder(default, setter(into))]
        pub sort: Option<Sort>,
        /// Type of video. Valid values: "all", "upload", "archive", "highlight". Default: "all".
        #[serde(rename = "type")]
        #[builder(default, setter(into))]
        pub type_: Option<VideoType>,
    }

    /// Return Values for [Get Videos](super::get_videos)
    ///
    /// [`get-videos`](https://dev.twitch.tv/docs/api/reference#get-videos)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Video {
        /// Date when the video was created.
        created_at: types::Timestamp,
        /// Description of the video.
        description: String,
        /// Length of the video.
        duration: String,
        /// ID of the video.
        id: types::VideoId,
        /// Language of the video.
        language: String,
        /// Date when the video was published.
        published_at: types::Timestamp,
        /// Template URL for the thumbnail of the video.
        thumbnail_url: String,
        /// Title of the video.
        title: String,
        /// Type of video. Valid values: "upload", "archive", "highlight".
        #[serde(rename = "type")]
        type_: VideoType,
        /// URL of the video.
        url: String,
        /// ID of the user who owns the video.
        user_id: types::UserId,
        /// Display name corresponding to user_id.
        user_name: types::DisplayName,
        /// Number of times the video has been viewed.
        view_count: i64,
        /// Indicates whether the video is publicly viewable. Valid values: "public", "private".
        viewable: VideoPrivacy,
    }

    impl helix::Request for GetVideosRequest {
        type Response = Vec<Video>;

        const PATH: &'static str = "videos";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetVideosRequest {}

    impl helix::Paginated for GetVideosRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetVideosRequest::builder()
            .id(vec!["234482848".to_string()])
            .build();

        // From twitch docs
        let data = br#"
{
    "data": [{
      "id": "234482848",
      "user_id": "67955580",
      "user_name": "ChewieMelodies",
      "title": "-",
      "description": "",
      "created_at": "2018-03-02T20:53:41Z",
      "published_at": "2018-03-02T20:53:41Z",
      "url": "https://www.twitch.tv/videos/234482848",
      "thumbnail_url": "https://static-cdn.jtvnw.net/s3_vods/bebc8cba2926d1967418_chewiemelodies_27786761696_805342775/thumb/thumb0-%{width}x%{height}.jpg",
      "viewable": "public",
      "view_count": 142,
      "language": "en",
      "type": "archive",
      "duration": "3h8m33s"
    }],
    "pagination":{"cursor":"eyJiIjpudWxsLCJhIjoiMTUwMzQ0MTc3NjQyNDQyMjAwMCJ9"}
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/videos?id=234482848"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

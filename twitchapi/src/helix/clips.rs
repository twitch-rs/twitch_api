pub use get_clips::{GetClips, GetClipsRequest};

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub mod get_clips {
    use super::*;
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    pub struct GetClipsRequest {
        /// ID of the broadcaster for whom clips are returned. The number of clips returned is determined by the first query-string parameter (default: 20). Results are ordered by view count.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        broadcaster_id: Option<String>,
        /// ID of the game for which clips are returned. The number of clips returned is determined by the first query-string parameter (default: 20). Results are ordered by view count.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        game_id: Option<String>,
        /// ID of the clip being queried. Limit: 100.
        /// FIXME: This is not currently supported, we don't query correctly on ids. See [crate::helix::streams]
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        id: Vec<String>,
        // one of above is needed.
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries specifying broadcaster_id or game_id. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        after: Option<String>,
        /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries specifying broadcaster_id or game_id. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        before: Option<String>,
        /// Ending date/time for returned clips, in RFC3339 format. (Note that the seconds value is ignored.) If this is specified, started_at also must be specified; otherwise, the time period is ignored.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        ended_at: Option<String>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        first: Option<usize>,
        /// Starting date/time for returned clips, in RFC3339 format. (Note that the seconds value is ignored.) If this is specified, ended_at also should be specified; otherwise, the ended_at date/time will be 1 week after the started_at value.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        started_at: Option<String>,
    }

    #[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
    pub struct GetClips {
        /// User ID of the stream from which the clip was created.
        pub broadcaster_id: String,
        /// Display name corresponding to broadcaster_id.
        pub broadcaster_name: String,
        /// Date when the clip was created.
        pub created_at: String,
        /// ID of the user who created the clip.
        pub creator_id: String,
        /// Display name corresponding to creator_id.
        pub creator_name: String,
        /// URL to embed the clip.
        pub embed_url: String,
        /// ID of the game assigned to the stream when the clip was created.
        pub game_id: String,
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
        pub video_id: String,
        /// Number of times the clip has been viewed.
        pub view_count: usize,
    }

    impl helix::Request for GetClipsRequest {
        type Response = GetClips;

        const PATH: &'static str = "clips";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetClipsRequest {}

    impl helix::Paginated for GetClipsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }
}

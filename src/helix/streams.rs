//! Endpoints regarding streams
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, streams::GetStreamsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetStreamsRequest::builder()
//!     .user_login(vec!["justinfan1337".to_string()])
//!     .build();
//!
//! // If this doesn't return a result, that would mean the stream is not live.
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use get_streams::{Stream, GetStreamsRequest};

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Gotten from [Stream.type_](get_streams::Stream#structfield.type_)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum StreamType {
    /// Stream is live.
    #[serde(rename = "live")]
    Live,

    // Got error from endpoint
    //Error, TODO: Should this be here?

    //
    /// Stream not live
    ///
    /// # Notes
    /// This is never returned from twitch endpoints. To get this
    /// Just do a [GetStreamsRequest] and if there is no response for your user_id/user_login, you can be
    /// sure that the channel is not live
    #[serde(other)]
    NotLive,
}

impl StreamType {
    /// Check if the stream is live or not
    pub fn is_live(&self) -> bool { matches!(self, StreamType::Live) }
}
/// Gets information about active streams.
/// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
pub mod get_streams {
    use super::*;

    /// Query Parameters for [Get Streams](super::get_streams)
    ///
    /// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetStreamsRequest {
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
        /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub before: Option<String>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[builder(setter(strip_option))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first: Option<usize>,
        /// Returns streams broadcasting a specified game ID. You can specify up to 10 IDs.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub game_id: Vec<String>,
        /// Stream language. You can specify up to 100 languages.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<String>,
        /// Returns streams broadcast by one or more specified user IDs. You can specify up to 100 IDs.
        #[builder(default, setter(into))]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<String>,
        /// Returns streams broadcast by one or more specified user login names. You can specify up to 100 names.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_login: Vec<String>,
    }

    /// Return Values for [Get Streams](super::get_streams)
    ///
    /// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Stream {
        /// ID of the game being played on the stream.
        pub game_id: String,
        /// Stream ID.
        pub id: String,
        /// Stream language.
        pub language: String,
        /// UTC timestamp.
        pub started_at: String,
        /// Shows tag IDs that apply to the stream.
        pub tag_ids: Vec<String>,
        /// Thumbnail URL of the stream. All image URLs have variable width and height. You can replace {width} and {height} with any values to get that size image
        pub thumbnail_url: String,
        /// Stream title.
        pub title: String,
        /// Stream type: "live" or "" (in case of error).
        #[serde(rename = "type")]
        pub type_: StreamType,
        /// ID of the user who is streaming.
        pub user_id: String,
        /// Display name corresponding to user_id.
        pub user_name: String,
        /// Number of viewers watching the stream at the time of the query.
        pub viewer_count: usize,
    }

    impl helix::Request for GetStreamsRequest {
        type Response = Stream;

        const PATH: &'static str = "streams";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetStreamsRequest {}

    impl helix::Paginated for GetStreamsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }
}

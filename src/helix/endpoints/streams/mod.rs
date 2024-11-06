#![doc(alias = "live")]
//! Helix endpoints regarding streams
//!
//! # Examples
//!
//! See [`HelixClient::get_streams_from_logins`](crate::helix::HelixClient::get_streams_from_logins) and
//! [`HelixClient::get_streams_from_ids`](crate::helix::HelixClient::get_streams_from_ids)
//!
//! ## Manual request
//!
//! ```rust,no_run
//! # use twitch_api::{helix::{HelixClient, streams::GetStreamsRequest}, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetStreamsRequest::user_logins(&"justinfan1337");
//!
//! // If this doesn't return a result, that would mean the stream is not live.
//! println!("{:?}", &client.req_get(req, &token).await?.data.first());
//! # Ok(())
//! # }
//! ```
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Streams 🟢 5/5</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Stream Key](https://dev.twitch.tv/docs/api/reference#get-stream-key) | [`HelixClient::get_stream_key`](crate::helix::HelixClient::get_stream_key) | [`get_stream_key`] |
//! | [Get Streams](https://dev.twitch.tv/docs/api/reference#get-streams) | [`HelixClient::get_streams_from_ids`](crate::helix::HelixClient::get_streams_from_ids), [`HelixClient::get_streams_from_logins`](crate::helix::HelixClient::get_streams_from_logins) | [`get_streams`] |
//! | [Get Followed Streams](https://dev.twitch.tv/docs/api/reference#get-followed-streams) | [`HelixClient::get_followed_streams`](crate::helix::HelixClient::get_followed_streams) | [`get_followed_streams`] |
//! | [Create Stream Marker](https://dev.twitch.tv/docs/api/reference#create-stream-marker) | [`HelixClient::create_stream_marker`](crate::helix::HelixClient::create_stream_marker) | [`create_stream_marker`] |
//! | [Get Stream Markers](https://dev.twitch.tv/docs/api/reference#get-stream-markers) | - | [`get_stream_markers`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->
use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

#[doc(inline)]
pub use create_stream_marker::{
    CreateStreamMarkerBody, CreateStreamMarkerRequest, CreatedStreamMarker,
};
#[doc(inline)]
pub use get_followed_streams::GetFollowedStreamsRequest;
#[doc(inline)]
pub use get_stream_key::{GetStreamKeyRequest, GetStreamKeyResponse};
#[doc(inline)]
pub use get_stream_markers::{
    GetStreamMarkersRequest, StreamMarker, StreamMarkerGroup, StreamMarkerVideo,
};
#[doc(inline)]
#[allow(deprecated)]
pub use get_stream_tags::{GetStreamTagsRequest, Tag};
#[doc(inline)]
pub use get_streams::{GetStreamsRequest, Stream};
#[doc(inline)]
#[allow(deprecated)]
pub use replace_stream_tags::{ReplaceStreamTags, ReplaceStreamTagsBody, ReplaceStreamTagsRequest};

pub mod create_stream_marker;
pub mod get_followed_streams;
pub mod get_stream_key;
pub mod get_stream_markers;
pub mod get_stream_tags;
pub mod get_streams;
pub mod replace_stream_tags;

/// Gotten from [`Stream.type_`](get_streams::Stream#structfield.type_)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
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
    /// Just do a [`GetStreamsRequest`] and if there is no response for your user_id/user_login, you can be
    /// sure that the channel is not live
    #[serde(other)]
    NotLive,
}

impl StreamType {
    /// Check if the stream is live or not
    pub fn is_live(&self) -> bool { matches!(self, StreamType::Live) }
}

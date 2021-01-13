#![doc(alias = "live")]
//! Helix endpoints regarding streams
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, streams::GetStreamsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
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
use crate::{helix, types};
use serde::{Deserialize, Serialize};

pub use get_stream_tags::{GetStreamTagsRequest, Tag};
pub use get_streams::{GetStreamsRequest, Stream};

pub mod get_stream_tags;
pub mod get_streams;

/// Gotten from [`Stream.type_`](get_streams::Stream#structfield.type_)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
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

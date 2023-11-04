#![doc(alias = "live")]
//! Helix endpoints regarding streams
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::{helix::{HelixClient, streams::GetStreamsRequest}, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let logins: &[&types::UserNameRef] = &["justinfan1337".into()];
//! let req = GetStreamsRequest::builder().user_login(logins).build();
//!
//! // If this doesn't return a result, that would mean the stream is not live.
//! println!("{:?}", &client.req_get(req, &token).await?.data.first());
//! # Ok(())
//! # }
//! ```
use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[doc(inline)]
pub use get_followed_streams::GetFollowedStreamsRequest;
#[doc(inline)]
#[allow(deprecated)]
pub use get_stream_tags::{GetStreamTagsRequest, Tag};
#[doc(inline)]
pub use get_streams::{GetStreamsRequest, Stream};
#[doc(inline)]
#[allow(deprecated)]
pub use replace_stream_tags::{ReplaceStreamTags, ReplaceStreamTagsBody, ReplaceStreamTagsRequest};

pub mod get_followed_streams;
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

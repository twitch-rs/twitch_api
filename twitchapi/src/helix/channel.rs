//! Endpoints regarding channels'
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, channel::GetChannelRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(token, None).await?;
//! let client = HelixClient::new();
//! let req = GetChannelRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//!
//! // Get Channel Request only returns one entry.
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```

#[doc(inline)]
pub use get_channel_information::{GetChannel, GetChannelRequest};

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Gets channel information for users.
/// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
pub mod get_channel_information {
    use super::*;
    /// Query Parameters for [Get Channel Information](super::get_channel_information)
    ///
    /// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetChannelRequest {
        /// ID of the channel
        #[builder(setter(into))]
        pub broadcaster_id: String,
    }

    /// Return Values for [Get Channel Information](super::get_channel_information)
    ///
    /// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct GetChannel {
        /// Channel’s streaming status
        pub status: Option<String>,
        /// Twitch User ID of this channel owner
        pub broadcaster_id: String,
        /// Current game ID being played on the channel
        pub game_id: String,
        /// Language of the channel
        pub broadcaster_language: String,
        /// Title of the stream
        pub title: String,
        /// Description of the stream
        #[serde(default)]
        pub description: String,
    }

    impl helix::Request for GetChannelRequest {
        type Response = GetChannel;

        const PATH: &'static str = "channels";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetChannelRequest {}
}

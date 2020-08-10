pub use get_channel_information::{GetChannel, GetChannelRequest};

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub mod get_channel_information {
    use super::*;
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    pub struct GetChannelRequest {
        /// ID of the channel
        #[builder(default)]
        pub broadcaster_id: String,
    }

    #[derive(PartialEq, Deserialize, Debug, Clone)]
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

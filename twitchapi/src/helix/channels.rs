//! Endpoints regarding channels
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, channels::GetChannelInformationRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetChannelInformationRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//!
//! // Get Channel Request only returns one entry.
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```

#[doc(inline)]
pub use get_channel_information::{GetChannelInformation, GetChannelInformationRequest};
#[doc(inline)]
pub use modify_channel_information::{
    ModifyChannelInformation, ModifyChannelInformationBody, ModifyChannelInformationRequest,
};
#[doc(inline)]
pub use start_commercial::{
    CommercialLength, StartCommercial, StartCommercialBody, StartCommercialRequest,
};

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
    pub struct GetChannelInformationRequest {
        /// ID of the channel
        #[builder(setter(into))]
        pub broadcaster_id: String,
    }

    /// Return Values for [Get Channel Information](super::get_channel_information)
    ///
    /// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct GetChannelInformation {
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

    impl helix::Request for GetChannelInformationRequest {
        type Response = GetChannelInformation;

        const PATH: &'static str = "channels";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetChannelInformationRequest {}
}

/// Gets channel information for users.
/// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
pub mod modify_channel_information {
    use super::*;
    /// Query Parameters for [Modify Channel Information](super::modify_channel_information)
    ///
    /// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct ModifyChannelInformationRequest {
        /// ID of the channel
        #[builder(setter(into))]
        pub broadcaster_id: String,
    }

    // FIXME: Twitch docs sucks...
    /// Body Parameters for [Modify Channel Information](super::modify_channel_information)
    ///
    /// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct ModifyChannelInformationBody {
        /// Current game ID being played on the channel
        #[builder(default, setter(into, strip_option))]
        pub game_id: Option<String>,
        /// Language of the channel
        #[builder(default, setter(into))]
        pub broadcaster_language: Option<String>,
        /// Title of the stream
        #[builder(default, setter(into, strip_option))]
        pub title: Option<String>,
    }
    /// Return Values for [Modify Channel Information](super::modify_channel_information)
    ///
    /// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub enum ModifyChannelInformation {
        /// 204 - Channel/Stream updated successfully
        Success,
    }

    impl std::convert::TryFrom<http::StatusCode> for ModifyChannelInformation {
        type Error = std::borrow::Cow<'static, str>;

        fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
            match s {
                http::StatusCode::NO_CONTENT => Ok(ModifyChannelInformation::Success),
                other => Err(other.canonical_reason().unwrap_or("").into()),
            }
        }
    }

    impl helix::Request for ModifyChannelInformationRequest {
        type Response = ModifyChannelInformation;

        const PATH: &'static str = "channels";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditBroadcast];
    }

    impl helix::RequestPatch for ModifyChannelInformationRequest {
        type Body = ModifyChannelInformationBody;
    }
}

/// Starts a commercial on a specified channel.
/// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
pub mod start_commercial {
    use super::*;
    /// Length of the commercial in seconds
    #[derive(
        displaydoc::Display,
        serde_repr::Serialize_repr,
        serde_repr::Deserialize_repr,
        Debug,
        Clone,
        PartialEq,
        Eq,
    )]
    #[repr(u64)]
    #[non_exhaustive]
    pub enum CommercialLength {
        /// 30s
        Length30 = 30,
        /// 60s
        Length60 = 60,
        /// 90s
        Length90 = 90,
        /// 120s
        Length120 = 120,
        /// 150s
        Length150 = 150,
        /// 180s
        Length180 = 180,
    }

    /// Error for the `TryFrom` on [CommercialLength]
    #[derive(thiserror::Error, Debug, displaydoc::Display)]
    pub enum CommercialLengthParseError {
        /// invalid length of {0}
        InvalidLength(u64),
    }

    impl StartCommercialRequest {
        /// Create a new [StartCommercialRequest]
        pub fn new() -> Self { StartCommercialRequest {} }
    }
    // Not implementing builder since it's not really needed...
    /// Query Parameters for [Start Commercial](super::start_commercial)
    ///
    /// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
    #[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct StartCommercialRequest {}

    impl Default for StartCommercialRequest {
        fn default() -> Self { StartCommercialRequest::new() }
    }

    impl std::convert::TryFrom<u64> for CommercialLength {
        type Error = CommercialLengthParseError;

        fn try_from(l: u64) -> Result<Self, Self::Error> {
            match l {
                30 => Ok(CommercialLength::Length30),
                60 => Ok(CommercialLength::Length60),
                90 => Ok(CommercialLength::Length90),
                120 => Ok(CommercialLength::Length120),
                150 => Ok(CommercialLength::Length150),
                180 => Ok(CommercialLength::Length180),
                other => Err(CommercialLengthParseError::InvalidLength(other)),
            }
        }
    }
    /// Body Parameters for [Start Commercial](super::start_commercial)
    ///
    /// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct StartCommercialBody {
        /// ID of the channel requesting a commercial
        pub broadcaster_id: String,
        /// Desired length of the commercial in seconds. Valid options are 30, 60, 90, 120, 150, 180.
        #[builder(setter(into))]
        pub length: CommercialLength,
    }

    /// Return Values for [Start Commercial](super::start_commercial)
    ///
    /// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct StartCommercial {
        /// Length of the triggered commercial
        pub length: CommercialLength,
        /// Provides contextual information on why the request failed
        pub message: String,
        /// Seconds until the next commercial can be served on this channel
        pub retry_after: u64,
    }

    impl helix::Request for StartCommercialRequest {
        type Response = StartCommercial;

        const PATH: &'static str = "channels/commercial";
        const SCOPE: &'static [twitch_oauth2::Scope] =
            &[twitch_oauth2::Scope::ChannelEditCommercial];
    }

    impl helix::RequestPost for StartCommercialRequest {
        type Body = StartCommercialBody;
    }
}

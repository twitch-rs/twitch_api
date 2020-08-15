//! Endpoints regarding channels'
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, channels::GetChannelRequest};
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
pub use get_channel_information::{GetChannelInformation, GetChannelInformationRequest};
#[doc(inline)]
pub use modify_channel_information::{
    ModifyChannelInformation, ModifyChannelInformationBody, ModifyChannelInformationRequest,
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
        /// 400 - Missing Query Parameter
        MissingQueryParameter,
        /// 500 - Internal Server Error; Failed to update channel
        InternalServerError,
    }

    impl std::convert::TryFrom<http::StatusCode> for ModifyChannelInformation {
        type Error = std::borrow::Cow<'static, str>;

        fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
            match s {
                http::StatusCode::NO_CONTENT => Ok(ModifyChannelInformation::Success),
                http::StatusCode::BAD_REQUEST => {
                    Ok(ModifyChannelInformation::MissingQueryParameter)
                }
                http::StatusCode::INTERNAL_SERVER_ERROR => {
                    Ok(ModifyChannelInformation::InternalServerError)
                }
                other => Err(format!("got status code: {:?}", other).into()),
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

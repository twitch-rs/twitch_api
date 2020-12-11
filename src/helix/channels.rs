//! Endpoints regarding channels
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, channels::GetChannelInformationRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetChannelInformationRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//!
//! // Get Channel Information Request only returns one entry.
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```

#[doc(inline)]
pub use get_channel_information::{ChannelInformation, GetChannelInformationRequest};
#[doc(inline)]
pub use modify_channel_information::{
    ModifyChannelInformation, ModifyChannelInformationBody, ModifyChannelInformationRequest,
};
#[doc(inline)]
pub use start_commercial::{StartCommercial, StartCommercialBody, StartCommercialRequest};

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Gets channel information for users.
/// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
///
/// # Accessing the endpoint
///
/// ## Request: [GetChannelInformationRequest]
///
/// To use this endpoint, construct a [`GetChannelInformationRequest`] with the [`GetChannelInformationRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::channels::get_channel_information;
/// let request = get_channel_information::GetChannelInformationRequest::builder()
///     .broadcaster_id("1234")
///     .build();
/// ```
///
/// ## Response: [ChannelInformation]
///
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, channels::get_channel_information};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
/// let request = get_channel_information::GetChannelInformationRequest::builder()
///     .broadcaster_id("1234")
///     .build();
/// let response: Option<get_channel_information::ChannelInformation> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_channel_information {
    use std::convert::TryInto;

    use super::*;
    /// Query Parameters for [Get Channel Information](super::get_channel_information)
    ///
    /// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetChannelInformationRequest {
        /// ID of the channel
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
    }

    /// Return Values for [Get Channel Information](super::get_channel_information)
    ///
    /// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct ChannelInformation {
        /// Twitch User ID of this channel owner
        pub broadcaster_id: types::UserId,
        /// User name of this channel owner
        pub broadcaster_name: types::UserName,
        /// Current game ID being played on the channel
        pub game_id: types::CategoryId,
        /// Name of current game being played on the channel
        pub game_name: types::CategoryId,
        /// Language of the channel
        pub broadcaster_language: String,
        /// Title of the stream
        pub title: String,
        /// Description of the stream
        #[serde(default)]
        pub description: String,
    }

    impl helix::Request for GetChannelInformationRequest {
        type Response = Option<ChannelInformation>;

        const PATH: &'static str = "channels";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetChannelInformationRequest {
        fn parse_response(
            self,
            uri: &http::Uri,
            response: http::Response<Vec<u8>>,
        ) -> Result<helix::Response<Self, Option<ChannelInformation>>, helix::HelixRequestGetError>
        where
            Self: Sized,
        {
            let text = std::str::from_utf8(&response.body())
                .map_err(|e| helix::HelixRequestGetError::Utf8Error(response.body().clone(), e))?;
            //eprintln!("\n\nmessage is ------------ {} ------------", text);
            if let Ok(helix::HelixRequestError {
                error,
                status,
                message,
            }) = serde_json::from_str::<helix::HelixRequestError>(&text)
            {
                return Err(helix::HelixRequestGetError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                });
            }
            let response: helix::InnerResponse<Vec<_>> = serde_json::from_str(&text)?;
            Ok(helix::Response {
                data: response.data.into_iter().next(),
                pagination: response.pagination.cursor,
                request: self,
            })
        }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetChannelInformationRequest::builder()
            .broadcaster_id("44445592".to_string())
            .build();

        // From twitch docs
        let data = br#"
        {
          "data": [
            {
              "broadcaster_id": "44445592",
              "broadcaster_name": "pokimane",
              "broadcaster_language": "en",
              "game_id": "21779",
              "game_name": "League of Legends",
              "title": "title"
            }
          ]
        }
        "#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/channels?broadcaster_id=44445592"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Modify channel information for users.
/// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
///
/// # Accessing the endpoint
///
/// ## Request: [ModifyChannelInformationRequest]
///
/// To use this endpoint, construct a [`ModifyChannelInformationRequest`] with the [`ModifyChannelInformationRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::channels::modify_channel_information;
/// let request = modify_channel_information::ModifyChannelInformationRequest::builder()
///     .broadcaster_id("1234")
///     .build();
/// ```
///
/// ## Body: [ModifyChannelInformationBody]
///
/// We also need to provide a body to the request containing what we want to change.
///
/// ```
/// # use twitch_api2::helix::channels::modify_channel_information;
/// let body = modify_channel_information::ModifyChannelInformationBody::builder()
///     .title("Hello World!".to_string())
///     .build();
/// ```
///
/// ## Response: [ModifyChannelInformation]
///
///
/// Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
///
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, channels::modify_channel_information};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
/// let request = modify_channel_information::ModifyChannelInformationRequest::builder()
///     .broadcaster_id("1234")
///     .build();
/// let body = modify_channel_information::ModifyChannelInformationBody::builder()
///     .title("Hello World!".to_string())
///     .build();
/// let response: modify_channel_information::ModifyChannelInformation = client.req_patch(request, body, &token).await?;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPatch::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestPatch::parse_response())
pub mod modify_channel_information {
    use super::*;
    /// Query Parameters for [Modify Channel Information](super::modify_channel_information)
    ///
    /// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct ModifyChannelInformationRequest {
        /// ID of the channel
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
    }

    // FIXME: Twitch docs sucks...
    /// Body Parameters for [Modify Channel Information](super::modify_channel_information)
    ///
    /// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct ModifyChannelInformationBody {
        /// Current game ID being played on the channel
        #[builder(default, setter(into, strip_option))]
        pub game_id: Option<types::CategoryId>,
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
        MissingQuery,
        /// Internal Server Error; Failed to update channel
        InternalServerError,
    }

    impl std::convert::TryFrom<http::StatusCode> for ModifyChannelInformation {
        type Error = std::borrow::Cow<'static, str>;

        fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
            match s {
                http::StatusCode::NO_CONTENT => Ok(ModifyChannelInformation::Success),
                // FIXME: Twitch docs says 204 is success...
                http::StatusCode::OK => Ok(ModifyChannelInformation::Success),
                other => Err(other.canonical_reason().unwrap_or("").into()),
            }
        }
    }

    impl helix::Request for ModifyChannelInformationRequest {
        type Response = ModifyChannelInformation;

        const PATH: &'static str = "channels";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditBroadcast];
    }

    impl helix::RequestPatch for ModifyChannelInformationRequest {
        type Body = ModifyChannelInformationBody;
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = ModifyChannelInformationRequest::builder()
            .broadcaster_id(String::from("0"))
            .build();

        // From twitch docs
        let data = br#""#.to_vec();

        let http_response = http::Response::builder().status(200).body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/channels?broadcaster_id=0"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Starts a commercial on a specified channel.
/// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
///
/// # Accessing the endpoint
///
/// ## Request: [StartCommercialRequest]
///
/// To use this endpoint, construct a [`StartCommercialRequest`] with the [`StartCommercialRequest::new()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::channels::start_commercial;
/// let request = start_commercial::StartCommercialRequest::new();
/// ```
///
/// ## Body: [StartCommercialBody]
///
/// We also need to provide a body to the request specifying length of commercial and where to start it.
///
/// ```
/// # use twitch_api2::helix::channels::start_commercial;
/// let body = start_commercial::StartCommercialBody::builder()
///     .broadcaster_id("1234".to_string())
///     .length(twitch_api2::types::CommercialLength::Length90)
///     .build();
/// ```
///
/// ## Response: [StartCommercialRequest]
///
/// Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, channels::start_commercial};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
/// let request = start_commercial::StartCommercialRequest::new();
/// let body = start_commercial::StartCommercialBody::builder()
///     .broadcaster_id("1234".to_string())
///     .length(twitch_api2::types::CommercialLength::Length90)
///     .build();
/// let response: Vec<start_commercial::StartCommercial> = client.req_post(request, body, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPost::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestPost::parse_response())
pub mod start_commercial {
    use super::*;

    impl StartCommercialRequest {
        /// Create a new [`StartCommercialRequest`]
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

    /// Body Parameters for [Start Commercial](super::start_commercial)
    ///
    /// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct StartCommercialBody {
        /// ID of the channel requesting a commercial
        pub broadcaster_id: types::UserId,
        /// Desired length of the commercial in seconds. Valid options are 30, 60, 90, 120, 150, 180.
        #[builder(setter(into))]
        pub length: types::CommercialLength,
    }

    /// Return Values for [Start Commercial](super::start_commercial)
    ///
    /// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct StartCommercial {
        /// Length of the triggered commercial
        pub length: types::CommercialLength,
        /// Provides contextual information on why the request failed
        pub message: String,
        /// Seconds until the next commercial can be served on this channel
        pub retry_after: u64,
    }

    impl helix::Request for StartCommercialRequest {
        /// FIXME: Make non-vec
        type Response = Vec<StartCommercial>;

        const PATH: &'static str = "channels/commercial";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] =
            &[twitch_oauth2::Scope::ChannelEditCommercial];
    }

    impl helix::RequestPost for StartCommercialRequest {
        type Body = StartCommercialBody;
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = StartCommercialRequest {};

        // From twitch docs
        let data = br#"
{
    "data": [{
      "length" : 60,
      "message" : "",
      "retry_after" : 480
    }]
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/channels/commercial?"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

//! Modify channel information for users.
//! [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
//!
//! # Accessing the endpoint
//!
//! ## Request: [ModifyChannelInformationRequest]
//!
//! To use this endpoint, construct a [`ModifyChannelInformationRequest`] with the [`ModifyChannelInformationRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::channels::modify_channel_information;
//! let request = modify_channel_information::ModifyChannelInformationRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Body: [ModifyChannelInformationBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::channels::modify_channel_information;
//! let body = modify_channel_information::ModifyChannelInformationBody::builder()
//!     .title("Hello World!".to_string())
//!     .build();
//! ```
//!
//! ## Response: [ModifyChannelInformation]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, channels::modify_channel_information};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = modify_channel_information::ModifyChannelInformationRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let body = modify_channel_information::ModifyChannelInformationBody::builder()
//!     .title("Hello World!".to_string())
//!     .build();
//! let response: modify_channel_information::ModifyChannelInformation = client.req_patch(request, body, &token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPatch::create_request)
//! and parse the [`http::Response`] with [`ModifyChannelInformationRequest::parse_response(&request.get_uri(), response)`](ModifyChannelInformationRequest::parse_response)
use super::*;
use helix::RequestPatch;

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
    /// Current game ID being played on the channel. Use “0” or “” (an empty string) to unset the game.
    #[builder(default, setter(into, strip_option))]
    pub game_id: Option<types::CategoryId>,
    /// Language of the channel
    #[builder(default, setter(into))]
    pub broadcaster_language: Option<String>,
    /// Title of the stream. Value must not be an empty string.
    #[builder(default, setter(into, strip_option))]
    pub title: Option<String>,
}

impl helix::private::SealedSerialize for ModifyChannelInformationBody {}
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

impl Request for ModifyChannelInformationRequest {
    type Response = ModifyChannelInformation;

    const PATH: &'static str = "channels";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditBroadcast];
}

impl RequestPatch for ModifyChannelInformationRequest {
    type Body = ModifyChannelInformationBody;
}

#[test]
fn test_request() {
    use helix::*;
    let req = ModifyChannelInformationRequest::builder()
        .broadcaster_id(String::from("0"))
        .build();

    let body = ModifyChannelInformationBody::builder()
        .title("Hello World!")
        .build();

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels?broadcaster_id=0"
    );

    dbg!(ModifyChannelInformationRequest::parse_response(&uri, http_response).unwrap());
}

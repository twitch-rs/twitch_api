//! Modify channel information for users.
//! [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
//!
//! # Accessing the endpoint
//!
//! ## Request: [ModifyChannelInformationRequest]
//!
//! To use this endpoint, construct a [`ModifyChannelInformationRequest`] with the [`ModifyChannelInformationRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::modify_channel_information;
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
//! # use twitch_api::helix::channels::modify_channel_information;
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
//! use twitch_api::helix::{self, channels::modify_channel_information};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = modify_channel_information::ModifyChannelInformationRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let body = modify_channel_information::ModifyChannelInformationBody::builder()
//!     .title("Hello World!".to_string())
//!     .build();
//! let response: modify_channel_information::ModifyChannelInformation = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPatch::create_request)
//! and parse the [`http::Response`] with [`ModifyChannelInformationRequest::parse_response(None, &request.get_uri(), response)`](ModifyChannelInformationRequest::parse_response)
use super::*;
use helix::RequestPatch;

/// Query Parameters for [Modify Channel Information](super::modify_channel_information)
///
/// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ModifyChannelInformationRequest {
    /// ID of the channel
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_id: types::UserId,
}

// FIXME: Twitch docs sucks...
/// Body Parameters for [Modify Channel Information](super::modify_channel_information)
///
/// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
#[derive(
    PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default,
)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ModifyChannelInformationBody {
    /// Current game ID being played on the channel. Use “0” or “” (an empty string) to unset the game.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub game_id: Option<types::CategoryId>,
    /// Language of the channel
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub broadcaster_language: Option<String>,
    /// Title of the stream. Value must not be an empty string.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub title: Option<String>,
}

impl helix::private::SealedSerialize for ModifyChannelInformationBody {}
/// Return Values for [Modify Channel Information](super::modify_channel_information)
///
/// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum ModifyChannelInformation {
    /// 204 - Channel/Stream updated successfully
    Success,
}

impl Request for ModifyChannelInformationRequest {
    type Response = ModifyChannelInformation;

    const PATH: &'static str = "channels";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditBroadcast];
}

impl RequestPatch for ModifyChannelInformationRequest {
    type Body = ModifyChannelInformationBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPatchError>
    where
        Self: Sized,
    {
        Ok(helix::Response {
            data: match status {
                http::StatusCode::NO_CONTENT | http::StatusCode::OK => {
                    ModifyChannelInformation::Success
                }
                _ => {
                    return Err(helix::HelixRequestPatchError::InvalidResponse {
                        reason: "unexpected status code",
                        response: response.to_string(),
                        status,
                        uri: uri.clone(),
                    })
                }
            },
            pagination: None,
            request,
            total: None,
            other: None,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = ModifyChannelInformationRequest::builder()
        .broadcaster_id(String::from("0"))
        .build();

    let body = ModifyChannelInformationBody::builder()
        .title("Hello World!".to_string())
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

    dbg!(ModifyChannelInformationRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

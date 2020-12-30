//! Gets channel information for users.
//! [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetChannelInformationRequest]
//!
//! To use this endpoint, construct a [`GetChannelInformationRequest`] with the [`GetChannelInformationRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::channels::get_channel_information;
//! let request = get_channel_information::GetChannelInformationRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [ChannelInformation]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, channels::get_channel_information};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_channel_information::GetChannelInformationRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Option<get_channel_information::ChannelInformation> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
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
    /// Twitch user display name of this channel owner
    pub broadcaster_name: types::DisplayName,
    /// Current game ID being played on the channel
    pub game_id: types::CategoryId,
    /// Name of the game being played on the channel
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
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            helix::HelixRequestGetError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
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
        let response: helix::InnerResponse<Vec<_>> = serde_json::from_str(&text).map_err(|e| {
            helix::HelixRequestGetError::DeserializeError(text.to_string(), e, uri.clone())
        })?;
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

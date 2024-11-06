//! Gets the channel’s stream key.
//! [`get-stream-key`](https://dev.twitch.tv/docs/api/reference#get-stream-key)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetStreamKeyRequest]
//!
//! To use this endpoint, construct a [`GetStreamKeyRequest`] with the [`GetStreamKeyRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::streams::get_stream_key;
//! let request = get_stream_key::GetStreamKeyRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [GetStreamKeyResponse](helix::streams::GetStreamKeyResponse)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, streams::get_stream_key};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_stream_key::GetStreamKeyRequest::broadcaster_id("1234");
//! let response: get_stream_key::GetStreamKeyResponse = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetStreamKeyRequest::parse_response(None, &request.get_uri(), response)`](GetStreamKeyRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Stream Key](super::get_stream_key)
///
/// [`get-stream-key`](https://dev.twitch.tv/docs/api/reference#get-stream-key)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetStreamKeyRequest<'a> {
    /// The ID of the broadcaster that owns the channel. The ID must match the user ID in the access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetStreamKeyRequest<'a> {
    /// ID of the broadcaster whose stream key should be retrieved
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Stream Key](super::get_stream_key)
///
/// [`get-stream-key`](https://dev.twitch.tv/docs/api/reference#get-stream-key)
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GetStreamKeyResponse {
    /// The channel’s stream key.
    pub stream_key: types::StreamKey,
}

impl Request for GetStreamKeyRequest<'_> {
    type Response = GetStreamKeyResponse;

    const PATH: &'static str = "streams/key";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelReadStreamKey];
}

impl RequestGet for GetStreamKeyRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        helix::parse_single_return(request, uri, response, status)
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetStreamKeyRequest::broadcaster_id("198704263");

    // From twitch docs
    let data = br#"
        {
          "data": [
            {
              "stream_key": "live_44322889_a34ub37c8ajv98a0"
            }
          ]
        }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/streams/key?broadcaster_id=198704263"
    );

    let res = GetStreamKeyRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;

    assert_eq!(res.stream_key.as_str(), "live_44322889_a34ub37c8ajv98a0");
}

//! Sends a whisper message to the specified user.
//! [`send-whisper`](https://dev.twitch.tv/docs/api/reference#send-whisper)
//!
//! # Accessing the endpoint
//!
//! ## Request: [SendWhisperRequest]
//!
//! To use this endpoint, construct a [`SendWhisperRequest`] with the [`SendWhisperRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::whispers::send_whisper;
//! let request = send_whisper::SendWhisperRequest::builder()
//!     .to_user_id("456")
//!     .from_user_id("123")
//!     .build();
//! ```
//!
//! ## Body: [SendWhisperBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::whispers::send_whisper;
//! let body = send_whisper::SendWhisperBody::new("Hello!");
//! ```
//!
//! ## Response: [SendWhisperResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, whispers::send_whisper};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = send_whisper::SendWhisperRequest::builder()
//!     .to_user_id("456")
//!     .from_user_id("123")
//!     .build();
//! let body = send_whisper::SendWhisperBody::new("Hello!");
//! let response: send_whisper::SendWhisperResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`SendWhisperRequest::parse_response(None, &request.get_uri(), response)`](SendWhisperRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Send Whisper](super::send_whisper)
///
/// [`send-whisper`](https://dev.twitch.tv/docs/api/reference#send-whisper)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct SendWhisperRequest {
    /// The ID of the user sending the whisper. This user must have a verified phone number.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub from_user_id: types::UserId,
    /// The ID of the user to receive the whisper.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub to_user_id: types::UserId,
}

impl SendWhisperRequest {
    /// Create a new [`SendWhisperRequest`]
    pub fn new(from: impl Into<types::UserId>, to: impl Into<types::UserId>) -> Self {
        Self {
            from_user_id: from.into(),
            to_user_id: to.into(),
        }
    }
}

/// Body Parameters for [Send Whisper](super::send_whisper)
///
/// [`send-whisper`](https://dev.twitch.tv/docs/api/reference#send-whisper)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SendWhisperBody {
    /// The whisper message to send. The message must not be empty.
    ///
    /// The maximum message lengths are:
    ///
    /// 500 characters if the user you're sending the message to hasn't whispered you before.
    /// 10,000 characters if the user you're sending the message to has whispered you before.
    ///
    /// Messages that exceed the maximum length are truncated.
    pub message: String,
}

impl From<String> for SendWhisperBody {
    fn from(string: String) -> Self { Self::new(string) }
}

impl SendWhisperBody {
    /// Create a new message
    pub fn new(message: impl std::fmt::Display) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl helix::private::SealedSerialize for SendWhisperBody {}

/// Return Values for [Send Whisper](super::send_whisper)
///
/// [`send-whisper`](https://dev.twitch.tv/docs/api/reference#send-whisper)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum SendWhisperResponse {
    /// Successfully sent the whisper message or the message was silently dropped.
    Success,
}

impl Request for SendWhisperRequest {
    type Response = SendWhisperResponse;

    const PATH: &'static str = "whispers";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserManageWhispers];
}

impl RequestPost for SendWhisperRequest {
    type Body = SendWhisperBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response_str: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response {
                data: SendWhisperResponse::Success,
                pagination: None,
                request,
                total: None,
                other: None,
            }),
            _ => Err(helix::HelixRequestPostError::InvalidResponse {
                reason: "unexpected status",
                response: response_str.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = SendWhisperRequest::builder()
        .to_user_id("456")
        .from_user_id("123")
        .build();

    let body = SendWhisperBody::new("hello");

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = vec![];

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/whispers?from_user_id=123&to_user_id=456"
    );

    dbg!(SendWhisperRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

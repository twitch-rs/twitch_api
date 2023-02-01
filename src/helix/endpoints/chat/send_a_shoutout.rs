//! Sends a Shoutout to the specified broadcaster.
//! [`send-a-shoutout`](https://dev.twitch.tv/docs/api/reference#send-a-shoutout)
//!
//! # Accessing the endpoint
//!
//! ## Request: [SendAShoutoutRequest]
//!
//! To use this endpoint, construct a [`SendAShoutoutRequest`] with the [`SendAShoutoutRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::send_a_shoutout;
//! let request =
//!     send_a_shoutout::SendAShoutoutRequest::new("1234", "5678", "9123");
//! ```
//! ## Response: [SendAShoutoutResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::send_a_shoutout};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = send_a_shoutout::SendAShoutoutRequest::new("1234", "5678", "9123");
//! let response: send_a_shoutout::SendAShoutoutResponse = client.req_post(request, Default::default(), &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`SendAShoutoutRequest::parse_response(None, &request.get_uri(), response)`](SendAShoutoutRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Send A Shoutout](super::send_a_shoutout)
///
/// [`send-a-shoutout`](https://dev.twitch.tv/docs/api/reference#send-a-shoutout)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct SendAShoutoutRequest<'a> {
    /// The ID of the broadcaster that’s sending the Shoutout.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub from_broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster that’s receiving the Shoutout.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub to_broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that is one of the broadcaster’s moderators. This ID must match the user ID in the access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> SendAShoutoutRequest<'a> {
    /// Create a new [`SendAShoutoutRequest`]
    pub fn new(
        from_broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        to_broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            from_broadcaster_id: from_broadcaster_id.into_cow(),
            to_broadcaster_id: to_broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
        }
    }
}

/// Return Values for [Send A Shoutout](super::send_a_shoutout)
///
/// [`send-a-shoutout`](https://dev.twitch.tv/docs/api/reference#send-a-shoutout)
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum SendAShoutoutResponse {
    /// Shoutout successful
    Success,
}
impl Request for SendAShoutoutRequest<'_> {
    type Response = SendAShoutoutResponse;

    const PATH: &'static str = "chat/shoutouts";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorManageShoutouts];
}

impl RequestPost for SendAShoutoutRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response {
                data: SendAShoutoutResponse::Success,
                pagination: None,
                request,
                total: None,
                other: None,
            }),
            _ => Err(helix::HelixRequestPostError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
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
    let req = SendAShoutoutRequest::new("12345", "626262", "98765");

    dbg!(req
        .create_request(Default::default(), "token", "clientid")
        .unwrap());

    // From twitch docs
    let data = b"".to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/shoutouts?from_broadcaster_id=12345&to_broadcaster_id=626262&moderator_id=98765"
    );

    dbg!(SendAShoutoutRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

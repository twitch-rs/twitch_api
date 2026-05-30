//! Unpins a pinned chat message from the broadcaster’s chat room.
//! [`unpin-chat-message`](https://dev.twitch.tv/docs/api/reference#unpin-chat-message)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UnpinChatMessageRequest]
//!
//! To use this endpoint, construct a [`UnpinChatMessageRequest`] with the [`UnpinChatMessageRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::unpin_chat_message;
//! let request = unpin_chat_message::UnpinChatMessageRequest::new(
//!     "1234",
//!     "5678",
//!     "abc-def-123",
//! );
//! ```
//!
//! ## Response: [UnpinChatMessageResponse]
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::unpin_chat_message};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = unpin_chat_message::UnpinChatMessageRequest::new("1234", "5678", "abc-def-123");
//! let response: helix::chat::UnpinChatMessageResponse = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`UnpinChatMessageRequest::parse_response(None, &request.get_uri(), response)`](UnpinChatMessageRequest::parse_response)

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Unpin Chat Message](super::unpin_chat_message)
///
/// [`unpin-chat-message`](https://dev.twitch.tv/docs/api/reference#unpin-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UnpinChatMessageRequest<'a> {
    /// The ID of the broadcaster that owns the chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The ID of the message to unpin.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub message_id: Cow<'a, types::MsgIdRef>,
}

impl<'a> UnpinChatMessageRequest<'a> {
    /// Unpin a pinned chat message from the broadcaster’s chat room.
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        message_id: impl types::IntoCow<'a, types::MsgIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            message_id: message_id.into_cow(),
        }
    }
}

/// Return Values for [Unpin Chat Message](super::unpin_chat_message)
///
/// [`unpin-chat-message`](https://dev.twitch.tv/docs/api/reference#unpin-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum UnpinChatMessageResponse {
    /// Successfully unpinned the message.
    Success,
}

impl Request for UnpinChatMessageRequest<'_> {
    type PaginationData = ();
    type Response = UnpinChatMessageResponse;

    const PATH: &'static str = "chat/pins";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageChatMessages];
}

impl<'a> RequestDelete for UnpinChatMessageRequest<'a> {
    fn parse_inner_response<'d>(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response::with_data(
                UnpinChatMessageResponse::Success,
                request,
            )),
            _ => Err(helix::HelixRequestDeleteError::InvalidResponse {
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
    let req = UnpinChatMessageRequest::new("197886470", "141981764", "789-xyz");

    dbg!(req.create_request("token", "clientid").unwrap());

    let data = b"".to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/pins?broadcaster_id=197886470&moderator_id=141981764&message_id=789-xyz"
    );

    dbg!(UnpinChatMessageRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

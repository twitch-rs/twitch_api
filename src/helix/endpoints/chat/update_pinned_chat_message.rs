//! Updates the duration of a pinned chat message.
//! [`update-pinned-chat-message`](https://dev.twitch.tv/docs/api/reference#update-pinned-chat-message)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdatePinnedChatMessageRequest]
//!
//! To use this endpoint, construct a [`UpdatePinnedChatMessageRequest`] with the [`UpdatePinnedChatMessageRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::update_pinned_chat_message;
//! let request =
//!     update_pinned_chat_message::UpdatePinnedChatMessageRequest::new(
//!         "1234",
//!         "5678",
//!         "abc-def-123",
//!     );
//! ```
//!
//! ## Response: [UpdatePinnedChatMessageResponse]
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::update_pinned_chat_message};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_pinned_chat_message::UpdatePinnedChatMessageRequest::new("1234", "5678", "abc-def-123");
//! let response: helix::chat::UpdatePinnedChatMessageResponse = client.req_patch(request, helix::EmptyBody, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPatch::create_request)
//! and parse the [`http::Response`] with [`UpdatePinnedChatMessageRequest::parse_response(None, &request.get_uri(), response)`](UpdatePinnedChatMessageRequest::parse_response)

use super::*;
use helix::RequestPatch;

/// Query Parameters for [Update Pinned Chat Message](super::update_pinned_chat_message)
///
/// [`update-pinned-chat-message`](https://dev.twitch.tv/docs/api/reference#update-pinned-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UpdatePinnedChatMessageRequest<'a> {
    /// The ID of the broadcaster that owns the chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The ID of the pinned message to update.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub message_id: Cow<'a, types::MsgIdRef>,
    /// The new number of seconds the message should remain pinned, starting from now.
    ///
    /// Minimum: 30. Maximum: 1800. If [None] is specified, the message will be pinned until the stream ends.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
}

impl<'a> UpdatePinnedChatMessageRequest<'a> {
    /// Update the duration of a pinned chat message.
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        message_id: impl types::IntoCow<'a, types::MsgIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            message_id: message_id.into_cow(),
            duration_seconds: None,
        }
    }

    /// Set the duration in seconds. If [None] is passed, the message will be unpinned at the end of the stream.
    pub fn duration_seconds(mut self, duration_seconds: impl Into<Option<u32>>) -> Self {
        self.duration_seconds = duration_seconds.into();
        self
    }
}

/// Return Values for [Update Pinned Chat Message](super::update_pinned_chat_message)
///
/// [`update-pinned-chat-message`](https://dev.twitch.tv/docs/api/reference#update-pinned-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum UpdatePinnedChatMessageResponse {
    /// Successfully updated the message.
    Success,
}

impl Request for UpdatePinnedChatMessageRequest<'_> {
    type PaginationData = ();
    type Response = UpdatePinnedChatMessageResponse;

    const PATH: &'static str = "chat/pins";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageChatMessages];
}

impl<'a> RequestPatch for UpdatePinnedChatMessageRequest<'a> {
    type Body = helix::EmptyBody;

    fn parse_inner_response<'d>(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPatchError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response::with_data(
                UpdatePinnedChatMessageResponse::Success,
                request,
            )),
            _ => Err(helix::HelixRequestPatchError::InvalidResponse {
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
    let req = UpdatePinnedChatMessageRequest::new("197886470", "141981764", "789-xyz")
        .duration_seconds(600);

    dbg!(req
        .create_request(Default::default(), "token", "clientid")
        .unwrap());

    let data = b"".to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/pins?broadcaster_id=197886470&moderator_id=141981764&message_id=789-xyz&duration_seconds=600"
    );

    assert_eq!(
        req.clone().duration_seconds(None).get_uri().unwrap().to_string(),
        "https://api.twitch.tv/helix/chat/pins?broadcaster_id=197886470&moderator_id=141981764&message_id=789-xyz"
    );

    dbg!(UpdatePinnedChatMessageRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

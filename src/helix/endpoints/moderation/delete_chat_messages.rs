//! Removes a single chat message or all chat messages from the broadcaster’s chat room.
//! [`delete-chat-messages`](https://dev.twitch.tv/docs/api/reference#delete-chat-messages)
//!
//! # Accessing the endpoint
//!
//! ## Request: [DeleteChatMessagesRequest]
//!
//! To use this endpoint, construct a [`DeleteChatMessagesRequest`] with the [`DeleteChatMessagesRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::delete_chat_messages;
//! let request =
//!     delete_chat_messages::DeleteChatMessagesRequest::new("1234", "5678")
//!         .message_id("abc-123-def");
//! ```
//!
//! ## Response: [DeleteChatMessagesResponse]
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::delete_chat_messages};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = delete_chat_messages::DeleteChatMessagesRequest::new("1234", "5678")
//!     .message_id("abc-123-def");
//! let response: delete_chat_messages::DeleteChatMessagesResponse = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`DeleteChatMessagesRequest::parse_response(None, &request.get_uri(), response)`](DeleteChatMessagesRequest::parse_response)

use super::*;
use helix::RequestDelete;
/// Query Parameters for [Delete Chat Messages](super::delete_chat_messages)
///
/// [`delete-chat-messages`](https://dev.twitch.tv/docs/api/reference#delete-chat-messages)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct DeleteChatMessagesRequest<'a> {
    /// The ID of the broadcaster that owns the chat room to remove messages from.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room.
    ///
    /// This ID must match the user ID in the OAuth token. If the broadcaster wants to remove messages themselves, set this parameter to the broadcaster’s ID, too.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The ID of the message to remove.
    ///
    /// The id tag in the PRIVMSG contains the message’s ID (see [PRIVMSG Tags](https://dev.twitch.tv/docs/irc/tags#privmsg-tags)).
    ///
    /// # Restrictions
    ///
    /// The message must have been created within the last 6 hours.
    /// The message must not belong to the broadcaster.
    /// The message must not belong to another moderator.
    ///
    /// If not specified, the request removes all messages in the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub message_id: Option<Cow<'a, types::MsgIdRef>>,
}

impl<'a> DeleteChatMessagesRequest<'a> {
    /// Remove chat message(s)
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            message_id: None,
        }
    }

    /// A specific message to remove
    pub fn message_id(mut self, message_id: impl types::IntoCow<'a, types::MsgIdRef> + 'a) -> Self {
        self.message_id = Some(message_id.into_cow());
        self
    }
}

/// Return Values for [Delete Chat Messages](super::delete_chat_messages)
///
/// [`delete-chat-messages`](https://dev.twitch.tv/docs/api/reference#delete-chat-messages)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum DeleteChatMessagesResponse {
    /// Successfully removed the specified messages.
    Success,
}

impl Request for DeleteChatMessagesRequest<'_> {
    type Response = DeleteChatMessagesResponse;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "moderation/chat";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorManageChatMessages];
}

impl RequestDelete for DeleteChatMessagesRequest<'_> {
    fn parse_inner_response(
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
                DeleteChatMessagesResponse::Success,
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
fn test_request_all() {
    use helix::*;
    let req = DeleteChatMessagesRequest::new("11111", "44444");

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/chat?broadcaster_id=11111&moderator_id=44444"
    );

    dbg!(DeleteChatMessagesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_specific() {
    use helix::*;
    let req = DeleteChatMessagesRequest::new("11111", "44444").message_id("abc-123-def");

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/chat?broadcaster_id=11111&moderator_id=44444&message_id=abc-123-def"
    );

    dbg!(DeleteChatMessagesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

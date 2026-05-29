//! Gets the currently pinned message for the broadcaster’s chat room.
//! [`get-pinned-chat-message`](https://dev.twitch.tv/docs/api/reference#get-pinned-chat-message)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetPinnedChatMessageRequest]
//!
//! To use this endpoint, construct a [`GetPinnedChatMessageRequest`] with the [`GetPinnedChatMessageRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::get_pinned_chat_message;
//! let request = get_pinned_chat_message::GetPinnedChatMessageRequest::new(
//!     "1234", "5678",
//! );
//! ```
//!
//! ## Response: [PinnedChatMessage]
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::get_pinned_chat_message};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_pinned_chat_message::GetPinnedChatMessageRequest::new("1234", "5678");
//! let response: Option<helix::chat::PinnedChatMessage> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`GetPinnedChatMessageRequest::parse_response(None, &request.get_uri(), response)`](GetPinnedChatMessageRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Pinned Chat Message](super::get_pinned_chat_message)
///
/// [`get-pinned-chat-message`](https://dev.twitch.tv/docs/api/reference#get-pinned-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetPinnedChatMessageRequest<'a> {
    /// The ID of the broadcaster that owns the chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetPinnedChatMessageRequest<'a> {
    /// Get the currently pinned message for the specified broadcaster’s chat room.
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
        }
    }
}

/// Return Values for [Get Pinned Chat Message](super::get_pinned_chat_message)
///
/// [`get-pinned-chat-message`](https://dev.twitch.tv/docs/api/reference#get-pinned-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PinnedChatMessage {
    /// The ID of the pinned chat message.
    pub message_id: types::MsgId,
    /// The ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// The ID of the user who sent the pinned message.
    pub sender_user_id: types::UserId,
    /// The login of the user who sent the pinned message.
    pub sender_user_login: types::UserName,
    /// The display name of the user who sent the pinned message.
    pub sender_user_name: types::DisplayName,
    /// The ID of the user who pinned the message.
    pub pinned_by_user_id: types::UserId,
    /// The login of the user who pinned the message.
    pub pinned_by_user_login: types::UserName,
    /// The display name of the user who pinned the message.
    pub pinned_by_user_name: types::DisplayName,
    /// The pinned message content.
    pub message: crate::common::chat::Message,
    /// RFC3339 timestamp of when the message was pinned.
    pub starts_at: types::Timestamp,
    /// RFC3339 expiry timestamp. [None] if pinned until stream ends.
    pub ends_at: Option<types::Timestamp>,
    /// RFC3339 timestamp of last update.
    pub updated_at: types::Timestamp,
}

impl Request for GetPinnedChatMessageRequest<'_> {
    type PaginationData = ();
    type Response = Option<PinnedChatMessage>;

    const PATH: &'static str = "chat/pins";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![
        twitch_oauth2::Scope::ModeratorManageChatMessages,
        twitch_oauth2::Scope::ModeratorReadChatMessages
    ];
}

impl<'a> RequestGet for GetPinnedChatMessageRequest<'a> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        let resp = match status {
            http::StatusCode::OK => {
                let resp: helix::InnerResponse<helix::request::ZeroOrOne<PinnedChatMessage>> =
                    helix::parse_json(response, true).map_err(|e| {
                        helix::HelixRequestGetError::DeserializeError(
                            response.to_string(),
                            e,
                            uri.clone(),
                            status,
                        )
                    })?;
                resp.data.0
            }
            _ => {
                return Err(helix::HelixRequestGetError::InvalidResponse {
                    reason: "unexpected status code",
                    response: response.to_string(),
                    status,
                    uri: uri.clone(),
                })
            }
        };
        Ok(helix::Response::with_data(resp, request))
    }
}

#[cfg(test)]
#[test]
fn test() {
    use helix::*;
    let req = GetPinnedChatMessageRequest::new("197886470", "141981764");

    dbg!(req.create_request("token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "message_id": "abc-def-123-456",
                "broadcaster_id": "197886470",
                "sender_user_id": "12826",
                "sender_user_login": "twitch",
                "sender_user_name": "Twitch",
                "pinned_by_user_id": "141981764",
                "pinned_by_user_login": "twitchdev",
                "pinned_by_user_name": "TwitchDev",
                "message": {
                    "text": "Welcome! bleedPurple Type !rules",
                    "fragments": [
                        {
                            "type": "text",
                            "text": "Welcome! ",
                            "cheermote": null,
                            "emote": null,
                            "mention": null
                        },
                        {
                            "type": "emote",
                            "text": "bleedPurple",
                            "cheermote": null,
                            "emote": {
                            "id": "62835",
                            "emote_set_id": "0",
                            "owner_id": "0",
                            "format": ["static"]
                            },
                            "mention": null
                        },
                        {
                            "type": "text",
                            "text": " Type !rules",
                            "cheermote": null,
                            "emote": null,
                            "mention": null
                        }
                    ]
                },
                "starts_at": "2026-05-06T12:30:00Z",
                "ends_at": "2026-05-06T12:35:00Z",
                "updated_at": "2026-05-06T12:30:00Z"
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/pins?broadcaster_id=197886470&moderator_id=141981764"
    );

    let res =
        dbg!(GetPinnedChatMessageRequest::parse_response(Some(req), &uri, http_response).unwrap())
            .data;

    let res = res.expect("Must hold Some()");
    assert_eq!(res.message_id.as_str(), "abc-def-123-456");
    assert_eq!(res.message.text, "Welcome! bleedPurple Type !rules");
}

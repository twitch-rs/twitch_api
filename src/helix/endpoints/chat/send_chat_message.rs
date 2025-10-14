//! Sends an message in the broadcaster’s chat room.
//! [`send-chat-message`](https://dev.twitch.tv/docs/api/reference#send-chat-message)
//!
//! # Accessing the endpoint
//!
//! ## Request: [SendChatMessageRequest]
//!
//! To use this endpoint, construct a [`SendChatMessageRequest`] with the [`SendChatMessageRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::send_chat_message;
//! let request = send_chat_message::SendChatMessageRequest::new();
//! ```
//!
//! ## Body: [SendChatMessageBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::chat::send_chat_message;
//! let body = send_chat_message::SendChatMessageBody::new(
//!     "12826",                       // broadcaster_id
//!     "141981764",                   // sender_id
//!     "Hello, world! twitchdevHype", // message
//! );
//! ```
//!
//! ## Response: [SendChatMessageResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::send_chat_message};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = send_chat_message::SendChatMessageRequest::new();
//! let body =
//!     send_chat_message::SendChatMessageBody::new(
//!         "12826",                        // broadcaster_id
//!         "141981764",                    // sender_id
//!         "Hello, world! twitchdevHype"   // message
//!     );
//! let response: helix::chat::SendChatMessageResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`SendChatMessageRequest::parse_response(None, &request.get_uri(), response)`](SendChatMessageRequest::parse_response)

use std::marker::PhantomData;

use super::*;
use helix::RequestPost;
use serde::Serialize;

// Not implementing builder since it's not really needed...
/// Query Parameters for [Send Chat message](super::send_chat_message)
///
/// [`send-chat-message`](https://dev.twitch.tv/docs/api/reference#send-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[must_use]
#[non_exhaustive]
pub struct SendChatMessageRequest<'a> {
    #[serde(skip)]
    _marker: PhantomData<&'a ()>,
}

impl SendChatMessageRequest<'_> {
    /// Create a new [`SendChatMessageRequest`]
    pub fn new() -> Self { SendChatMessageRequest::default() }
}

/// Body Parameters for [Send Chat message](super::send_chat_message)
///
/// [`send-chat-message`](https://dev.twitch.tv/docs/api/reference#send-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct SendChatMessageBody<'a> {
    /// The ID of the broadcaster whose chat room the message will be sent to.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the user sending the message. This ID must match the user ID in the user access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub sender_id: Cow<'a, types::UserIdRef>,
    /// The message to send.
    ///
    /// The message is limited to a maximum of 500 characters.
    /// Chat messages can also include emoticons.
    /// To include emoticons, use the name of the emote. The names are case sensitive.
    /// Don't include colons around the name (e.g., :bleedPurple:).
    /// If Twitch recognizes the name, Twitch converts the name to the emote
    /// before writing the chat message to the chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub message: Cow<'a, str>,
    /// The ID of the chat message being replied to.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parent_message_id: Option<Cow<'a, types::MsgIdRef>>,
    /// Determines if the chat message is sent only to the source channel (broadcaster_id) during a shared chat session.
    ///
    /// # Notes
    ///
    /// Only available when using an App Access Token.
    /// Has no effect if the message is not sent during a shared chat session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_source_only: Option<bool>,
}

impl<'a> SendChatMessageBody<'a> {
    /// Send a message in the broadcaster's channel
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        sender_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        message: impl types::IntoCow<'a, str> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            sender_id: sender_id.into_cow(),
            message: message.into_cow(),
            reply_parent_message_id: None,
            for_source_only: None,
        }
    }

    /// Set the reply parent message-id
    pub fn reply_parent_message_id(
        mut self,
        reply_parent_message_id: impl types::IntoCow<'a, types::MsgIdRef> + 'a,
    ) -> Self {
        self.reply_parent_message_id = Some(reply_parent_message_id.into_cow());
        self
    }

    /// Send the chat message only to the source channel (broadcaster_id) during a shared chat session.
    ///
    /// # Notes
    ///
    /// Only available when using an App Access Token.
    /// Has no effect if the message is not sent during a shared chat session.
    pub fn for_source_only(mut self, source_only: bool) -> Self {
        self.for_source_only = Some(source_only);
        self
    }
}

impl helix::private::SealedSerialize for SendChatMessageBody<'_> {}

impl helix::HelixRequestBody for [SendChatMessageBody<'_>] {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a [SendChatMessageBody<'a>],
        }

        serde_json::to_vec(&InnerBody { data: self })
            .map_err(Into::into)
            .map(Into::into)
    }
}

// The variants below were automatically generated with the following script on
// https://dev.twitch.tv/docs/irc/msg-id/ (assumed to be the same codes)
// [...document.querySelectorAll("table > tbody > tr")]
//   .map((row) => [...row.querySelectorAll("td")].map((x) => x.textContent))
//   .filter(([id]) => id.startsWith("msg_"))
//   .map(
//     ([id, description]) =>
//       `/// ${description}\n${id
//         .split("_")
//         .map((x) => x[0].toUpperCase() + x.slice(1))
//         .join("")},`,
//   )
//   .join("\n");

/// Code for why a message was dropped.
///
/// See <https://dev.twitch.tv/docs/irc/msg-id/>
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatMessageDropCode {
    /// You are permanently banned from talking in `<channel>`.
    MsgBanned,
    /// Your message was not sent because it contained too many unprocessable characters. If you believe this is an error, please rephrase and try again.
    MsgBadCharacters,
    /// Your message was not sent because your account is not in good standing in this channel.
    MsgChannelBlocked,
    /// This channel does not exist or has been suspended.
    MsgChannelSuspended,
    /// Your message was not sent because it is identical to the previous one you sent, less than 30 seconds ago.
    MsgDuplicate,
    /// This room is in emote-only mode. You can find your currently available emoticons using the smiley in the chat text area.
    MsgEmoteonly,
    /// This room is in `<duration>` followers-only mode. Follow `<channel>` to join the community! Note: These msg_followers tags are kickbacks to a user who does not meet the criteria; that is, does not follow or has not followed long enough.
    MsgFollowersonly,
    /// This room is in `<duration1>` followers-only mode. You have been following for `<duration2>`. Continue following to chat!
    MsgFollowersonlyFollowed,
    /// This room is in followers-only mode. Follow `<channel>` to join the community!
    MsgFollowersonlyZero,
    /// This room is in unique-chat mode and the message you attempted to send is not unique.
    MsgR9k,
    /// Your message was not sent because you are sending messages too quickly.
    MsgRatelimit,
    /// Hey! Your message is being checked by mods and has not been sent.
    MsgRejected,
    /// Your message wasn’t posted due to conflicts with the channel’s moderation settings.
    MsgRejectedMandatory,
    /// A verified phone number is required to chat in this channel. Please visit <https://www.twitch.tv/settings/security> to verify your phone number.
    MsgRequiresVerifiedPhoneNumber,
    /// This room is in slow mode and you are sending messages too quickly. You will be able to talk again in `<number>` seconds.
    MsgSlowmode,
    /// This room is in subscribers only mode. To talk, purchase a channel subscription at `https://www.twitch.tv/products/<broadcaster login name>/ticket?ref=subscriber_only_mode_chat`.
    MsgSubsonly,
    /// You don’t have permission to perform that action.
    MsgSuspended,
    /// You are timed out for `<number>` more seconds.
    MsgTimedout,
    /// This room requires a verified account to chat. Please verify your account at <https://www.twitch.tv/settings/security>.
    MsgVerifiedEmail,
    /// An unknown drop-code.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for ChatMessageDropCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
}

/// A drop reason of a sent message.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChatMessageDropReason {
    /// Code for why the message was dropped.
    ///
    /// See [ChatMessageDropCode] for possible values.
    pub code: ChatMessageDropCode,
    /// Message for why the message was dropped.
    pub message: String,
}

/// Return Values for [Send Chat message](super::send_chat_message)
///
/// [`send-chat-message`](https://dev.twitch.tv/docs/api/reference#send-chat-message)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SendChatMessageResponse {
    /// The message id for the message that was sent.
    #[serde(deserialize_with = "crate::deserialize_none_from_empty_string")]
    pub message_id: Option<types::MsgId>,
    /// If the message passed all checks and was sent.
    pub is_sent: bool,
    /// The reason the message was dropped, if any.
    pub drop_reason: Option<ChatMessageDropReason>,
}

impl Request for SendChatMessageRequest<'_> {
    type PaginationData = ();
    type Response = SendChatMessageResponse;

    const PATH: &'static str = "chat/messages";
    /// Requires an app access token or user access token that includes the [user:write:chat][twitch_oauth2::Scope::UserWriteChat] scope.
    /// If app access token used, then additionally requires [user:bot][twitch_oauth2::Scope::UserBot] scope from chatting user,
    /// and either [channel:bot][twitch_oauth2::Scope::ChannelBot] scope from broadcaster or moderator status.
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserWriteChat];
}

impl<'a> RequestPost for SendChatMessageRequest<'a> {
    type Body = SendChatMessageBody<'a>;

    fn parse_inner_response<'d>(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        let resp = match status {
            http::StatusCode::OK => {
                let resp: helix::InnerResponse<[SendChatMessageResponse; 1]> =
                    helix::parse_json(response, true).map_err(|e| {
                        helix::HelixRequestPostError::DeserializeError(
                            response.to_string(),
                            e,
                            uri.clone(),
                            status,
                        )
                    })?;
                let [s] = resp.data;
                s
            }
            _ => {
                return Err(helix::HelixRequestPostError::InvalidResponse {
                    reason: "unexpected status",
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
fn test_success() {
    use helix::*;
    let req = SendChatMessageRequest::new();

    let body = SendChatMessageBody::new("12826", "141981764", "Hello, world! twitchdevHype");

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"broadcaster_id":"12826","sender_id":"141981764","message":"Hello, world! twitchdevHype"}"#
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
      "data": [
          {
            "message_id": "abc-123-def",
            "is_sent": true
          }
      ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/messages?"
    );

    dbg!(SendChatMessageRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_reject() {
    use helix::*;
    let req = SendChatMessageRequest::new();
    let data = br#"
    {
      "data": [
        {
          "message_id": "",
          "is_sent": false,
          "drop_reason": {
            "code": "msg_rejected",
            "message": "Your message is being checked by mods and has not been sent."
          }
        }
      ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();
    let uri = req.get_uri().unwrap();

    let res = SendChatMessageRequest::parse_response(None, &uri, http_response).unwrap();
    assert_eq!(res.data.message_id, None);
    assert_eq!(
        res.data.drop_reason.unwrap().code,
        ChatMessageDropCode::MsgRejected
    );

    let data = br#"
    {
      "data": [
        {
          "message_id": "",
          "is_sent": false,
          "drop_reason": {
            "code": "Foo",
            "message": "Super unknown"
          }
        }
      ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let res = SendChatMessageRequest::parse_response(None, &uri, http_response).unwrap();
    assert_eq!(res.data.message_id, None);
    assert_eq!(
        res.data.drop_reason.unwrap().code,
        ChatMessageDropCode::Unknown("Foo".to_string())
    );
}

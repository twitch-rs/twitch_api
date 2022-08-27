//! Sends an announcement to the broadcaster’s chat room.
//! [`send-chat-announcement`](https://dev.twitch.tv/docs/api/reference#send-chat-announcement)
//!
//! # Accessing the endpoint
//!
//! ## Request: [SendChatAnnouncementRequest]
//!
//! To use this endpoint, construct a [`SendChatAnnouncementRequest`] with the [`SendChatAnnouncementRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::send_chat_announcement;
//! let request = send_chat_announcement::SendChatAnnouncementRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! ```
//!
//! ## Body: [SendChatAnnouncementBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::chat::send_chat_announcement;
//! let body =
//!     send_chat_announcement::SendChatAnnouncementBody::new("Hello chat!".to_owned(), "purple")
//!         .unwrap();
//! ```
//!
//! ## Response: [SendChatAnnouncementResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::send_chat_announcement};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = send_chat_announcement::SendChatAnnouncementRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! let body = send_chat_announcement::SendChatAnnouncementBody::new(
//!     "Hello chat!".to_owned(),
//!     "purple",
//! ).unwrap();
//! let response: helix::chat::SendChatAnnouncementResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`SendChatAnnouncementRequest::parse_response(None, &request.get_uri(), response)`](SendChatAnnouncementRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Send Chat Announcement](super::send_chat_announcement)
///
/// [`send-chat-announcement`](https://dev.twitch.tv/docs/api/reference#send-chat-announcement)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct SendChatAnnouncementRequest {
    /// The ID of the broadcaster that owns the chat room to send the announcement to.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_id: types::UserId,
    /// The ID of a user who has permission to moderate the broadcaster’s chat room.
    ///
    /// This ID must match the user ID in the OAuth token, which can be a moderator or the broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_id: types::UserId,
}

impl SendChatAnnouncementRequest {
    pub fn new(
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into(),
            moderator_id: moderator_id.into(),
        }
    }
}

/// Body Parameters for [Send Chat Announcement](super::send_chat_announcement)
///
/// [`send-chat-announcement`](https://dev.twitch.tv/docs/api/reference#send-chat-announcement)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct SendChatAnnouncementBody {
    /// The announcement to make in the broadcaster’s chat room. Announcements are limited to a maximum of 500 characters; announcements longer than 500 characters are truncated.
    pub message: String,
    // FIXME: Enumify?
    /// The color used to highlight the announcement. Possible case-sensitive values are:
    ///
    /// * blue
    /// * green
    /// * orange
    /// * purple
    /// * primary (default)
    ///
    /// If color is set to primary or is not set, the channel’s accent color is used to highlight the announcement (see Profile Accent Color under [profile settings](https://www.twitch.tv/settings/profile), Channel and Videos, and Brand).
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub color: AnnouncementColor,
}

impl SendChatAnnouncementBody {
    /// Create a new announcement with specified color
    pub fn new<E>(
        message: String,
        color: impl std::convert::TryInto<AnnouncementColor, Error = E>,
    ) -> Result<Self, E> {
        Ok(Self {
            message,
            color: color.try_into()?,
        })
    }
}

impl helix::private::SealedSerialize for SendChatAnnouncementBody {}

impl helix::HelixRequestBody for Vec<SendChatAnnouncementBody> {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a Vec<SendChatAnnouncementBody>,
        }

        serde_json::to_vec(&InnerBody { data: self })
            .map_err(Into::into)
            .map(Into::into)
    }
}

/// Return Values for [Send Chat Announcement](super::send_chat_announcement)
///
/// [`send-chat-announcement`](https://dev.twitch.tv/docs/api/reference#send-chat-announcement)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum SendChatAnnouncementResponse {
    /// Successfully sent the announcement.
    Success,
}

impl Request for SendChatAnnouncementRequest {
    // FIXME: this is a single entry
    type Response = SendChatAnnouncementResponse;

    const PATH: &'static str = "chat/announcements";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorManageAnnouncements];
}

impl RequestPost for SendChatAnnouncementRequest {
    type Body = SendChatAnnouncementBody;

    fn parse_inner_response<'d>(
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
                data: SendChatAnnouncementResponse::Success,
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
    let req = SendChatAnnouncementRequest::new("1234", "5678");

    let body = SendChatAnnouncementBody::new("hello chat!".to_owned(), "purple").unwrap();

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = b"".to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/announcements?broadcaster_id=1234&moderator_id=5678"
    );

    dbg!(SendChatAnnouncementRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

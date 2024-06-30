//! Warns a user in a broadcaster’s chat room, preventing them from chatting until the warning is acknowledged.
//! [`warn-chat-user`](https://dev.twitch.tv/docs/api/reference#warn-chat-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [WarnChatUserRequest]
//!
//! To use this endpoint, construct a [`WarnChatUserRequest`] with the [`WarnChatUserRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::warn_chat_user;
//! let request = warn_chat_user::WarnChatUserRequest::new("404040", "404041");
//! ```
//!
//! ## Body: [WarnChatUserBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::moderation::warn_chat_user;
//! let body = warn_chat_user::WarnChatUserBody::new("9876", "stop doing that!");
//! ```
//!
//! ## Response: [WarnChatUser]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::warn_chat_user};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = warn_chat_user::WarnChatUserRequest::new("404040", "404041");
//! let body = warn_chat_user::WarnChatUserBody::new("9876", "stop doing that!");
//! let response: warn_chat_user::WarnChatUser = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`WarnChatUserRequest::parse_response(None, &request.get_uri(), response)`](WarnChatUserRequest::parse_response)

use std::borrow::Cow;
use serde_derive::{Deserialize, Serialize};
use crate::{helix, types};
use crate::helix::{Request, RequestPost};
/// Query Parameters for [Warn Chat User](super::warn_chat_user)
///
/// [`warn-chat-user`](https://dev.twitch.tv/docs/api/reference#warn-chat-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct WarnChatUserRequest<'a> {
    /// The ID of the broadcaster whose chat room the user is being warned in.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room.
    /// This ID must match the user ID associated with the user OAuth token.
    ///
    /// If the broadcaster wants to warn the user (instead of having the moderator do it),
    /// set this parameter to the broadcaster’s ID, too.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> WarnChatUserRequest<'a> {
    /// Warn a user on this channel
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

/// Body Parameters for [Warn Chat User](super::warn_chat_user)
///
/// [`warn-chat-user`](https://dev.twitch.tv/docs/api/reference#warn-chat-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct WarnChatUserBody<'a> {
    /// The ID of the user to warn.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// The reason the user is being warned. The text is user defined and limited to a maximum of 500 characters.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub reason: Cow<'a, str>,
}

impl<'a> WarnChatUserBody<'a> {
    /// Create a new [`WarnChatUserBody`]
    pub fn new(
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        reason: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            user_id: user_id.into_cow(),
            reason: reason.into(),
        }
    }
}

impl helix::HelixRequestBody for WarnChatUserBody<'_> {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a WarnChatUserBody<'a>,
        }
        serde_json::to_vec(&InnerBody { data: self })
            .map_err(Into::into)
            .map(Into::into)
    }
}

/// Return Values for [Warn Chat User](super::warn_chat_user)
///
/// [`warn-chat-user`](https://dev.twitch.tv/docs/api/reference#warn-chat-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct WarnChatUser {
    /// The broadcaster whose chat room the user was warned in.
    pub broadcaster_id: types::UserId,
    /// The moderator that warned the user.
    pub moderator_id: types::UserId,
    /// The user that was warned.
    pub user_id: types::UserId,
    /// The reason provided for the warning.
    pub reason: String,
}

impl Request for WarnChatUserRequest<'_> {
    type Response = WarnChatUser;

    const PATH: &'static str = "moderation/warnings";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageWarnings];
}

impl<'a> RequestPost for  WarnChatUserRequest<'a> {
    type Body = WarnChatUserBody<'a>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
        where
            Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug, Clone)]
        struct InnerResponse {
            data: Vec<WarnChatUser>,
        }
        let InnerResponse { data } = helix::parse_json(response, true).map_err(|e| {
            helix::HelixRequestPostError::DeserializeError(
                response.to_string(),
                e,
                uri.clone(),
                status,
            )
        })?;
        Ok(helix::Response::with_data(
            data.into_iter().next().ok_or_else(|| {
                helix::HelixRequestPostError::InvalidResponse {
                    reason: "missing response data",
                    response: response.to_string(),
                    status,
                    uri: uri.clone(),
                }
            })?,
            request,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = WarnChatUserRequest::new("404040", "404041");

    let body = WarnChatUserBody::new("9876", "stop doing that!");

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"data":{"user_id":"9876","reason":"stop doing that!"}}"#
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
      "data": [
        {
          "broadcaster_id": "404040",
          "user_id": "9876",
          "moderator_id": "404041",
          "reason": "stop doing that!"
        }
      ]
    }
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/warnings?broadcaster_id=404040&moderator_id=404041"
    );

    dbg!(WarnChatUserRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
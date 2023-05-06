//! Updates the color used for the user’s name in chat.
//!
//! [`update-user-chat-color`](https://dev.twitch.tv/docs/api/reference#update-user-chat-color)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateUserChatColorRequest]
//!
//! To use this endpoint, construct an [`UpdateUserChatColorRequest`] with the [`UpdateUserChatColorRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::update_user_chat_color;
//! let request = update_user_chat_color::UpdateUserChatColorRequest::new(
//!     "123",
//!     twitch_types::NamedUserColor::Blue,
//! );
//! ```
//!
//! ## Response: [UpdateUserChatColorResponse]
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::update_user_chat_color};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_user_chat_color::UpdateUserChatColorRequest::new(
//!     "123",
//!     twitch_types::NamedUserColor::Blue,
//! );
//! let response: helix::chat::UpdateUserChatColorResponse = client.req_put(request, helix::EmptyBody, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPut::create_request)
//! and parse the [`http::Response`] with [`UpdateUserChatColorRequest::parse_response(None, &request.get_uri(), response)`](UpdateUserChatColorRequest::parse_response)

use super::*;
use helix::RequestPut;
/// Query Parameters for [Update Chat Settings](super::update_user_chat_color)
///
/// [`update-user-chat-color`](https://dev.twitch.tv/docs/api/reference#update-user-chat-color)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UpdateUserChatColorRequest<'a> {
    /// The ID of the user whose chat color you want to update.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// The color to use for the user’s name in chat.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub color: types::NamedUserColor<'a>,
}

impl<'a> UpdateUserChatColorRequest<'a> {
    /// Update the users chat color
    pub fn new(
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        color: types::NamedUserColor<'static>,
    ) -> Self {
        Self {
            user_id: user_id.into_cow(),
            color,
        }
    }
}

/// Return Values for [Update Chat Settings](super::update_user_chat_color)
///
/// [`update-user-chat-color`](https://dev.twitch.tv/docs/api/reference#update-user-chat-color)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum UpdateUserChatColorResponse {
    /// Successfully updated the user’s chat color.
    Success,
}

impl Request for UpdateUserChatColorRequest<'_> {
    type Response = UpdateUserChatColorResponse;

    const PATH: &'static str = "chat/color";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserManageChatColor];
}

impl RequestPut for UpdateUserChatColorRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response<'d>(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPutError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response {
                data: UpdateUserChatColorResponse::Success,
                pagination: None,
                request,
                total: None,
                other: None,
            }),
            _ => Err(helix::HelixRequestPutError::InvalidResponse {
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
fn test_request_named() {
    use helix::*;
    let req = UpdateUserChatColorRequest::new("123", types::NamedUserColor::Blue);

    dbg!(req.create_request(EmptyBody, "token", "clientid").unwrap());

    // From twitch docs
    let data = vec![];

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/color?user_id=123&color=blue"
    );

    dbg!(UpdateUserChatColorRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_hex() {
    use std::convert::TryInto;

    use helix::*;
    let req = UpdateUserChatColorRequest::new("123", "#9146FF".try_into().unwrap());

    dbg!(req.create_request(EmptyBody, "token", "clientid").unwrap());

    // From twitch docs
    let data = vec![];

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/color?user_id=123&color=%239146FF"
    );

    dbg!(UpdateUserChatColorRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

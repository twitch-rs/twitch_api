//! Allow or deny a message that was held for review by AutoMod.
//! [`manage-held-automod-messages`](https://dev.twitch.tv/docs/api/reference#manage-held-automod-messages)
//!
//! # Accessing the endpoint
//!
//! ## Request: [ManageHeldAutoModMessagesRequest]
//!
//! To use this endpoint, construct a [`ManageHeldAutoModMessagesRequest`] with the [`ManageHeldAutoModMessagesRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::manage_held_automod_messages;
//! let request =
//!     manage_held_automod_messages::ManageHeldAutoModMessagesRequest::new();
//! ```
//!
//! ## Body: [ManageHeldAutoModMessagesBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::moderation::manage_held_automod_messages;
//! let body = manage_held_automod_messages::ManageHeldAutoModMessagesBody::new(
//!     "9327994",
//!     "836013710",
//!     true,
//! );
//! ```
//!
//! ## Response: [ManageHeldAutoModMessages]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::manage_held_automod_messages};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = manage_held_automod_messages::ManageHeldAutoModMessagesRequest::new();
//! let body = manage_held_automod_messages::ManageHeldAutoModMessagesBody::new(
//!     "9327994",
//!     "836013710",
//!     true,
//! );
//! let response: manage_held_automod_messages::ManageHeldAutoModMessages = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`ManageHeldAutoModMessagesRequest::parse_response(None, &request.get_uri(), response)`](ManageHeldAutoModMessagesRequest::parse_response)

use std::marker::PhantomData;

use super::*;
use helix::RequestPost;
/// Query Parameters for [Manage Held AutoMod Messages](super::manage_held_automod_messages)
///
/// [`manage-held-automod-messages`](https://dev.twitch.tv/docs/api/reference#manage-held-automod-messages)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct ManageHeldAutoModMessagesRequest<'a> {
    #[serde(skip)]
    _marker: PhantomData<&'a ()>,
}

impl ManageHeldAutoModMessagesRequest<'_> {
    /// Create a new [`ManageHeldAutoModMessagesRequest`]
    pub fn new() -> Self { Self::default() }
}

/// Body Parameters for [Manage Held AutoMod Messages](super::manage_held_automod_messages)
///
/// [`manage-held-automod-messages`](https://dev.twitch.tv/docs/api/reference#manage-held-automod-messages)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ManageHeldAutoModMessagesBody<'a> {
    /// The moderator who is approving or rejecting the held message. Must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// ID of the message to be allowed or denied. These message IDs are retrieved from IRC or PubSub. Only one message ID can be provided.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub msg_id: Cow<'a, types::MsgIdRef>,
    /// The action to take for the message. Must be "ALLOW" or "DENY".
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub action: AutoModAction,
}

impl<'a> ManageHeldAutoModMessagesBody<'a> {
    /// Create a new [`ManageHeldAutoModMessagesBody`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::moderation::ManageHeldAutoModMessagesBody;
    ///
    /// let body = ManageHeldAutoModMessagesBody::new("1234", "5678", true);
    /// ```
    pub fn new(
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        msg_id: impl types::IntoCow<'a, types::MsgIdRef> + 'a,
        action: impl Into<AutoModAction>,
    ) -> Self {
        Self {
            user_id: user_id.into_cow(),
            msg_id: msg_id.into_cow(),
            action: action.into(),
        }
    }
}

/// Action to take for a message.
#[derive(PartialEq, Eq, Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum AutoModAction {
    /// Allow the message
    Allow,
    /// Deny the message
    Deny,
}

impl From<bool> for AutoModAction {
    fn from(b: bool) -> Self {
        match b {
            true => Self::Allow,
            false => Self::Deny,
        }
    }
}

impl helix::private::SealedSerialize for ManageHeldAutoModMessagesBody<'_> {}

/// Return Values for [Manage Held AutoMod Messages](super::manage_held_automod_messages)
///
/// [`manage-held-automod-messages`](https://dev.twitch.tv/docs/api/reference#manage-held-automod-messages)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum ManageHeldAutoModMessages {
    /// Successfully approved or denied the message
    Success,
}

impl Request for ManageHeldAutoModMessagesRequest<'_> {
    type Response = ManageHeldAutoModMessages;

    const PATH: &'static str = "moderation/automod/message";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageAutoMod];
}

impl<'a> RequestPost for ManageHeldAutoModMessagesRequest<'a> {
    type Body = ManageHeldAutoModMessagesBody<'a>;

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
            http::StatusCode::NO_CONTENT => Ok(helix::Response::with_data(
                ManageHeldAutoModMessages::Success,
                request,
            )),
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
    let req = ManageHeldAutoModMessagesRequest::new();

    let body = ManageHeldAutoModMessagesBody::new("9327994", "836013710", true);

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"user_id":"9327994","msg_id":"836013710","action":"ALLOW"}"#
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/automod/message?"
    );

    dbg!(ManageHeldAutoModMessagesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

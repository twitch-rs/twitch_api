//! Updates the broadcaster’s chat settings.
//! [`update-chat-settings`](https://dev.twitch.tv/docs/api/reference#update-chat-settings)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateChatSettingsRequest]
//!
//! To use this endpoint, construct an [`UpdateChatSettingsRequest`] with the [`UpdateChatSettingsRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::update_chat_settings;
//! let request =
//!     update_chat_settings::UpdateChatSettingsRequest::new("1234", "5678");
//! ```
//!
//! ## Body: [UpdateChatSettingsBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::chat::update_chat_settings;
//! let mut body = update_chat_settings::UpdateChatSettingsBody::default();
//! body.slow_mode = Some(true);
//! body.slow_mode_wait_time = Some(10);
//! ```
//!
//! ## Response: [ChatSettings]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::update_chat_settings};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_chat_settings::UpdateChatSettingsRequest::new("1234", "5678");
//! let mut body = update_chat_settings::UpdateChatSettingsBody::default();
//! body.slow_mode = Some(true);
//! body.slow_mode_wait_time = Some(10);
//! let response: helix::chat::ChatSettings = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`UpdateChatSettingsRequest::parse_response(None, &request.get_uri(), response)`](UpdateChatSettingsRequest::parse_response)

use crate::helix::{parse_json, HelixRequestPatchError};

use super::*;
use helix::RequestPatch;
/// Query Parameters for [Update Chat Settings](super::update_chat_settings)
///
/// [`update-chat-settings`](https://dev.twitch.tv/docs/api/reference#update-chat-settings)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UpdateChatSettingsRequest<'a> {
    /// The ID of the broadcaster whose chat settings you want to update.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room.
    /// This ID must match the user ID associated with the user OAuth token.
    ///
    /// If the broadcaster is making the update, also set this parameter to the broadcaster’s ID.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

///FIXME: The moderator_id parameter is redundant, we should make this a client ext function
impl<'a> UpdateChatSettingsRequest<'a> {
    /// Update the chat settings for the specified broadcaster as the specified moderator
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

/// Body Parameters for [Update Chat Settings](super::update_chat_settings)
///
/// [`update-chat-settings`](https://dev.twitch.tv/docs/api/reference#update-chat-settings)
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateChatSettingsBody {
    /// A Boolean value that determines whether chat messages must contain only emotes.
    ///
    /// Set to true, if only messages that are 100% emotes are allowed; otherwise, false. Default is false.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emote_mode: Option<bool>,
    /// A Boolean value that determines whether the broadcaster restricts the chat room to followers only, based on how long they’ve followed.
    ///
    /// Set to true, if the broadcaster restricts the chat room to followers only; otherwise, false. Default is true.
    ///
    /// See [`follower_mode_duration`](Self::follower_mode_duration) for how long the followers must have followed the broadcaster to participate in the chat room.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_mode: Option<bool>,
    /// The length of time, in minutes, that the followers must have followed the broadcaster to participate in the chat room (see follower_mode).
    ///
    /// You may specify a value in the range: 0 (no restriction) through 129600 (3 months). The default is 0.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_mode_duration: Option<u64>,
    /// A Boolean value that determines whether the broadcaster adds a short delay before chat messages appear in the chat room. This gives chat moderators and bots a chance to remove them before viewers can see the message.
    ///
    /// Set to true, if the broadcaster applies a delay; otherwise, false. Default is false.
    ///
    /// See [`non_moderator_chat_delay_duration`](Self::non_moderator_chat_delay_duration) for the length of the delay.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_moderator_chat_delay: Option<bool>,
    /// The amount of time, in seconds, that messages are delayed from appearing in chat.
    ///
    /// Possible values are:
    ///
    /// * 2 — 2 second delay (recommended)
    /// * 4 — 4 second delay
    /// * 6 — 6 second delay
    ///
    /// See [`non_moderator_chat_delay`](Self::non_moderator_chat_delay).
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_moderator_chat_delay_duration: Option<u64>,
    /// A Boolean value that determines whether the broadcaster limits how often users in the chat room are allowed to send messages.
    ///
    /// Set to true, if the broadcaster applies a wait period messages; otherwise, false. Default is false.
    ///
    /// See [`slow_mode_wait_time`](Self::slow_mode_wait_time) for the delay.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode: Option<bool>,
    /// The amount of time, in seconds, that users need to wait between sending messages (see slow_mode).
    ///
    /// You may specify a value in the range: 3 (3 second delay) through 120 (2 minute delay). The default is 30 seconds.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_wait_time: Option<u64>,
    /// A Boolean value that determines whether only users that subscribe to the broadcaster’s channel can talk in the chat room.
    ///
    /// Set to true, if the broadcaster restricts the chat room to subscribers only; otherwise, false. Default is false.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_mode: Option<bool>,
    /// A Boolean value that determines whether the broadcaster requires users to post only unique messages in the chat room.
    ///
    /// Set to true, if the broadcaster requires unique messages only; otherwise, false. Default is false.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_chat_mode: Option<bool>,
}

impl helix::private::SealedSerialize for UpdateChatSettingsBody {}

/// Return Values for [Update Chat Settings](super::update_chat_settings)
///
/// [`update-chat-settings`](https://dev.twitch.tv/docs/api/reference#update-chat-settings)
pub type UpdateChatSettingsResponse = ChatSettings;

impl Request for UpdateChatSettingsRequest<'_> {
    type PaginationData = ();
    type Response = ChatSettings;

    const PATH: &'static str = "chat/settings";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageChatSettings];
}

impl RequestPatch for UpdateChatSettingsRequest<'_> {
    type Body = UpdateChatSettingsBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPatchError>
    where
        Self: Sized,
    {
        let resp = match status {
            http::StatusCode::OK => {
                let resp: helix::InnerResponse<Vec<ChatSettings>> = parse_json(response, true)
                    .map_err(|e| {
                        HelixRequestPatchError::DeserializeError(
                            response.to_string(),
                            e,
                            uri.clone(),
                            status,
                        )
                    })?;
                resp.data.into_iter().next().ok_or(
                    helix::HelixRequestPatchError::InvalidResponse {
                        reason: "expected at least one element in data",
                        response: response.to_string(),
                        status,
                        uri: uri.clone(),
                    },
                )?
            }
            _ => {
                return Err(helix::HelixRequestPatchError::InvalidResponse {
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
#[allow(clippy::field_reassign_with_default)]
fn test_request() {
    use helix::*;
    let req = UpdateChatSettingsRequest::new("1234", "5678");

    let mut body = UpdateChatSettingsBody::default();
    // FIXME: Setters
    body.slow_mode = Some(true);
    body.slow_mode_wait_time = Some(10);

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"slow_mode":true,"slow_mode_wait_time":10}"#
    );
    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br##"
    {
        "data": [
          {
            "broadcaster_id": "1234",
            "moderator_id": "5678",
            "slow_mode": true,
            "slow_mode_wait_time": 10,
            "follower_mode": false,
            "follower_mode_duration": null,
            "subscriber_mode": false,
            "emote_mode": false,
            "unique_chat_mode": false,
            "non_moderator_chat_delay": false,
            "non_moderator_chat_delay_duration": null
          }
        ]
    }"##
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/settings?broadcaster_id=1234&moderator_id=5678"
    );

    dbg!(UpdateChatSettingsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

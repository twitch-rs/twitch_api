//! Updates the broadcaster’s chat settings.
//! [`update-chat-settings`](https://dev.twitch.tv/docs/api/reference#update-chat-settings)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateChatSettingsRequest]
//!
//! To use this endpoint, construct an [`UpdateChatSettingsRequest`] with the [`UpdateChatSettingsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::update_chat_settings;
//! let request = update_chat_settings::UpdateChatSettingsRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! ```
//!
//! ## Body: [UpdateChatSettingsBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::chat::update_chat_settings;
//! let body = update_chat_settings::UpdateChatSettingsBody::builder()
//!     .slow_mode(true)
//!     .slow_mode_wait_time(10)
//!     .build();
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
//! let request = update_chat_settings::UpdateChatSettingsRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! let body = update_chat_settings::UpdateChatSettingsBody::builder()
//!     .slow_mode(true)
//!     .slow_mode_wait_time(10)
//!     .build();
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
#[derive(PartialEq, Eq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateChatSettingsRequest {
    /// The ID of the broadcaster whose chat settings you want to update.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room.
    /// This ID must match the user ID associated with the user OAuth token.
    ///
    /// If the broadcaster is making the update, also set this parameter to the broadcaster’s ID.
    #[builder(setter(into))]
    pub moderator_id: types::UserId,
}

/// Body Parameters for [Update Chat Settings](super::update_chat_settings)
///
/// [`update-chat-settings`](https://dev.twitch.tv/docs/api/reference#update-chat-settings)
#[derive(PartialEq, Eq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateChatSettingsBody {
    /// A Boolean value that determines whether chat messages must contain only emotes.
    ///
    /// Set to true, if only messages that are 100% emotes are allowed; otherwise, false. Default is false.
    #[builder(default, setter(into))]
    pub emote_mode: Option<bool>,
    /// A Boolean value that determines whether the broadcaster restricts the chat room to followers only, based on how long they’ve followed.
    ///
    /// Set to true, if the broadcaster restricts the chat room to followers only; otherwise, false. Default is true.
    ///
    /// See [`follower_mode_duration`](Self::follower_mode_duration) for how long the followers must have followed the broadcaster to participate in the chat room.
    #[builder(default, setter(into))]
    pub follower_mode: Option<bool>,
    /// The length of time, in minutes, that the followers must have followed the broadcaster to participate in the chat room (see follower_mode).
    ///
    /// You may specify a value in the range: 0 (no restriction) through 129600 (3 months). The default is 0.
    #[builder(default, setter(into))]
    pub follower_mode_duration: Option<u64>,
    /// A Boolean value that determines whether the broadcaster adds a short delay before chat messages appear in the chat room. This gives chat moderators and bots a chance to remove them before viewers can see the message.
    ///
    /// Set to true, if the broadcaster applies a delay; otherwise, false. Default is false.
    ///
    /// See [`non_moderator_chat_delay_duration`](Self::non_moderator_chat_delay_duration) for the length of the delay.
    #[builder(default, setter(into))]
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
    #[builder(default, setter(into))]
    pub non_moderator_chat_delay_duration: Option<u64>,
    /// A Boolean value that determines whether the broadcaster limits how often users in the chat room are allowed to send messages.
    ///
    /// Set to true, if the broadcaster applies a wait period messages; otherwise, false. Default is false.
    ///
    /// See [`slow_mode_wait_time`](Self::slow_mode_wait_time) for the delay.
    #[builder(default, setter(into))]
    pub slow_mode: Option<bool>,
    /// The amount of time, in seconds, that users need to wait between sending messages (see slow_mode).
    ///
    /// You may specify a value in the range: 3 (3 second delay) through 120 (2 minute delay). The default is 30 seconds.
    #[builder(default, setter(into))]
    pub slow_mode_wait_time: Option<u64>,
    /// A Boolean value that determines whether only users that subscribe to the broadcaster’s channel can talk in the chat room.
    ///
    /// Set to true, if the broadcaster restricts the chat room to subscribers only; otherwise, false. Default is false.
    #[builder(default, setter(into))]
    pub subscriber_mode: Option<bool>,
    /// A Boolean value that determines whether the broadcaster requires users to post only unique messages in the chat room.
    ///
    /// Set to true, if the broadcaster requires unique messages only; otherwise, false. Default is false.
    #[builder(default, setter(into))]
    pub unique_chat_mode: Option<bool>,
}

impl helix::private::SealedSerialize for UpdateChatSettingsBody {}

/// Return Values for [Update Chat Settings](super::update_chat_settings)
///
/// [`update-chat-settings`](https://dev.twitch.tv/docs/api/reference#update-chat-settings)
pub type UpdateChatSettingsResponse = ChatSettings;

impl Request for UpdateChatSettingsRequest {
    type Response = ChatSettings;

    const PATH: &'static str = "chat/settings";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorManageChatSettings];
}

impl RequestPatch for UpdateChatSettingsRequest {
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
        Ok(helix::Response {
            data: resp,
            pagination: None,
            request,
            total: None,
            other: None,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = UpdateChatSettingsRequest::builder()
        .broadcaster_id("1234")
        .moderator_id("5678")
        .build();

    let body = UpdateChatSettingsBody::builder()
        .slow_mode(true)
        .slow_mode_wait_time(10)
        .build();

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

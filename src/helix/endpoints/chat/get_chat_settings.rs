//! Gets the broadcaster’s chat settings.
//! [`get-chat-settings`](https://dev.twitch.tv/docs/api/reference#get-chat-settings)
//!
//! # Accessing the endpoint
//!
//! ## Authorization
//!
//! Requires an App access token.
//! However, to include the [`non_moderator_chat_delay`](ChatSettings::non_moderator_chat_delay)
//! or [`non_moderator_chat_delay_duration`](ChatSettings::non_moderator_chat_delay_duration)
//! settings in the response, you must specify a User access token with scope set to
//! [`moderator:read:chat_settings`](twitch_oauth2::Scope::ModeratorReadChatSettings).
//!
//! ## Request: [GetChatSettingsRequest]
//!
//! To use this endpoint, construct a [`GetChatSettingsRequest`] with the [`GetChatSettingsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::{
//!     helix::{self, chat::get_chat_settings},
//!     types,
//! };
//! let request = get_chat_settings::GetChatSettingsRequest::builder()
//!     .broadcaster_id("1234567")
//!     // optional
//!     .moderator_id(types::UserIdRef::from_str("9876543"))
//!     .build();
//! ```
//!
//! ## Response: [ChatSettings]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::{helix::{self, chat::get_chat_settings}, types};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_chat_settings::GetChatSettingsRequest::builder()
//!     .broadcaster_id("1234567")
//!     // optional
//!     .moderator_id(types::UserIdRef::from_str("9876543"))
//!     .build();
//! let response: helix::chat::ChatSettings = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetEmoteSetsRequest::parse_response(None, &request.get_uri(), response)`](GetEmoteSetsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Chat Settings](super::get_chat_settings)
///
/// [`get-chat-settings`](https://dev.twitch.tv/docs/api/reference#get-chat-settings)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetChatSettingsRequest<'a> {
    /// The ID of the broadcaster whose chat settings you want to get.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Required only to access the [`non_moderator_chat_delay`](ChatSettings::non_moderator_chat_delay)
    /// or [`non_moderator_chat_delay_duration`](ChatSettings::non_moderator_chat_delay_duration) settings.
    /// If you want to access these settings, you need to provide a valid [`moderator_id`](Self::moderator_id)
    /// and a user token with the [`moderator:read:chat_settings`](twitch_oauth2::Scope::ModeratorReadChatSettings)
    /// scope.
    ///
    /// The ID of a user that has permission to moderate the broadcaster’s chat room.
    /// This ID must match the user ID associated with the user OAuth token.
    ///
    /// If the broadcaster wants to get their own settings (instead of having the moderator do it),
    /// set this parameter to the broadcaster’s ID, too.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(borrow)]
    pub moderator_id: Option<Cow<'a, types::UserIdRef>>,
}

impl<'a> GetChatSettingsRequest<'a> {
    /// Get chat settings for broadcasters channel
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
            moderator_id: None,
        }
    }

    /// The ID of a user that has permission to moderate the broadcaster’s chat room.
    ///
    /// Required only to access the [`non_moderator_chat_delay`](ChatSettings::non_moderator_chat_delay)
    /// or [`non_moderator_chat_delay_duration`](ChatSettings::non_moderator_chat_delay_duration) settings.
    pub fn moderator_id(
        mut self,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        self.moderator_id = Some(moderator_id.to_cow());
        self
    }
}

impl Request for GetChatSettingsRequest<'_> {
    type Response = ChatSettings;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorReadChatSettings];
    const PATH: &'static str = "chat/settings";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetChatSettingsRequest<'_> {
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
                let resp: helix::InnerResponse<[ChatSettings; 1]> =
                    helix::parse_json(response, true).map_err(|e| {
                        helix::HelixRequestGetError::DeserializeError(
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
                return Err(helix::HelixRequestGetError::InvalidResponse {
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
fn test_request_as_mod() {
    use helix::*;
    let req = GetChatSettingsRequest::broadcaster_id("1234").moderator_id("713936733");

    // Twitch's example is wrong,
    // they didn't include a moderator id in the request
    // but received `non_moderator_chat_delay`.
    // From twitch docs
    let data = br#"
    {
      "data": [
        {
          "broadcaster_id": "713936733",
          "slow_mode": false,
          "slow_mode_wait_time": null,
          "follower_mode": true,
          "follower_mode_duration": 0,
          "subscriber_mode": false,
          "emote_mode": false,
          "unique_chat_mode": false,
          "non_moderator_chat_delay": true,
          "non_moderator_chat_delay_duration": 4
        }
      ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/settings?broadcaster_id=1234&moderator_id=713936733"
    );

    dbg!(GetChatSettingsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_as_user() {
    use helix::*;
    let req = GetChatSettingsRequest::broadcaster_id("11148817");

    // From twitch docs
    let data = br#"
    {
      "data": [
        {
          "broadcaster_id": "11148817",
          "emote_mode": false,
          "follower_mode": false,
          "follower_mode_duration": null,
          "slow_mode": false,
          "slow_mode_wait_time": null,
          "subscriber_mode": false,
          "unique_chat_mode": false
        }
      ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/settings?broadcaster_id=11148817"
    );

    dbg!(GetChatSettingsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

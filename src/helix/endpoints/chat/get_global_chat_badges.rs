//! Gets a list of chat badges that can be used in chat for any channel.
//! [`get-global-chat-badges`](https://dev.twitch.tv/docs/api/reference#get-global-chat-badges)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetGlobalChatBadgesRequest]
//!
//! To use this endpoint, construct a [`GetGlobalChatBadgesRequest`] with the [`GetGlobalChatBadgesRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::get_global_chat_badges;
//! let request = get_global_chat_badges::GetGlobalChatBadgesRequest::new();
//! ```
//!
//! ## Response: [BadgeSet]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::get_global_chat_badges};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_global_chat_badges::GetGlobalChatBadgesRequest::new();
//! let response: Vec<helix::chat::BadgeSet> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetGlobalChatBadgesRequest::parse_response(None, &request.get_uri(), response)`](GetGlobalChatBadgesRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Global Chat Badges](super::get_global_chat_badges)
///
/// [`get-global-chat-badges`](https://dev.twitch.tv/docs/api/reference#get-global-chat-badges)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
#[must_use]
pub struct GetGlobalChatBadgesRequest {}

impl GetGlobalChatBadgesRequest {
    /// Get global chat badges
    pub fn new() -> Self { Self::default() }
}

/// Return Values for [Get Global Chat Badges](super::get_global_chat_badges)
///
/// [`get-global-chat-badges`](https://dev.twitch.tv/docs/api/reference#get-global-chat-badges)
pub type GetGlobalChatBadgesResponse = BadgeSet;

impl Request for GetGlobalChatBadgesRequest {
    type Response = Vec<GetGlobalChatBadgesResponse>;

    const PATH: &'static str = "chat/badges/global";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetGlobalChatBadgesRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetGlobalChatBadgesRequest::new();

    // From twitch docs
    // FIXME: Example has ...
    let data = br#"
    {
        "data": [
          {
            "set_id": "vip",
            "versions": [
              {
                "id": "1",
                "image_url_1x": "https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/1",
                "image_url_2x": "https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/2",
                "image_url_4x": "https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/3"
              }
            ]
          }
        ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/badges/global?"
    );

    dbg!(GetGlobalChatBadgesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

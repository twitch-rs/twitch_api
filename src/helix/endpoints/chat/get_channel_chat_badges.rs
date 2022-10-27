//! Gets a list of custom chat badges that can be used in chat for the specified channel. This includes subscriber badges and Bit badges.
//! [`get-channel-chat-badges`](https://dev.twitch.tv/docs/api/reference#get-channel-chat-badges)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetChannelChatBadgesRequest]
//!
//! To use this endpoint, construct a [`GetChannelChatBadgesRequest`] with the [`GetChannelChatBadgesRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::get_channel_chat_badges;
//! let request = get_channel_chat_badges::GetChannelChatBadgesRequest::builder()
//!     .broadcaster_id("1234".to_string())
//!     .build();
//! ```
//!
//! ## Response: [BadgeSet]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::get_channel_chat_badges};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_channel_chat_badges::GetChannelChatBadgesRequest::builder()
//!     .broadcaster_id("1234".to_string())
//!     .build();
//! let response: Vec<helix::chat::BadgeSet> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelChatBadgesRequest::parse_response(None, &request.get_uri(), response)`](GetChannelChatBadgesRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel Chat Badges](super::get_channel_chat_badges)
///
/// [`get-channel-chat-badges`](https://dev.twitch.tv/docs/api/reference#get-channel-chat-badges)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetChannelChatBadgesRequest<'a> {
    /// The broadcaster whose chat badges are being requested. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub broadcaster_id: &'a types::UserIdRef,
}

impl<'a> GetChannelChatBadgesRequest<'a> {
    /// Get chat badges for the specified broadcaster.
    pub fn broadcaster_id(broadcaster_id: impl Into<&'a types::UserIdRef>) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into(),
        }
    }
}

/// Return Values for [Get Channel Chat Badges](super::get_channel_chat_badges)
///
/// [`get-channel-chat-badges`](https://dev.twitch.tv/docs/api/reference#get-channel-chat-badges)
pub type GetChannelChatBadgesResponse = BadgeSet;

impl Request for GetChannelChatBadgesRequest<'_> {
    type Response = Vec<GetChannelChatBadgesResponse>;

    const PATH: &'static str = "chat/badges";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetChannelChatBadgesRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelChatBadgesRequest::broadcaster_id("135093069");

    // From twitch docs
    // FIXME: Example has ...
    let data = br#"
    {
        "data": [
          {
            "set_id": "bits",
            "versions": [
              {
                "id": "1",
                "image_url_1x": "https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/1",
                "image_url_2x": "https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/2",
                "image_url_4x": "https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/3"
              }
            ]
          },
          {
            "set_id": "subscriber",
            "versions": [
              {
                "id": "0",
                "image_url_1x": "https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/1",
                "image_url_2x": "https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/2",
                "image_url_4x": "https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/3"
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
        "https://api.twitch.tv/helix/chat/badges?broadcaster_id=135093069"
    );

    dbg!(GetChannelChatBadgesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

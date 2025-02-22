//! Gets all custom emotes for a specific Twitch channel including subscriber emotes, Bits tier emotes, and follower emotes.
//!
//! [`get-channel-emotes`](https://dev.twitch.tv/docs/api/reference#get-channel-emotes)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetChannelEmotesRequest]
//!
//! To use this endpoint, construct a [`GetChannelEmotesRequest`] with the [`GetChannelEmotesRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::get_channel_emotes;
//! let request =
//!     get_channel_emotes::GetChannelEmotesRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [ChannelEmote]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::get_channel_emotes};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_channel_emotes::GetChannelEmotesRequest::broadcaster_id("1234");
//! let response: Vec<helix::chat::ChannelEmote> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelEmotesRequest::parse_response(None, &request.get_uri(), response)`](GetChannelEmotesRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel Emotes](super::get_channel_emotes)
///
/// [`get-channel-emotes`](https://dev.twitch.tv/docs/api/reference#get-channel-emotes)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetChannelEmotesRequest<'a> {
    /// The broadcaster whose emotes are being requested.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetChannelEmotesRequest<'a> {
    /// Get emotes in a specific broadcasters channel.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Channel Emotes](super::get_channel_emotes)
///
/// [`get-channel-emotes`](https://dev.twitch.tv/docs/api/reference#get-channel-emotes)
pub type GetChannelEmotesResponse = ChannelEmote;

impl Request for GetChannelEmotesRequest<'_> {
    type Response = Vec<GetChannelEmotesResponse>;

    const PATH: &'static str = "chat/emotes";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetChannelEmotesRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelEmotesRequest::broadcaster_id("304456832");

    // From twitch docs
    // FIXME: Example has ... and is malformed, uses [] in images
    let data = br#"
    {
      "data": [
        {
          "id": "304456832",
          "name": "twitchdevPitchfork",
          "images": {
            "url_1x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0",
            "url_2x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0",
            "url_4x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0"
          },
          "tier": "1000",
          "emote_type": "subscriptions",
          "emote_set_id": "301590448",
          "format": [
            "static"
          ],
          "scale": [
            "1.0",
            "2.0",
            "3.0"
          ],
          "theme_mode": [
            "light",
            "dark"
          ]
        }
      ],
      "template": "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}"
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/emotes?broadcaster_id=304456832"
    );

    dbg!(GetChannelEmotesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

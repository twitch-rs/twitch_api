//! Gets a list of users who have editor permissions for a specific channel.
//! [`get-channel-editors`](https://dev.twitch.tv/docs/api/reference#get-channel-editors)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetChannelEditorsRequest]
//!
//! To use this endpoint, construct a [`GetChannelEditorsRequest`] with the [`GetChannelEditorsRequest::broadcaster_id()`] or [`GetChannelEditorsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::get_channel_editors;
//! let request =
//!     get_channel_editors::GetChannelEditorsRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [Editor]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::get_channel_editors};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_channel_editors::GetChannelEditorsRequest::broadcaster_id("1234");
//! let response: Vec<get_channel_editors::Editor> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelEditorsRequest::parse_response(None, &request.get_uri(), response)`](GetChannelEditorsRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel Editors](super::get_channel_editors)
///
/// [`get-channel-editors`](https://dev.twitch.tv/docs/api/reference#get-channel-editors)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetChannelEditorsRequest<'a> {
    /// Broadcaster’s user ID associated with the channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetChannelEditorsRequest<'a> {
    /// Get specified broadcasters channel editors
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
        }
    }
}

/// Return Values for [Get Channel Editors](super::get_channel_editors)
///
/// [`get-channel-editors`](https://dev.twitch.tv/docs/api/reference#get-channel-editors)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Editor {
    /// User ID of the editor.
    pub user_id: types::UserId,
    /// Display name of the editor.
    pub user_name: types::DisplayName,
    /// Date and time the editor was given editor permissions.
    pub created_at: types::Timestamp,
}

impl Request for GetChannelEditorsRequest<'_> {
    type Response = Vec<Editor>;

    const PATH: &'static str = "channels/editors";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadEditors];
}

impl RequestGet for GetChannelEditorsRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelEditorsRequest::broadcaster_id("44445592");

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "user_id": "182891647",
            "user_name": "mauerbac",
            "created_at": "2019-02-15T21:19:50.380833Z"
          },
          {
            "user_id": "135093069",
            "user_name": "BlueLava",
            "created_at": "2018-03-07T16:28:29.872937Z"
          }
        ]
      }
        "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/editors?broadcaster_id=44445592"
    );

    dbg!(GetChannelEditorsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

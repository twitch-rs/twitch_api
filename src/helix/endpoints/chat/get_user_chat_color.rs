//! Gets the color used for the user’s name in chat.
//! [`get-user-chat-color`](https://dev.twitch.tv/docs/api/reference#get-user-chat-color)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetUserChatColorRequest]
//!
//! To use this endpoint, construct a [`GetUserChatColorRequest`] with the [`GetUserChatColorRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::get_user_chat_color;
//! let request = get_user_chat_color::GetUserChatColorRequest::builder()
//!     .user_id(&["4321".into()][..])
//!     .build();
//! ```
//!
//! ## Response: [UserChatColor]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::get_user_chat_color};
//! # use twitch_api::{client, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let ids: &[&types::UserIdRef] = &["4321".into()];
//! let request = get_user_chat_color::GetUserChatColorRequest::builder()
//!     .user_id(ids)
//!     .build();
//! let response: Vec<helix::chat::UserChatColor> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUserChatColorRequest::parse_response(None, &request.get_uri(), response)`](GetUserChatColorRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Chatters](super::get_user_chat_color)
///
/// [`get-user-chat-color`](https://dev.twitch.tv/docs/api/reference#get-user-chat-color)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetUserChatColorRequest<'a> {
    /// The ID of the user whose color you want to get.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "Cow::Borrowed(&[])", setter(into))
    )]
    #[serde(borrow="'a")]
    pub user_id: Cow<'a, [&'a types::UserIdRef]>,
}

impl<'a> GetUserChatColorRequest<'a> {
    /// Get chat colors of specified users
    pub fn user_ids(user_ids: impl Into<Cow<'a, [&'a types::UserIdRef]>>) -> Self {
        Self {
            user_id: user_ids.into(),
        }
    }
}

/// Return Values for [Get Chatters](super::get_user_chat_color)
///
/// [`get-user-chat-color`](https://dev.twitch.tv/docs/api/reference#get-user-chat-color)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserChatColor {
    /// The ID of the user.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,
    /// The Hex color code that the user uses in chat for their name.
    ///
    /// If the user hasn’t specified a color in their settings, the value is [`None`].
    #[serde(
        default,
        deserialize_with = "helix::deserialize_none_from_empty_string"
    )]
    pub color: Option<types::HexColor>,
}

impl Request for GetUserChatColorRequest<'_> {
    type Response = Vec<UserChatColor>;

    const PATH: &'static str = "chat/color";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetUserChatColorRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let ids: &[&types::UserIdRef] = &["11111".into(), "44444".into()];
    let req = GetUserChatColorRequest::user_ids(ids);

    // From twitch docs
    // FIXME: Example has ...
    let data = br##"
    {
        "data": [
          {
            "user_id": "11111",
            "user_name": "SpeedySpeedster1",
            "user_login": "speedyspeedster1",
            "color": "#9146FF"
          },
          {
            "user_id": "44444",
            "user_name": "SpeedySpeedster2",
            "user_login": "speedyspeedster2",
            "color": ""
          }
        ]
      }
"##
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/color?user_id=11111&user_id=44444"
    );

    let resp =
        dbg!(GetUserChatColorRequest::parse_response(Some(req), &uri, http_response).unwrap());
    assert!(resp.data.get(1).unwrap().color.is_none())
}

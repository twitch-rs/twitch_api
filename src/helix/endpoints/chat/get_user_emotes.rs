//! Retrieves emotes available to the user across all channels.
//! [`get-user-emotes`](https://dev.twitch.tv/docs/api/reference/#get-user-emotes)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetUserEmotesRequest]
//!
//! To use this endpoint, construct a [`GetUserEmotesRequest`] with the [`GetUserEmotesRequest::user_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::get_user_emotes;
//! let request = get_user_emotes::GetUserEmotesRequest::user_id("1234");
//! ```
//!
//! ## Response: [UserEmote]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::get_user_emotes};
//! # use twitch_api::{client, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_user_emotes::GetUserEmotesRequest::user_id("1234");
//! let response: Vec<helix::chat::get_user_emotes::UserEmote> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUserEmotesRequest::parse_response(None, &request.get_uri(), response)`](GetUserEmotesRequest::parse_response)

use super::*;
use helix::{PaginationState, RequestGet};

/// Query Parameters for [Get User Emotes](super::get_user_emotes)
///
/// [`get-user-emotes`](https://dev.twitch.tv/docs/api/reference/#get-user-emotes)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetUserEmotesRequest<'a> {
    /// The ID of the user. This ID must match the user ID in the user access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// The cursor used to get the next page of results. The Pagination object in the response contains the cursor’s value.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// The User ID of a broadcaster you wish to get follower emotes of. Using this query parameter will guarantee inclusion of the broadcaster’s follower emotes in the response body.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Option<Cow<'a, types::UserIdRef>>,
}

impl<'a> GetUserEmotesRequest<'a> {
    /// Get emotes available to the user across all channels.
    pub fn user_id(user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: user_id.into_cow(),
            after: None,
            broadcaster_id: None,
        }
    }
}

impl helix::Paginated for GetUserEmotesRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

/// Return Values for [Get User Emotes](super::get_user_emotes)
///
/// [`get-user-emotes`](https://dev.twitch.tv/docs/api/reference/#get-user-emotes)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserEmote {
    /// Emote ID.
    pub id: types::EmoteId,
    /// Name of the emote a viewer types into Twitch chat for the image to appear.
    pub name: String,
    // FIXME: Enumify?
    /// The type of emote.
    pub emote_type: String,
    /// ID of the emote set the emote belongs to.
    pub emote_set_id: types::EmoteSetId,
    /// User ID of the broadcaster who owns the emote.
    pub owner_id: types::UserId,
    /// The formats that the emote is available in.
    pub format: Vec<types::EmoteAnimationSetting>,
    /// The sizes that the emote is available in.
    pub scale: Vec<types::EmoteScale>,
    /// The background themes that the emote is available in.
    pub theme_mode: Vec<types::EmoteThemeMode>,
}

impl UserEmote {
    /// Create an emote builder for this emote.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use twitch_api::helix::{self, chat::get_channel_emotes};
    /// use futures::TryStreamExt;
    /// use twitch_oauth2::TwitchToken;
    /// # use twitch_api::{client, types};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// let emotes: Vec<_> = client.get_user_emotes(token.user_id().unwrap(), &token).try_collect().await?;
    /// assert_eq!(emotes[0].url().size_3x().dark_mode().render(), "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_dc24652ada1e4c84a5e3ceebae4de709/default/dark/3.0");
    /// # Ok(())
    /// # }
    /// ```
    pub fn url(&self) -> types::EmoteUrlBuilder<'_> { EmoteUrlBuilder::new(&self.id) }
}

impl Request for GetUserEmotesRequest<'_> {
    type PaginationData = PaginationState<Self>;
    type Response = Vec<UserEmote>;

    const PATH: &'static str = "chat/emotes/user";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadEmotes];
}

impl RequestGet for GetUserEmotesRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;

    let req = GetUserEmotesRequest::user_id("123456");

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "emote_set_id": "",
            "emote_type": "hypetrain",
            "format": [
              "static"
            ],
            "id": "304420818",
            "name": "HypeLol",
            "owner_id": "477339272",
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
        "template": "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}",
        "pagination": {
          "cursor": "eyJiIjpudWxsLJxhIjoiIn0gf5"
        }
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/emotes/user?user_id=123456"
    );

    dbg!(GetUserEmotesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

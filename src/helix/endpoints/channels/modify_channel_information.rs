//! Modify channel information for users.
//! [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
//!
//! # Accessing the endpoint
//!
//! ## Request: [ModifyChannelInformationRequest]
//!
//! To use this endpoint, construct a [`ModifyChannelInformationRequest`] with the [`ModifyChannelInformationRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::modify_channel_information;
//! let request =
//!     modify_channel_information::ModifyChannelInformationRequest::broadcaster_id("1234");
//! ```
//!
//! ## Body: [ModifyChannelInformationBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::channels::modify_channel_information;
//! let mut body =
//!     modify_channel_information::ModifyChannelInformationBody::new();
//! body.title("Hello World!");
//! ```
//!
//! ## Response: [ModifyChannelInformation]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::modify_channel_information};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = modify_channel_information::ModifyChannelInformationRequest::broadcaster_id("1234");
//! let mut body = modify_channel_information::ModifyChannelInformationBody::new();
//! body.title("Hello World!");
//! let response: modify_channel_information::ModifyChannelInformation = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPatch::create_request)
//! and parse the [`http::Response`] with [`ModifyChannelInformationRequest::parse_response(None, &request.get_uri(), response)`](ModifyChannelInformationRequest::parse_response)
use super::*;
use helix::RequestPatch;

/// Query Parameters for [Modify Channel Information](super::modify_channel_information)
///
/// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ModifyChannelInformationRequest<'a> {
    /// ID of the channel
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> ModifyChannelInformationRequest<'a> {
    /// Modify specified broadcasters channel
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        ModifyChannelInformationRequest {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

// FIXME: Twitch docs sucks...
/// Body Parameters for [Modify Channel Information](super::modify_channel_information)
///
/// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ModifyChannelInformationBody<'a> {
    /// Current game ID being played on the channel. Use “0” or “” (an empty string) to unset the game.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub game_id: Option<Cow<'a, types::CategoryIdRef>>,
    /// Language of the channel
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_language: Option<Cow<'a, str>>,
    /// Title of the stream. Value must not be an empty string.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub title: Option<Cow<'a, str>>,
    /// The number of seconds you want your broadcast buffered before streaming it live.
    ///
    /// The delay helps ensure fairness during competitive play.
    /// Only users with Partner status may set this field. The maximum delay is 900 seconds (15 minutes).
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// A list of channel-defined tags to apply to the channel. To remove all tags from the channel, set tags to an empty array.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub tags: Option<Cow<'a, [&'a str]>>,
}

impl<'a> ModifyChannelInformationBody<'a> {
    /// Data to set on the stream.
    ///
    ///  # Examples
    ///
    /// ```rust
    /// # use twitch_api::helix::channels::modify_channel_information::ModifyChannelInformationBody;
    ///
    /// let body = ModifyChannelInformationBody::new().game_id("123");
    /// ```
    pub fn new() -> Self { Default::default() }

    /// Current game ID being played on the channel. Use “0” or “” (an empty string) to unset the game.
    pub fn game_id(
        &mut self,
        game_id: impl types::IntoCow<'a, types::CategoryIdRef> + 'a,
    ) -> &mut Self {
        self.game_id = Some(game_id.into_cow());
        self
    }

    /// Language of the channel
    pub fn broadcaster_language(
        &mut self,
        broadcaster_language: impl Into<Cow<'a, str>>,
    ) -> &mut Self {
        self.broadcaster_language = Some(broadcaster_language.into());
        self
    }

    /// Title of the stream. Value must not be an empty string.
    pub fn title(&mut self, title: impl Into<Cow<'a, str>>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    /// The number of seconds you want your broadcast buffered before streaming it live.
    pub fn delay(&mut self, delay: i32) -> &mut Self {
        self.delay = Some(delay);
        self
    }

    /// A list of channel-defined tags to apply to the channel. To remove all tags from the channel, set tags to an empty array.
    pub fn tags(&mut self, tags: &'a [&str]) -> &mut Self {
        self.tags = Some(tags.into());
        self
    }
}

impl helix::private::SealedSerialize for ModifyChannelInformationBody<'_> {}

/// Return Values for [Modify Channel Information](super::modify_channel_information)
///
/// [`modify-channel-information`](https://dev.twitch.tv/docs/api/reference#modify-channel-information)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum ModifyChannelInformation {
    /// 204 - Channel/Stream updated successfully
    Success,
}

impl Request for ModifyChannelInformationRequest<'_> {
    type Response = ModifyChannelInformation;

    const PATH: &'static str = "channels";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditBroadcast];
}

impl<'a> RequestPatch for ModifyChannelInformationRequest<'a> {
    type Body = ModifyChannelInformationBody<'a>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPatchError>
    where
        Self: Sized,
    {
        Ok(helix::Response {
            data: match status {
                http::StatusCode::NO_CONTENT | http::StatusCode::OK => {
                    ModifyChannelInformation::Success
                }
                _ => {
                    return Err(helix::HelixRequestPatchError::InvalidResponse {
                        reason: "unexpected status code",
                        response: response.to_string(),
                        status,
                        uri: uri.clone(),
                    })
                }
            },
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
    let req = ModifyChannelInformationRequest::broadcaster_id("0");

    let mut body = ModifyChannelInformationBody::new();
    body.title("Hello World!");

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels?broadcaster_id=0"
    );

    dbg!(ModifyChannelInformationRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

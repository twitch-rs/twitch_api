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
#[must_use]
#[non_exhaustive]
pub struct ModifyChannelInformationRequest<'a> {
    /// ID of the channel
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> ModifyChannelInformationRequest<'a> {
    /// Modify specified broadcasters channel
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::channels::ModifyChannelInformationRequest;
    ///
    /// let request = ModifyChannelInformationRequest::broadcaster_id("1337");
    /// ```
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
    /// Title of the stream. Value must not be an empty string.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub title: Option<Cow<'a, str>>,
    /// Language of the channel
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_language: Option<Cow<'a, str>>,
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
    /// List of labels that should be set as the Channel’s CCLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_classification_labels: Option<Cow<'a, [ContentClassificationLabel]>>,
    /// Boolean flag indicating if the channel has branded content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_branded_content: Option<bool>,
}

/// List of labels that should be set as the Channel’s CCLs.
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ContentClassificationLabel {
    /// Boolean flag indicating whether the label should be enabled (true) or disabled for the channel.
    pub is_enabled: bool,
    /// ID of the Content Classification Labels that must be added/removed from the channel.
    pub id: types::ContentClassificationId,
}

impl ContentClassificationLabel {
    /// Create a new [`ContentClassificationLabel`]
    pub const fn new(is_enabled: bool, id: types::ContentClassificationId) -> Self {
        Self { is_enabled, id }
    }
}

impl From<(bool, types::ContentClassificationId)> for ContentClassificationLabel {
    fn from(tup: (bool, types::ContentClassificationId)) -> Self {
        Self {
            is_enabled: tup.0,
            id: tup.1,
        }
    }
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

    /// List of labels that should be set as the Channel’s CCLs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use twitch_api::helix::channels::modify_channel_information::ModifyChannelInformationBody;
    /// use twitch_types;
    ///
    /// let mut body = ModifyChannelInformationBody::new();
    /// body.content_classification_labels(vec![
    ///     (true, twitch_types::ContentClassificationId::SexualThemes).into(),
    ///     (false, twitch_types::ContentClassificationId::ViolentGraphic).into(),
    /// ]);
    /// ```
    pub fn content_classification_labels(
        &mut self,
        content_classification_labels: impl Into<Cow<'a, [ContentClassificationLabel]>>,
    ) -> &mut Self {
        self.content_classification_labels = Some(content_classification_labels.into());
        self
    }

    /// Boolean flag indicating if the channel has branded content.
    pub fn is_branded_content(&mut self, is_branded_content: bool) -> &mut Self {
        self.is_branded_content = Some(is_branded_content);
        self
    }
}

#[test]
fn t() {
    let mut body = ModifyChannelInformationBody::new();
    body.content_classification_labels(vec![
        (true, types::ContentClassificationId::SexualThemes).into()
    ]);
    dbg!(body);
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
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelManageBroadcast];
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
        match status {
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(helix::Response::with_data(
                ModifyChannelInformation::Success,
                request,
            )),
            _ => Err(helix::HelixRequestPatchError::InvalidResponse {
                reason: "unexpected status code",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = ModifyChannelInformationRequest::broadcaster_id("41245072");

    let mut body = ModifyChannelInformationBody::new();
    body.game_id("33214");
    body.title("there are helicopters in the game? REASON TO PLAY FORTNITE found");
    body.broadcaster_language("en");
    body.tags(&["LevelingUp"]);

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"game_id":"33214","title":"there are helicopters in the game? REASON TO PLAY FORTNITE found","broadcaster_language":"en","tags":["LevelingUp"]}"#
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = vec![];

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels?broadcaster_id=41245072"
    );

    dbg!(ModifyChannelInformationRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

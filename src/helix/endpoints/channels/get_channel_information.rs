//! Gets channel information for users.
//! [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetChannelInformationRequest]
//!
//! To use this endpoint, construct a [`GetChannelInformationRequest`] with the [`GetChannelInformationRequest::broadcaster_ids`] or [`GetChannelInformationRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::get_channel_information;
//! let ids: &[&twitch_types::UserIdRef] = &["1234".into()];
//! let request =
//!     get_channel_information::GetChannelInformationRequest::broadcaster_ids(
//!         ids,
//!     );
//! ```
//!
//! ## Response: [ChannelInformation]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::get_channel_information};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let ids: &[&twitch_types::UserIdRef] = &["1234".into()];
//! let request = get_channel_information::GetChannelInformationRequest::broadcaster_ids(ids);
//! let response: Vec<get_channel_information::ChannelInformation> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelInformationRequest::parse_response(None, &request.get_uri(), response)`](GetChannelInformationRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel Information](super::get_channel_information)
///
/// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetChannelInformationRequest<'a> {
    /// ID of the channel
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub broadcaster_id: Cow<'a, [&'a types::UserIdRef]>,
}

impl<'a> GetChannelInformationRequest<'a> {
    /// Get channel information for a specific broadcaster.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::channels::get_channel_information;
    /// let ids: &[&twitch_types::UserIdRef] = &["1234".into()];
    /// let request =
    ///     get_channel_information::GetChannelInformationRequest::broadcaster_ids(
    ///         ids,
    ///     );
    /// ```
    pub fn broadcaster_ids(broadcaster_ids: impl Into<Cow<'a, [&'a types::UserIdRef]>>) -> Self {
        Self {
            broadcaster_id: broadcaster_ids.into(),
        }
    }
}

/// Return Values for [Get Channel Information](super::get_channel_information)
///
/// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelInformation {
    /// Twitch User ID of this channel owner
    pub broadcaster_id: types::UserId,
    /// Twitch User login of this channel owner
    pub broadcaster_login: types::UserName,
    /// Twitch user display name of this channel owner
    pub broadcaster_name: types::DisplayName,
    /// Current game ID being played on the channel
    pub game_id: types::CategoryId,
    /// Name of the game being played on the channel
    pub game_name: types::CategoryId,
    /// Language of the channel
    pub broadcaster_language: String,
    /// Title of the stream
    pub title: String,
    /// Description of the stream
    #[serde(default)]
    pub description: String,
    /// Stream delay in seconds
    ///
    /// # Notes
    ///
    /// This value may not be accurate, it'll only be accurate when the token belongs to the broadcaster and they are partnered.
    #[serde(default)]
    pub delay: i64,
    /// The tags applied to the channel.
    pub tags: Vec<String>,
    /// The [CCLs](https://blog.twitch.tv/en/2023/06/20/introducing-content-classification-labels/) applied to the channel.
    pub content_classification_labels: Vec<types::ContentClassificationId>,
    /// Boolean flag indicating if the channel has branded content.
    pub is_branded_content: bool,
}

impl Request for GetChannelInformationRequest<'_> {
    type Response = Vec<ChannelInformation>;

    const PATH: &'static str = "channels";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetChannelInformationRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let ids: &[&types::UserIdRef] = &["141981764".into()];
    let req = GetChannelInformationRequest::broadcaster_ids(ids);

    // From twitch docs
    let data = br#"
        {
          "data": [
            {
              "broadcaster_id": "141981764",
              "broadcaster_login": "twitchdev",
              "broadcaster_name": "TwitchDev",
              "broadcaster_language": "en",
              "game_id": "509670",
              "game_name": "Science & Technology",
              "title": "TwitchDev Monthly Update // May 6, 2021",
              "delay": 0,
              "tags": ["DevsInTheKnow"],
              "content_classification_labels": ["Gambling", "DrugsIntoxication", "MatureGame"],
              "is_branded_content": false
            }
          ]
        }
        "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels?broadcaster_id=141981764"
    );

    dbg!(GetChannelInformationRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

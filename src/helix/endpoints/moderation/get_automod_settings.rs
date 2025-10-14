//! Gets the broadcaster’s AutoMod settings.
//! [`get-automod-settings`](https://dev.twitch.tv/docs/api/reference/#get-automod-settings)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetAutoModSettingsRequest]
//!
//! To use this endpoint, construct a [`GetAutoModSettingsRequest`] with the [`GetAutoModSettingsRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::get_automod_settings;
//! let request =
//!     get_automod_settings::GetAutoModSettingsRequest::new("1234", "5678");
//! ```
//!
//! ## Response: [AutoModSettings]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::get_automod_settings};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_automod_settings::GetAutoModSettingsRequest::new("1234", "5678");
//! let response: helix::moderation::AutoModSettings = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetAutoModSettingsRequest::parse_response(None, &request.get_uri(), response)`](GetAutoModSettingsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get AutoMod Settings](super::get_automod_settings)
///
/// [`get-automod-settings`](https://dev.twitch.tv/docs/api/reference/#get-automod-settings)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetAutoModSettingsRequest<'a> {
    /// The ID of the broadcaster whose AutoMod settings you want to get.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID in the user access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetAutoModSettingsRequest<'a> {
    /// Get AutoMod settings in a broadcasters channel as specified moderator
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
        }
    }
}

/// Return Values for [Get AutoMod Settings](super::get_automod_settings)
///
/// [`get-automod-settings`](https://dev.twitch.tv/docs/api/reference/#get-automod-settings)
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutoModSettings {
    /// The broadcaster’s ID.
    pub broadcaster_id: types::UserId,
    /// The moderator’s ID.
    pub moderator_id: types::UserId,
    /// The default AutoMod level for the broadcaster. This field is [None] if the broadcaster has set one or more of the individual settings.
    pub overall_level: Option<u8>,
    /// The Automod level for discrimination against disability.
    pub disability: u8,
    /// The Automod level for hostility involving aggression.
    pub aggression: u8,
    /// The AutoMod level for discrimination based on sexuality, sex, or gender.
    pub sexuality_sex_or_gender: u8,
    /// The Automod level for discrimination against women.
    pub misogyny: u8,
    /// The Automod level for hostility involving name calling or insults.
    pub bullying: u8,
    /// The Automod level for profanity.
    pub swearing: u8,
    /// The Automod level for racial discrimination.
    pub race_ethnicity_or_religion: u8,
    /// The Automod level for sexual content.
    pub sex_based_terms: u8,
}

impl Request for GetAutoModSettingsRequest<'_> {
    type PaginationData = ();
    type Response = AutoModSettings;

    const PATH: &'static str = "moderation/automod/settings";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ModeratorReadAutomodSettings,
        twitch_oauth2::Scope::ModeratorManageAutomodSettings
    )];
}

impl RequestGet for GetAutoModSettingsRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        helix::parse_single_return(request, uri, response, status)
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetAutoModSettingsRequest::new("1234", "5678");

    let data = br#"
    {
        "data": [
            {
                "broadcaster_id": "1234",
                "moderator_id": "5678",
                "overall_level": null,
                "disability": 0,
                "aggression": 0,
                "sexuality_sex_or_gender": 0,
                "misogyny": 0,
                "bullying": 0,
                "swearing": 0,
                "race_ethnicity_or_religion": 0,
                "sex_based_terms": 0
            }
        ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/automod/settings?broadcaster_id=1234&moderator_id=5678"
    );

    let res = GetAutoModSettingsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.overall_level, None);
    assert_eq!(res.disability, 0);
    assert_eq!(res.broadcaster_id.as_str(), "1234");
    assert_eq!(res.moderator_id.as_str(), "5678");
}

#[cfg(test)]
#[test]
fn test_request_with_overall() {
    use helix::*;
    let req = GetAutoModSettingsRequest::new("1234", "5678");

    let data = br#"
    {
        "data": [
            {
                "aggression": 1,
                "broadcaster_id": "1234",
                "bullying": 0,
                "disability": 0,
                "misogyny": 0,
                "moderator_id": "5678",
                "overall_level": 1,
                "race_ethnicity_or_religion": 1,
                "sex_based_terms": 0,
                "sexuality_sex_or_gender": 1,
                "swearing": 0
            }
        ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/automod/settings?broadcaster_id=1234&moderator_id=5678"
    );

    let res = GetAutoModSettingsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.overall_level, Some(1));
    assert_eq!(res.aggression, 1);
    assert_eq!(res.disability, 0);
    assert_eq!(res.broadcaster_id.as_str(), "1234");
    assert_eq!(res.moderator_id.as_str(), "5678");
}

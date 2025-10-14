//! Updates the broadcaster’s AutoMod settings.
//!
//! [`update-automod-settings`](https://dev.twitch.tv/docs/api/reference#update-automod-settings)
//! The settings are used to automatically block inappropriate or harassing messages from appearing in the broadcaster’s chat room.
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateAutoModSettingsRequest]
//!
//! To use this endpoint, construct an [`UpdateAutoModSettingsRequest`] with the [`UpdateAutoModSettingsRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::update_automod_settings;
//! let request = update_automod_settings::UpdateAutoModSettingsRequest::new(
//!     "123", "456",
//! );
//! ```
//!
//! ## Body: [UpdateAutoModSettingsBody]
//!
//! We also need to provide a body to the request.
//!
//! ```
//! # use twitch_api::helix::moderation::update_automod_settings;
//! // Set the overall level to 3
//! let body = update_automod_settings::UpdateAutoModSettingsBody::overall(3);
//! ```
//!
//!
//! ## Response: [AutoModSettings]
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::update_automod_settings};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_automod_settings::UpdateAutoModSettingsRequest::new(
//!     "123",
//!     "456"
//! );
//! let body =
//!     update_automod_settings::UpdateAutoModSettingsBody::overall(3);
//! let response: helix::moderation::AutoModSettings = client.req_put(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPut::create_request)
//! and parse the [`http::Response`] with [`UpdateAutoModSettingsRequest::parse_response(None, &request.get_uri(), response)`](UpdateAutoModSettingsRequest::parse_response)

use super::*;
use helix::RequestPut;

pub use super::AutoModSettings;

/// Query Parameters for [Update AutoMod Settings](super::update_automod_settings)
///
/// [`update-automod-settings`](https://dev.twitch.tv/docs/api/reference#update-automod-settings)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateAutoModSettingsRequest<'a> {
    /// The ID of the broadcaster whose AutoMod settings you want to update.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID in the user access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> UpdateAutoModSettingsRequest<'a> {
    /// Update the AutoMod settings on the specified channel as the specified moderator
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

/// Body Parameters for [Update AutoMod Settings](super::update_automod_settings)
///
/// [`update-automod-settings`](https://dev.twitch.tv/docs/api/reference#update-automod-settings)
///
/// Because PUT is an overwrite operation, you must include all the fields that you want set after the operation completes.
/// Typically, you’ll send a GET request, update the fields you want to change, and pass that object in the PUT request.
///
/// You may set either `overall_level` (`Overall`) or the individual settings like `aggression` (`Individual`), but not both.
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum UpdateAutoModSettingsBody {
    /// Set the `overall_level`
    ///
    /// Setting `overall_level` applies default values to the individual settings.
    /// However, setting `overall_level` to 4 does not necessarily mean that it applies 4 to all the individual settings.
    /// Instead, it applies a set of recommended defaults to the rest of the settings.
    #[non_exhaustive]
    Overall {
        /// The default AutoMod level for the broadcaster.
        overall_level: u8,
    },
    /// Set the individual levels for each setting
    Individual(UpdateAutoModSettingsIndividual),
}

/// Set the individual levels for each setting
///
/// Note that because PUT is an overwrite operation, you must include all the fields that you want set after the operation completes.
/// Use [from_settings](Self::from_settings) to initialize this struct to previously returned [AutoModSettings].
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateAutoModSettingsIndividual {
    /// The Automod level for hostility involving aggression.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggression: Option<u8>,
    /// The Automod level for hostility involving name calling or insults.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullying: Option<u8>,
    /// The Automod level for discrimination against disability.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disability: Option<u8>,
    /// The Automod level for discrimination against women.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misogyny: Option<u8>,
    /// The Automod level for racial discrimination.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_ethnicity_or_religion: Option<u8>,
    /// The Automod level for sexual content.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex_based_terms: Option<u8>,
    /// The AutoMod level for discrimination based on sexuality, sex, or gender.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexuality_sex_or_gender: Option<u8>,
    /// The Automod level for profanity.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swearing: Option<u8>,
}

impl helix::private::SealedSerialize for UpdateAutoModSettingsBody {}

impl UpdateAutoModSettingsBody {
    /// Set the `overall_level`
    pub const fn overall(overall_level: u8) -> Self { Self::Overall { overall_level } }

    /// Constructs an `Individual` from [AutoModSettings]
    pub const fn from_settings(settings: &AutoModSettings) -> Self {
        Self::Individual(UpdateAutoModSettingsIndividual::from_settings(settings))
    }
}

impl UpdateAutoModSettingsIndividual {
    /// Constructs an update on individual settings from [AutoModSettings]
    pub const fn from_settings(settings: &AutoModSettings) -> Self {
        Self {
            aggression: Some(settings.aggression),
            bullying: Some(settings.bullying),
            disability: Some(settings.disability),
            misogyny: Some(settings.misogyny),
            race_ethnicity_or_religion: Some(settings.race_ethnicity_or_religion),
            sex_based_terms: Some(settings.sex_based_terms),
            sexuality_sex_or_gender: Some(settings.sexuality_sex_or_gender),
            swearing: Some(settings.swearing),
        }
    }
}

impl Request for UpdateAutoModSettingsRequest<'_> {
    type PaginationData = ();
    type Response = super::AutoModSettings;

    const PATH: &'static str = "moderation/automod/settings";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageAutomodSettings];
}

impl RequestPut for UpdateAutoModSettingsRequest<'_> {
    type Body = UpdateAutoModSettingsBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestPutError>
    where
        Self: Sized,
    {
        helix::parse_single_return(request, uri, response, status)
    }
}

#[cfg(test)]
#[test]
fn test_request_overall() {
    use helix::*;
    let req = UpdateAutoModSettingsRequest::new("1234", "5678");
    let body = UpdateAutoModSettingsBody::overall(3);

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"overall_level":3}"#
    );

    req.create_request(body, "token", "clientid").unwrap();

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "broadcaster_id": "1234",
                "moderator_id": "5678",
                "overall_level": 3,
                "disability": 3,
                "aggression": 3,
                "sexuality_sex_or_gender": 3,
                "misogyny": 3,
                "bullying": 2,
                "swearing": 0,
                "race_ethnicity_or_religion": 3,
                "sex_based_terms": 3
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/automod/settings?broadcaster_id=1234&moderator_id=5678"
    );

    let res = UpdateAutoModSettingsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.overall_level, Some(3));
    assert_eq!(res.disability, 3);
}

#[cfg(test)]
#[test]
fn test_request_individual() {
    use helix::*;
    let req = UpdateAutoModSettingsRequest::new("1234", "5678");
    let body = UpdateAutoModSettingsBody::Individual(UpdateAutoModSettingsIndividual {
        aggression: Some(0),
        bullying: Some(1),
        disability: None,
        misogyny: None,
        race_ethnicity_or_religion: None,
        sex_based_terms: None,
        sexuality_sex_or_gender: None,
        swearing: Some(2),
    });

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"aggression":0,"bullying":1,"swearing":2}"#
    );

    req.create_request(body, "token", "clientid").unwrap();

    let data = br#"
    {
        "data": [
            {
                "aggression": 0,
                "broadcaster_id": "1234",
                "bullying": 1,
                "disability": 0,
                "misogyny": 0,
                "moderator_id": "5678",
                "overall_level": null,
                "race_ethnicity_or_religion": 0,
                "sex_based_terms": 0,
                "sexuality_sex_or_gender": 0,
                "swearing": 2
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/automod/settings?broadcaster_id=1234&moderator_id=5678"
    );

    let res = UpdateAutoModSettingsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.overall_level, None);
    assert_eq!(res.swearing, 2);
}

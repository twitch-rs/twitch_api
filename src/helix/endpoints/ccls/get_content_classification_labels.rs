//! Gets information about Twitch content classification labels.
//! [`get-content-classification-labels`](https://dev.twitch.tv/docs/api/reference#get-content-classification-labels)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetContentClassificationLabelsRequest]
//!
//! To use this endpoint, construct a [`GetContentClassificationLabelsRequest`] with the [`GetContentClassificationLabelsRequest::new()`] or [`GetContentClassificationLabelsRequest::locale()`] methods.
//!
//! ```rust, no_run
//! use twitch_api::helix::ccls::get_content_classification_labels;
//! let request =
//!     get_content_classification_labels::GetContentClassificationLabelsRequest::new();
//! // Get content classification labels for a specific locale
//! let request = get_content_classification_labels::GetContentClassificationLabelsRequest::locale("es-MX");
//! ```
//!
//! ## Response: [ContentClassificationLabel]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, ccls::get_content_classification_labels};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_content_classification_labels::GetContentClassificationLabelsRequest::new();
//! let response: Vec<get_content_classification_labels::ContentClassificationLabel> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetContentClassificationLabelsRequest::parse_response(None, &request.get_uri(), response)`](GetContentClassificationLabelsRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Content Classification Labels](super::get_content_classification_labels)
///
/// [`get-content-classification-labels`](https://dev.twitch.tv/docs/api/reference#get-content-classification-labels)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetContentClassificationLabelsRequest<'a> {
    /// Locale for the Content Classification Labels. You may specify a maximum of 1 locale. Default: `"en-US"`
    ///
    /// Supported locales: `"bg-BG", "cs-CZ", "da-DK", "da-DK", "de-DE", "el-GR", "en-GB", "en-US", "es-ES", "es-MX", "fi-FI", "fr-FR", "hu-HU", "it-IT", "ja-JP", "ko-KR", "nl-NL", "no-NO", "pl-PL", "pt-BT", "pt-PT", "ro-RO", "ru-RU", "sk-SK", "sv-SE", "th-TH", "tr-TR", "vi-VN", "zh-CN", "zh-TW"`
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub locale: Option<Cow<'a, str>>,
}

impl<'a> GetContentClassificationLabelsRequest<'a> {
    /// Request content classification labels for some locale
    pub fn locale(locale: impl Into<Cow<'a, str>>) -> Self {
        Self {
            locale: Some(locale.into()),
        }
    }

    /// Returns an new [`GetContentClassificationLabelsRequest`]
    pub fn new() -> Self { Self::default() }
}

/// Return Values for [Get Content Classification Labels](super::get_content_classification_labels)
///
/// [`get-content-classification-labels`](https://dev.twitch.tv/docs/api/reference#get-content-classification-labels)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ContentClassificationLabel {
    /// Unique identifier for the CCL.
    pub id: types::ContentClassificationId,
    /// Localized description of the CCL.
    pub description: String,
    /// Localized name of the CCL.
    pub name: String,
}

impl Request for GetContentClassificationLabelsRequest<'_> {
    type Response = Vec<ContentClassificationLabel>;

    const PATH: &'static str = "content_classification_labels";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetContentClassificationLabelsRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetContentClassificationLabelsRequest::new();

    // From api call
    let data = br#"
    {
      "data": [
        {
          "description": "Discussions or debates about politics or sensitive social issues such as elections, civic integrity, military conflict, and civil rights in a polarizing manner.",
          "id": "DebatedSocialIssuesAndPolitics",
          "name": "Politics and Sensitive Social Issues"
        },
        {
          "description": "Excessive tobacco glorification or promotion, any marijuana consumption/use, legal drug and alcohol induced intoxication, discussions of illegal drugs.",
          "id": "DrugsIntoxication",
          "name": "Drugs, Intoxication, or Excessive Tobacco Use"
        },
        {
          "description": "Participating in online or in-person gambling, poker or fantasy sports, that involve the exchange of real money.",
          "id": "Gambling",
          "name": "Gambling"
        },
        {
          "description": "Games that are rated Mature or less suitable for a younger audience.",
          "id": "MatureGame",
          "name": "Mature-rated game"
        },
        {
          "description": "Prolonged, and repeated use of obscenities, profanities, and vulgarities, especially as a regular part of speech.",
          "id": "ProfanityVulgarity",
          "name": "Significant Profanity or Vulgarity"
        },
        {
          "description": "Content that focuses on sexualized physical attributes and activities, sexual topics, or experiences.",
          "id": "SexualThemes",
          "name": "Sexual Themes"
        },
        {
          "description": "Simulations and/or depictions of realistic violence, gore, extreme injury, or death.",
          "id": "ViolentGraphic",
          "name": "Violent and Graphic Depictions"
        }
      ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/content_classification_labels?"
    );

    let res = GetContentClassificationLabelsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;

    assert_eq!(res.len(), 7);

    assert_eq!(res[0].description, "Discussions or debates about politics or sensitive social issues such as elections, civic integrity, military conflict, and civil rights in a polarizing manner.");
    assert_eq!(res[0].name, "Politics and Sensitive Social Issues");

    assert_eq!(
        res[0].id,
        types::ContentClassificationId::DebatedSocialIssuesAndPolitics
    );
    assert_eq!(res[1].id, types::ContentClassificationId::DrugsIntoxication);
    assert_eq!(res[2].id, types::ContentClassificationId::Gambling);
    assert_eq!(res[3].id, types::ContentClassificationId::MatureGame);
    assert_eq!(
        res[4].id,
        types::ContentClassificationId::ProfanityVulgarity
    );
    assert_eq!(res[5].id, types::ContentClassificationId::SexualThemes);
    assert_eq!(res[6].id, types::ContentClassificationId::ViolentGraphic);
}

#[cfg(test)]
#[test]
fn test_request_locale() {
    use helix::*;
    let req = GetContentClassificationLabelsRequest::locale("th-TH");

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/content_classification_labels?locale=th-TH"
    );
}

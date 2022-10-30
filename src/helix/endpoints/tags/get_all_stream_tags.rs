//! Gets the list of all stream tags defined by Twitch, optionally filtered by tag ID(s).
//! [`get-all-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-all-stream-tags)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetAllStreamTagsRequest]
//!
//! To use this endpoint, construct a [`GetAllStreamTagsRequest`] with the [`GetAllStreamTagsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::tags::get_all_stream_tags;
//! let request = get_all_stream_tags::GetAllStreamTagsRequest::builder()
//!     .first(100)
//!     .build();
//! ```
//!
//! ## Response: [Tag](helix::tags::TwitchTag)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, tags::get_all_stream_tags};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_all_stream_tags::GetAllStreamTagsRequest::builder()
//!     .build();
//! let response: Vec<get_all_stream_tags::Tag> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetAllStreamTagsRequest::parse_response(None, &request.get_uri(), response)`](GetAllStreamTagsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get All Stream Tags](super::get_all_stream_tags)
///
/// [`get-all-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-all-stream-tags)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetAllStreamTagsRequest<'a> {
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// ID of a tag. Multiple IDs can be specified. If provided, only the specified tag(s) is(are) returned. Maximum of 100.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(borrow)]
    pub tag_id: Cow<'a, [&'a types::TagIdRef]>,
}

impl<'a> GetAllStreamTagsRequest<'a> {
    /// Filter the results for specific tag.
    pub fn tag_ids(mut self, tag_ids: impl Into<Cow<'a, [&'a types::TagIdRef]>>) -> Self {
        self.tag_id = tag_ids.into();
        self
    }
}

impl Default for GetAllStreamTagsRequest<'_> {
    fn default() -> Self {
        Self {
            after: None,
            first: None,
            tag_id: Cow::Borrowed(&[]),
        }
    }
}

/// Return Values for [Get All Stream Tags](super::get_all_stream_tags)
///
/// [`get-all-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-all-stream-tags)
pub type Tag = helix::tags::TwitchTag;

impl Request for GetAllStreamTagsRequest<'_> {
    type Response = Vec<Tag>;

    const PATH: &'static str = "tags/streams";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetAllStreamTagsRequest<'_> {}

impl helix::Paginated for GetAllStreamTagsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetAllStreamTagsRequest {
        first: Some(3),
        ..GetAllStreamTagsRequest::default()
    };

    // From twitch docs.
    let data = "\
{\n\
    \"data\": [\n\
        {\n\
            \"tag_id\": \"621fb5bf-5498-4d8f-b4ac-db4d40d401bf\",\n\
            \"is_auto\": false,\n\
            \"localization_names\": {\n\
                \"bg-bg\": \"Завършване без продължаване\",\n\
                \"cs-cz\": \"Na jeden z&aacute;tah\",\n\
                \"da-dk\": \"1 Continue klaret\",\n\
                \"de-de\": \"Mit nur 1 Leben\",\n\
                \"el-gr\": \"1 χωρίς συνέχεια\",\n\
                \"en-us\": \"1 Credit Clear\"\n\
            },\n\
            \"localization_descriptions\": {\n\
                \"bg-bg\": \"За потоци с акцент върху завършване на аркадна игра с монети, в която не се използва продължаване\",\n\
                \"cs-cz\": \"Pro vys&iacute;l&aacute;n&iacute; s důrazem na plněn&iacute; mincov&yacute;ch ark&aacute;dov&yacute;ch her bez použit&iacute; pokračov&aacute;n&iacute;.\",\n\
                \"da-dk\": \"Til streams med v&aelig;gt p&aring; at gennemf&oslash;re et arkadespil uden at bruge continues\",\n\
                \"de-de\": \"F&uuml;r Streams mit dem Ziel, ein Coin-op-Arcade-Game mit nur einem Leben abzuschlie&szlig;en.\",\n\
                \"el-gr\": \"Για μεταδόσεις με έμφαση στην ολοκλήρωση παλαιού τύπου ηλεκτρονικών παιχνιδιών που λειτουργούν με κέρμα, χωρίς να χρησιμοποιούν συνέχειες\",\n\
                \"en-us\": \"For streams with an emphasis on completing a coin-op arcade game without using any continues\"\n\
            }\n\
        },\n\
        {\n\
            \"tag_id\": \"7b49f69a-5d95-4c94-b7e3-66e2c0c6f6c6\",\n\
            \"is_auto\": false,\n\
            \"localization_names\": {\n\
                \"bg-bg\": \"Дизайн\",\n\
                \"cs-cz\": \"Design\",\n\
                \"da-dk\": \"Design\",\n\
                \"de-de\": \"Design\",\n\
                \"el-gr\": \"Σχέδιο\",\n\
                \"en-us\": \"Design\"\n\
            },\n\
            \"localization_descriptions\": {\n\
                \"en-us\": \"For streams with an emphasis on the creative process of designing an object or system\"\n\
            }\n\
        },\n\
        {\n\
            \"tag_id\": \"1c628b75-b1c3-4a2f-9d1d-056c1f555f0e\",\n\
            \"is_auto\": true,\n\
            \"localization_names\": {\n\
                \"bg-bg\": \"Ð¨Ð°Ð¼Ð¿Ð¸Ð¾Ð½: Lux\",\n\
                \"cs-cz\": \"Å ampion: Lux\",\n\
                \"da-dk\": \"Champion: Lux\"\n\
            },\n\
            \"localization_descriptions\": {\n\
                \"en-us\": \"For streams featuring the champion Lux in League of Legends\"\n\
            }\n\
        }\n\
    ],\n\
    \"pagination\": {\n\
        \"cursor\": \"eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6ImV5SnBaQ0k2ZXlKQ0lqcHVkV3hzTENKQ1QwOU1JanB1ZFd4c0xDS kNVeUk2Ym5Wc2JDd2lUQ0k2Ym5Wc2JDd2lUU0k2Ym5Wc2JDd2lUaUk2Ym5Wc2JDd2lUbE1pT201MWJHd3NJazV WVEV3aU9tNTFiR3dzSWxNaU9pSXhZell5T0dJM05TMWlNV016TFRSaE1tWXRPV1F4WkMwd05UWmpNV1kxTlRWb U1HVWlMQ0pUVXlJNmJuVnNiSDE5In19\"\n\
    }\n\
}\n\
"
        .as_bytes().to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/tags/streams?first=3"
    );

    dbg!(GetAllStreamTagsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

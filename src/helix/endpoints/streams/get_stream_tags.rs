//! Gets the list of tags for a specified stream (channel).
//! [`get-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-stream-tags)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetStreamTagsRequest]
//!
//! To use this endpoint, construct a [`GetStreamTagsRequest`] with the [`GetStreamTagsRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::streams::get_stream_tags;
//! let request = get_stream_tags::GetStreamTagsRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [Tag](helix::tags::TwitchTag)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, streams::get_stream_tags};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_stream_tags::GetStreamTagsRequest::broadcaster_id("1234");
//! let response: Vec<get_stream_tags::Tag> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetStreamTagsRequest::parse_response(None, &request.get_uri(), response)`](GetStreamTagsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Stream Tags](super::get_stream_tags)
///
/// [`get-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-stream-tags)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetStreamTagsRequest<'a> {
    // FIXME: twitch docs sucks
    /// ID of the stream whose tags are going to be fetched
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetStreamTagsRequest<'a> {
    /// ID of the stream whose tags are going to be fetched
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Stream Tags](super::get_stream_tags)
///
/// [`get-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-stream-tags)
pub type Tag = helix::tags::TwitchTag;

impl Request for GetStreamTagsRequest<'_> {
    type Response = Vec<Tag>;

    const PATH: &'static str = "streams/tags";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetStreamTagsRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetStreamTagsRequest::broadcaster_id("198704263");

    // From twitch docs
    let data = "\
{\n\
    \"data\": [\n\
        {\n\
            \"tag_id\": \"621fb5bf-5498-4d8f-b4ac-db4d40d401bf\",\n\
            \"is_auto\": false,\n\
            \"localization_names\": {\n\
                \"bg-bg\": \"Завършване без продължаване\",\n\
                \"cs-cz\": \"Na jeden z&aacute;tah\",\n\
                \"da-dk\": \"Continue klaret\"\n\
            },\n\
            \"localization_descriptions\": {\n\
                \"bg-bg\": \"За потоци с акцент върху завършване на аркадна игра с монети, в която не се използва продължаване\",\n\
                \"cs-cz\": \"Pro vys&iacute;l&aacute;n&iacute; s důrazem na plněn&iacute; mincov&yacute;ch ark&aacute;dov&yacute;ch her bez použit&iacute; pokračov&aacute;n&iacute;.\",\n\
                \"da-dk\": \"Til streams med v&aelig;gt p&aring; at gennemf&oslash;re et arkadespil uden at bruge continues\"\n\
            }\n\
        },\n\
        {\n\
            \"tag_id\": \"79977fb9-f106-4a87-a386-f1b0f99783dd\",\n\
            \"is_auto\": false,\n\
            \"localization_names\": {\n\
                \"bg-bg\": \"PvE\",\n\
                \"cs-cz\": \"PvE\"\n\
            },\n\
            \"localization_descriptions\": {\n\
                \"bg-bg\": \"За потоци с акцент върху PVE геймплей\",\n\
                \"cs-cz\": \"Pro vys&iacute;l&aacute;n&iacute; s důrazem na hratelnost \\\"hr&aacute;č vs. prostřed&iacute;\\\".\",\n\
                \"da-dk\": \"Til streams med v&aelig;gt p&aring; spil, hvor det er spilleren mod omgivelserne.\"\n\
            }\n\
        }\n\
    ]\n\
}\n\
"
        .as_bytes().to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/streams/tags?broadcaster_id=198704263"
    );

    dbg!(GetStreamTagsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

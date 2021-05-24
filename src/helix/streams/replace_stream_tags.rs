//! Applies specified tags to a specified stream, overwriting any existing tags applied to that stream. If no tags are specified, all tags previously applied to the stream are removed. Automated tags are not affected by this operation.
//! [`replace-stream-tags`](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
//!
//! # Accessing the endpoint
//!
//! ## Request: [ReplaceStreamTagsRequest]
//!
//! To use this endpoint, construct a [`ReplaceStreamTagsRequest`] with the [`ReplaceStreamTagsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::streams::replace_stream_tags;
//! let request = replace_stream_tags::ReplaceStreamTagsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Body: [ReplaceStreamTagsBody]
//!
//! We also need to provide a body to the request containing the tags we want to set.
//!
//! ```
//! # use twitch_api2::helix::streams::replace_stream_tags;
//! let body = replace_stream_tags::ReplaceStreamTagsBody::builder()
//!     .tag_ids(vec![
//!         "621fb5bf-5498-4d8f-b4ac-db4d40d401bf".to_string(),
//!         "79977fb9-f106-4a87-a386-f1b0f99783dd".to_string(),
//!     ])
//!     .build();
//! ```
//!
//! ## Response: [ReplaceStreamTags]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_put()`](helix::HelixClient::req_put).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, streams::replace_stream_tags};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = replace_stream_tags::ReplaceStreamTagsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let body = replace_stream_tags::ReplaceStreamTagsBody::builder()
//!     .tag_ids(vec![
//!         "621fb5bf-5498-4d8f-b4ac-db4d40d401bf".to_string(),
//!         "79977fb9-f106-4a87-a386-f1b0f99783dd".to_string(),
//!     ])
//!     .build();
//! let response: replace_stream_tags::ReplaceStreamTags = client.req_put(request, body, &token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPut::create_request)
//! and parse the [`http::Response`] with [`ReplaceStreamTagsRequest::parse_response(&request.get_uri(), response)`](ReplaceStreamTagsRequest::parse_response)
use super::*;
use helix::RequestPut;

/// Query Parameters for [Replace Stream Tags](super::replace_stream_tags)
///
/// [`replace-stream-tags`](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ReplaceStreamTagsRequest {
    /// ID of the stream for which tags are to be replaced.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
}

/// Body Parameters for [Replace Stream Tags](super::replace_stream_tags)
///
/// [`replace-stream-tags`](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
///
/// # Notes
///
/// Up to five tags can be applied to a stream. If no `tag_ids` is provided, all tags are removed from the stream.
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ReplaceStreamTagsBody {
    /// IDs of tags to be applied to the stream.
    #[builder(default, setter(into))]
    pub tag_ids: Vec<types::CategoryId>,
}
/// Return Values for [Replace Stream Tags](super::replace_stream_tags)
///
/// [`replace-stream-tags`](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum ReplaceStreamTags {
    /// 204 - Stream Tags replaced successfully
    Success,
    /// Internal Server Error; Failed to replace tags
    InternalServerError,
}

impl helix::private::SealedSerialize for ReplaceStreamTagsBody {}

impl Request for ReplaceStreamTagsRequest {
    type Response = ReplaceStreamTags;

    const PATH: &'static str = "streams/tags";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageBroadcast];
}

impl RequestPut for ReplaceStreamTagsRequest {
    type Body = ReplaceStreamTagsBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPutError>
    where
        Self: Sized,
    {
        match status {
            // FIXME: I've seen OK as the status code
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(helix::Response {
                data: ReplaceStreamTags::Success,
                pagination: None,
                request,
            }),
            _ => Err(helix::HelixRequestPutError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[test]
fn test_request() {
    use helix::*;
    let req = ReplaceStreamTagsRequest::builder()
        .broadcaster_id("0")
        .build();

    let body = ReplaceStreamTagsBody::builder()
        .tag_ids(vec![
            "621fb5bf-5498-4d8f-b4ac-db4d40d401bf".to_string(),
            "79977fb9-f106-4a87-a386-f1b0f99783dd".to_string(),
        ])
        .build();

    dbg!(req.create_request(body, "token", "clientid").unwrap());
    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/streams/tags?broadcaster_id=0"
    );

    dbg!(ReplaceStreamTagsRequest::parse_response(&uri, http_response).unwrap());
}

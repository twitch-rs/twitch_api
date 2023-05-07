//! Applies specified tags to a specified stream, overwriting any existing tags applied to that stream. If no tags are specified, all tags previously applied to the stream are removed. Automated tags are not affected by this operation.
//! [`replace-stream-tags`](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
//!
//! # Accessing the endpoint
//!
//! ## Request: [ReplaceStreamTagsRequest]
//!
//! To use this endpoint, construct a [`ReplaceStreamTagsRequest`] with the [`ReplaceStreamTagsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::streams::replace_stream_tags;
//! let request =
//!     replace_stream_tags::ReplaceStreamTagsRequest::broadcaster_id("1234");
//! ```
//!
//! ## Body: [ReplaceStreamTagsBody]
//!
//! We also need to provide a body to the request containing the tags we want to set.
//!
//! ```
//! # use twitch_api::helix::streams::replace_stream_tags;
//! let body = replace_stream_tags::ReplaceStreamTagsBody::tag_ids(vec![
//!     "621fb5bf-5498-4d8f-b4ac-db4d40d401bf".into(),
//!     "79977fb9-f106-4a87-a386-f1b0f99783dd".into(),
//! ]);
//! ```
//!
//! ## Response: [ReplaceStreamTags]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_put()`](helix::HelixClient::req_put).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, streams::replace_stream_tags};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = replace_stream_tags::ReplaceStreamTagsRequest::broadcaster_id("1234");
//! let body = replace_stream_tags::ReplaceStreamTagsBody::tag_ids(vec![
//!     "621fb5bf-5498-4d8f-b4ac-db4d40d401bf".into(),
//!     "79977fb9-f106-4a87-a386-f1b0f99783dd".into(),
//! ]);
//! let response: replace_stream_tags::ReplaceStreamTags = client.req_put(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPut::create_request)
//! and parse the [`http::Response`] with [`ReplaceStreamTagsRequest::parse_response(None, &request.get_uri(), response)`](ReplaceStreamTagsRequest::parse_response)
#![allow(deprecated)]

use super::*;
use helix::RequestPut;

/// Query Parameters for [Replace Stream Tags](super::replace_stream_tags)
///
/// [`replace-stream-tags`](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
#[deprecated(
    note = "Twitch-defined tags have been deprecated. See https://discuss.dev.twitch.tv/t/adding-customizable-tags-to-the-twitch-api/42921"
)]
pub struct ReplaceStreamTagsRequest<'a> {
    /// ID of the stream for which tags are to be replaced.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> ReplaceStreamTagsRequest<'a> {
    /// ID of the stream for which tags are to be replaced.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Body Parameters for [Replace Stream Tags](super::replace_stream_tags)
///
/// [`replace-stream-tags`](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
///
/// # Notes
///
/// Up to five tags can be applied to a stream. If no `tag_ids` is provided, all tags are removed from the stream.
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
#[deprecated(
    note = "Twitch-defined tags have been deprecated. See https://discuss.dev.twitch.tv/t/adding-customizable-tags-to-the-twitch-api/42921"
)]
pub struct ReplaceStreamTagsBody<'a> {
    /// IDs of tags to be applied to the stream.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub tag_ids: Cow<'a, [&'a types::TagIdRef]>,
}

impl<'a> ReplaceStreamTagsBody<'a> {
    /// IDs of tags to be applied to the stream.
    pub fn tag_ids(tag_ids: impl Into<Cow<'a, [&'a types::TagIdRef]>>) -> Self {
        Self {
            tag_ids: tag_ids.into(),
            ..Self::default()
        }
    }
}
/// Return Values for [Replace Stream Tags](super::replace_stream_tags)
///
/// [`replace-stream-tags`](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
#[deprecated(
    note = "Twitch-defined tags have been deprecated. See https://discuss.dev.twitch.tv/t/adding-customizable-tags-to-the-twitch-api/42921"
)]
pub enum ReplaceStreamTags {
    /// 204 - Stream Tags replaced successfully
    Success,
}

impl helix::private::SealedSerialize for ReplaceStreamTagsBody<'_> {}

impl Request for ReplaceStreamTagsRequest<'_> {
    type Response = ReplaceStreamTags;

    const PATH: &'static str = "streams/tags";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelManageBroadcast];
}

impl<'a> RequestPut for ReplaceStreamTagsRequest<'a> {
    type Body = ReplaceStreamTagsBody<'a>;

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
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(helix::Response {
                data: ReplaceStreamTags::Success,
                pagination: None,
                request,
                total: None,
                other: <_>::default(),
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

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = ReplaceStreamTagsRequest::broadcaster_id("0");

    let ids: &[&types::TagIdRef] = &[
        "621fb5bf-5498-4d8f-b4ac-db4d40d401bf".into(),
        "79977fb9-f106-4a87-a386-f1b0f99783dd".into(),
    ];
    let body = ReplaceStreamTagsBody::tag_ids(ids);

    dbg!(req.create_request(body, "token", "clientid").unwrap());
    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/streams/tags?broadcaster_id=0"
    );

    dbg!(ReplaceStreamTagsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

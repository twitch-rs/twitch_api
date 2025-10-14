//! Adds a marker to a live stream.
//! [`create-stream-marker`](https://dev.twitch.tv/docs/api/reference#create-stream-marker)
//!
//! A marker is an arbitrary point in a live stream that the broadcaster or editor wants to mark, so they can return to that spot later to create video highlights (see Video Producer, Highlights in the Twitch UX).
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreateStreamMarkerRequest]
//!
//! To use this endpoint, construct a [`CreateStreamMarkerRequest`] with the [`CreateStreamMarkerRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::streams::create_stream_marker;
//! let request = create_stream_marker::CreateStreamMarkerRequest::new();
//! ```
//!
//! ## Body: [CreateStreamMarkerBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use std::convert::TryFrom;
//! # use twitch_api::helix::streams::create_stream_marker;
//! let body = create_stream_marker::CreateStreamMarkerBody::new(
//!     "123", // user-id of the broadcaster
//!     "marker description",
//! );
//! ```
//!
//! ## Response: [CreatedStreamMarker]
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, streams::create_stream_marker};
//! # use twitch_api::client;
//! # use std::convert::TryFrom;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request =
//!     create_stream_marker::CreateStreamMarkerRequest::new();
//! let body =
//!     create_stream_marker::CreateStreamMarkerBody::new(
//!         "123",
//!         "my description"
//!     );
//! let response: create_stream_marker::CreatedStreamMarker = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&body, &token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreateStreamMarkerRequest::parse_response(None, &request.get_uri(), response)`](CreateStreamMarkerRequest::parse_response)

use std::marker::PhantomData;

use super::*;
use helix::RequestPost;

/// Query Parameters for [Create Stream Marker](super::create_stream_marker)
///
/// [`create-stream-marker`](https://dev.twitch.tv/docs/api/reference#create-stream-marker)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct CreateStreamMarkerRequest<'a> {
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip)]
    _marker: PhantomData<&'a ()>,
}

impl CreateStreamMarkerRequest<'_> {
    /// Create a new [CreateStreamMarkerRequest]
    pub fn new() -> Self { Self::default() }
}

/// Body Parameters for [Create Stream Marker](super::create_stream_marker)
///
/// [`create-stream-marker`](https://dev.twitch.tv/docs/api/reference#create-stream-marker)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreateStreamMarkerBody<'a> {
    /// The ID of the broadcaster that’s streaming content. This ID must match the user ID in the access token or the user in the access token must be one of the broadcaster’s editors.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// A short description of the marker to help the user remember why they marked the location. The maximum length of the description is 140 characters.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub description: Option<Cow<'a, str>>,
}

impl<'a> CreateStreamMarkerBody<'a> {
    /// Create a new stream marker with a description
    pub fn new(
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        description: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            user_id: user_id.into_cow(),
            description: Some(description.into()),
        }
    }

    /// Create a new stream marker without a description
    pub fn user_id(user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: user_id.into_cow(),
            description: None,
        }
    }
}

impl helix::private::SealedSerialize for CreateStreamMarkerBody<'_> {}

/// Return Value for [Create Stream Marker](super::create_stream_marker)
///
/// [`create-stream-marker`](https://dev.twitch.tv/docs/api/reference#create-stream-marker)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CreatedStreamMarker {
    /// An ID that identifies this marker.
    pub id: types::StreamMarkerId,
    /// The UTC date and time (in RFC3339 format) of when the user created the marker.
    pub created_at: types::Timestamp,
    /// The relative offset (in seconds) of the marker from the beginning of the stream.
    pub position_seconds: u64,
    /// A description that the user gave the marker to help them remember why they marked the location.
    pub description: String,
}

impl Request for CreateStreamMarkerRequest<'_> {
    type PaginationData = ();
    type Response = CreatedStreamMarker;

    const PATH: &'static str = "streams/markers";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelManageBroadcast];
}

impl<'a> RequestPost for CreateStreamMarkerRequest<'a> {
    type Body = CreateStreamMarkerBody<'a>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestPostError>
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

    let req = CreateStreamMarkerRequest::new();

    let body = CreateStreamMarkerBody::new("123", "hello, this is a marker!");

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"user_id":"123","description":"hello, this is a marker!"}"#
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
      "data": [
         {
            "id": "123",
            "created_at": "2018-08-20T20:10:03Z",
            "description": "hello, this is a marker!",
            "position_seconds": 244
         }
      ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/streams/markers?"
    );

    let res = CreateStreamMarkerRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;

    assert_eq!(res.id.as_str(), "123");
    assert_eq!(res.created_at.as_str(), "2018-08-20T20:10:03Z");
    assert_eq!(res.description, "hello, this is a marker!");
    assert_eq!(res.position_seconds, 244);
}

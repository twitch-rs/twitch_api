//! Starts a commercial on a specified channel.
//! [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
//!
//! # Accessing the endpoint
//!
//! ## Request: [StartCommercialRequest]
//!
//! To use this endpoint, construct a [`StartCommercialRequest`] with the [`StartCommercialRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::start_commercial;
//! let request = start_commercial::StartCommercialRequest::new();
//! ```
//!
//! ## Body: [StartCommercialBody]
//!
//! We also need to provide a body to the request specifying length of commercial and where to start it.
//!
//! ```
//! # use twitch_api::helix::channels::start_commercial;
//! let body = start_commercial::StartCommercialBody::builder()
//!     .broadcaster_id("1234")
//!     .length(twitch_api::types::CommercialLength::Length90)
//!     .build();
//! ```
//!
//! ## Response: [StartCommercialRequest]
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::start_commercial};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = start_commercial::StartCommercialRequest::new();
//! let body = start_commercial::StartCommercialBody::builder()
//!     .broadcaster_id("1234")
//!     .length(twitch_api::types::CommercialLength::Length90)
//!     .build();
//! let response: Vec<start_commercial::StartCommercial> = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`StartCommercialRequest::parse_response(None, &request.get_uri(), response)`](StartCommercialRequest::parse_response)
use std::marker::PhantomData;

use super::*;
use helix::RequestPost;

// Not implementing builder since it's not really needed...
/// Query Parameters for [Start Commercial](super::start_commercial)
///
/// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct StartCommercialRequest<'a> {
    #[serde(skip)]
    _marker: PhantomData<&'a ()>,
}

impl StartCommercialRequest<'_> {
    /// Create a new [`StartCommercialRequest`]
    pub fn new() -> Self { StartCommercialRequest::default() }
}

/// Body Parameters for [Start Commercial](super::start_commercial)
///
/// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct StartCommercialBody<'a> {
    /// ID of the channel requesting a commercial
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Desired length of the commercial in seconds. Valid options are 30, 60, 90, 120, 150, 180.
    pub length: types::CommercialLength,
}

impl<'a> StartCommercialBody<'a> {
    /// Start a commercial in this broadcasters channel
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        length: impl Into<types::CommercialLength>,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
            length: length.into(),
        }
    }
}

/// Return Values for [Start Commercial](super::start_commercial)
///
/// [`start-commercial`](https://dev.twitch.tv/docs/api/reference#start-commercial)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StartCommercial {
    /// Length of the triggered commercial
    pub length: types::CommercialLength,
    /// Provides contextual information on why the request failed
    pub message: String,
    /// Seconds until the next commercial can be served on this channel
    pub retry_after: u64,
}

impl Request for StartCommercialRequest<'_> {
    /// FIXME: Make non-vec
    type Response = Vec<StartCommercial>;

    const PATH: &'static str = "channels/commercial";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelEditCommercial];
}

impl<'a> RequestPost for StartCommercialRequest<'a> {
    type Body = StartCommercialBody<'a>;
}

impl helix::private::SealedSerialize for StartCommercialBody<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = StartCommercialRequest::default();

    let body = StartCommercialBody::new("1234", types::CommercialLength::Length120);

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
{
    "data": [{
      "length" : 60,
      "message" : "",
      "retry_after" : 480
    }]
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/commercial?"
    );

    dbg!(StartCommercialRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

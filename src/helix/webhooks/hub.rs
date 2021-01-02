//! Subscribe to or unsubscribe from events for a specified topic.
//! [`subscribe-tounsubscribe-from-events`](https://dev.twitch.tv/docs/api/webhooks-reference#subscribe-tounsubscribe-from-events)
//!
//! # Accessing the endpoint
//!
//! ## Request: [WebhookHubRequest]
//!
//! To use this endpoint, construct a [`WebhookHubRequest`] with the [`WebhookHubRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::moderation::webhooks;
//! let request = webhooks::WebhookHubRequest::builder()
//!     .build();
//! ```
//!
//! ## Body: [WebhookHubBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::moderation::webhooks;
//! let body = webhooks::WebhookHubBody::builder()
//!     .build();
//! ```
//!
//! ## Response: [WebhookHub]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::webhooks};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = webhooks::WebhookHubRequest::builder()
//!     .build();
//! let body = vec![webhooks::WebhookHubBody::builder()
//!     .build()];
//! let response: Vec<webhooks::WebhookHub> = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestPost::parse_response())

use std::convert::TryInto;

use super::*;
/// Query Parameters for [Subscribe to/Unsubscribe From Events](super::webhooks)
///
/// [`subscribe-tounsubscribe-from-events`](https://dev.twitch.tv/docs/api/webhooks-reference#subscribe-tounsubscribe-from-events)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct WebhookHubRequest<T: Topic> {
    #[builder(setter(skip), default)]
    #[serde(skip)]
    _phantom: std::marker::PhantomData<T>,
}

/// Body Parameters for [Subscribe to/Unsubscribe From Events](super::webhooks)
///
/// [`subscribe-tounsubscribe-from-events`](https://dev.twitch.tv/docs/api/webhooks-reference#subscribe-tounsubscribe-from-events)
///
/// # Notes
///
/// This body is quite different from the official body. If you want the true representation in text, see [`helix::RequestPost::body`] on [`WebhookHubRequest<T: Topic>`](WebhookHubRequest)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct WebhookHubBody<T: Topic> {
    /// URL where notifications will be delivered.
    #[builder(setter(into))]
    pub callback: String,
    /// Type of request. Valid values: subscribe, unsubscribe
    pub mode: WebhookSubscriptionMode,
    /// URL for the topic to subscribe to or unsubscribe from. topic maps to a new Twitch API endpoint.
    #[serde(bound = "T: Topic")]
    pub topic: T,
    /// Number of seconds until the subscription expires. Default: 0. Maximum: 864000.
    pub lease_seconds: u32,
    /// Secret used to sign notification payloads
    #[builder(setter(into))]
    pub secret: Option<String>,
}

#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WebhookSubscriptionMode {
    Subscribe,
    Unsubscribe,
}

/// Return Values for [Subscribe to/Unsubscribe From Events](super::webhooks)
///
/// [`subscribe-tounsubscribe-from-events`](https://dev.twitch.tv/docs/api/webhooks-reference#subscribe-tounsubscribe-from-events)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum WebhookHub {
    Success,
    // FIXME: better description
    Error,
}
impl std::convert::TryFrom<http::StatusCode> for WebhookHub {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(
        s: http::StatusCode,
    ) -> Result<Self, <Self as std::convert::TryFrom<http::StatusCode>>::Error> {
        match s {
            http::StatusCode::ACCEPTED | http::StatusCode::OK => Ok(WebhookHub::Success),
            http::StatusCode::BAD_REQUEST => Ok(WebhookHub::Error),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}
impl<T: Topic> helix::Request for WebhookHubRequest<T> {
    type Response = WebhookHub;

    const PATH: &'static str = "webhooks/hub";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl<T: Topic> helix::RequestPost for WebhookHubRequest<T> {
    type Body = WebhookHubBody<T>;

    fn body(&self, body: &Self::Body) -> Result<String, helix::BodyError> {
        #[derive(PartialEq, Serialize)]
        struct IWebhookHubBody<'a> {
            #[serde(rename = "hub.callback")]
            callback: &'a str,
            #[serde(rename = "hub.mode")]
            mode: &'a WebhookSubscriptionMode,
            #[serde(rename = "hub.topic")]
            topic: String,
            #[serde(rename = "hub.lease_seconds")]
            lease_seconds: u32,
            #[serde(rename = "hub.secret")]
            secret: Option<&'a str>,
        }

        let b = IWebhookHubBody {
            callback: &body.callback,
            mode: &body.mode,
            topic: body.topic.get_uri()?.to_string(),
            lease_seconds: body.lease_seconds,
            secret: body.secret.as_deref(),
        };
        serde_json::to_string(&b).map_err(Into::into)
    }

    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<
        helix::Response<Self, <Self as helix::Request>::Response>,
        helix::HelixRequestPostError,
    >
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            helix::HelixRequestPostError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(helix::HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<helix::HelixRequestError>(&text)
        {
            return Err(helix::HelixRequestPostError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }

        let response = response.status().try_into().map_err(|_| {
            // This path should never be taken, but just to be sure we do this
            helix::HelixRequestPostError::Error {
                status: response.status(),
                uri: uri.clone(),
                body: response.body().clone(),
                message: String::new(), // FIXME: None, but this branch should really never be hit
                error: String::new(),
            }
        })?;
        Ok(helix::Response {
            data: response, // FIXME: This should be a bit better...
            pagination: <_>::default(),
            request,
        })
    }
}

#[test]
fn test_request() {
    use helix::*;
    let req = WebhookHubRequest::<topics::users::user_follows::UserFollowsTopic>::builder().build();

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(202).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/webhooks/hub?");

    dbg!(WebhookHubRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

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
//! use twitch_api2::helix::webhooks::hub;
//! let request = hub::WebhookHubRequest::<twitch_api2::helix::webhooks::topics::users::UserFollowsTopic>::builder()
//!     .build();
//! ```
//!
//! ## Body: [WebhookHubBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::webhooks::hub;
//! let body = hub::WebhookHubBody::builder()
//!     .callback("https://example.com/this-is-a-callback")
//!     .lease_seconds(864000)
//!     .mode(hub::WebhookSubscriptionMode::Subscribe)
//!     .secret("12233213890390".to_string())
//!     .topic(twitch_api2::helix::webhooks::topics::users::UserFollowsTopic::builder().from_id(Some("1336".into())).build())
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
//! use twitch_api2::helix::{self, webhooks::hub};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = hub::WebhookHubRequest::builder()
//!     .build();
//! let body = hub::WebhookHubBody::builder()
//!     .callback("https://example.com/this-is-a-callback")
//!     .lease_seconds(864000)
//!     .mode(hub::WebhookSubscriptionMode::Subscribe)
//!     .secret("12233213890390".to_string())
//!     .topic(twitch_api2::helix::webhooks::topics::users::UserFollowsTopic::builder().from_id(Some("1336".into())).build())
//!     .build();
//! let response: hub::WebhookHub = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`WebhookHubRequest::parse_response(None, &request.get_uri(), response)`](WebhookHubRequest::parse_response)

use crate::helix;
use std::convert::TryInto;

use serde::{Deserialize, Serialize};

use super::*;
use helix::RequestPost;
/// Query Parameters for [Subscribe to/Unsubscribe From Events](super::hub)
///
/// [`subscribe-tounsubscribe-from-events`](https://dev.twitch.tv/docs/api/webhooks-reference#subscribe-tounsubscribe-from-events)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct WebhookHubRequest<T: Topic> {
    #[builder(setter(skip), default)]
    #[serde(skip)]
    _phantom: std::marker::PhantomData<T>,
}

/// Body Parameters for [Subscribe to/Unsubscribe From Events](super::hub)
///
/// [`subscribe-tounsubscribe-from-events`](https://dev.twitch.tv/docs/api/webhooks-reference#subscribe-tounsubscribe-from-events)
///
/// # Notes
///
/// This body is quite different from the official body. If you want the true representation in text, see [`helix::HelixRequestBody::try_to_body`] on [`WebhookHubRequest<T: Topic>`](WebhookHubRequest)
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

/// Subscription mode
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WebhookSubscriptionMode {
    /// Subscribe
    Subscribe,
    /// Unsubscribe
    Unsubscribe,
}

impl<T: Topic> helix::HelixRequestBody for WebhookHubBody<T> {
    fn try_to_body(&self) -> Result<Vec<u8>, helix::BodyError> {
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
            callback: &self.callback,
            mode: &self.mode,
            topic: self.topic.get_uri()?.to_string(),
            lease_seconds: self.lease_seconds,
            secret: self.secret.as_deref(),
        };
        serde_json::to_vec(&b).map_err(Into::into)
    }
}

/// Return Values for [Subscribe to/Unsubscribe From Events](super::hub)
///
/// [`subscribe-tounsubscribe-from-events`](https://dev.twitch.tv/docs/api/webhooks-reference#subscribe-tounsubscribe-from-events)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum WebhookHub {
    /// 202 - Success
    Success,
}
impl std::convert::TryFrom<http::StatusCode> for WebhookHub {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(
        s: http::StatusCode,
    ) -> Result<Self, <Self as std::convert::TryFrom<http::StatusCode>>::Error> {
        match s {
            http::StatusCode::ACCEPTED | http::StatusCode::OK => Ok(WebhookHub::Success),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}
impl<T: Topic> Request for WebhookHubRequest<T> {
    type Response = WebhookHub;

    const PATH: &'static str = "webhooks/hub";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl<T: Topic> RequestPost for WebhookHubRequest<T> {
    type Body = WebhookHubBody<T>;

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
        }) = helix::parse_json::<helix::HelixRequestError>(&text, false)
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

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req =
        WebhookHubRequest::<webhooks::topics::users::user_follows::UserFollowsTopic>::builder()
            .build();

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(202).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/webhooks/hub?");

    dbg!(WebhookHubRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

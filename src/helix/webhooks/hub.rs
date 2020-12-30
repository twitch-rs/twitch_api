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
pub struct WebhookHub {}

impl<T: Topic> helix::Request for WebhookHubRequest<T> {
    type Response = Vec<WebhookHub>;

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
}

#[test]
#[cfg(ignore)]
fn test_request() {
    use helix::*;
    let req = WebhookHubRequest::builder().build();

    // From twitch docs
    let data = br#"
{
   "data": [
     {
       "msg_id": "123",
       "is_permitted": true
     },
     {
       "msg_id": "393",
       "is_permitted": false
     }
   ]
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/enforcements/status?broadcaster_id=198704263"
    );

    dbg!(req.parse_response(&uri, http_response).unwrap());
}

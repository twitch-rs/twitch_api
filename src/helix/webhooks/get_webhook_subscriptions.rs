//! Get a list of your Webhook subscriptions.

use super::*;

/// Query Parameters for [Get webhook Subscriptions](super::get_webhook_subscriptions)
///
/// [`get-webhook-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-webhook-subscriptions)
#[derive(PartialEq, typed_builder::TypedBuilder, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct GetWebhookSubscriptionsRequest {
    /// Cursor for forward pagination
    #[builder(setter(into), default)]
    pub after: Option<helix::Cursor>,
    /// Number of values to be returned per page. Limit: 100. Default: 20.
    #[builder(setter(into), default)]
    pub first: Option<String>,
}

impl helix::Request for GetWebhookSubscriptionsRequest {
    type Response = Vec<WebhookSubscription>;

    const PATH: &'static str = "webhooks/subscriptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Return Values for [Get webhook Subscriptions](super::get_webhook_subscriptions)
///
/// [`get-webhook-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-webhook-subscriptions)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct WebhookSubscription {
    /// The callback provided for this subscription.
    pub callback: String,
    /// Date and time when this subscription expires. Encoded as RFC3339. The timezone is always UTC (“Z”).
    pub expires_at: types::Timestamp,
    /// The topic used in the initial subscription.
    pub topic: String,
}

impl helix::RequestGet for GetWebhookSubscriptionsRequest {}

impl helix::Paginated for GetWebhookSubscriptionsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor; }
}

#[test]
fn test_request() {
    use helix::*;
    let req: GetWebhookSubscriptionsRequest = GetWebhookSubscriptionsRequest::builder().build();

    let data = br#"
        {
            "total": 12,
            "data": [
                {
                    "topic": "https://api.twitch.tv/helix/streams?user_id=123",
                    "callback": "http://example.com/your_callback",
                    "expires_at": "2018-07-30T20:00:00Z"
                },
                {
                    "topic": "https://api.twitch.tv/helix/streams?user_id=345",
                    "callback": "http://example.com/your_callback",
                    "expires_at": "2018-07-30T20:03:00Z"
                }
            ],
            "pagination": {
                "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IkFYc2laU0k2TVN3aWFTSTZNWDAifX0"
            }
         }
    "#
    .to_vec();
    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/webhooks/subscriptions?"
    );

    dbg!(
        "{:#?}",
        GetWebhookSubscriptionsRequest::parse_response(Some(req), &uri, http_response).unwrap()
    );
}

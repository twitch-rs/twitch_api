//! Get a list of your Webhook subscriptions.

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Webhook Subscriptions](super::get_webhook_subscriptions)
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

impl Request for GetWebhookSubscriptionsRequest {
    type Response = WebhookSubscriptions;

    const PATH: &'static str = "webhooks/subscriptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Return Values for [Get Webhook Subscriptions](super::get_webhook_subscriptions)
///
/// [`get-webhook-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-webhook-subscriptions)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct WebhookSubscriptions {
    /// Subscriptions
    pub subscriptions: Vec<WebhookSubscription>,
    /// A hint at the total number of results returned, on all pages.
    ///
    /// # Notes
    ///
    /// This is an approximation: as you page through the list, some subscriptions may expire and others may be added.
    pub total: i64,
}

/// Describes a Webhook Subscription.
///
/// Used in [WebhookSubscriptions]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct WebhookSubscription {
    /// The callback provided for this subscription.
    pub callback: String,
    /// Date and time when this subscription expires. Encoded as RFC3339. The timezone is always UTC (“Z”).
    pub expires_at: types::Timestamp,
    /// The topic used in the initial subscription.
    pub topic: String,
}

impl RequestGet for GetWebhookSubscriptionsRequest {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug)]
        struct InnerResponse {
            data: Vec<WebhookSubscription>,
            /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
            #[serde(default)]
            pagination: helix::Pagination,
            total: i64,
        }

        let response: InnerResponse = helix::parse_json(response, true).map_err(|e| {
            helix::HelixRequestGetError::DeserializeError(
                response.to_string(),
                e,
                uri.clone(),
                status,
            )
        })?;
        Ok(helix::Response {
            data: WebhookSubscriptions {
                subscriptions: response.data,
                total: response.total,
            },
            pagination: response.pagination.cursor,
            request,
        })
    }
}

impl helix::Paginated for GetWebhookSubscriptionsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor; }
}

#[cfg(test)]
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

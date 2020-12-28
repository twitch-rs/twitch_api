//! Get a list of your EventSub subscriptions.

use super::*;
use crate::eventsub;

/// Query Parameters for [Get EventSub Subscriptions](super::get_eventsub_subscriptions)
///
/// [`get-eventsub-subscriptions`](https://dev.twitch.tv/docs/eventsub/helix-endpoints#get-eventsub-subscriptions)
#[derive(PartialEq, typed_builder::TypedBuilder, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct GetEventSubSubscriptionsRequest {
    #[builder(default, setter(into))]
    status: Option<eventsub::Status>,
}

impl helix::Request for GetEventSubSubscriptionsRequest {
    type Response = Vec<EventSubSubscription>;

    const PATH: &'static str = "eventsub/subscriptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Return Values for [Get EventSub Subscriptions](super::get_eventsub_subscriptions)
///
/// [`get-eventsub-subscriptions`](https://dev.twitch.tv/docs/eventsub/helix-endpoints#get-eventsub-subscriptions)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct EventSubSubscription {
    /// JSON object specifying custom parameters for the subscription.
    // FIXME: Should be [eventsub::Condition]
    pub condition: serde_json::Value,
    /// RFC3339 timestamp indicating when the subscription was created.
    pub created_at: types::Timestamp,
    /// ID of the subscription.
    pub id: types::EventSubId,
    /// Status of the subscription.
    pub status: eventsub::Status,
    /// Notification delivery specific information. Includes the transport method and callback URL.
    pub transport: eventsub::TransportResponse,
    /// The category of the subscription.
    #[serde(rename = "type")]
    pub type_: String,
    /// The version of the subscription.
    pub version: String,
}

impl helix::RequestGet for GetEventSubSubscriptionsRequest {}

#[test]
fn test_request() {
    use helix::*;
    let req: GetEventSubSubscriptionsRequest = GetEventSubSubscriptionsRequest::builder().build();

    let data = br#"{
        "total": 2,
        "data": [
            {
                "id": "26b1c993-bfcf-44d9-b876-379dacafe75a",
                "status": "enabled",
                "type": "streams.online",
                "version": "1",
                "condition": {
                    "broadcaster_user_id": "1234"
                },
                "created_at": "2020-11-10T20:08:33Z",
                "transport": {
                    "method": "webhook",
                    "callback": "https://this-is-a-callback.com"
                }
            },
            {
                "id": "35016908-41ff-33ce-7879-61b8dfc2ee16",
                "status": "webhook_callback_verification_pending",
                "type": "users.update",
                "version": "1",
                "condition": {
                    "user_id": "1234"
                },
                "created_at": "2020-11-10T20:31:52Z",
                "transport": {
                    "method": "webhook",
                    "callback": "https://this-is-a-callback.com"
                }
            }
        ],
        "limit": 10000,
        "pagination": {}
    }
    "#
    .to_vec();
    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/subscriptions?"
    );

    dbg!("{:#?}", req.parse_response(&uri, http_response).unwrap());
}

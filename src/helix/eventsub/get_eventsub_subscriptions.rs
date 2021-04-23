//! Get a list of your EventSub subscriptions.

use super::*;
use crate::eventsub;
use helix::RequestGet;

/// Query Parameters for [Get EventSub Subscriptions](super::get_eventsub_subscriptions)
///
/// [`get-eventsub-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-eventsub-subscriptions)
#[derive(PartialEq, typed_builder::TypedBuilder, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetEventSubSubscriptionsRequest {
    /// Include this parameter to filter subscriptions by their status.
    #[builder(default, setter(into))]
    pub status: Option<eventsub::Status>,
    // FIXME: https://github.com/twitchdev/issues/issues/272
    /// Cursor for forward pagination
    #[builder(default, setter(into))]
    pub after: Option<helix::Cursor>,
    // FIXME: https://github.com/twitchdev/issues/issues/271
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[builder(default, setter(into))]
    pub first: Option<usize>,
}

impl Request for GetEventSubSubscriptionsRequest {
    type Response = EventSubSubscriptions;

    const PATH: &'static str = "eventsub/subscriptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Return Values for [Get EventSub Subscriptions](super::get_eventsub_subscriptions)
///
/// [`get-eventsub-subscriptions`](https://dev.twitch.tv/docs/api/reference#get-eventsub-subscriptions)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct EventSubSubscriptions {
    /// Total number of subscriptions for the client ID that made the subscription creation request.
    pub total: usize,
    /// Total cost of all the subscriptions for the client ID that made the subscription creation request.
    pub total_cost: usize,
    /// The maximum total cost allowed for all of the subscriptions for the client ID that made the subscription creation request.
    pub max_total_cost: usize,
    #[deprecated(
        since = "0.5.0",
        note = "on 2021-05-11, this will no longer be returned. Use max_total_cost instead"
    )]
    #[serde(default)]
    /// Subscription limit for client id that made the subscription creation request.
    pub limit: Option<usize>,
    /// Array containing subscriptions.
    pub subscriptions: Vec<eventsub::EventSubSubscription>,
}

impl RequestGet for GetEventSubSubscriptionsRequest {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        _: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct InnerResponse {
            data: Vec<eventsub::EventSubSubscription>,
            /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
            #[serde(default)]
            pagination: helix::Pagination,
            total: usize,
            total_cost: usize,
            max_total_cost: usize,
            limit: Option<usize>,
        }

        let response: InnerResponse = helix::parse_json(response).map_err(|e| {
            helix::HelixRequestGetError::DeserializeError(response.to_string(), e, uri.clone())
        })?;
        #[allow(deprecated)]
        Ok(helix::Response {
            data: EventSubSubscriptions {
                total: response.total,
                total_cost: response.total_cost,
                max_total_cost: response.max_total_cost,
                limit: response.limit,
                subscriptions: response.data,
            },
            pagination: response.pagination.cursor,
            request,
        })
    }
}

impl helix::Paginated for GetEventSubSubscriptionsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[test]
fn test_request() {
    use helix::*;
    let req: GetEventSubSubscriptionsRequest = GetEventSubSubscriptionsRequest::builder().build();
    // From twitch docs.
    // FIXME: Twitch says in example that status is kebab-case, it's actually snake_case. also, users vs user and stream vs streams
    let data = br#"{
        "total": 2,
        "data": [
            {
                "id": "26b1c993-bfcf-44d9-b876-379dacafe75a",
                "status": "enabled",
                "type": "stream.online",
                "version": "1",
                "condition": {
                    "broadcaster_user_id": "1234"
                },
                "created_at": "2020-11-10T20:08:33Z",
                "transport": {
                    "method": "webhook",
                    "callback": "https://this-is-a-callback.com"
                },
                "cost": 1
            },
            {
                "id": "35016908-41ff-33ce-7879-61b8dfc2ee16",
                "status": "webhook_callback_verification_pending",
                "type": "user.update",
                "version": "1",
                "condition": {
                    "user_id": "1234"
                },
                "created_at": "2020-11-10T20:31:52Z",
                "transport": {
                    "method": "webhook",
                    "callback": "https://this-is-a-callback.com"
                },
                "cost": 0
            }
        ],
        "limit": 10000,
        "total_cost": 1,
        "max_total_cost": 10000,
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

    dbg!(
        "{:#?}",
        GetEventSubSubscriptionsRequest::parse_response(Some(req), &uri, http_response).unwrap()
    );
}

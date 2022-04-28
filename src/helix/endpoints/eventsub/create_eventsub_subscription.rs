//! Creates an EventSub subscription.
use super::*;
use crate::eventsub::{EventSubscription, EventType, Status, Transport, TransportResponse};

/// Query Parameters for [Create EventSub Subscription](super::create_eventsub_subscription)
///
/// [`create-eventsub-subscription`](https://dev.twitch.tv/docs/api/reference#create-eventsub-subscription)
#[derive(PartialEq, typed_builder::TypedBuilder, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreateEventSubSubscriptionRequest<E: EventSubscription> {
    #[builder(setter(skip), default)]
    #[serde(skip)]
    phantom: std::marker::PhantomData<E>,
}

impl<E: EventSubscription> Default for CreateEventSubSubscriptionRequest<E> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<E: EventSubscription> helix::Request for CreateEventSubSubscriptionRequest<E> {
    type Response = CreateEventSubSubscription<E>;

    const PATH: &'static str = "eventsub/subscriptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Body Parameters for [Create EventSub Subscription](super::create_eventsub_subscription)
///
/// [`create-eventsub-subscription`](https://dev.twitch.tv/docs/api/reference#create-eventsub-subscription)
///
/// # Notes
///
/// This body is quite different from the official body. If you want the true representation in text, see [`helix::HelixRequestBody::try_to_body`] on [`CreateEventSubSubscriptionRequest<E: EventSubscription>`](CreateEventSubSubscriptionRequest)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreateEventSubSubscriptionBody<E: EventSubscription> {
    /// Subscription that will be created
    #[serde(bound(deserialize = "E: EventSubscription"))]
    pub subscription: E,
    /// The notification delivery specific information
    pub transport: Transport,
}

impl<E: EventSubscription> helix::HelixRequestBody for CreateEventSubSubscriptionBody<E> {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(PartialEq, Serialize, Debug)]
        struct IEventSubRequestBody<'a> {
            r#type: EventType,
            version: &'static str,
            condition: serde_json::Value,
            transport: &'a Transport,
        }

        let b = IEventSubRequestBody {
            r#type: E::EVENT_TYPE,
            version: E::VERSION,
            condition: self.subscription.condition()?,
            transport: &self.transport,
        };
        serde_json::to_vec(&b).map_err(Into::into).map(Into::into)
    }
}

// FIXME: Builder?
impl<E: EventSubscription> CreateEventSubSubscriptionBody<E> {
    /// Create a new [`CreateEventSubSubscriptionBody`]
    pub fn new(subscription: E, transport: Transport) -> CreateEventSubSubscriptionBody<E> {
        CreateEventSubSubscriptionBody {
            subscription,
            transport,
        }
    }
}

/// Return Values for [Create EventSub Subscription](super::create_eventsub_subscription)
///
/// [`create-eventsub-subscription`](https://dev.twitch.tv/docs/api/reference#create-eventsub-subscription)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct CreateEventSubSubscription<E: EventSubscription> {
    /// ID of the subscription created.
    pub id: types::EventSubId,
    /// Status of the subscription.
    pub status: Status,
    /// The category of the subscription that was created.
    #[serde(rename = "type")]
    pub type_: EventType,
    /// The version of the subscription type that was created.
    pub version: String,
    /// JSON object specifying custom parameters for the subscription.
    #[serde(bound(deserialize = "E: EventSubscription"))]
    pub condition: E,
    /// RFC3339 timestamp indicating when the subscription was created.
    pub created_at: types::Timestamp,
    /// JSON object indicating the notification delivery specific information. Includes the transport method and callback URL.
    pub transport: TransportResponse,
    #[deprecated(
        since = "0.5.0",
        note = "on 2021-05-11, this will no longer be returned. Use max_total_cost instead"
    )]
    #[serde(default)]
    /// Subscription limit for client id that made the subscription creation request.
    pub limit: Option<usize>,
    /// Total number of subscriptions for the client ID that made the subscription creation request.
    pub total: usize,
    /// Total cost of all the subscriptions for the client ID that made the subscription creation request.
    pub total_cost: usize,
    /// The maximum total cost allowed for all of the subscriptions for the client ID that made the subscription creation request.
    pub max_total_cost: usize,
    /// How much the subscription counts against your limit.
    pub cost: usize,
}

impl<E: EventSubscription> helix::RequestPost for CreateEventSubSubscriptionRequest<E> {
    type Body = CreateEventSubSubscriptionBody<E>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        text: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        pub struct InnerResponseData<E: EventSubscription> {
            cost: usize,
            #[serde(bound(deserialize = "E: EventSubscription"))]
            condition: E,
            created_at: types::Timestamp,
            id: types::EventSubId,
            status: Status,
            transport: TransportResponse,
            #[serde(rename = "type")]
            type_: EventType,
            version: String,
        }
        #[derive(PartialEq, Deserialize, Debug)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct InnerResponse<E: EventSubscription> {
            #[serde(bound(deserialize = "E: EventSubscription"))]
            data: Vec<InnerResponseData<E>>,
            limit: Option<usize>,
            total: usize,
            total_cost: usize,
            max_total_cost: usize,
        }
        let response: InnerResponse<E> = helix::parse_json(text, true).map_err(|e| {
            helix::HelixRequestPostError::DeserializeError(text.to_string(), e, uri.clone(), status)
        })?;
        let data = response.data.into_iter().next().ok_or_else(|| {
            helix::HelixRequestPostError::InvalidResponse {
                reason: "missing response data",
                response: text.to_string(),
                status,
                uri: uri.clone(),
            }
        })?;
        #[allow(deprecated)]
        Ok(helix::Response {
            data: CreateEventSubSubscription {
                limit: response.limit,
                total: response.total,
                total_cost: response.total_cost,
                max_total_cost: response.max_total_cost,
                cost: data.cost,
                id: data.id,
                status: data.status,
                type_: data.type_,
                version: data.version,
                condition: data.condition,
                created_at: data.created_at,
                transport: data.transport,
            },
            pagination: None,
            request,
            // helix::Response total is generally the total number of results, not what the total for this endpoint means. Thus, we set it to None.
            total: None,
            other: None,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use crate::eventsub::{self, user::UserUpdateV1};
    use helix::*;
    let req: CreateEventSubSubscriptionRequest<UserUpdateV1> =
        CreateEventSubSubscriptionRequest::builder().build();

    let body = CreateEventSubSubscriptionBody::new(
        UserUpdateV1::builder().user_id("1234").build(),
        eventsub::Transport {
            method: eventsub::TransportMethod::Webhook,
            callback: "example.com".to_string(),
            secret: "heyhey13".to_string(),
        },
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs, FIXME: docs say `users.update` in example for Create EventSub Subscription, they also use kebab-case for status
    // "{"type":"users.update","version":"1","condition":{"user_id":"1234"},"transport":{"method":"webhook","callback":"https://this-is-a-callback.com","secret":"s3cre7"}}"
    let data = br#"{
    "data": [
        {
            "id": "26b1c993-bfcf-44d9-b876-379dacafe75a",
            "status": "webhook_callback_verification_pending",
            "type": "user.update",
            "version": "1",
            "condition": {
                "user_id": "1234"
            },
            "created_at": "2020-11-10T20:29:44Z",
            "transport": {
                "method": "webhook",
                "callback": "https://this-is-a-callback.com"
            },
            "cost": 1
        }
    ],
    "limit": 10000,
    "total": 1,
    "total_cost": 1,
    "max_total_cost": 10000
}
    "#
    .to_vec();
    let http_response = http::Response::builder().status(202).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/subscriptions?"
    );

    dbg!(
        "{:#?}",
        CreateEventSubSubscriptionRequest::parse_response(Some(req), &uri, http_response).unwrap()
    );
}

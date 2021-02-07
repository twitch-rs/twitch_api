//! Creates an EventSub subscription.

use std::convert::TryInto;

use super::*;
use crate::eventsub::{EventSubscription, EventType, Status, Transport, TransportResponse};

/// Query Parameters for [Create EventSub Subscription](super::create_eventsub_subscription)
///
/// [`create-eventsub-subscription`](https://dev.twitch.tv/docs/api/reference#create-eventsub-subscription)
#[derive(PartialEq, typed_builder::TypedBuilder, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreateEventSubSubscriptionRequest<E: EventSubscription> {
    #[builder(setter(skip), default)]
    #[serde(skip)]
    phantom: std::marker::PhantomData<E>,
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
/// This body is quite different from the official body. If you want the true representation in text, see [`helix::RequestPost::body`] on [`CreateEventSubSubscriptionRequest<E: EventSubscription>`](CreateEventSubSubscriptionRequest)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreateEventSubSubscriptionBody<E: EventSubscription> {
    /// Subscription that will be created
    #[serde(bound(deserialize = "E: EventSubscription"))]
    pub subscription: E,
    /// The notification delivery specific information
    pub transport: Transport,
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
    /// Subscription limit for client id that made the subscription creation request.
    pub limit: usize,
    /// Total number of subscriptions for the client ID that made the subscription creation request.
    pub total: usize,
}

impl<E: EventSubscription> helix::RequestPost for CreateEventSubSubscriptionRequest<E> {
    type Body = CreateEventSubSubscriptionBody<E>;

    fn body(&self, body: &Self::Body) -> Result<String, helix::BodyError> {
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
            condition: body.subscription.condition()?,
            transport: &body.transport,
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
        #[derive(PartialEq, Deserialize, Debug)]
        pub struct InnerResponseData<E: EventSubscription> {
            id: String,
            status: Status,
            #[serde(rename = "type")]
            type_: EventType,
            version: String,
            #[serde(bound(deserialize = "E: EventSubscription"))]
            condition: E,
            created_at: String,
            transport: TransportResponse,
        }
        #[derive(PartialEq, Deserialize, Debug)]
        struct InnerResponse<E: EventSubscription> {
            #[serde(bound(deserialize = "E: EventSubscription"))]
            data: Vec<InnerResponseData<E>>,
            limit: usize,
            total: usize,
        }

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

        let response: InnerResponse<E> = serde_json::from_str(&text).map_err(|e| {
            helix::HelixRequestPostError::DeserializeError(text.to_string(), e, uri.clone())
        })?;
        let data = response.data.into_iter().next().ok_or_else(|| {
            helix::HelixRequestPostError::DeserializeError(
                text.to_string(),
                serde::de::Error::custom("missing response `data`"),
                uri.clone(),
            )
        })?;
        Ok(helix::Response {
            data: CreateEventSubSubscription {
                limit: response.limit,
                total: response.total,
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
        })
    }
}

#[test]
fn test_request() {
    use crate::eventsub::user::UserUpdateV1;
    use helix::*;
    let req: CreateEventSubSubscriptionRequest<UserUpdateV1> =
        CreateEventSubSubscriptionRequest::builder().build();

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
            }
        }
    ],
    "limit": 10000,
    "total": 1
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

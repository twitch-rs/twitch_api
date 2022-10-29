//! Delete an EventSub subscription.

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Delete EventSub Subscriptions](super::delete_eventsub_subscription)
///
/// [`delete-eventsub-subscriptions`](https://dev.twitch.tv/docs/api/reference#delete-eventsub-subscription)
#[derive(PartialEq, Eq, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct DeleteEventSubSubscriptionRequest<'a> {
    /// The subscription ID for the subscription you want to delete.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub id: Cow<'a, types::EventSubIdRef>,
}

impl<'a> DeleteEventSubSubscriptionRequest<'a> {
    /// Delete this eventsub subscription.
    pub fn id(id: impl types::IntoCow<'a, types::EventSubIdRef> + 'a) -> Self {
        Self { id: id.to_cow() }
    }
}

impl Request for DeleteEventSubSubscriptionRequest<'_> {
    type Response = DeleteEventSubSubscription;

    const PATH: &'static str = "eventsub/subscriptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Return Values for [Delete EventSub Subscriptions](super::delete_eventsub_subscription)
///
/// [`delete-eventsub-subscriptions`](https://dev.twitch.tv/docs/api/reference#delete-eventsub-subscription)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum DeleteEventSubSubscription {
    /// 204 - Subscription deleted
    Success,
}

impl RequestDelete for DeleteEventSubSubscriptionRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(helix::Response {
                data: DeleteEventSubSubscription::Success,
                pagination: None,
                request,
                total: None,
                other: None,
            }),
            _ => Err(helix::HelixRequestDeleteError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req: DeleteEventSubSubscriptionRequest = DeleteEventSubSubscriptionRequest::id("deadbeef");

    let data = vec![];
    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/subscriptions?id=deadbeef"
    );

    dbg!(
        "{:#?}",
        DeleteEventSubSubscriptionRequest::parse_response(Some(req), &uri, http_response).unwrap()
    );
}

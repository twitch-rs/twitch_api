//! Get a list of your EventSub subscriptions.

use super::*;

/// Query Parameters for [Get EventSub Subscriptions](super::get_eventsub_subscriptions)
///
/// [`get-eventsub-subscriptions`](https://dev.twitch.tv/docs/api/reference#delete-eventsub-subscription)
#[derive(PartialEq, typed_builder::TypedBuilder, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct DeleteEventSubSubscriptionRequest {
    /// The subscription ID for the subscription you want to delete.
    #[builder(setter(into))]
    pub id: types::EventSubId,
}

impl helix::Request for DeleteEventSubSubscriptionRequest {
    type Response = DeleteEventSubSubscription;

    const PATH: &'static str = "eventsub/subscriptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Return Values for [Get EventSub Subscriptions](super::get_eventsub_subscriptions)
///
/// [`get-eventsub-subscriptions`](https://dev.twitch.tv/docs/api/reference#delete-eventsub-subscription)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum DeleteEventSubSubscription {
    /// 204 - Subscription deleted
    Success,
    /// 404 - Subscription not found
    NotFound,
    /// 400 - Missing Query
    ///
    /// # Notes
    ///
    /// This will never be encountered if using [DeleteEventSubSubscriptionRequest]
    MissingQuery,
}

impl std::convert::TryFrom<http::StatusCode> for DeleteEventSubSubscription {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
        match s {
            http::StatusCode::NO_CONTENT => Ok(DeleteEventSubSubscription::Success),
            http::StatusCode::BAD_REQUEST => Ok(DeleteEventSubSubscription::MissingQuery),
            http::StatusCode::NOT_FOUND => Ok(DeleteEventSubSubscription::NotFound),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}

impl helix::RequestDelete for DeleteEventSubSubscriptionRequest {}

#[test]
fn test_request() {
    use helix::*;
    let req: DeleteEventSubSubscriptionRequest = DeleteEventSubSubscriptionRequest::builder()
        .id("deadbeef")
        .build();

    let data = vec![];
    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/subscriptions?id=deadbeef"
    );

    dbg!(
        "{:#?}",
        DeleteEventSubSubscriptionRequest::parse_response(&uri, http_response).unwrap()
    );
}

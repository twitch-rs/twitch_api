//! Returns a list of subscription events.
//! [`get-broadcaster-subscriptions-events`](https://twitch.uservoice.com/forums/310213-developers/suggestions/39847468-get-broadcaster-subscriptions-events)
//!
//!
//! # Notes
//!
//! This endpoint seems to have been [removed from the docs by mistake](https://twitch.uservoice.com/forums/310213-developers/suggestions/39847468-get-broadcaster-subscriptions-events).
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetBroadcasterSubscriptionsEventsRequest]
//!
//! To use this endpoint, construct a [`GetBroadcasterSubscriptionsEventsRequest`] with the [`GetBroadcasterSubscriptionsEventsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::subscriptions::get_broadcaster_subscriptions_events;
//! let request = get_broadcaster_subscriptions_events::GetBroadcasterSubscriptionsEventsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [BroadcasterSubscriptionEvent]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, subscriptions::get_broadcaster_subscriptions_events};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_broadcaster_subscriptions_events::GetBroadcasterSubscriptionsEventsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_broadcaster_subscriptions_events::BroadcasterSubscriptionEvent> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetBroadcasterSubscriptionsEventsRequest::parse_response(None, &request.get_uri(), response)`](GetBroadcasterSubscriptionsEventsRequest::parse_response)
// FIXME: Twitch docs sucks... This entire endpoint is removed from docs

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Broadcaster Subscriptions Events](super::get_broadcaster_subscriptions_events)
///
/// [`get-broadcaster-subscriptions-events`](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions-events)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetBroadcasterSubscriptionsEventsRequest {
    /// Must match the User ID in the Bearer token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// Filters the results and only returns a status object for users who have a subscribe event in this channel and have a matching user_id.
    /// Maximum: 100
    #[builder(default)]
    pub user_id: Vec<types::UserId>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[builder(default, setter(into))]
    pub first: Option<usize>,
    /// Retreive a single event by event ID
    #[builder(default, setter(into))]
    pub id: Option<String>,
}

/// Return Values for [Get Broadcaster Subscriptions Events](super::get_broadcaster_subscriptions_events)
///
/// [`get-broadcaster-subscriptions-events`](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions-events)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BroadcasterSubscriptionEvent {
    /// Event ID
    pub id: String,
    /// `subscriptions.subscribe`, `subscriptions.unsubscribe` or `subscriptions.notification`
    pub event_type: BroadcasterSubscriptionEventType,
    /// RFC3339 formatted timestamp for events.
    pub event_timestamp: types::Timestamp,
    /// Returns the version of the endpoint.
    pub version: String,
    /// Returns `broadcaster_id`, `broadcaster_name`, `user_id`, `user_name`, and `expires_at`.
    pub event_data: BroadcasterSubscriptionEventData,
}

/// Type of event
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum BroadcasterSubscriptionEventType {
    /// User has subscribed
    #[serde(rename = "subscriptions.subscribe")]
    Subscribe,
    /// User has unsubscribed
    #[serde(rename = "subscriptions.unsubscribe")]
    Unsubscribe,
    /// User shared a subscription in channel
    #[serde(rename = "subscriptions.notification")]
    Notification,
}

/// Event data for broadcaster events.
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BroadcasterSubscriptionEventData {
    /// User ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// Display name of the broadcaster.
    pub broadcaster_name: types::DisplayName,
    /// Determines if the subscription is a gift subscription.
    pub is_gift: bool,
    /// Type of subscription (Tier 1, Tier 2, Tier 3). 1000 = Tier 1, 2000 = Tier 2, 3000 = Tier 3 subscriptions.
    #[serde(deserialize_with = "subscription_tier")]
    pub tier: Option<types::SubscriptionTier>,
    /// Name of the subscription.
    #[serde(
        default,
        deserialize_with = "helix::deserialize_none_from_empty_string"
    )]
    pub plan_name: Option<String>,
    /// ID of the subscribed user.
    pub user_id: types::UserId,
    /// Display name of the subscribed user.
    pub user_name: types::DisplayName,
    /// ID of the subscribed user.
    #[serde(
        default,
        deserialize_with = "helix::deserialize_none_from_empty_string"
    )]
    pub gifter_id: Option<types::UserId>,
    /// Display name of the gifter.
    #[serde(
        default,
        deserialize_with = "helix::deserialize_none_from_empty_string"
    )]
    pub gifter_name: Option<types::DisplayName>,
}

/// Deserialize [`SubscriptionTier::Other("")`](types::SubscriptionTier::Other) as [`Option::None`]
fn subscription_tier<'de, D>(deserializer: D) -> Result<Option<types::SubscriptionTier>, D::Error>
where D: serde::de::Deserializer<'de> {
    Ok(match types::SubscriptionTier::deserialize(deserializer)? {
        types::SubscriptionTier::Other(s) if s.is_empty() => None,
        other => Some(other),
    })
}

impl Request for GetBroadcasterSubscriptionsEventsRequest {
    type Response = Vec<BroadcasterSubscriptionEvent>;

    const PATH: &'static str = "subscriptions/events";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelReadSubscriptions];
}

impl RequestGet for GetBroadcasterSubscriptionsEventsRequest {}

impl helix::Paginated for GetBroadcasterSubscriptionsEventsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetBroadcasterSubscriptionsEventsRequest::builder()
        .broadcaster_id("1337".to_string())
        .build();

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "id": "1mZCpIomSWc9PR2Ldeadbeef",
            "event_type": "subscriptions.subscribe",
            "event_timestamp": "2021-01-03T16:38:27Z",
            "version": "1.0",
            "event_data": {
              "broadcaster_id": "1337",
              "broadcaster_name": "justintv",
              "gifter_id": "",
              "gifter_name": "",
              "is_gift": false,
              "plan_name": "Channel Subscription (justintv)",
              "tier": "1000",
              "user_id": "1336",
              "user_name": "twitchuser"
            }
          },
          {
            "id": "1mY9qZVbbl77PpGydeadbeef",
            "event_type": "subscriptions.unsubscribe",
            "event_timestamp": "2021-01-03T07:44:08Z",
            "version": "1.0",
            "event_data": {
              "broadcaster_id": "1337",
              "broadcaster_name": "justintv",
              "gifter_id": "",
              "gifter_name": "",
              "is_gift": false,
              "plan_name": "",
              "tier": "",
              "user_id": "1336",
              "user_name": "twitchuser"
            }
          },    {
            "id": "1mRxcgkkAVfej5n7deadbeef",
            "event_type": "subscriptions.notification",
            "event_timestamp": "2021-01-01T03:04:45Z",
            "version": "1.0",
            "event_data": {
              "broadcaster_id": "1337",
              "broadcaster_name": "justintv",
              "gifter_id": "",
              "gifter_name": "",
              "is_gift": false,
              "plan_name": "Channel Subscription (justintv)",
              "tier": "1000",
              "user_id": "1336",
              "user_name": "twitchuser"
            }
          }
        ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/subscriptions/events?broadcaster_id=1337"
    );

    dbg!(
        GetBroadcasterSubscriptionsEventsRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
    );
}

#![allow(deprecated_in_future, deprecated)]
//! Gets the information of the most recent Hype Train of the given channel ID.
//! [`get-hype-train-events`](https://dev.twitch.tv/docs/api/reference#get-hype-train-events)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetHypeTrainEventsRequest]
//!
//! To use this endpoint, construct a [`GetHypeTrainEventsRequest`] with the [`GetHypeTrainEventsRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::hypetrain::get_hypetrain_events;
//! let request = get_hypetrain_events::GetHypeTrainEventsRequest::broadcaster_id("4321");
//! ```
//!
//! ## Response: [HypeTrainEvent](types::TwitchCategory)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, hypetrain::get_hypetrain_events};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_hypetrain_events::GetHypeTrainEventsRequest::broadcaster_id("4321");
//! let response: Vec<get_hypetrain_events::HypeTrainEvent> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetHypeTrainEventsRequest::parse_response(None, &request.get_uri(), response)`](GetHypeTrainEventsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Hype Train Events](super::get_hypetrain_events)
///
/// [`get-hype-train-events`](https://dev.twitch.tv/docs/api/reference#get-hype-train-events)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetHypeTrainEventsRequest<'a> {
    /// Must match the User ID in the Bearer token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow="'a")]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub cursor: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// Retreive a single event by event ID
    #[deprecated(
        since = "0.6.0",
        note = "this does nothing, see https://discuss.dev.twitch.tv/t/get-hype-train-events-api-endpoint-id-query-parameter-deprecation/37613"
    )]
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(borrow="'a")]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> GetHypeTrainEventsRequest<'a> {
    /// Get hypetrain evens
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
            cursor: Default::default(),
            first: Default::default(),
            id: Default::default(),
        }
    }
}

/// Return Values for [Get Hype Train Events](super::get_hypetrain_events)
///
/// [`get-hype-train-events`](https://dev.twitch.tv/docs/api/reference#get-hype-train-events)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainEvent {
    /// Event ID
    pub id: String,
    /// Displays hypetrain.{event_name}, currently only hypetrain.progression
    pub event_type: HypeTrainEventType,
    /// RFC3339 formatted timestamp for events.
    pub event_timestamp: types::Timestamp,
    /// Returns the version of the endpoint.
    pub version: String,
    /// Returns `broadcaster_id`, `broadcaster_name`, `user_id`, `user_name`, and `expires_at`.
    pub event_data: HypeTrainEventData,
}

/// Type of Hype Train event
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum HypeTrainEventType {
    /// Progression
    #[serde(rename = "hypetrain.progression")]
    Progression,
}

/// Event data for
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainEventData {
    /// The requested broadcaster ID.
    pub broadcaster_id: types::UserId,
    /// The time at which the hype train expires. The expiration is extended when the hype train reaches a new level.
    pub expires_at: types::Timestamp,
    /// RFC3339 formatted timestamp of when another hype train can be started again
    pub cooldown_end_time: types::Timestamp,
    /// The number of points required to reach the next level.
    pub goal: i64,
    /// The most recent contribution.
    pub last_contribution: Contribution,
    /// Current level of hype train event.
    pub level: i64,
    /// The timestamp at which the hype train started.
    pub started_at: types::Timestamp,
    // FIXME: Contains a maximum of two user objects
    /// The contributors with the most points contributed.
    pub top_contributions: Vec<Contribution>,
    /// Total points contributed to the hype train.
    pub total: i64,
    /// The distinct ID of this Hype Train
    pub id: types::HypeTrainId,
}

impl Request for GetHypeTrainEventsRequest<'_> {
    type Response = Vec<HypeTrainEvent>;

    const PATH: &'static str = "hypetrain/events";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetHypeTrainEventsRequest<'_> {}

impl helix::Paginated for GetHypeTrainEventsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.cursor = cursor }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetHypeTrainEventsRequest::broadcaster_id("270954519");

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
            "event_type": "hypetrain.progression",
            "event_timestamp": "2020-04-24T20:07:24Z",
            "version": "1.0",
            "event_data": {
              "broadcaster_id": "270954519",
              "cooldown_end_time": "2020-04-24T20:13:21.003802269Z",
              "expires_at": "2020-04-24T20:12:21.003802269Z",
              "goal": 1800,
              "id": "70f0c7d8-ff60-4c50-b138-f3a352833b50",
              "last_contribution": {
                "total": 200,
                "type": "BITS",
                "user": "134247454"
              },
              "level": 2,
              "started_at": "2020-04-24T20:05:47.30473127Z",
              "top_contributions": [
                {
                  "total": 600,
                  "type": "BITS",
                  "user": "134247450"
                }
              ],
              "total": 600
            }
          }
        ],
        "pagination": {
          "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjI3MDk1NDUxOToxNTg3NzU4ODQ0OjFiMEFzYkluQ0haVzJTUUZRa0N6cU4wN0liMiJ9fQ"
        }
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/hypetrain/events?broadcaster_id=270954519"
    );

    dbg!(GetHypeTrainEventsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

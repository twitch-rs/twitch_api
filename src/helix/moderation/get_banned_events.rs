//! Returns all banned and timed-out users in a channel.
//! [`get-banned-events`](https://dev.twitch.tv/docs/api/reference#get-banned-events)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetBannedEventsRequest]
//!
//! To use this endpoint, construct a [`GetBannedEventsRequest`] with the [`GetBannedEventsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::moderation::get_banned_events;
//! let request = get_banned_events::GetBannedEventsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [BannedEvent]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::get_banned_events};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_banned_events::GetBannedEventsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_banned_events::BannedEvent> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())

use super::*;
use std::collections::HashMap;

/// Query Parameters for [Get Banned Events](super::get_banned_events)
///
/// [`get-banned-events`](https://dev.twitch.tv/docs/api/reference#get-banned-events)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetBannedEventsRequest {
    /// Must match the User ID in the Bearer token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
    /// Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
    /// Maximum: 100
    #[builder(default)]
    pub user_id: Vec<types::UserId>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[builder(default, setter(into))]
    pub first: Option<usize>,
}

/// Return Values for [Get Banned Events](super::get_banned_events)
///
/// [`get-banned-events`](https://dev.twitch.tv/docs/api/reference#get-banned-events)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BannedEvent {
    /// Event ID
    pub id: String,
    /// Displays `moderation.user.ban` or `moderation.user.unban`
    pub event_type: String,
    /// RFC3339 formatted timestamp for events.
    pub event_timestamp: types::Timestamp,
    /// Returns the version of the endpoint.
    pub version: String,
    // FIXME: Should be a struct, maybe
    /// Returns `broadcaster_id`, `broadcaster_name`, `user_id`, `user_name`, and `expires_at`.
    pub event_data: HashMap<String, String>,
}

impl helix::Request for GetBannedEventsRequest {
    type Response = Vec<BannedEvent>;

    const PATH: &'static str = "moderation/banned/events";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
}

impl helix::RequestGet for GetBannedEventsRequest {}

impl helix::Paginated for GetBannedEventsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetBannedEventsRequest::builder()
        .broadcaster_id("198704263".to_string())
        .build();

    // From twitch docs
    let data = br#"
{
    "data": [
    {
        "id": "1IPFqAb0p0JncbPSTEPhx8JF1Sa",
        "event_type": "moderation.user.ban",
        "event_timestamp": "2019-03-13T15:55:14Z",
        "version": "1.0",
        "event_data": {
        "broadcaster_id": "198704263",
        "broadcaster_name": "aan22209",
        "user_id": "424596340",
        "user_name": "quotrok",
        "expires_at": ""
        }
    },
    {
        "id": "1IPFsDv5cs4mxfJ1s2O9Q5flf4Y",
        "event_type": "moderation.user.unban",
        "event_timestamp": "2019-03-13T15:55:30Z",
        "version": "1.0",
        "event_data": {
        "broadcaster_id": "198704263",
        "broadcaster_name": "aan22209",
        "user_id": "424596340",
        "user_name": "quotrok",
        "expires_at": ""
        }
    },
    {
        "id": "1IPFqmlu9W2q4mXXjULyM8zX0rb",
        "event_type": "moderation.user.ban",
        "event_timestamp": "2019-03-13T15:55:19Z",
        "version": "1.0",
        "event_data": {
        "broadcaster_id": "198704263",
        "broadcaster_name": "aan22209",
        "user_id": "424596340",
        "user_name": "quotrok",
        "expires_at": ""
        }
    }
    ],
    "pagination": {
    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjE5OTYwNDI2MzoyMDIxMjA1MzE6MUlQRnFtbHU5VzJxNG1YWGpVTHlNOHpYMHJiIn19"
    }
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/banned/events?broadcaster_id=198704263"
    );

    dbg!(req.parse_response(&uri, http_response).unwrap());
}

//! Returns a list of moderators or users added and removed as moderators from a channel.
//! [`get-moderator-events`](https://dev.twitch.tv/docs/api/reference#get-moderator-events)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetModeratorEventsRequest]
//!
//! To use this endpoint, construct a [`GetModeratorEventsRequest`] with the [`GetModeratorEventsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::moderation::get_moderator_events;
//! let request = get_moderator_events::GetModeratorEventsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [ModeratorEvent]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::get_moderator_events};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_moderator_events::GetModeratorEventsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_moderator_events::ModeratorEvent> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetModeratorEventsRequest::parse_response(None, &request.get_uri(), response)`](GetModeratorEventsRequest::parse_response)

use super::*;
use helix::RequestGet;
use std::collections::HashMap;

/// Query Parameters for [Get Moderators Events](super::get_moderator_events)
///
/// [`get-moderator-events`](https://dev.twitch.tv/docs/api/reference#get-moderator-events)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetModeratorEventsRequest {
    /// Must match the User ID in the Bearer token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// Filters the results and only returns a status object for users who have been added or removed as moderators in this channel and have a matching user_id.
    /// Format: Repeated Query Parameter, eg. /moderation/moderators?broadcaster_id=1&user_id=2&user_id=3
    /// Maximum: 100
    #[builder(default)]
    pub user_id: Vec<types::UserId>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
    /// Number of values to be returned per page. Limit: 100. Default: 20.
    #[builder(setter(into), default)]
    pub first: Option<String>,
}

/// Return Values for [Get Moderators Events](super::get_moderator_events)
///
/// [`get-moderator-events`](https://dev.twitch.tv/docs/api/reference#get-moderator-events)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ModeratorEvent {
    /// Event ID
    pub id: String,
    // FIXME: Twitch docs sucks...
    /// Displays `moderation.moderator.add` or `moderation.moderator.remove`
    pub event_type: String,
    /// RFC3339 formatted timestamp for events.
    pub event_timestamp: types::Timestamp,
    /// Returns the version of the endpoint.
    pub version: String,
    /// Returns `broadcaster_id`, `broadcaster_name`, `broadcaster_login`, `user_id`, `user_name`, `user_login` and `expires_at`.
    pub event_data: HashMap<String, String>,
}

impl Request for GetModeratorEventsRequest {
    type Response = Vec<ModeratorEvent>;

    const PATH: &'static str = "moderation/moderators/events";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
}

impl RequestGet for GetModeratorEventsRequest {}

impl helix::Paginated for GetModeratorEventsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetModeratorEventsRequest::builder()
        .broadcaster_id("198704263".to_string())
        .build();

    // From twitch docs
    let data = br#"
{
    "data": [
        {
        "id": "1IVBTnDSUDApiBQW4UBcVTK4hPr",
        "event_type": "moderation.moderator.remove",
        "event_timestamp": "2019-03-15T18:18:14Z",
        "version": "1.0",
        "event_data": {
            "broadcaster_id": "198704263",
            "broadcaster_login": "aan22209",
            "broadcaster_name": "aan22209",
            "user_id": "423374343",
            "user_login": "glowillig",
            "user_name": "glowillig"
        }
        },
        {
        "id": "1IVIPQdYIEnD8nJ376qkASDzsj7",
        "event_type": "moderation.moderator.add",
        "event_timestamp": "2019-03-15T19:15:13Z",
        "version": "1.0",
        "event_data": {
            "broadcaster_id": "198704263",
            "broadcaster_login": "aan22209",
            "broadcaster_name": "aan22209",
            "user_id": "423374343",
            "user_login": "glowillig",
            "user_name": "glowillig"
        }
        },
        {
        "id": "1IVBTP7gG61oXLMu7fvnRhrpsro",
        "event_type": "moderation.moderator.remove",
        "event_timestamp": "2019-03-15T18:18:11Z",
        "version": "1.0",
        "event_data": {
            "broadcaster_id": "198704263",
            "broadcaster_login": "aan22209",
            "broadcaster_name": "aan22209",
            "user_id": "424596340",
            "user_login": "quotrok",
            "user_name": "quotrok"
        }
        }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
    }
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/moderators/events?broadcaster_id=198704263"
    );

    dbg!(GetModeratorEventsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

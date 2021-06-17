//! Gets all scheduled broadcasts or specific scheduled broadcasts from a channel’s stream schedule. Scheduled broadcasts are defined as “stream segments” in the API.
//! [`get-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference#get-channel-stream-schedule)
//!
//! ## Request: [GetChannelStreamScheduleRequest]
//!
//! To use this endpoint, construct a [`GetChannelStreamScheduleRequest`] with the [`GetChannelStreamScheduleRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::predictions::get_channel_stream_schedule;
//! let request = get_channel_stream_schedule::GetChannelStreamScheduleRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [ScheduledBroadcasts]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, predictions::get_channel_stream_schedule};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_channel_stream_schedule::GetChannelStreamScheduleRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_channel_stream_schedule::ScheduledBroadcasts> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelStreamScheduleRequest::parse_response(None, &request.get_uri(), response)`](GetChannelStreamScheduleRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get predictions](super::get_channel_stream_schedule)
///
/// [`get-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference#get-channel-stream-schedule)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetChannelStreamScheduleRequest {
    /// User ID of the broadcaster who owns the channel streaming schedule. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// The ID of the stream segment to return. Maximum: 100.
    #[builder(default, setter(into))]
    pub id: Option<types::StreamSegmentId>,
    /// A timestamp in RFC3339 format to start returning stream segments from. If not specified, the current date and time is used.
    #[builder(default, setter(into))]
    pub start_time: Option<types::Timestamp>,
    /// A timezone offset for the requester specified in minutes. This is recommended to ensure stream segments are returned for the correct week. For example, a timezone that is +4 hours from GMT would be “240.” If not specified, “0” is used for GMT.
    #[builder(default, setter(into))]
    pub utc_offset: Option<String>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
    /// Maximum number of stream segments to return. Maximum: 25. Default: 20.
    #[builder(default, setter(into))]
    pub first: Option<usize>,
}

/// Return Values for [Get predictions](super::get_channel_stream_schedule)
///
/// [`get-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference#get-channel-stream-schedule)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ScheduledBroadcasts {
    /// Scheduled broadcasts for this stream schedule.
    pub segments: Vec<Segment>,
    /// User ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// Display name of the broadcaster.
    pub broadcaster_name: types::DisplayName,
    /// Login of the broadcaster.
    pub broadcaster_login: types::UserName,
    /// If Vacation Mode is enabled, this includes start and end dates for the vacation. If Vacation Mode is disabled, value is set to null.
    pub vacation: Option<Vacation>,
}

/// Scheduled broadcast for a stream schedule.
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Segment {
    /// The ID for the scheduled broadcast.
    pub id: types::StreamSegmentId,
    /// Scheduled start time for the scheduled broadcast in RFC3339 format.
    pub start_time: types::Timestamp,
    /// Scheduled end time for the scheduled broadcast in RFC3339 format.
    pub end_time: types::Timestamp,
    /// Title for the scheduled broadcast.
    pub title: String,
    /// Used with recurring scheduled broadcasts. Specifies the date of the next recurring broadcast in RFC3339 format if one or more specific broadcasts have been deleted in the series. Set to null otherwise.
    pub canceled_until: Option<types::Timestamp>,
    /// The category for the scheduled broadcast. Set to null if no category has been specified.
    pub category: Option<Category>,
    /// Indicates if the scheduled broadcast is recurring weekly.
    pub is_recurring: bool,
}

// FIXME: Similar to types::TwitchCategory, missing box_art
/// The category for a scheduled broadcast.
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Category {
    /// Game/category ID.
    pub id: types::CategoryId,
    /// Game/category name.
    pub name: String,
}

/// Information on Vacation Mode
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Vacation {
    /// Start time for vacation specified in RFC3339 format.
    pub start_time: types::Timestamp,
    /// End time for vacation specified in RFC3339 format.
    pub end_time: types::Timestamp,
}

impl Request for GetChannelStreamScheduleRequest {
    type Response = ScheduledBroadcasts;

    const PATH: &'static str = "schedule";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetChannelStreamScheduleRequest {}

impl helix::Paginated for GetChannelStreamScheduleRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor; }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelStreamScheduleRequest::builder()
        .broadcaster_id("141981764")
        .build();

    // From twitch docs. FIXME: Docs has ...
    let data = br#"
    {
        "data": {
          "segments": [
            {
              "id": "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=",
              "start_time": "2021-07-01T18:00:00Z",
              "end_time": "2021-07-01T19:00:00Z",
              "title": "TwitchDev Monthly Update // July 1, 2021",
              "canceled_until": null,
              "category": {
                  "id": "509670",
                  "name": "Science & Technology"
              },
              "is_recurring": false
            }
          ],
          "broadcaster_id": "141981764",
          "broadcaster_name": "TwitchDev",
          "broadcaster_login": "twitchdev",
          "vacation": null
        },
        "pagination": {}
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/schedule?broadcaster_id=141981764"
    );

    dbg!(GetChannelStreamScheduleRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

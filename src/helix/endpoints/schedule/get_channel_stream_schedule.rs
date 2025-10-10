//! Gets all scheduled broadcasts or specific scheduled broadcasts from a channel’s stream schedule. Scheduled broadcasts are defined as “stream segments” in the API.
//!
//! [`get-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference#get-channel-stream-schedule)
//!
//! ## Notes
//!
//! See also [`get_channel_schedule`](helix::HelixClient::get_channel_schedule)
//!
//! ## Request: [GetChannelStreamScheduleRequest]
//!
//! To use this endpoint, construct a [`GetChannelStreamScheduleRequest`] with the [`GetChannelStreamScheduleRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::schedule::get_channel_stream_schedule;
//! let request =
//!     get_channel_stream_schedule::GetChannelStreamScheduleRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [ScheduledBroadcasts]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, schedule::get_channel_stream_schedule};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_channel_stream_schedule::GetChannelStreamScheduleRequest::broadcaster_id("1234");
//! let response: helix::schedule::ScheduledBroadcasts = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelStreamScheduleRequest::parse_response(None, &request.get_uri(), response)`](GetChannelStreamScheduleRequest::parse_response)

use super::*;
use helix::{PaginationState, RequestGet};

/// Query Parameters for [Get Channel Stream Schedule](super::get_channel_stream_schedule)
///
/// [`get-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference#get-channel-stream-schedule)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetChannelStreamScheduleRequest<'a> {
    /// User ID of the broadcaster who owns the channel streaming schedule. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the stream segment to return. Maximum: 100.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub id: Option<Cow<'a, types::StreamSegmentIdRef>>,
    /// A timestamp in RFC3339 format to start returning stream segments from. If not specified, the current date and time is used.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub start_time: Option<Cow<'a, types::TimestampRef>>,
    /// A timezone offset for the requester specified in minutes. This is recommended to ensure stream segments are returned for the correct week. For example, a timezone that is +4 hours from GMT would be “240.” If not specified, “0” is used for GMT.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub utc_offset: Option<Cow<'a, str>>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Maximum number of stream segments to return. Maximum: 25. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
}

impl<'a> GetChannelStreamScheduleRequest<'a> {
    /// Get a broadcasters schedule
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            id: Default::default(),
            start_time: Default::default(),
            utc_offset: Default::default(),
            after: Default::default(),
            first: Default::default(),
        }
    }

    /// Set the id for the request.
    pub fn id(mut self, id: impl types::IntoCow<'a, types::StreamSegmentIdRef> + 'a) -> Self {
        self.id = Some(id.into_cow());
        self
    }

    /// Set the start_time for the request.
    pub fn start_time(
        mut self,
        start_time: impl types::IntoCow<'a, types::TimestampRef> + 'a,
    ) -> Self {
        self.start_time = Some(start_time.into_cow());
        self
    }

    /// Set the utc_offset for the request.
    pub fn utc_offset(mut self, utc_offset: impl Into<Cow<'a, str>>) -> Self {
        self.utc_offset = Some(utc_offset.into());
        self
    }

    /// Set amount of results returned per page.
    pub const fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Channel Stream Schedule](super::get_channel_stream_schedule)
///
/// [`get-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference#get-channel-stream-schedule)
pub type GetChannelStreamScheduleResponse = ScheduledBroadcasts;

impl Request for GetChannelStreamScheduleRequest<'_> {
    type PaginationData = PaginationState<Self>;
    type Response = ScheduledBroadcasts;

    const PATH: &'static str = "schedule";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetChannelStreamScheduleRequest<'_> {}

impl helix::Paginated for GetChannelStreamScheduleRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelStreamScheduleRequest::broadcaster_id("141981764");

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

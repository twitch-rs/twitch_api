//! Create a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
//! [`create-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreateChannelStreamScheduleSegmentRequest]
//!
//! To use this endpoint, construct a [`CreateChannelStreamScheduleSegmentRequest`] with the [`CreateChannelStreamScheduleSegmentRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::schedule::create_channel_stream_schedule_segment;
//! let request = create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentRequest::new();
//! ```
//!
//! ## Body: [CreateChannelStreamScheduleSegmentBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::schedule::create_channel_stream_schedule_segment;
//! let body = create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentBody::builder()
//!     .action(true)
//!     .user_id("9327994")
//!     .msg_id("836013710")
//!     .build();
//! ```
//!
//! ## Response: [CreateChannelStreamScheduleSegment]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, schedule::create_channel_stream_schedule_segment};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentRequest::new();
//! let body = create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentBody::builder()
//!     .action(true)
//!     .user_id("9327994")
//!     .msg_id("836013710")
//!     .build();
//! let response: create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegment = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreateChannelStreamScheduleSegmentRequest::parse_response(None, &request.get_uri(), response)`](CreateChannelStreamScheduleSegmentRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Create Channel Stream Schedule Segment](super::create_channel_stream_schedule_segment)
///
/// [`create-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreateChannelStreamScheduleSegmentRequest {
    /// User ID of the broadcaster who owns the channel streaming schedule. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
}

/// Body Parameters for [Create Channel Stream Schedule Segment](super::create_channel_stream_schedule_segment)
///
/// [`create-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreateChannelStreamScheduleSegmentBody {
    /// Start time for the scheduled broadcast specified in RFC3339 format.
    #[builder(setter(into))]
    pub start_time: types::Timestamp,
    // FIXME: specific braid?
    /// The timezone of the application creating the scheduled broadcast using the IANA time zone database format.
    #[builder(setter(into))]
    pub timezone: String,
    /// Indicates if the scheduled broadcast is recurring weekly.
    pub is_recurring: bool,
    /// Duration of the scheduled broadcast in minutes from the start_time. Default: 240.
    #[builder(default, setter(into))]
    pub duration: Option<String>,
    /// Game/Category ID for the scheduled broadcast.
    #[builder(default, setter(into))]
    pub category_id: Option<types::CategoryId>,
    /// Title for the scheduled broadcast. Maximum: 140 characters.
    #[builder(default, setter(into))]
    pub title: Option<String>,
}

impl helix::private::SealedSerialize for CreateChannelStreamScheduleSegmentBody {}

/// Return Values for [Create Channel Stream Schedule Segment](super::create_channel_stream_schedule_segment)
///
/// [`create-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
pub type CreateChannelStreamScheduleSegmentResponse = ScheduledBroadcasts;

impl Request for CreateChannelStreamScheduleSegmentRequest {
    type Response = CreateChannelStreamScheduleSegmentResponse;

    const PATH: &'static str = "schedule/segment";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageSchedule];
}

impl RequestPost for CreateChannelStreamScheduleSegmentRequest {
    type Body = CreateChannelStreamScheduleSegmentBody;
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = CreateChannelStreamScheduleSegmentRequest::builder()
        .broadcaster_id("141981764")
        .build();

    let body = CreateChannelStreamScheduleSegmentBody::builder()
        .start_time("2021-07-01T18:00:00Z")
        .timezone("America/New_York")
        .is_recurring(false)
        .duration("60".to_string())
        .category_id(Some("509670".into()))
        .title("TwitchDev Monthly Update // July 1, 2021".to_string())
        .build();

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
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
        }
      }
    "#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/schedule/segment?broadcaster_id=141981764"
    );

    dbg!(
        CreateChannelStreamScheduleSegmentRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
    );
}

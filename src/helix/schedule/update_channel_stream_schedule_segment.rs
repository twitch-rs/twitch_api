//! Update a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
//! [`update-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateChannelStreamScheduleSegmentRequest]
//!
//! To use this endpoint, construct a [`UpdateChannelStreamScheduleSegmentRequest`] with the [`UpdateChannelStreamScheduleSegmentRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::schedule::update_channel_stream_schedule_segment;
//! let request = update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentRequest::builder()
//!     .broadcaster_id("141981764")
//!     .id("eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=")
//!     .build();
//! ```
//!
//! ## Body: [UpdateChannelStreamScheduleSegmentBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::schedule::update_channel_stream_schedule_segment;
//! let body =
//!     update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentBody::builder()
//!         .duration("120".to_string())
//!         .build();
//! ```
//!
//! ## Response: [UpdateChannelStreamScheduleSegmentResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, schedule::update_channel_stream_schedule_segment};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentRequest::builder()
//!     .broadcaster_id("141981764")
//!     .id("eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=")
//!     .build();
//! let body = update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentBody::builder()
//!     .duration("120".to_string())
//!     .build();
//! let response: update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentResponse = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&body, &token, &client_id)`](helix::RequestPatch::create_request)
//! and parse the [`http::Response`] with [`UpdateChannelStreamScheduleSegmentRequest::parse_response(None, &request.get_uri(), response)`](UpdateChannelStreamScheduleSegmentRequest::parse_response)

use super::*;
use helix::RequestPatch;
/// Query Parameters for [Update Channel Stream Schedule Segment](super::update_channel_stream_schedule_segment)
///
/// [`update-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateChannelStreamScheduleSegmentRequest {
    /// User ID of the broadcaster who owns the channel streaming schedule. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// The ID of the streaming segment to update.
    #[builder(setter(into))]
    pub id: types::StreamSegmentId,
}

/// Body Parameters for [Update Channel Stream Schedule Segment](super::update_channel_stream_schedule_segment)
///
/// [`update-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateChannelStreamScheduleSegmentBody {
    /// Start time for the scheduled broadcast specified in RFC3339 format.
    #[builder(default, setter(into))]
    pub start_time: Option<String>,
    /// Duration of the scheduled broadcast in minutes from the start_time. Default: 240.
    #[builder(default, setter(into))]
    pub duration: Option<String>,
    /// Game/Category ID for the scheduled broadcast.
    #[builder(default, setter(into))]
    pub category_id: Option<String>,
    /// Title for the scheduled broadcast. Maximum: 140 characters.
    #[builder(default, setter(into))]
    pub title: Option<String>,
    /// Indicated if the scheduled broadcast is canceled.
    #[builder(default, setter(into))]
    pub is_canceled: Option<bool>,
    // FIXME: Enum?
    /// The timezone of the application creating the scheduled broadcast using the IANA time zone database format.
    #[builder(default, setter(into))]
    pub timezone: Option<String>,
}

impl helix::private::SealedSerialize for UpdateChannelStreamScheduleSegmentBody {}

/// Return Values for [Update Channel Stream Schedule Segment](super::update_channel_stream_schedule_segment)
///
/// [`update-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
pub type UpdateChannelStreamScheduleSegmentResponse = ScheduledBroadcasts;

impl Request for UpdateChannelStreamScheduleSegmentRequest {
    type Response = UpdateChannelStreamScheduleSegmentResponse;

    const PATH: &'static str = "schedule/segment";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageSchedule];
}

impl RequestPatch for UpdateChannelStreamScheduleSegmentRequest {
    type Body = UpdateChannelStreamScheduleSegmentBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPatchError>
    where
        Self: Sized,
    {
        let response: helix::InnerResponse<<Self as Request>::Response> =
            helix::parse_json(response, true).map_err(|e| {
                helix::HelixRequestPatchError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response {
            data: response.data,
            pagination: response.pagination.cursor,
            request,
            total: response.total,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = UpdateChannelStreamScheduleSegmentRequest::builder()
        .broadcaster_id("141981764")
        .id("eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=")
        .build();

    let body = UpdateChannelStreamScheduleSegmentBody::builder()
        .duration("120".to_string())
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
              "end_time": "2021-07-01T20:00:00Z",
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

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/schedule/segment?broadcaster_id=141981764&id=eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0%3D"
    );

    dbg!(
        UpdateChannelStreamScheduleSegmentRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
    );
}

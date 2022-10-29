//! Update a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
//! [`update-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateChannelStreamScheduleSegmentRequest]
//!
//! To use this endpoint, construct a [`UpdateChannelStreamScheduleSegmentRequest`] with the [`UpdateChannelStreamScheduleSegmentRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::schedule::update_channel_stream_schedule_segment;
//! let request = update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentRequest::new(
//!     "141981764",
//!     "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=",
//! );
//! ```
//!
//! ## Body: [UpdateChannelStreamScheduleSegmentBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::schedule::update_channel_stream_schedule_segment;
//! let body =
//!     update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentBody::builder()
//!         .duration("120")
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
//! use twitch_api::helix::{self, schedule::update_channel_stream_schedule_segment};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentRequest::new(
//!     "141981764",
//!     "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=",
//! );
//! let body = update_channel_stream_schedule_segment::UpdateChannelStreamScheduleSegmentBody::builder()
//!     .duration("120")
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
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateChannelStreamScheduleSegmentRequest<'a> {
    /// User ID of the broadcaster who owns the channel streaming schedule. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the streaming segment to update.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub id: Cow<'a, types::StreamSegmentIdRef>,
}

impl<'a> UpdateChannelStreamScheduleSegmentRequest<'a> {
    /// Update a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        id: impl types::IntoCow<'a, types::StreamSegmentIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
            id: id.to_cow(),
        }
    }
}

/// Body Parameters for [Update Channel Stream Schedule Segment](super::update_channel_stream_schedule_segment)
///
/// [`update-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateChannelStreamScheduleSegmentBody<'a> {
    /// Start time for the scheduled broadcast specified in RFC3339 format.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub start_time: Option<&'a str>,
    /// Duration of the scheduled broadcast in minutes from the start_time. Default: 240.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub duration: Option<&'a str>,
    /// Game/Category ID for the scheduled broadcast.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub category_id: Option<Cow<'a, types::CategoryIdRef>>,
    /// Title for the scheduled broadcast. Maximum: 140 characters.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub title: Option<&'a str>,
    /// Indicated if the scheduled broadcast is canceled.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_canceled: Option<bool>,
    // FIXME: Enum?
    /// The timezone of the application creating the scheduled broadcast using the IANA time zone database format.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub timezone: Option<&'a str>,
}

impl helix::private::SealedSerialize for UpdateChannelStreamScheduleSegmentBody<'_> {}

/// Return Values for [Update Channel Stream Schedule Segment](super::update_channel_stream_schedule_segment)
///
/// [`update-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment)
pub type UpdateChannelStreamScheduleSegmentResponse = ScheduledBroadcasts;

impl Request for UpdateChannelStreamScheduleSegmentRequest<'_> {
    type Response = UpdateChannelStreamScheduleSegmentResponse;

    const PATH: &'static str = "schedule/segment";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageSchedule];
}

impl<'a> RequestPatch for UpdateChannelStreamScheduleSegmentRequest<'a> {
    type Body = UpdateChannelStreamScheduleSegmentBody<'a>;

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
            other: None,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = UpdateChannelStreamScheduleSegmentRequest::new(
        "141981764",
        "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=");

    let body = UpdateChannelStreamScheduleSegmentBody {
        duration: Some("120"),
        ..<_>::default()
    };

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

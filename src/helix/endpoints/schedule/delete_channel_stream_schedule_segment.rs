//! Delete a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
//! [`delete-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#delete-channel-stream-schedule-segment)
//!
//! # Notes
//!
//! # Accessing the endpoint
//!
//! ## Request: [DeleteChannelStreamScheduleSegmentRequest]
//!
//! To use this endpoint, construct a [`DeleteChannelStreamScheduleSegmentRequest`] with the [`DeleteChannelStreamScheduleSegmentRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::schedule::delete_channel_stream_schedule_segment;
//! let request = delete_channel_stream_schedule_segment::DeleteChannelStreamScheduleSegmentRequest::new(
//!     "1234",
//!     "eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0="
//! );
//! ```
//!
//! ## Response: [DeleteChannelStreamScheduleSegment]
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, schedule::delete_channel_stream_schedule_segment};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = delete_channel_stream_schedule_segment::DeleteChannelStreamScheduleSegmentRequest::new(
//!     "1234",
//!     "eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0="
//! );
//! let response: delete_channel_stream_schedule_segment::DeleteChannelStreamScheduleSegment = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`DeleteChannelStreamScheduleSegmentRequest::parse_response(None, &request.get_uri(), response)`](DeleteChannelStreamScheduleSegmentRequest::parse_response)

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Delete Channel Stream Schedule Segment](super::delete_channel_stream_schedule_segment)
///
/// [`delete-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#delete-channel-stream-schedule-segment)
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct DeleteChannelStreamScheduleSegmentRequest<'a> {
    /// User ID of the broadcaster who owns the channel streaming schedule. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the streaming segment to delete.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub id: Cow<'a, types::StreamSegmentIdRef>,
}

impl<'a> DeleteChannelStreamScheduleSegmentRequest<'a> {
    /// Delete a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        id: impl types::IntoCow<'a, types::StreamSegmentIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            id: id.into_cow(),
        }
    }
}

/// Return Values for [Delete Channel Stream Schedule Segment](super::delete_channel_stream_schedule_segment)
///
/// [`delete-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#delete-channel-stream-schedule-segment)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum DeleteChannelStreamScheduleSegment {
    /// 204 - User successfully deleted from list of channel followers
    Success,
}

impl Request for DeleteChannelStreamScheduleSegmentRequest<'_> {
    type Response = DeleteChannelStreamScheduleSegment;

    const PATH: &'static str = "schedule/segment";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelManageSchedule];
}

impl RequestDelete for DeleteChannelStreamScheduleSegmentRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(helix::Response {
                data: DeleteChannelStreamScheduleSegment::Success,
                pagination: None,
                request,
                total: None,
                other: None,
            }),
            _ => Err(helix::HelixRequestDeleteError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = DeleteChannelStreamScheduleSegmentRequest::new("41245071", "eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0=");

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();
    // FIXME: I have not tested this in production

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/schedule/segment?broadcaster_id=41245071&id=eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0%3D"
    );

    dbg!(
        DeleteChannelStreamScheduleSegmentRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
    );
}

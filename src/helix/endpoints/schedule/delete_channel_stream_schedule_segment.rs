//! Delete a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
//! [`delete-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#delete-channel-stream-schedule-segment)
//!
//! # Notes
//!
//! This doesn't seem to work for removing people who follow owner of token. Use [Block User](crate::helix::schedule::block_user) for that
//!
//! # Accessing the endpoint
//!
//! ## Request: [DeleteChannelStreamScheduleSegmentRequest]
//!
//! To use this endpoint, construct a [`DeleteChannelStreamScheduleSegmentRequest`] with the [`DeleteChannelStreamScheduleSegmentRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::schedule::delete_channel_stream_schedule_segment;
//! let request = delete_channel_stream_schedule_segment::DeleteChannelStreamScheduleSegmentRequest::builder()
//!     .from_id("1234").to_id("4321")
//!     .build();
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
//! let request = delete_channel_stream_schedule_segment::DeleteChannelStreamScheduleSegmentRequest::builder()
//!     .from_id("1234").to_id("4321")
//!     .build();
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
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct DeleteChannelStreamScheduleSegmentRequest {
    /// User ID of the follower
    #[builder(setter(into))]
    pub from_id: types::UserId,
    /// Channel to be unfollowed by the user
    #[builder(setter(into))]
    pub to_id: types::UserId,
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

impl Request for DeleteChannelStreamScheduleSegmentRequest {
    type Response = DeleteChannelStreamScheduleSegment;

    const PATH: &'static str = "schedule/segment";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageSchedule];
}

impl RequestDelete for DeleteChannelStreamScheduleSegmentRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = DeleteChannelStreamScheduleSegmentRequest::builder()
        .to_id("41245072".to_string())
        .from_id("41245071".to_string())
        .build();

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();
    // FIXME: I have not tested this in production

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/schedule/follows?from_id=41245071&to_id=41245072"
    );

    dbg!(
        DeleteChannelStreamScheduleSegmentRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
    );
}

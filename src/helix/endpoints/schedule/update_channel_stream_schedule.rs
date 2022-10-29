//! Update the settings for a channel’s stream schedule. This can be used for setting vacation details.
//!
//! [`update-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateChannelStreamScheduleRequest]
//!
//! To use this endpoint, construct an [`UpdateChannelStreamScheduleRequest`] with the [`UpdateChannelStreamScheduleRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::schedule::update_channel_stream_schedule;
//! let request = update_channel_stream_schedule::UpdateChannelStreamScheduleRequest::builder()
//!     .broadcaster_id("274637212")
//!     .build();
//! ```
//!
//! ## Response: [UpdateChannelStreamSchedule]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, schedule::update_channel_stream_schedule};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_channel_stream_schedule::UpdateChannelStreamScheduleRequest::builder()
//!     .broadcaster_id("274637212")
//!     .is_vacation_enabled(false)
//!     .build();
//! let body = helix::EmptyBody;
//! let response: update_channel_stream_schedule::UpdateChannelStreamSchedule = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`UpdateChannelStreamScheduleRequest::parse_response(None, &request.get_uri(), response)`](UpdateChannelStreamScheduleRequest::parse_response)
use super::*;
use helix::RequestPatch;
/// Query Parameters for [Update Channel Stream Schedule](super::update_channel_stream_schedule)
///
/// [`update-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateChannelStreamScheduleRequest<'a> {
    /// User ID of the broadcaster who owns the channel streaming schedule. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Indicates if Vacation Mode is enabled. Set to true to add a vacation or false to remove vacation from the channel streaming schedule.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub is_vacation_enabled: Option<bool>,
    /// Start time for vacation specified in RFC3339 format. Required if is_vacation_enabled is set to true.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(borrow)]
    pub vacation_start_time: Option<Cow<'a, types::TimestampRef>>,
    /// End time for vacation specified in RFC3339 format. Required if is_vacation_enabled is set to true.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(borrow)]
    pub vacation_end_time: Option<Cow<'a, types::TimestampRef>>,
    /// The timezone for when the vacation is being scheduled using the IANA time zone database format. Required if is_vacation_enabled is set to true.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(borrow)]
    pub timezone: Option<&'a str>,
}

impl<'a> UpdateChannelStreamScheduleRequest<'a> {
    /// Update the settings for a channel’s stream schedule.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
            is_vacation_enabled: Default::default(),
            vacation_start_time: Default::default(),
            vacation_end_time: Default::default(),
            timezone: Default::default(),
        }
    }
}

/// Return Values for [Update Channel Stream Schedule](super::update_channel_stream_schedule)
///
/// [`update-channel-stream-schedule`](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum UpdateChannelStreamSchedule {
    /// Stream schedule settings updated successfully.
    Success,
}

impl Request for UpdateChannelStreamScheduleRequest<'_> {
    type Response = UpdateChannelStreamSchedule;

    const PATH: &'static str = "schedule/settings";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageSchedule];
}

impl RequestPatch for UpdateChannelStreamScheduleRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPatchError>
    where
        Self: Sized,
    {
        let resp = match status {
            http::StatusCode::OK | http::StatusCode::NO_CONTENT => {
                UpdateChannelStreamSchedule::Success
            }
            _ => {
                return Err(helix::HelixRequestPatchError::InvalidResponse {
                    reason: "unexpected status code",
                    response: response.to_string(),
                    status,
                    uri: uri.clone(),
                })
            }
        };
        Ok(helix::Response {
            data: resp,
            pagination: None,
            request,
            total: None,
            other: None,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use std::convert::TryFrom;

    use helix::*;

    let start = types::Timestamp::try_from("2021-05-16T00:00:00Z").unwrap();
    let end = types::Timestamp::try_from("2021-05-23T00:00:00Z").unwrap();
    let req = UpdateChannelStreamScheduleRequest {
        is_vacation_enabled: Some(true),
        vacation_start_time: Some(types::IntoCow::to_cow(&start)),
        vacation_end_time: Some(types::IntoCow::to_cow(&end)),
        timezone: Some("America/New_York"),
        ..UpdateChannelStreamScheduleRequest::broadcaster_id("141981764")
    };

    let body = helix::EmptyBody;

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br"".to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/schedule/settings?broadcaster_id=141981764&is_vacation_enabled=true&vacation_start_time=2021-05-16T00%3A00%3A00Z&vacation_end_time=2021-05-23T00%3A00%3A00Z&timezone=America%2FNew_York"
    );

    dbg!(
        UpdateChannelStreamScheduleRequest::parse_response(Some(req), &uri, http_response).unwrap()
    );
}

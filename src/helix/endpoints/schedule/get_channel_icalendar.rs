//! Gets the broadcaster’s streaming schedule as an [iCalendar](https://datatracker.ietf.org/doc/html/rfc5545).
//! [`get-channel-icalendar`](https://dev.twitch.tv/docs/api/reference#get-channel-icalendar)
//!
//! ## Notes
//!
//! See also [`get_channel_schedule`](helix::HelixClient::get_channel_schedule)
//!
//! ## Request: [GetChannelICalendar]
//!
//! To use this endpoint, construct a [`GetChannelICalendar`] with the [`GetChannelICalendar::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::schedule::get_channel_icalendar;
//! let request =
//!     get_channel_icalendar::GetChannelICalendar::broadcaster_id("1234");
//! ```
//!
//! ## Response: [String]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, schedule::get_channel_icalendar};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_channel_icalendar::GetChannelICalendar::broadcaster_id("1234");
//! let response: String = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelICalendar::parse_response(None, &request.get_uri(), response)`](GetChannelICalendar::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel iCalendar](super::get_channel_icalendar)
///
/// [`get-channel-icalendar`](https://dev.twitch.tv/docs/api/reference#get-channel-icalendar)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetChannelICalendar<'a> {
    /// The ID of the broadcaster that owns the streaming schedule you want to get.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetChannelICalendar<'a> {
    /// Get a broadcasters schedule as an iCalendar
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Channel iCalendar](super::get_channel_icalendar)
///
/// [`get-channel-icalendar`](https://dev.twitch.tv/docs/api/reference#get-channel-icalendar)
pub type GetChannelICalendarResponse = ScheduledBroadcasts;

impl Request for GetChannelICalendar<'_> {
    type Response = String;

    const PATH: &'static str = "schedule/icalendar";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetChannelICalendar<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        _uri: &http::Uri,
        response: &str,
        _status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        Ok(helix::Response::new(
            response.to_owned(),
            None,
            request,
            None,
            None,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelICalendar::broadcaster_id("141981764");

    let data = br#"BEGIN:VCALENDAR
PRODID:-//twitch.tv//StreamSchedule//1.0
VERSION:2.0
CALSCALE:GREGORIAN
REFRESH-INTERVAL;VALUE=DURATION:PT1H
NAME:TwitchDev
BEGIN:VEVENT
UID:e4acc724-371f-402c-81ca-23ada79759d4
DTSTAMP:20210323T040131Z
DTSTART;TZID=/America/New_York:20210701T140000
DTEND;TZID=/America/New_York:20210701T150000
SUMMARY:TwitchDev Monthly Update // July 1, 2021
DESCRIPTION:Science & Technology.
CATEGORIES:Science & Technology
END:VEVENT
END:VCALENDAR%"#
        .to_vec();

    let http_response = http::Response::builder().body(data.clone()).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/schedule/icalendar?broadcaster_id=141981764"
    );

    let res = GetChannelICalendar::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.as_bytes(), data);
}

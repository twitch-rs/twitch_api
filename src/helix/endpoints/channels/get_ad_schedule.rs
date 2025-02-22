//! Gets ad schedule related information, including snooze, when the last ad was run, when the next ad is scheduled, and if the channel is currently in pre-roll free time.
//!
//! [`get-ad-schedule`](https://dev.twitch.tv/docs/api/reference#get-ad-schedule)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetAdScheduleRequest]
//!
//! To use this endpoint, construct a [`GetAdScheduleRequest`] with the [`GetAdScheduleRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::get_ad_schedule;
//! let request = get_ad_schedule::GetAdScheduleRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [AdSchedule]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::get_ad_schedule};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_ad_schedule::GetAdScheduleRequest::broadcaster_id("1234");
//! let response: Option<get_ad_schedule::AdSchedule> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetAdScheduleRequest::parse_response(None, &request.get_uri(), response)`](GetAdScheduleRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Ad Schedule](super::get_ad_schedule)
///
/// [`get-ad-schedule`](https://dev.twitch.tv/docs/api/reference#get-ad-schedule)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetAdScheduleRequest<'a> {
    /// Broadcaster’s user ID associated with the channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetAdScheduleRequest<'a> {
    /// Get specified broadcasters ad schedule information
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::channels::GetAdScheduleRequest;
    /// let request = GetAdScheduleRequest::broadcaster_id("1234");
    /// ```
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Ad Schedule](super::get_ad_schedule)
///
/// [`get-ad-schedule`](https://dev.twitch.tv/docs/api/reference#get-ad-schedule)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AdSchedule {
    /// The number of snoozes available for the broadcaster.
    pub snooze_count: i32,
    /// The UTC timestamp when the broadcaster will gain an additional snooze, in RFC3339 format.
    pub snooze_refresh_at: types::Timestamp,
    /// The UTC timestamp of the broadcaster’s next scheduled ad, in RFC3339 format. Empty if the channel has no ad scheduled or is not live.
    #[serde(
        default,
        deserialize_with = "helix::deserialize_none_from_empty_or_zero_string"
    )]
    pub next_ad_at: Option<types::Timestamp>,
    /// The length in seconds of the scheduled upcoming ad break.
    pub duration: i32, /* TODO: Is this a types::CommercialLength? is it 0 if no ad is scheduled? */
    /// The UTC timestamp of the broadcaster’s last ad-break, in RFC3339 format. Empty if the channel has not run an ad or is not live.
    pub last_ad_at: types::Timestamp,
    /// The amount of pre-roll free time remaining for the channel in seconds. Returns 0 if they are currently not pre-roll free.
    #[serde(
        default,
        deserialize_with = "helix::deserialize_none_from_empty_or_zero_string"
    )]
    pub preroll_free_time: Option<i32>,
}

impl Request for GetAdScheduleRequest<'_> {
    type Response = Option<AdSchedule>;

    const PATH: &'static str = "channels/ads";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadAds,
        twitch_oauth2::Scope::ChannelManageAds
    )];
}

impl RequestGet for GetAdScheduleRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        str_response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        let response: Result<helix::InnerResponse<Vec<_>>, _> =
            crate::parse_json(str_response, true).map_err(|e| {
                helix::HelixRequestGetError::DeserializeError(
                    str_response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            });
        let response = match response {
            Ok(resp) => resp,
            Err(error) => {
                #[derive(PartialEq, Eq, Deserialize, Default)]
                struct IsWrong {
                    snooze_count: i32,
                    snooze_refresh_at: i32,
                    next_ad_at: i32,
                    duration: i32,
                    last_ad_at: i32,
                    preroll_free_time: i32,
                }
                let fake: Result<helix::InnerResponse<Vec<IsWrong>>, _> =
                    crate::parse_json(str_response, true);
                // if all fields are 0 or we couldn't parse it
                let empty = match fake {
                    Ok(fake) if fake.data.first() == Some(&IsWrong::default()) => fake,
                    _ => {
                        // return the original error
                        return Err(error);
                    }
                };
                helix::InnerResponse {
                    data: vec![],
                    pagination: empty.pagination,
                    total: empty.total,
                    other: empty.other,
                }
            }
        };
        Ok(helix::Response::new(
            response.data.into_iter().next(),
            response.pagination.cursor,
            request,
            response.total,
            response.other,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetAdScheduleRequest::broadcaster_id("123");

    // From twitch docs
    // FIXME: twitch docs sucks https://github.com/twitchdev/issues/issues/857, also trailing comma
    let data = br#"
    {
        "data": [
          {
            "next_ad_at" : "2023-08-01T23:08:18+00:00",
            "last_ad_at" : "2023-08-01T23:08:18+00:00",
            "duration" : 60,
            "preroll_free_time" : 90,
            "snooze_count" : 1,
            "snooze_refresh_at" : "2023-08-01T23:08:18+00:00"
          }
        ]
      }
        "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/ads?broadcaster_id=123"
    );

    dbg!(GetAdScheduleRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_empty_wrong() {
    use helix::*;
    let req = GetAdScheduleRequest::broadcaster_id("123");

    // From twitch docs
    let data = br#"{
        "data": [
          {
            "duration": 0,
            "last_ad_at": 0,
            "next_ad_at": 0,
            "preroll_free_time": 0,
            "snooze_count": 0,
            "snooze_refresh_at": 0
          }
        ]
      }"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/ads?broadcaster_id=123"
    );

    assert_eq!(
        GetAdScheduleRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
            .data,
        None
    );
}

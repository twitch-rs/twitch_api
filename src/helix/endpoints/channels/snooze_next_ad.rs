//! If available, pushes back the timestamp of the upcoming automatic mid-roll ad by 5 minutes.
//! This endpoint duplicates the snooze functionality in the creator dashboard’s Ads Manager.
//! [`snooze-next-ad`](https://dev.twitch.tv/docs/api/reference#snooze-next-ad)
//!
//! # Accessing the endpoint
//!
//! ## Request: [SnoozeNextAdRequest]
//!
//! To use this endpoint, construct a [`SnoozeNextAdRequest`] with the [`SnoozeNextAdRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::snooze_next_ad;
//! let request = snooze_next_ad::SnoozeNextAdRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [SnoozedAdSchedule]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::snooze_next_ad};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = snooze_next_ad::SnoozeNextAdRequest::broadcaster_id("1234");
//! let response: snooze_next_ad::SnoozedAdSchedule = client.req_post(request, helix::EmptyBody, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`SnoozeNextAdRequest::parse_response(None, &request.get_uri(), response)`](SnoozeNextAdRequest::parse_response)
use super::*;
use helix::RequestPost;

/// Query Parameters for [Snooze Next Ad](super::snooze_next_ad)
///
/// [`snooze-next-ad`](https://dev.twitch.tv/docs/api/reference#snooze-next-ad)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct SnoozeNextAdRequest<'a> {
    /// ID of the channel
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> SnoozeNextAdRequest<'a> {
    /// Modify specified broadcasters channel
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::channels::SnoozeNextAdRequest;
    ///
    /// let request = SnoozeNextAdRequest::broadcaster_id("1337");
    /// ```
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        SnoozeNextAdRequest {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Snooze Next Ad](super::snooze_next_ad)
///
/// [`snooze-next-ad`](https://dev.twitch.tv/docs/api/reference#snooze-next-ad)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct SnoozedAdSchedule {
    /// The number of snoozes available for the broadcaster.
    pub snooze_count: i32,
    /// The UTC timestamp when the broadcaster will gain an additional snooze, in RFC3339 format.
    pub snooze_refresh_at: types::Timestamp,
    /// The UTC timestamp of the broadcaster’s next scheduled ad, in RFC3339 format.
    pub next_ad_at: types::Timestamp,
}

impl Request for SnoozeNextAdRequest<'_> {
    type Response = SnoozedAdSchedule;

    const PATH: &'static str = "channels/ads/schedule/snooze";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelManageAds];
}

impl<'a> RequestPost for SnoozeNextAdRequest<'a> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        let resp: helix::InnerResponse<Vec<_>> =
            crate::parse_json(response, true).map_err(|e| {
                helix::HelixRequestPostError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response::new(
            resp.data
                .into_iter()
                .next()
                .ok_or(helix::HelixRequestPostError::InvalidResponse {
                    reason: "expected an entry in `data`",
                    response: response.to_string(),
                    status,
                    uri: uri.clone(),
                })?,
            resp.pagination.cursor,
            request,
            resp.total,
            resp.other,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = SnoozeNextAdRequest::broadcaster_id("123");

    // From twitch docs
    // FIXME: WRONG!!!!!!!1 https://github.com/twitchdev/issues/issues/857#issuecomment-1793777950 also trailing comma
    let data = r#"
    {
        "data": [
          {
            "snooze_count" : 1,
            "snooze_refresh_at" : "2023-08-01T23:08:18+00:00",
            "next_ad_at" : "2023-08-01T23:08:18+00:00"
          }
        ]
      }
      "#;

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/ads/schedule/snooze?broadcaster_id=123"
    );

    dbg!(SnoozeNextAdRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

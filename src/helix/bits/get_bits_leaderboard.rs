//! Gets a ranked list of Bits leaderboard information for an authorized broadcaster.
//! [`get-bits-leaderboard`](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetBitsLeaderboardRequest]
//!
//! To use this endpoint, construct a [`GetBitsLeaderboardRequest`] with the [`GetBitsLeaderboardRequest::builder()`] method.
//! Provide [`started_at`](GetBitsLeaderboardRequest::started_at) and [`period`](GetBitsLeaderboardRequest::period) to get a different leaderboard than default
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::bits::get_bits_leaderboard;
//! let request = get_bits_leaderboard::GetBitsLeaderboardRequest::builder()
//!     .started_at("2020-01-01T07:00:00Z".to_string())
//!     .period("day".to_string())
//!     .build();
//! // Get leaderbord for the lifetime of the channel
//! let request = get_bits_leaderboard::GetBitsLeaderboardRequest::builder().build();
//! ```
//!
//! ## Response: [BitsLeaderboard]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, bits::get_bits_leaderboard};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_bits_leaderboard::GetBitsLeaderboardRequest::builder().build();
//! let response: get_bits_leaderboard::BitsLeaderboard = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
use std::convert::TryInto;

use super::*;

/// Query Parameters for [Get Bits Leaderboard](super::get_bits_leaderboard)
///
/// [`get-bits-leaderboard`](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetBitsLeaderboardRequest {
    /// Number of results to be returned. Maximum: 100. Default: 10.
    #[builder(default, setter(into))]
    pub count: Option<i32>,
    // TODO: Enum
    /// Time period over which data is aggregated (PST time zone). This parameter interacts with started_at. Valid values follow. Default: "all".
    ///
    /// * "day" – 00:00:00 on the day specified in started_at, through 00:00:00 on the following day.
    /// * "week" – 00:00:00 on Monday of the week specified in started_at, through 00:00:00 on the following Monday.
    /// * "month" – 00:00:00 on the first day of the month specified in started_at, through 00:00:00 on the first day of the following month.
    /// * "year" – 00:00:00 on the first day of the year specified in started_at, through 00:00:00 on the first day of the following year.
    /// * "all" – The lifetime of the broadcaster's channel. If this is specified (or used by default), started_at is ignored.
    #[builder(default, setter(into))]
    pub period: Option<String>,
    /// Timestamp for the period over which the returned data is aggregated. Must be in RFC 3339 format. If this is not provided, data is aggregated over the current period; e.g., the current day/week/month/year. This value is ignored if period is "all".
    #[builder(default, setter(into))]
    pub started_at: Option<types::Timestamp>,
    /// ID of the user whose results are returned; i.e., the person who paid for the Bits.
    #[builder(default, setter(into))]
    pub user_id: Option<types::UserId>,
}

/// Return Values for [Get Bits Leaderboard](super::get_bits_leaderboard)
///
/// [`get-bits-leaderboard`](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BitsLeaderboard {
    /// Leaderboard
    pub leaderboard: Vec<LeaderboardUser>,
    /// Period over which the returned data is aggregated.
    pub date_range: DateRange,
    /// Total number of results (users) returned. This is count or the total number of entries in the leaderboard, whichever is less.
    pub total: i64,
}

/// Period over which the returned data is aggregated.
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct DateRange {
    /// Start of the date range for the returned data.
    pub started_at: types::Timestamp,
    /// End of the date range for the returned data.
    pub ended_at: types::Timestamp,
}

/// Information about user in leaderboard
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct LeaderboardUser {
    /// Leaderboard rank of the user.
    pub rank: i64,
    /// Leaderboard score (number of Bits) of the user
    pub score: i64,
    /// ID of the user (viewer) in the leaderboard entry.
    pub user_id: types::UserId,
    /// Display name corresponding to user_id.
    pub user_name: types::UserName,
}

impl helix::Request for GetBitsLeaderboardRequest {
    type Response = BitsLeaderboard;

    const PATH: &'static str = "bits/leaderboard";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl helix::RequestGet for GetBitsLeaderboardRequest {
    fn parse_response(
        self,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<
        helix::Response<Self, <Self as helix::Request>::Response>,
        helix::HelixRequestGetError,
    >
    where
        Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug, Clone)]
        struct InnerResponse {
            data: Vec<LeaderboardUser>,
            date_range: DateRange,
            /// Total number of results (users) returned. This is count or the total number of entries in the leaderboard, whichever is less.
            total: i64,
        }

        let text = std::str::from_utf8(&response.body())
            .map_err(|e| helix::HelixRequestGetError::Utf8Error(response.body().clone(), e))?;
        //eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(helix::HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<helix::HelixRequestError>(&text)
        {
            return Err(helix::HelixRequestGetError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }
        let response: InnerResponse = serde_json::from_str(&text)?;
        Ok(helix::Response {
            data: BitsLeaderboard {
                leaderboard: response.data,
                date_range: response.date_range,
                total: response.total,
            },
            pagination: None,
            request: self,
        })
    }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetBitsLeaderboardRequest::builder().build();

    // From api call
    let data = br##"
{
    "data": [
        {
        "user_id": "1234",
        "user_name": "tmi",
        "rank": 1,
        "score": 1234567
        }
    ],
    "date_range": {
        "started_at": "2020-01-01T07:00:00Z",
        "ended_at": "2021-01-01T07:00:00Z"
    },
    "total": 10
    }
"##
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/bits/leaderboard?"
    );

    dbg!(req.parse_response(&uri, http_response).unwrap());
}

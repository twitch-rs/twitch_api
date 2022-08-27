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
//! use twitch_api::helix::bits::get_bits_leaderboard;
//! let request = get_bits_leaderboard::GetBitsLeaderboardRequest::builder()
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
//! use twitch_api::helix::{self, bits::get_bits_leaderboard};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_bits_leaderboard::GetBitsLeaderboardRequest::builder().build();
//! let response: get_bits_leaderboard::BitsLeaderboard = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetBitsLeaderboardRequest::parse_response(None, &request.get_uri(), response)`](GetBitsLeaderboardRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Bits Leaderboard](super::get_bits_leaderboard)
///
/// [`get-bits-leaderboard`](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetBitsLeaderboardRequest {
    /// Number of results to be returned. Maximum: 100. Default: 10.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub count: Option<i32>,
    // TODO: Enum
    /// Time period over which data is aggregated (PST time zone). This parameter interacts with started_at. Valid values follow. Default: "all".
    ///
    /// * "day" – 00:00:00 on the day specified in started_at, through 00:00:00 on the following day.
    /// * "week" – 00:00:00 on Monday of the week specified in started_at, through 00:00:00 on the following Monday.
    /// * "month" – 00:00:00 on the first day of the month specified in started_at, through 00:00:00 on the first day of the following month.
    /// * "year" – 00:00:00 on the first day of the year specified in started_at, through 00:00:00 on the first day of the following year.
    /// * "all" – The lifetime of the broadcaster's channel. If this is specified (or used by default), started_at is ignored.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub period: Option<String>,
    /// Timestamp for the period over which the returned data is aggregated. Must be in RFC 3339 format. If this is not provided, data is aggregated over the current period; e.g., the current day/week/month/year. This value is ignored if period is "all".
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub started_at: Option<types::Timestamp>,
    /// ID of the user whose results are returned; i.e., the person who paid for the Bits.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub user_id: Option<types::UserId>,
}

impl GetBitsLeaderboardRequest {
    /// Number of results to be returned. Maximum: 100. Default: 10.
    pub fn count(count: i32) -> Self {
        Self {
            count: Some(count),
            ..Self::empty()
        }
    }

    /// Get loaderboard for this period. Valid values: `"day"`, `"week"`, `"month"`, `"year"`, `"all"`
    pub fn period(period: String) -> Self {
        Self {
            period: Some(period),
            ..Self::empty()
        }
    }

    /// Get leaderboard starting at this timestamp
    pub fn started_at(started_at: impl Into<types::Timestamp>) -> Self {
        Self {
            started_at: Some(started_at.into()),
            ..Self::empty()
        }
    }

    /// Get leaderboard for this user
    pub fn user_id(user_id: impl Into<types::UserId>) -> Self {
        Self {
            user_id: Some(user_id.into()),
            ..Self::empty()
        }
    }

    /// Returns an empty [`GetBitsLeaderboardRequest`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api2::helix::bits::GetBitsLeaderboardRequest;
    /// GetBitsLeaderboardRequest {
    ///     period: Some("day".to_string()),
    ///     ..GetBitsLeaderboardRequest::empty()
    /// }
    /// ```
    pub fn empty() -> Self {
        Self {
            count: None,
            period: None,
            started_at: None,
            user_id: None,
        }
    }
}

/// Return Values for [Get Bits Leaderboard](super::get_bits_leaderboard)
///
/// [`get-bits-leaderboard`](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct DateRange {
    /// Start of the date range for the returned data.
    pub started_at: types::Timestamp,
    /// End of the date range for the returned data.
    pub ended_at: types::Timestamp,
}

/// Information about user in leaderboard
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct LeaderboardUser {
    /// Leaderboard rank of the user.
    pub rank: i64,
    /// Leaderboard score (number of Bits) of the user
    pub score: i64,
    /// ID of the user (viewer) in the leaderboard entry.
    pub user_id: types::UserId,
    /// Display name corresponding to user_id.
    pub user_name: types::DisplayName,
    /// User login name.
    pub user_login: types::UserName,
}

impl Request for GetBitsLeaderboardRequest {
    type Response = BitsLeaderboard;

    const PATH: &'static str = "bits/leaderboard";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetBitsLeaderboardRequest {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestGetError>
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
        let response: InnerResponse = helix::parse_json(response, true).map_err(|e| {
            helix::HelixRequestGetError::DeserializeError(
                response.to_string(),
                e,
                uri.clone(),
                status,
            )
        })?;
        Ok(helix::Response {
            data: BitsLeaderboard {
                leaderboard: response.data,
                date_range: response.date_range,
                total: response.total,
            },
            pagination: None,
            request,
            total: Some(response.total),
            other: None,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetBitsLeaderboardRequest::empty();

    // From api call
    let data = br##"
{
    "data": [
        {
        "user_id": "158010205",
        "user_login": "tundracowboy",
        "user_name": "TundraCowboy",
        "rank": 1,
        "score": 12543
        },
        {
        "user_id": "7168163",
        "user_login": "topramens",
        "user_name": "Topramens",
        "rank": 2,
        "score": 6900
        }
    ],
    "date_range": {
        "started_at": "2018-02-05T08:00:00Z",
        "ended_at": "2018-02-12T08:00:00Z"
    },
    "total": 2
    }
"##
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/bits/leaderboard?"
    );

    dbg!(GetBitsLeaderboardRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

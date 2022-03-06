//! Get information about all polls or specific polls for a Twitch channel. Poll information is available for 90 days.
//! [`get-polls`](https://dev.twitch.tv/docs/api/reference#get-polls)
//!
//! ## Request: [GetPollsRequest]
//!
//! To use this endpoint, construct a [`GetPollsRequest`] with the [`GetPollsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api2::helix::polls::get_polls;
//! let request = get_polls::GetPollsRequest::builder()
//!     .id(vec!["ed961efd-8a3f-4cf5-a9d0-e616c590cd2a".into()])
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [Poll]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, polls::get_polls};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_polls::GetPollsRequest::builder()
//!     .id(vec!["ed961efd-8a3f-4cf5-a9d0-e616c590cd2a".into()])
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_polls::Poll> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetPollsRequest::parse_response(None, &request.get_uri(), response)`](GetPollsRequest::parse_response)

use super::*;
use helix::RequestGet;
pub use types::{PollChoice, PollStatus};

/// Query Parameters for [Get polls](super::get_polls)
///
/// [`get-polls`](https://dev.twitch.tv/docs/api/reference#get-polls)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetPollsRequest {
    /// The broadcaster running polls. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// ID of a poll. Filters results to one or more specific polls. Not providing one or more IDs will return the full list of polls for the authenticated channel.
    #[builder(default, setter(into))]
    pub id: Vec<types::PollId>,
    /// Cursor for forward pagination
    #[builder(default, setter(into))]
    pub after: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 20. Default: 20.
    #[builder(default, setter(into))]
    pub first: Option<usize>,
}

/// Return Values for [Get polls](super::get_polls)
///
/// [`get-polls`](https://dev.twitch.tv/docs/api/reference#get-polls)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Poll {
    /// ID of the poll.
    pub id: types::PollId,
    /// ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// Name of the broadcaster.
    pub broadcaster_name: types::UserName,
    /// Login of the broadcaster.
    pub broadcaster_login: types::DisplayName,
    /// Question displayed for the poll.
    pub title: String,
    /// Array of the poll choices.
    pub choices: Vec<PollChoice>,
    /// Indicates if Bits can be used for voting.
    pub bits_voting_enabled: bool,
    /// Number of Bits required to vote once with Bits.
    pub bits_per_vote: i64,
    /// Indicates if Channel Points can be used for voting.
    pub channel_points_voting_enabled: bool,
    /// Number of Channel Points required to vote once with Channel Points.
    pub channel_points_per_vote: i64,
    /// Poll status. Valid values are:
    pub status: PollStatus,
    /// Total duration for the poll (in seconds).
    pub duration: i64,
    /// UTC timestamp for the poll’s start time.
    pub started_at: types::Timestamp,
    /// UTC timestamp for the poll’s end time. Set to null if the poll is active.
    pub ended_at: Option<types::Timestamp>,
}

impl Request for GetPollsRequest {
    type Response = Vec<Poll>;

    const PATH: &'static str = "polls";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadPolls];
}

impl RequestGet for GetPollsRequest {}

impl helix::Paginated for GetPollsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor; }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetPollsRequest::builder()
        .broadcaster_id("141981764")
        .id(vec!["ed961efd-8a3f-4cf5-a9d0-e616c590cd2a".into()])
        .build();

    // From twitch docs
    let data = br#"
{
    "data": [
        {
        "id": "ed961efd-8a3f-4cf5-a9d0-e616c590cd2a",
        "broadcaster_id": "55696719",
        "broadcaster_name": "TwitchDev",
        "broadcaster_login": "twitchdev",
        "title": "Heads or Tails?",
        "choices": [
            {
            "id": "4c123012-1351-4f33-84b7-43856e7a0f47",
            "title": "Heads",
            "votes": 0,
            "channel_points_votes": 0,
            "bits_votes": 0
            },
            {
            "id": "279087e3-54a7-467e-bcd0-c1393fcea4f0",
            "title": "Tails",
            "votes": 0,
            "channel_points_votes": 0,
            "bits_votes": 0
            }
        ],
        "bits_voting_enabled": false,
        "bits_per_vote": 0,
        "channel_points_voting_enabled": false,
        "channel_points_per_vote": 0,
        "status": "ACTIVE",
        "duration": 1800,
        "started_at": "2021-03-19T06:08:33.871278372Z"
        }
    ],
    "pagination": {}
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/polls?broadcaster_id=141981764&id=ed961efd-8a3f-4cf5-a9d0-e616c590cd2a"
    );

    dbg!(GetPollsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

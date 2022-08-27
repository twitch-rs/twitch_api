//! End a poll that is currently active.
//!
//! [`end-poll`](https://dev.twitch.tv/docs/api/reference#end-poll)
//!
//! # Accessing the endpoint
//!
//! ## Request: [EndPollRequest]
//!
//! To use this endpoint, construct an [`EndPollRequest`] with the [`EndPollRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::polls::end_poll;
//! let request = end_poll::EndPollRequest::new();
//! ```
//!
//! ## Body: [EndPollBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::polls::end_poll;
//! let body = end_poll::EndPollBody::builder()
//!     .broadcaster_id("274637212")
//!     .id("92af127c-7326-4483-a52b-b0da0be61c01")
//!     .status(end_poll::PollStatus::Terminated)
//!     .build();
//! ```
//!
//! ## Response: [EndPoll]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, polls::end_poll};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = end_poll::EndPollRequest::new();
//! let body = end_poll::EndPollBody::builder()
//!     .broadcaster_id("274637212")
//!     .id("92af127c-7326-4483-a52b-b0da0be61c01")
//!     .status(end_poll::PollStatus::Terminated)
//!     .build();
//! let response: end_poll::EndPoll = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`EndPollRequest::parse_response(None, &request.get_uri(), response)`](EndPollRequest::parse_response)

use crate::helix::{parse_json, HelixRequestPatchError};

use super::*;
use helix::RequestPatch;
pub use types::PollStatus;
/// Query Parameters for [End Poll](super::end_poll)
///
/// [`end-poll`](https://dev.twitch.tv/docs/api/reference#end-poll)
#[derive(
    PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default,
)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct EndPollRequest {}

impl EndPollRequest {
    /// Make a new [`EndPollRequest`]
    pub fn new() -> Self { Self {} }
}

/// Body Parameters for [End Poll](super::end_poll)
///
/// [`end-poll`](https://dev.twitch.tv/docs/api/reference#end-poll)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct EndPollBody {
    /// The broadcaster running polls. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_id: types::UserId,
    /// ID of the poll.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub id: types::PollId,
    /// The poll status to be set.
    ///
    /// Valid values:
    /// [`TERMINATED`](types::PollStatus::Terminated): End the poll manually, but allow it to be viewed publicly.
    /// [`ARCHIVED`](types::PollStatus::Archived): End the poll manually and do not allow it to be viewed publicly.
    pub status: PollStatus,
}

impl EndPollBody {
    pub fn new(
        broadcaster_id: impl Into<types::UserId>,
        id: impl Into<types::PollId>,
        status: PollStatus,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into(),
            id: id.into(),
            status,
        }
    }
}

impl helix::private::SealedSerialize for EndPollBody {}

/// Return Values for [Update CustomReward](super::end_poll)
///
/// [`end-poll`](https://dev.twitch.tv/docs/api/reference#end-poll)
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum EndPoll {
    /// Poll ended successfully.
    Success(Poll),
    /// Bad Request: Query/Body Parameter missing or invalid
    MissingQuery,
    /// Unauthenticated: Missing/invalid Token
    AuthFailed,
}

impl Request for EndPollRequest {
    type Response = EndPoll;

    const PATH: &'static str = "polls";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManagePolls];
}

impl RequestPatch for EndPollRequest {
    type Body = EndPollBody;

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
            http::StatusCode::OK => {
                let resp: helix::InnerResponse<Vec<Poll>> =
                    parse_json(response, true).map_err(|e| {
                        HelixRequestPatchError::DeserializeError(
                            response.to_string(),
                            e,
                            uri.clone(),
                            status,
                        )
                    })?;
                EndPoll::Success(resp.data.into_iter().next().ok_or(
                    helix::HelixRequestPatchError::InvalidResponse {
                        reason: "expected at least one element in data",
                        response: response.to_string(),
                        status,
                        uri: uri.clone(),
                    },
                )?)
            }
            http::StatusCode::BAD_REQUEST => EndPoll::MissingQuery,
            http::StatusCode::UNAUTHORIZED => EndPoll::AuthFailed,
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
    use helix::*;
    let req = EndPollRequest::new();

    let body = EndPollBody::new(
        "274637212",
        "92af127c-7326-4483-a52b-b0da0be61c01",
        PollStatus::Terminated,
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br##"
{
    "data": [
        {
        "id": "ed961efd-8a3f-4cf5-a9d0-e616c590cd2a",
        "broadcaster_id": "141981764",
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
        "channel_points_voting_enabled": true,
        "channel_points_per_vote": 100,
        "status": "TERMINATED",
        "duration": 1800,
        "started_at": "2021-03-19T06:08:33.871278372Z",
        "ended_at": "2021-03-19T06:11:26.746889614Z"
        }
    ]
}
    "##
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/polls?");

    dbg!(EndPollRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

//! Create a poll for a specific Twitch channel.
//! [`create-poll`](https://dev.twitch.tv/docs/api/reference#create-poll)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreatePollRequest]
//!
//! To use this endpoint, construct a [`CreatePollRequest`] with the [`CreatePollRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::polls::create_poll;
//! let request = create_poll::CreatePollRequest::new();
//! ```
//!
//! ## Body: [CreatePollBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::polls::create_poll;
//! let body = create_poll::CreatePollBody::builder()
//!     .broadcaster_id("141981764")
//!     .title("Heads or Tails?")
//!     .choices(vec![
//!         create_poll::NewPollChoice::new("Heads"),
//!         create_poll::NewPollChoice::new("Tails"),
//!     ])
//!     .channel_points_voting_enabled(true)
//!     .channel_points_per_vote(100)
//!     .duration(1800)
//!     .build();
//! ```
//!
//! ## Response: [CreatePollResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, polls::create_poll};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = create_poll::CreatePollRequest::builder()
//!     .build();
//! let body = create_poll::CreatePollBody::builder()
//!     .broadcaster_id("141981764")
//!     .title("Heads or Tails?")
//!     .choices(vec![create_poll::NewPollChoice::new("Heads"), create_poll::NewPollChoice::new("Tails")])
//!     .channel_points_voting_enabled(true)
//!     .channel_points_per_vote(100)
//!     .duration(1800)
//!     .build();
//! let response: create_poll::CreatePollResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreatePollRequest::parse_response(None, &request.get_uri(), response)`](CreatePollRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Create Poll](super::create_poll)
///
/// [`create-poll`](https://dev.twitch.tv/docs/api/reference#create-poll)
#[derive(
    PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default,
)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreatePollRequest {}

impl CreatePollRequest {
    /// Create a new [`CreatePollRequest`]
    pub fn new() -> Self { Self {} }
}

/// Body Parameters for [Create Poll](super::create_poll)
///
/// [`create-poll`](https://dev.twitch.tv/docs/api/reference#create-poll)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreatePollBody {
    /// The broadcaster running polls. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_id: types::UserId,
    /// Question displayed for the poll. Maximum: 60 characters.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub title: String,
    /// Total duration for the poll (in seconds). Minimum: 15. Maximum: 1800.
    pub duration: i64,
    /// Array of the poll choices. Minimum: 2 choices. Maximum: 5 choices.
    pub choices: Vec<NewPollChoice>,
    /// Indicates if Bits can be used for voting. Default: false
    #[deprecated(since = "0.7.0", note = "bit options for polls has been removed")]
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub bits_voting_enabled: Option<bool>,
    /// Number of Bits required to vote once with Bits. Minimum: 0. Maximum: 10000.
    #[deprecated(since = "0.7.0", note = "bit options for polls has been removed")]
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub bits_per_vote: Option<i64>,
    /// Indicates if Channel Points can be used for voting. Default: false
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub channel_points_voting_enabled: Option<bool>,
    /// Number of Channel Points required to vote once with Channel Points. Minimum: 0. Maximum: 1000000.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub channel_points_per_vote: Option<i64>,
}

impl helix::private::SealedSerialize for CreatePollBody {}

// FIXME: I'd prefer this to be a Vec<String> on CreatePollBody
/// Choice settings for a poll
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct NewPollChoice {
    /// Text displayed for the choice. Maximum: 25 characters.
    pub title: String,
}

impl NewPollChoice {
    /// Create a new [`NewPollChoice`]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

/// Return Values for [Create Poll](super::create_poll)
///
/// [`create-poll`](https://dev.twitch.tv/docs/api/reference#create-poll)
pub type CreatePollResponse = super::Poll;

impl Request for CreatePollRequest {
    type Response = CreatePollResponse;

    const PATH: &'static str = "polls";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManagePolls];
}

impl RequestPost for CreatePollRequest {
    type Body = CreatePollBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response_str: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        let response: helix::InnerResponse<Vec<Self::Response>> =
            helix::parse_json(response_str, true).map_err(|e| {
                helix::HelixRequestPostError::DeserializeError(
                    response_str.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        let data = response.data.into_iter().next().ok_or_else(|| {
            helix::HelixRequestPostError::InvalidResponse {
                reason: "response included no data",
                response: response_str.to_string(),
                status,
                uri: uri.clone(),
            }
        })?;
        Ok(helix::Response {
            data,
            pagination: response.pagination.cursor,
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
    let req = CreatePollRequest::builder().build();

    let body = CreatePollBody::builder()
        .broadcaster_id("141981764")
        .title("Heads or Tails?")
        .choices(vec![
            NewPollChoice::new("Heads"),
            NewPollChoice::new("Tails"),
        ])
        .channel_points_voting_enabled(true)
        .channel_points_per_vote(100)
        .duration(1800)
        .build();

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
        "status": "ACTIVE",
        "duration": 1800,
        "started_at": "2021-03-19T06:08:33.871278372Z"
        }
    ]
}
    "##
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();
    // This is marked as 204 in twitch docs, but in reality it's 200

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/polls?");

    dbg!(CreatePollRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

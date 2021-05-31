//! Get information about all Channel Points Predictions or specific Channel Points Predictions for a Twitch channel.
//!
//! Results are ordered by most recent, so it can be assumed that the currently active or locked Prediction will be the first item.
//! [`create-prediction`](https://dev.twitch.tv/docs/api/reference#create-prediction)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreatePredictionRequest]
//!
//! To use this endpoint, construct a [`CreatePredictionRequest`] with the [`CreatePredictionRequest::new()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::predictions::create_prediction;
//! let request = create_prediction::CreatePredictionRequest::new();
//! ```
//!
//! ## Body: [CreatePredictionBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::predictions::create_prediction;
//! let body = create_prediction::CreatePredictionBody::builder()
//!     .broadcaster_id("141981764")
//!     .title("Any leeks in the stream?")
//!     .outcomes(create_prediction::NewPredictionOutcome::new_tuple("Yes, give it time.", "Definitely not."))
//!     .prediction_window(120)
//!     .build();
//! ```
//!
//! ## Response: [CreatePredictionResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, predictions::create_prediction};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = create_prediction::CreatePredictionRequest::builder()
//!     .build();
//! let body = create_prediction::CreatePredictionBody::builder()
//!     .broadcaster_id("141981764")
//!     .title("Any leeks in the stream?")
//!     .outcomes(create_prediction::NewPredictionOutcome::new_tuple("Yes, give it time.", "Definitely not."))
//!     .prediction_window(120)
//!     .build();
//! let response: create_prediction::CreatePredictionResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreatePredictionRequest::parse_response(None, &request.get_uri(), response)`](CreatePredictionRequest::parse_response)

use super::*;
use helix::RequestPost;

/// Query Parameters for [Create Prediction](super::create_prediction)
///
/// [`create-prediction`](https://dev.twitch.tv/docs/api/reference#create-prediction)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreatePredictionRequest {}

impl CreatePredictionRequest {
    /// Create a new [`CreatePredictionRequest`]
    pub fn new() -> Self { Self {} }
}

/// Body Parameters for [Create Prediction](super::create_prediction)
///
/// [`create-prediction`](https://dev.twitch.tv/docs/api/reference#create-prediction)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreatePredictionBody {
    /// The broadcaster running Predictions. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// Title for the Prediction. Maximum: 45 characters.
    #[builder(setter(into))]
    pub title: String,
    /// Array of outcome objects with titles for the Prediction. Array size must be 2.
    pub outcomes: (NewPredictionOutcome, NewPredictionOutcome),
    /// Total duration for the Prediction (in seconds). Minimum: 1. Maximum: 1800.
    pub prediction_window: i64,
}

impl helix::private::SealedSerialize for CreatePredictionBody {}

/// Choice settings for a poll
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct NewPredictionOutcome {
    /// Text displayed for the choice. Maximum: 25 characters.
    pub title: String,
}

impl NewPredictionOutcome {
    /// Create a new [`NewPredictionOutcome`]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Create a two new [`NewPredictionOutcome`]s
    pub fn new_tuple(blue: impl Into<String>, pink: impl Into<String>) -> (Self, Self) {
        (Self::new(blue), Self::new(pink))
    }
}

/// Return Values for [Create Prediction](super::create_prediction)
///
/// [`create-prediction`](https://dev.twitch.tv/docs/api/reference#create-prediction)
pub type CreatePredictionResponse = super::Prediction;

impl Request for CreatePredictionRequest {
    type Response = CreatePredictionResponse;

    const PATH: &'static str = "predictions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelManagePredictions];
}

impl RequestPost for CreatePredictionRequest {
    type Body = CreatePredictionBody;

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
            helix::parse_json(&response_str, true).map_err(|e| {
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
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = CreatePredictionRequest::builder().build();

    let body = CreatePredictionBody::builder()
        .broadcaster_id("141981764")
        .title("Any leeks in the stream?")
        .outcomes(NewPredictionOutcome::new_tuple(
            "Yes, give it time.",
            "Definitely not.",
        ))
        .prediction_window(120)
        .build();

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br##"
{
    "data": [
        {
        "id": "bc637af0-7766-4525-9308-4112f4cbf178",
        "broadcaster_id": "141981764",
        "broadcaster_name": "TwitchDev",
        "broadcaster_login": "twitchdev",
        "title": "Any leeks in the stream?",
        "winning_outcome_id": null,
        "outcomes": [
            {
            "id": "73085848-a94d-4040-9d21-2cb7a89374b7",
            "title": "Yes, give it time.",
            "users": 0,
            "channel_points": 0,
            "top_predictors": null,
            "color": "BLUE"
            },
            {
            "id": "906b70ba-1f12-47ea-9e95-e5f93d20e9cc",
            "title": "Definitely not.",
            "users": 0,
            "channel_points": 0,
            "top_predictors": null,
            "color": "PINK"
            }
        ],
        "prediction_window": 120,
        "status": "ACTIVE",
        "created_at": "2021-04-28T17:11:22.595914172Z",
        "ended_at": null,
        "locked_at": null
        }
    ]
}
    "##
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();
    // This is marked as 204 in twitch docs, but in reality it's 200

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/predictions?");

    dbg!(CreatePredictionRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

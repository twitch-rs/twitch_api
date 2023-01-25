//! Create a Channel Points Prediction for a specific Twitch channel.
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
//! ```rust
//! use twitch_api::helix::predictions::create_prediction;
//! let request = create_prediction::CreatePredictionRequest::new();
//! ```
//!
//! ## Body: [CreatePredictionBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::predictions::create_prediction;
//! let body = create_prediction::CreatePredictionBody::new(
//!     "141981764",
//!     "Any leeks in the stream?",
//!     create_prediction::NewPredictionOutcome::new_tuple(
//!         "Yes, give it time.",
//!         "Definitely not.",
//!     ),
//!     120,
//! );
//! ```
//!
//! ## Response: [CreatePredictionResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, predictions::create_prediction};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = create_prediction::CreatePredictionRequest::new();
//! let body = create_prediction::CreatePredictionBody::new(
//!     "141981764",
//!     "Any leeks in the stream?",
//!     create_prediction::NewPredictionOutcome::new_tuple("Yes, give it time.", "Definitely not."),
//!     120,
//! );
//! let response: create_prediction::CreatePredictionResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreatePredictionRequest::parse_response(None, &request.get_uri(), response)`](CreatePredictionRequest::parse_response)

use std::marker::PhantomData;

use super::*;
use helix::RequestPost;

/// Query Parameters for [Create Prediction](super::create_prediction)
///
/// [`create-prediction`](https://dev.twitch.tv/docs/api/reference#create-prediction)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreatePredictionRequest<'a> {
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[serde(skip)]
    _marker: PhantomData<&'a ()>,
}

impl CreatePredictionRequest<'_> {
    /// Create a new [`CreatePredictionRequest`]
    pub fn new() -> Self { Self::default() }
}

/// Body Parameters for [Create Prediction](super::create_prediction)
///
/// [`create-prediction`](https://dev.twitch.tv/docs/api/reference#create-prediction)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreatePredictionBody<'a> {
    /// The broadcaster running Predictions. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Title for the Prediction. Maximum: 45 characters.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub title: Cow<'a, str>,
    /// Array of outcome objects with titles for the Prediction. Array size must be 2.
    pub outcomes: (NewPredictionOutcome<'a>, NewPredictionOutcome<'a>),
    /// Total duration for the Prediction (in seconds). Minimum: 1. Maximum: 1800.
    pub prediction_window: i64,
}

impl<'a> CreatePredictionBody<'a> {
    /// Create a Channel Points Prediction for a specific Twitch channel.
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        title: impl Into<Cow<'a, str>>,
        outcomes: (NewPredictionOutcome<'a>, NewPredictionOutcome<'a>),
        prediction_window: i64,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            title: title.into(),
            outcomes,
            prediction_window,
        }
    }
}

impl helix::private::SealedSerialize for CreatePredictionBody<'_> {}

/// Choice settings for a poll
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct NewPredictionOutcome<'a> {
    /// Text displayed for the choice. Maximum: 25 characters.
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub title: Cow<'a, str>,
}

impl<'a> NewPredictionOutcome<'a> {
    /// Create a new [`NewPredictionOutcome`]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Create a two new [`NewPredictionOutcome`]s
    pub fn new_tuple(blue: impl Into<Cow<'a, str>>, pink: impl Into<Cow<'a, str>>) -> (Self, Self) {
        (Self::new(blue), Self::new(pink))
    }
}

/// Return Values for [Create Prediction](super::create_prediction)
///
/// [`create-prediction`](https://dev.twitch.tv/docs/api/reference#create-prediction)
pub type CreatePredictionResponse = super::Prediction;

impl Request for CreatePredictionRequest<'_> {
    type Response = CreatePredictionResponse;

    const PATH: &'static str = "predictions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelManagePredictions];
}

impl<'a> RequestPost for CreatePredictionRequest<'a> {
    type Body = CreatePredictionBody<'a>;

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
    let req = CreatePredictionRequest::new();

    let body = CreatePredictionBody::new(
        "141981764",
        "Any leeks in the stream?",
        NewPredictionOutcome::new_tuple("Yes, give it time.", "Definitely not."),
        120,
    );

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

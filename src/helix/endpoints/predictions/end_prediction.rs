//! End a prediction that is currently active.
//!
//! [`end-prediction`](https://dev.twitch.tv/docs/api/reference#end-prediction)
//!
//! # Accessing the endpoint
//!
//! ## Request: [EndPredictionRequest]
//!
//! To use this endpoint, construct an [`EndPredictionRequest`] with the [`EndPredictionRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::predictions::end_prediction;
//! let request = end_prediction::EndPredictionRequest::new();
//! ```
//!
//! ## Body: [EndPredictionBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::predictions::end_prediction;
//! let body = end_prediction::EndPredictionBody::builder()
//!     .broadcaster_id("141981764")
//!     .id("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a")
//!     .status(end_prediction::PredictionStatus::Resolved)
//!     .build();
//! ```
//!
//! ## Response: [EndPrediction]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, predictions::end_prediction};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = end_prediction::EndPredictionRequest::new();
//! let body = end_prediction::EndPredictionBody::builder()
//!     .broadcaster_id("141981764")
//!     .id("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a")
//!     .status(end_prediction::PredictionStatus::Resolved)
//!     .build();
//! let response: end_prediction::EndPrediction = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`EndPredictionRequest::parse_response(None, &request.get_uri(), response)`](EndPredictionRequest::parse_response)

use crate::helix::{parse_json, HelixRequestPatchError};

use super::*;
use helix::RequestPatch;
/// Query Parameters for [End Prediction](super::end_prediction)
///
/// [`end-prediction`](https://dev.twitch.tv/docs/api/reference#end-prediction)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct EndPredictionRequest {}

impl EndPredictionRequest {
    /// Make a new [`EndPredictionRequest`]
    pub fn new() -> Self { Self {} }
}

/// Body Parameters for [End Prediction](super::end_prediction)
///
/// [`end-prediction`](https://dev.twitch.tv/docs/api/reference#end-prediction)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct EndPredictionBody {
    /// The broadcaster running predictions. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_id: types::UserId,
    /// ID of the prediction.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub id: types::PredictionId,
    /// The Prediction status to be set. Valid values:
    ///
    /// [`RESOLVED`](types::PredictionStatus): A winning outcome has been chosen and the Channel Points have been distributed to the users who predicted the correct outcome.
    /// [`CANCELED`](types::PredictionStatus): The Prediction has been canceled and the Channel Points have been refunded to participants.
    /// [`LOCKED`](types::PredictionStatus): The Prediction has been locked and viewers can no longer make predictions.
    pub status: types::PredictionStatus,
    /// ID of the winning outcome for the Prediction. This parameter is required if status is being set to [`RESOLVED`](types::PredictionStatus).
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub winning_outcome_id: Option<types::PredictionId>,
}

impl EndPredictionBody {
    pub fn new(
        broadcaster_id: impl Into<types::UserId>,
        id: impl Into<types::PredictionId>,
        status: impl Into<types::PredictionStatus>,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into(),
            id: id.into(),
            status: status.into(),
            winning_outcome_id: None,
        }
    }

    /// ID of the winning outcome for the Prediction
    pub fn winning_outcome_id(
        mut self,
        winning_outcome_id: impl Into<types::PredictionId>,
    ) -> Self {
        self.winning_outcome_id = Some(winning_outcome_id.into());
        self
    }
}

impl helix::private::SealedSerialize for EndPredictionBody {}

/// Return Values for [Update CustomReward](super::end_prediction)
///
/// [`end-prediction`](https://dev.twitch.tv/docs/api/reference#end-prediction)
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum EndPrediction {
    /// Prediction ended successfully.
    Success(Prediction),
    /// Bad Request: Query/Body Parameter missing or invalid
    MissingQuery,
    /// Unauthenticated: Missing/invalid Token
    AuthFailed,
}

impl Request for EndPredictionRequest {
    type Response = EndPrediction;

    const PATH: &'static str = "predictions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelManagePredictions];
}

impl RequestPatch for EndPredictionRequest {
    type Body = EndPredictionBody;

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
                let resp: helix::InnerResponse<Vec<Prediction>> = parse_json(response, true)
                    .map_err(|e| {
                        HelixRequestPatchError::DeserializeError(
                            response.to_string(),
                            e,
                            uri.clone(),
                            status,
                        )
                    })?;
                EndPrediction::Success(resp.data.into_iter().next().ok_or(
                    helix::HelixRequestPatchError::InvalidResponse {
                        reason: "expected at least one element in data",
                        response: response.to_string(),
                        status,
                        uri: uri.clone(),
                    },
                )?)
            }
            http::StatusCode::BAD_REQUEST => EndPrediction::MissingQuery,
            http::StatusCode::UNAUTHORIZED => EndPrediction::AuthFailed,
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
    let req = EndPredictionRequest::new();

    let body = EndPredictionBody::new(
        "141981764",
        "ed961efd-8a3f-4cf5-a9d0-e616c590cd2a",
        types::PredictionStatus::Resolved,
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
        "title": "Will we win all the games?",
        "winning_outcome_id": "73085848-a94d-4040-9d21-2cb7a89374b7",
        "outcomes": [
            {
            "id": "73085848-a94d-4040-9d21-2cb7a89374b7",
            "title": "yes",
            "users": 0,
            "channel_points": 0,
            "top_predictors": null,
            "color": "BLUE"
            },
            {
            "id": "86010b2e-9764-4136-9359-fd1c9c5a8033",
            "title": "no",
            "users": 0,
            "channel_points": 0,
            "top_predictors": null,
            "color": "PINK"
            }
        ],
        "prediction_window": 120,
        "status": "RESOLVED",
        "created_at": "2021-04-28T21:48:19.480371331Z",
        "ended_at": "2021-04-28T21:54:24.026833954Z",
        "locked_at": "2021-04-28T21:48:34.636685705Z"
        }
    ]
}
    "##
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/predictions?");

    dbg!(EndPredictionRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

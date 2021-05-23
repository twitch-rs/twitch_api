//! Updates the status of Custom Reward Redemption objects on a channel that are in the UNFULFILLED status.
//!
//! Only redemptions for a reward created by the same client_id as attached to the access token can be updated.
//! [`update-redemption-status`](https://dev.twitch.tv/docs/api/reference#update-redemption-status)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateRedemptionStatusRequest]
//!
//! To use this endpoint, construct a [`UpdateRedemptionStatusRequest`] with the [`UpdateRedemptionStatusRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::points::UpdateRedemptionStatusRequest;
//! let request = UpdateRedemptionStatusRequest::builder()
//!     .broadcaster_id("274637212".to_string())
//!     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
//!     .id("17fa2df1-ad76-4804-bfa5-a40ef63efe63".to_string())
//!     .build();
//! ```
//!
//! ## Body: [UpdateRedemptionStatusBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! use twitch_api2::helix::points::{CustomRewardRedemptionStatus, UpdateRedemptionStatusBody};
//! let body = UpdateRedemptionStatusBody::builder()
//!     .status(CustomRewardRedemptionStatus::Canceled)
//!     .build();
//! ```
//!
//! ## Response: [UpdateRedemptionStatusInformation]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix;
//! use twitch_api2::helix::points::{CustomRewardRedemptionStatus, UpdateRedemptionStatusRequest, UpdateRedemptionStatusBody, UpdateRedemptionStatusInformation};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = UpdateRedemptionStatusRequest::builder()
//!     .broadcaster_id("274637212".to_string())
//!     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
//!     .id("17fa2df1-ad76-4804-bfa5-a40ef63efe63".to_string())
//!     .build();
//! let body = UpdateRedemptionStatusBody::builder()
//!     .status(CustomRewardRedemptionStatus::Canceled)
//!     .build();
//! let response: UpdateRedemptionStatusInformation = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPatch::create_request)
//! and parse the [`http::Response`] with [`UpdateRedemptionStatusRequest::parse_response(None, &request.get_uri(), response)`](UpdateRedemptionStatusRequest::parse_response)

use crate::helix::{parse_json, HelixRequestPatchError};

pub use super::CustomRewardRedemption;
use super::*;
use helix::RequestPatch;

/// Query Parameters for [Update Redemption Status](super::update_redemption_status)
///
/// [`update-redemption-status`](https://dev.twitch.tv/docs/api/reference#update-redemption-status)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateRedemptionStatusRequest {
    /// Provided broadcaster_id must match the user_id in the auth token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,

    /// ID of the Custom Reward the redemptions to be updated are for.
    #[builder(setter(into))]
    pub reward_id: types::RewardId,

    /// ID of the Custom Reward Redemption to update, must match a Custom Reward Redemption on broadcaster_id’s channel
    #[builder(setter(into))]
    pub id: types::RedemptionId,
}

/// Body Parameters for [Update Redemption Status](super::update_redemption_status)
///
/// [`update-redemption-status`](https://dev.twitch.tv/docs/api/reference#update-redemption-status)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateRedemptionStatusBody {
    /// The new status to set redemptions to. Can be either FULFILLED or CANCELED. Updating to CANCELED will refund the user their points.
    #[builder(setter(into))]
    pub status: CustomRewardRedemptionStatus,
}

/// FIXME: Returns an object.
/// Return Values for [Update Redemption Status](super::update_redemption_status)
///
/// [`update-redemption-status`](https://dev.twitch.tv/docs/api/reference#update-redemption-status)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum UpdateRedemptionStatusInformation {
    /// 200 - OK
    Success(CustomRewardRedemption),
    /// 400 - Bad Request: Query Parameter missing or invalid
    MissingQuery,
    /// 403 - Forbidden: The Custom Reward was created by a different client_id or Channel Points are not available for the broadcaster
    Forbidden,
    /// 404 - Not Found: No Custom Reward Redemptions with the specified IDs were found with a status of UNFULFILLED.
    NotFound,
    /// Internal Server Error; Failed to update channel
    InternalServerError,
}

impl Request for UpdateRedemptionStatusRequest {
    type Response = UpdateRedemptionStatusInformation;

    const PATH: &'static str = "channel_points/custom_rewards/redemptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::scopes::Scope::ChannelManageBroadcast];
}

impl RequestPatch for UpdateRedemptionStatusRequest {
    type Body = UpdateRedemptionStatusBody;

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
                let resp: helix::InnerResponse<Vec<CustomRewardRedemption>> =
                    parse_json(response, true).map_err(|e| {
                        HelixRequestPatchError::DeserializeError(
                            response.to_string(),
                            e,
                            uri.clone(),
                            status,
                        )
                    })?;
                UpdateRedemptionStatusInformation::Success(resp.data.into_iter().next().ok_or(
                    helix::HelixRequestPatchError::InvalidResponse {
                        reason: "expected at least one element in data",
                        response: response.to_string(),
                        status,
                        uri: uri.clone(),
                    },
                )?)
            }
            http::StatusCode::BAD_REQUEST => UpdateRedemptionStatusInformation::MissingQuery,
            http::StatusCode::NOT_FOUND => UpdateRedemptionStatusInformation::NotFound,
            http::StatusCode::FORBIDDEN => UpdateRedemptionStatusInformation::Forbidden,
            http::StatusCode::INTERNAL_SERVER_ERROR => {
                UpdateRedemptionStatusInformation::InternalServerError
            }
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
        })
    }
}

impl helix::private::SealedSerialize for UpdateRedemptionStatusBody {}

#[test]
fn test_request() {
    use helix::*;
    let req = UpdateRedemptionStatusRequest::builder()
        .broadcaster_id("274637212".to_string())
        .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
        .id("17fa2df1-ad76-4804-bfa5-a40ef63efe63".to_string())
        .build();

    let body = UpdateRedemptionStatusBody::builder()
        .status(CustomRewardRedemptionStatus::Unfulfilled)
        .build();

    dbg!(req.create_request(body, "abcd", "client").unwrap());
    // From twitch docs
    let data = br##"
{
    "data": [
        {
            "broadcaster_name": "torpedo09",
            "broadcaster_login": "torpedo09",
            "broadcaster_id": "274637212",
            "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
            "user_id": "274637212",
            "user_name": "torpedo09",
            "user_login": "torpedo09",
            "user_input": "",
            "status": "CANCELED",
            "redeemed_at": "2020-07-01T18:37:32Z",
            "reward": {
                "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                "title": "game analysis",
                "prompt": "",
                "cost": 50000
            }
        }
    ]
}
"##
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/channel_points/custom_rewards/redemptions?broadcaster_id=274637212&reward_id=92af127c-7326-4483-a52b-b0da0be61c01&id=17fa2df1-ad76-4804-bfa5-a40ef63efe63"
        );

    dbg!(UpdateRedemptionStatusRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

//! Updates a Custom Reward created on a channel.
//!
//! Only rewards created programmatically by the same client_id can be updated.
//! [`update-custom-reward`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateCustomRewardRequest]
//!
//! To use this endpoint, construct an [`UpdateCustomRewardRequest`] with the [`UpdateCustomRewardRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::points::update_custom_reward;
//! let request = update_custom_reward::UpdateCustomRewardRequest::builder()
//!     .broadcaster_id("274637212")
//!     .id("reward-id")
//!     .build();
//! ```
//!
//! ## Body: [UpdateCustomRewardBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::points::update_custom_reward;
//! let body = update_custom_reward::UpdateCustomRewardBody::builder()
//!     .cost(501)
//!     .title("hydrate but differently now!".to_string())
//!     .build();
//! ```
//!
//! ## Response: [UpdateCustomReward]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, points::update_custom_reward};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_custom_reward::UpdateCustomRewardRequest::builder()
//!     .broadcaster_id("274637212")
//!     .id("reward-id")
//!     .build();
//! let body = update_custom_reward::UpdateCustomRewardBody::builder()
//!     .cost(501)
//!     .title("hydrate but differently now!".to_string())
//!     .build();
//! let response: update_custom_reward::UpdateCustomReward = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`UpdateCustomRewardRequest::parse_response(None, &request.get_uri(), response)`](UpdateCustomRewardRequest::parse_response)

use crate::helix::{parse_json, HelixRequestPatchError};

use super::*;
use helix::RequestPatch;
/// Query Parameters for [Update Custom Rewards](super::update_custom_reward)
///
/// [`update-custom-reward`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateCustomRewardRequest {
    /// Provided broadcaster_id must match the user_id in the auth token
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// ID of the Custom Reward to update, must match a Custom Reward on broadcaster_id’s channel.
    #[builder(setter(into))]
    pub id: types::RewardId,
}

/// Body Parameters for [Update Custom Rewards](super::update_custom_reward)
///
/// [`update-custom-reward`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateCustomRewardBody {
    /// The title of the reward
    #[builder(default, setter(into))]
    pub title: Option<String>,
    /// The prompt for the viewer when they are redeeming the reward
    #[builder(default, setter(into))]
    pub prompt: Option<String>,
    /// The cost of the reward
    #[builder(default, setter(into))]
    pub cost: Option<usize>,
    /// Custom background color for the reward. Format: Hex with # prefix. Example: #00E5CB.
    #[builder(default, setter(into))]
    pub background_color: Option<String>,
    /// Is the reward currently enabled, if false the reward won’t show up to viewers
    #[builder(default, setter(into))]
    pub is_enabled: Option<bool>,
    /// Does the user need to enter information when redeeming the reward.
    #[builder(default, setter(into))]
    pub is_user_input_required: Option<bool>,
    /// Whether a maximum per stream is enabled
    #[builder(default, setter(into))]
    pub is_max_per_stream_enabled: Option<bool>,
    /// The maximum number per stream if enabled
    #[builder(default, setter(into))]
    pub max_per_stream: Option<usize>,
    /// Whether a maximum per user per stream is enabled. Defaults to false.
    #[builder(default, setter(into))]
    pub is_max_per_user_per_stream_enabled: Option<bool>,
    /// The maximum number per user per stream if enabled
    #[builder(default, setter(into))]
    pub max_per_user_per_stream: Option<usize>,
    /// Whether a cooldown is enabled. Defaults to false.
    #[builder(default, setter(into))]
    pub is_global_cooldown_enabled: Option<bool>,
    /// The cooldown in seconds if enabled
    #[builder(default, setter(into))]
    pub global_cooldown_seconds: Option<usize>,
    /// Is the reward currently paused, if true viewers can’t redeem
    #[builder(default, setter(into))]
    pub is_paused: Option<bool>,
    /// Should redemptions be set to FULFILLED status immediately when redeemed and skip the request queue instead of the normal UNFULFILLED status.
    #[builder(default, setter(into))]
    pub should_redemptions_skip_request_queue: Option<bool>,
}

impl helix::private::SealedSerialize for UpdateCustomRewardBody {}

/// Return Values for [Update CustomReward](super::update_custom_reward)
///
/// [`update-custom-reward`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum UpdateCustomReward {
    /// Reward updated
    Success(CustomReward),
}

impl Request for UpdateCustomRewardRequest {
    type Response = UpdateCustomReward;

    const PATH: &'static str = "channel_points/custom_rewards";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelManageRedemptions];
}

impl RequestPatch for UpdateCustomRewardRequest {
    type Body = UpdateCustomRewardBody;

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
                let resp: helix::InnerResponse<Vec<CustomReward>> = parse_json(response, true)
                    .map_err(|e| {
                        HelixRequestPatchError::DeserializeError(
                            response.to_string(),
                            e,
                            uri.clone(),
                            status,
                        )
                    })?;
                UpdateCustomReward::Success(resp.data.into_iter().next().ok_or(
                    helix::HelixRequestPatchError::InvalidResponse {
                        reason: "expected at least one element in data",
                        response: response.to_string(),
                        status,
                        uri: uri.clone(),
                    },
                )?)
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

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = UpdateCustomRewardRequest::builder()
        .broadcaster_id("274637212")
        .id("92af127c-7326-4483-a52b-b0da0be61c01")
        .build();

    let body = UpdateCustomRewardBody::builder().is_enabled(false).build();

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br##"
{
    "data": [
        {
            "broadcaster_name": "torpedo09",
            "broadcaster_login": "torpedo09",
            "broadcaster_id": "274637212",
            "id": "92af127c-7326-4483-a52b-b0da0be61c01",
            "image": null,
            "background_color": "#00E5CB",
            "is_enabled": false,
            "cost": 30000,
            "title": "game analysis 2v2",
            "prompt": "",
            "is_user_input_required": false,
            "max_per_stream_setting": {
                "is_enabled": true,
                "max_per_stream": 60
            },
            "max_per_user_per_stream_setting": {
                "is_enabled": false,
                "max_per_user_per_stream": 0
            },
            "global_cooldown_setting": {
                "is_enabled": false,
                "global_cooldown_seconds": 0
            },
            "is_paused": false,
            "is_in_stock": false,
            "default_image": {
                "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
            },
            "should_redemptions_skip_request_queue": true,
            "redemptions_redeemed_current_stream": 60,
            "cooldown_expires_at": null
        }
    ]
}
    "##
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();
    // This is marked as 204 in twitch docs, but in reality it's 200

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212&id=92af127c-7326-4483-a52b-b0da0be61c01"
    );

    dbg!(UpdateCustomRewardRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

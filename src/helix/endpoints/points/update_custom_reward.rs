//! Updates a Custom Reward created on a channel.
//!
//! Only rewards created programmatically by the same client_id can be updated.
//! [`update-custom-reward`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateCustomRewardRequest]
//!
//! To use this endpoint, construct an [`UpdateCustomRewardRequest`] with the [`UpdateCustomRewardRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::points::update_custom_reward;
//! let request = update_custom_reward::UpdateCustomRewardRequest::new(
//!     "274637212",
//!     "reward-id",
//! );
//! ```
//!
//! ## Body: [UpdateCustomRewardBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::points::update_custom_reward;
//! let mut body = update_custom_reward::UpdateCustomRewardBody::default();
//! body.cost = Some(501);
//! body.title = Some("hydrate but differently now!".into());
//! ```
//!
//! ## Response: [UpdateCustomReward]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, points::update_custom_reward};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_custom_reward::UpdateCustomRewardRequest::new("274637212", "reward-id");
//! let mut body = update_custom_reward::UpdateCustomRewardBody::default();
//! body.cost = Some(501);
//! body.title = Some("hydrate but differently now!".into());
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
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UpdateCustomRewardRequest<'a> {
    /// Provided broadcaster_id must match the user_id in the auth token
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// ID of the Custom Reward to update, must match a Custom Reward on broadcaster_id’s channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub id: Cow<'a, types::RewardIdRef>,
}

impl<'a> UpdateCustomRewardRequest<'a> {
    /// Update a Custom Reward created on the broadcaster's channel
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        id: impl types::IntoCow<'a, types::RewardIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            id: id.into_cow(),
        }
    }
}

/// Body Parameters for [Update Custom Rewards](super::update_custom_reward)
///
/// [`update-custom-reward`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateCustomRewardBody<'a> {
    /// The title of the reward
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub title: Option<Cow<'a, str>>,
    /// The prompt for the viewer when they are redeeming the reward
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub prompt: Option<Cow<'a, str>>,
    /// The cost of the reward
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<usize>,
    /// Custom background color for the reward. Format: Hex with # prefix. Example: #00E5CB.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub background_color: Option<Cow<'a, str>>,
    /// Is the reward currently enabled, if false the reward won’t show up to viewers
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Does the user need to enter information when redeeming the reward.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_user_input_required: Option<bool>,
    /// Whether a maximum per stream is enabled
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_max_per_stream_enabled: Option<bool>,
    /// The maximum number per stream if enabled
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_per_stream: Option<usize>,
    /// Whether a maximum per user per stream is enabled. Defaults to false.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_max_per_user_per_stream_enabled: Option<bool>,
    /// The maximum number per user per stream if enabled
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_per_user_per_stream: Option<usize>,
    /// Whether a cooldown is enabled. Defaults to false.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_global_cooldown_enabled: Option<bool>,
    /// The cooldown in seconds if enabled
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cooldown_seconds: Option<usize>,
    /// Is the reward currently paused, if true viewers can’t redeem
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// Should redemptions be set to FULFILLED status immediately when redeemed and skip the request queue instead of the normal UNFULFILLED status.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_redemptions_skip_request_queue: Option<bool>,
}

impl helix::private::SealedSerialize for UpdateCustomRewardBody<'_> {}

/// Return Values for [Update CustomReward](super::update_custom_reward)
///
/// [`update-custom-reward`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum UpdateCustomReward {
    /// Reward updated
    Success(CustomReward),
}

impl Request for UpdateCustomRewardRequest<'_> {
    type PaginationData = ();
    type Response = UpdateCustomReward;

    const PATH: &'static str = "channel_points/custom_rewards";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelManageRedemptions];
}

impl<'a> RequestPatch for UpdateCustomRewardRequest<'a> {
    type Body = UpdateCustomRewardBody<'a>;

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
        Ok(helix::Response::with_data(resp, request))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = UpdateCustomRewardRequest::new("274637212", "92af127c-7326-4483-a52b-b0da0be61c01");

    let body = UpdateCustomRewardBody {
        is_enabled: Some(false),
        ..Default::default()
    };

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"is_enabled":false}"#
    );

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

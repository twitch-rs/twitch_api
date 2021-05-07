//! [`update-custom-rewards`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreateCustomRewardRequest]
//!
//! To use this endpoint, construct an [`UpdateCustomRewardRequest`] with the [`UpdateCustomRewardRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::points::update_custom_rewards;
//! let request = update_custom_rewards::UpdateCustomRewardRequest::builder()
//!     .broadcaster_id("274637212")
//!     .build();
//! ```
//!
//! ## Body: [UpdateCustomRewardBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::points::update_custom_rewards;
//! let body = update_custom_rewards::UpdateCustomRewardBody::builder()
//!     .cost(501)
//!     .title("hydrate but differently now!")
//!     .build();
//! ```
//!
//! ## Response: [UpdateCustomRewardResponse]
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
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = update_custom_reward::UpdateCustomRewardRequest::builder()
//!     .broadcaster_id("274637212")
//!     .build();
//! let body = update_custom_reward::UpdateCustomRewardBody::builder()
//!     .cost(501)
//!     .title("hydrate but differently now!")
//!     .build();
//! let response: update_custom_reward::UpdateCustomRewardResponse = client.req_patch(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`UpdateCustomRewardRequest::parse_response(None, &request.get_uri(), response)`](UpdateCustomRewardRequest::parse_response)

use super::*;
use helix::RequestPatch;
/// Query Parameters for [Update Custom Rewards](super::update_custom_rewards)
///
/// [`update-custom-rewards`](https://dev.twitch.tv/docs/api/reference#update-custom-rewards)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct UpdateCustomRewardRequest {
    /// Provided broadcaster_id must match the user_id in the auth token
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// ID of the Custom Reward to update, must match a Custom Reward on broadcaster_id’s channel.
    #[builder(setter(into))]
    pub id: types::RewardId,
}

/// Body Parameters for [Update Custom Rewards](super::update_custom_rewards)
///
/// [`update-custom-rewards`](https://dev.twitch.tv/docs/api/reference#update-custom-rewards)
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
    /// Is the reward currently enabled, if false the reward won’t show up to viewers. Defaults true
    #[builder(default, setter(into))]
    pub is_enabled: Option<bool>,
    /// Custom background color for the reward. Format: Hex with # prefix. Example: #00E5CB.
    #[builder(default, setter(into))]
    pub background_color: Option<String>,
    /// Does the user need to enter information when redeeming the reward. Defaults false
    #[builder(default, setter(into))]
    pub is_user_input_required: Option<bool>,
    /// Whether a maximum per stream is enabled. Defaults to false.
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
    /// Should redemptions be set to FULFILLED status immediately when redeemed and skip the request queue instead of the normal UNFULFILLED status. Defaults false
    #[builder(default, setter(into))]
    pub should_redemptions_skip_request_queue: Option<bool>,
}

impl helix::private::SealedSerialize for UpdateCustomRewardBody {}

// FIXME: Should return VideoIds
/// Return Values for [Update CustomReward](super::update_custom_reward)
///
/// [`update-custom-reward`](https://dev.twitch.tv/docs/api/reference#update-custom-reward)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum UpdateCustomReward {
    /// Reward updated
    Success,
    /// Bad Request: Query/Body Parameter missing or invalid
    BadRequest,
    /// Unauthenticated: Missing/invalid Token
    AuthFailed,
    /// Forbidden: The Custom Reward was created by a different client_id or Channel Points are not available for the broadcaster
    Forbidden,
    /// Not Found: The Custom Reward doesn’t exist with the id and broadcaster_id specified
    NotFound,
}


impl std::convert::TryFrom<http::StatusCode> for UpdateCustomReward {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
        match s {
            http::StatusCode::OK => Ok(UpdateCustomReward::Success),
            http::StatusCode::BAD_REQUEST => Ok(UpdateCustomReward::BadRequest),
            http::StatusCode::UNAUTHORIZED => Ok(UpdateCustomReward::AuthFailed),
            http::StatusCode::FORBIDDEN => Ok(UpdateCustomReward::Forbidden),
            http::StatusCode::NOT_FOUND => Ok(UpdateCustomReward::NotFound),
            // http::StatusCode::INTERNAL_SERVER_ERROR => Ok(DeleteCustomReward::InternalServerError),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
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

}

#[test]
fn test_request() {
    use helix::*;
    let req = UpdateCustomRewardRequest::builder()
        .broadcaster_id("274637212")
        .id("b045196d-9ce7-4a27-a9b9-279ed341ab28")
        .build();

    let body = UpdateCustomRewardBody::builder()
        .cost(50001)
        .title("game analysis 1v1")
        .build();

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br##"
{
    "data": [
        {
            "broadcaster_name": "torpedo09",
            "broadcaster_login": "torpedo09",
            "broadcaster_id": "274637212",
            "id": "afaa7e34-6b17-49f0-a19a-d1e76eaaf673",
            "image": null,
            "background_color": "#00E5CB",
            "is_enabled": true,
            "cost": 50000,
            "title": "game analysis 1v1",
            "prompt": "",
            "is_user_input_required": false,
            "max_per_stream_setting": {
                "is_enabled": false,
                "max_per_stream": 0
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
            "is_in_stock": true,
            "default_image": {
                "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
            },
            "should_redemptions_skip_request_queue": false,
            "redemptions_redeemed_current_stream": null,
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
        "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212"
    );

    dbg!(UpdateCustomRewardRequest::parse_response(&uri, http_response).unwrap());
}

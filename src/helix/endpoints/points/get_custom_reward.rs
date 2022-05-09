//! Returns a list of Custom Reward objects for the Custom Rewards on a channel.
//!
//! Developers only have access to update and delete rewards that were created programmatically by the same/calling client_id.
//! [`get-custom-reward-redemption`](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetCustomRewardRequest]
//!
//! To use this endpoint, construct a [`GetCustomRewardRequest`] with the [`GetCustomRewardRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::points::GetCustomRewardRequest;
//! let request = GetCustomRewardRequest::builder()
//!     .broadcaster_id("274637212".to_string())
//!     .build();
//! ```
//!
//! ## Response: [CustomReward]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix;
//! use twitch_api::helix::points::{CustomReward, GetCustomRewardRequest};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = GetCustomRewardRequest::builder()
//!     .broadcaster_id("274637212".to_string())
//!     .build();
//! let response: Vec<CustomReward> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetCustomRewardRequest::parse_response(None, &request.get_uri(), response)`](GetCustomRewardRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Custom Reward](super::get_custom_reward)
///
/// [`get-custom-reward`](https://dev.twitch.tv/docs/api/reference#get-custom-reward)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetCustomRewardRequest {
    /// Provided broadcaster_id must match the user_id in the auth token
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_id: types::UserId,
    /// When used, this parameter filters the results and only returns reward objects for the Custom Rewards with matching ID. Maximum: 50
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub id: Vec<types::RewardId>,
    /// When set to true, only returns custom rewards that the calling client_id can manage. Defaults false.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub only_manageable_rewards: Option<bool>,
}

/// Return Values for [Get Custom Reward](super::get_custom_reward)
///
/// [`get-custom-reward`](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CustomReward {
    /// ID of the channel the reward is for
    pub broadcaster_id: types::UserId,
    /// Login of the channel the reward is for
    pub broadcaster_login: types::UserName,
    /// Display name of the channel the reward is for
    pub broadcaster_name: types::DisplayName,
    /// ID of the reward
    pub id: types::RewardId,
    /// The title of the reward
    pub title: String,
    /// The prompt for the viewer when they are redeeming the reward
    pub prompt: String,
    /// The cost of the reward
    pub cost: usize,
    /// Set of custom images of 1x, 2x and 4x sizes for the reward { url_1x: string, url_2x: string, url_4x: string }, can be null if no images have been uploaded
    pub image: Option<types::Image>,
    /// Set of default images of 1x, 2x and 4x sizes for the reward { url_1x: string, url_2x: string, url_4x: string }
    pub default_image: Option<types::Image>,
    /// Custom background color for the reward. Format: Hex with # prefix. Example: #00E5CB.
    pub background_color: String,
    /// Is the reward currently enabled, if false the reward won’t show up to viewers
    pub is_enabled: bool,
    /// Does the user need to enter information when redeeming the reward
    pub is_user_input_required: bool,
    /// Whether a maximum per stream is enabled and what the maximum is. { is_enabled: bool, max_per_stream: int }
    pub max_per_stream_setting: types::Max,
    /// Whether a maximum per user per stream is enabled and what the maximum is. { is_enabled: bool, max_per_user_per_stream: int }
    pub max_per_user_per_stream_setting: types::Max,
    /// Whether a cooldown is enabled and what the cooldown is. { is_enabled: bool, global_cooldown_seconds: int }
    pub global_cooldown_setting: types::GlobalCooldown,
    /// Is the reward currently paused, if true viewers can’t redeem
    pub is_paused: bool,
    /// Is the reward currently in stock, if false viewers can’t redeem
    pub is_in_stock: bool,
    /// Should redemptions be set to FULFILLED status immediately when redeemed and skip the request queue instead of the normal UNFULFILLED status.
    pub should_redemptions_skip_request_queue: bool,
    /// The number of redemptions redeemed during the current live stream. Counts against the max_per_stream_setting limit. Null if the broadcasters stream isn’t live or max_per_stream_setting isn’t enabled.
    pub redemptions_redeemed_current_stream: Option<usize>,
    /// Timestamp of the cooldown expiration. Null if the reward isn’t on cooldown.
    pub cooldown_expires_at: Option<types::Timestamp>,
}

impl Request for GetCustomRewardRequest {
    type Response = Vec<CustomReward>;

    const PATH: &'static str = "channel_points/custom_rewards";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::scopes::Scope::ChannelReadRedemptions];
}

impl RequestGet for GetCustomRewardRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetCustomRewardRequest::builder()
        .broadcaster_id("274637212".to_string())
        .build();

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
             "is_enabled": true,
             "cost": 50000,
             "title": "game analysis",
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

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212"
    );

    dbg!(GetCustomRewardRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

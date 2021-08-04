//! Adds a specified user to the followers of a specified channel.
//! [`create-custom-rewards`](https://dev.twitch.tv/docs/api/reference#create-custom-rewards)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreateCustomRewardRequest]
//!
//! To use this endpoint, construct a [`CreateCustomRewardRequest`] with the [`CreateCustomRewardRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::points::create_custom_rewards;
//! let request = create_custom_rewards::CreateCustomRewardRequest::builder()
//!     .broadcaster_id("274637212")
//!     .build();
//! ```
//!
//! ## Body: [CreateCustomRewardBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::points::create_custom_rewards;
//! let body = create_custom_rewards::CreateCustomRewardBody::builder()
//!     .cost(500)
//!     .title("hydrate!")
//!     .build();
//! ```
//!
//! ## Response: [CreateCustomRewardResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, points::create_custom_rewards};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = create_custom_rewards::CreateCustomRewardRequest::builder()
//!     .broadcaster_id("274637212")
//!     .build();
//! let body = create_custom_rewards::CreateCustomRewardBody::builder()
//!     .cost(500)
//!     .title("hydrate!")
//!     .build();
//! let response: create_custom_rewards::CreateCustomRewardResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreateCustomRewardRequest::parse_response(None, &request.get_uri(), response)`](CreateCustomRewardRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Create Custom Rewards](super::create_custom_rewards)
///
/// [`create-custom-rewards`](https://dev.twitch.tv/docs/api/reference#create-custom-rewards)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreateCustomRewardRequest {
    /// Provided broadcaster_id must match the user_id in the auth token
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
}

/// Body Parameters for [Create Custom Rewards](super::create_custom_rewards)
///
/// [`create-custom-rewards`](https://dev.twitch.tv/docs/api/reference#create-custom-rewards)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreateCustomRewardBody {
    /// The title of the reward
    #[builder(setter(into))]
    pub title: String,
    /// The prompt for the viewer when they are redeeming the reward
    #[builder(default, setter(into))]
    pub prompt: Option<String>,
    /// The cost of the reward
    pub cost: usize,
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
    /// Should redemptions be set to FULFILLED status immediately when redeemed and skip the request queue instead of the normal UNFULFILLED status. Defaults false
    #[builder(default, setter(into))]
    pub should_redemptions_skip_request_queue: Option<bool>,
}

impl helix::private::SealedSerialize for CreateCustomRewardBody {}

/// Return Values for [Create Custom Rewards](super::create_custom_rewards)
///
/// [`create-custom-rewards`](https://dev.twitch.tv/docs/api/reference#create-custom-rewards)
pub type CreateCustomRewardResponse = super::CustomReward;

impl Request for CreateCustomRewardRequest {
    type Response = CreateCustomRewardResponse;

    const PATH: &'static str = "channel_points/custom_rewards";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelManageRedemptions];
}

impl RequestPost for CreateCustomRewardRequest {
    type Body = CreateCustomRewardBody;

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
            total: response.total,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = CreateCustomRewardRequest::builder()
        .broadcaster_id("274637212")
        .build();

    let body = CreateCustomRewardBody::builder()
        .cost(50000)
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

    dbg!(CreateCustomRewardRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

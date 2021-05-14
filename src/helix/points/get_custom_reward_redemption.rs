//! Returns Custom Reward Redemption objects for a Custom Reward on a channel that was created by the same client_id.
//!
//! Developers only have access to get and update redemptions for the rewards they created.
//! [`get-custom-reward-redemption`](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetCustomRewardRedemptionRequest]
//!
//! To use this endpoint, construct a [`GetCustomRewardRedemptionRequest`] with the [`GetCustomRewardRedemptionRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::points::{CustomRewardRedemptionStatus, GetCustomRewardRedemptionRequest};
//! let request = GetCustomRewardRedemptionRequest::builder()
//!     .broadcaster_id("274637212".to_string())
//!     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
//!     .status(CustomRewardRedemptionStatus::Canceled)
//!     .build();
//! ```
//!
//! ## Response: [CustomRewardRedemption]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix;
//! use twitch_api2::helix::points::{CustomRewardRedemptionStatus, CustomRewardRedemption, GetCustomRewardRedemptionRequest};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = GetCustomRewardRedemptionRequest::builder()
//!     .broadcaster_id("274637212".to_string())
//!     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
//!     .status(CustomRewardRedemptionStatus::Canceled)
//!     .build();
//! let response: Vec<CustomRewardRedemption> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetCustomRewardRedemptionRequest::parse_response(None, &request.get_uri(), response)`](GetCustomRewardRedemptionRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Custom Reward Redemption](super::get_custom_reward_redemption)
///
/// [`get-custom-reward-redemption`](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetCustomRewardRedemptionRequest {
    /// Provided broadcaster_id must match the user_id in the auth token
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,

    /// When ID is not provided, this parameter returns paginated Custom Reward Redemption objects for redemptions of the Custom Reward with ID reward_id
    #[builder(setter(into))]
    pub reward_id: types::RewardId,

    /// When id is not provided, this param is required and filters the paginated Custom Reward Redemption objects for redemptions with the matching status. Can be one of UNFULFILLED, FULFILLED or CANCELED
    #[builder(default, setter(into))]
    pub status: Option<CustomRewardRedemptionStatus>,

    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries without ID. If an ID is specified, it supersedes any cursor/offset combinations. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,

    /// Number of results to be returned when getting the paginated Custom Reward Redemption objects for a reward. Limit: 50. Default: 20.
    #[builder(default, setter(into))]
    pub first: Option<usize>,
}

/// Return Values for [Get Custom Reward Redemption](super::get_custom_reward_redemption)
///
/// [`get-custom-reward-redemption`](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CustomRewardRedemption {
    /// The id of the broadcaster that the reward belongs to.
    pub broadcaster_id: types::UserId,

    /// The display name of the broadcaster that the reward belongs to.
    pub broadcaster_name: types::DisplayName,

    /// Broadcaster’s user login name.
    pub broadcaster_login: types::UserName,

    /// The ID of the redemption.
    pub id: types::RedemptionId,

    /// The ID of the user that redeemed the reward
    pub user_id: types::UserId,

    /// The display name of the user that redeemed the reward.
    pub user_name: types::DisplayName,

    ///The login of the user who redeemed the reward.
    pub user_login: types::UserName,

    /// Basic information about the Custom Reward that was redeemed at the time it was redeemed. { “id”: string, “title”: string, “prompt”: string, “cost”: int, }
    pub reward: Reward,

    /// The user input provided. Empty string if not provided.
    pub user_input: String,

    /// One of UNFULFILLED, FULFILLED or CANCELED
    pub status: CustomRewardRedemptionStatus,

    /// RFC3339 timestamp of when the reward was redeemed.
    pub redeemed_at: types::Timestamp,
}

/// Information about the reward involved
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Reward {
    /// The ID of the custom reward.
    pub id: types::RewardId,

    /// The title of the custom reward.
    pub title: String,

    /// The prompt to the user, if any, for the reward.
    pub prompt: String,

    /// The cost of the reward in channel points.
    pub cost: i64,
}

impl Request for GetCustomRewardRedemptionRequest {
    type Response = Vec<CustomRewardRedemption>;

    const PATH: &'static str = "channel_points/custom_rewards/redemptions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::scopes::Scope::ChannelReadRedemptions];
}

impl RequestGet for GetCustomRewardRedemptionRequest {}

impl helix::Paginated for GetCustomRewardRedemptionRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetCustomRewardRedemptionRequest::builder()
        .broadcaster_id("274637212".to_string())
        .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
        .status(CustomRewardRedemptionStatus::Canceled)
        .build();

    // From twitch docs
    let data = br##"
 {
    "data": [
          {
            "broadcaster_name": "torpedo09",
            "broadcaster_login": "torpedo09",
            "broadcaster_id": "274637212",
            "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
            "user_login": "torpedo09",
            "user_id": "274637212",
            "user_name": "torpedo09",
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
    ],
    "pagination": {
        "cursor":      "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6Ik1UZG1ZVEprWmpFdFlXUTNOaTAwT0RBMExXSm1ZVFV0WVRRd1pXWTJNMlZtWlRZelgxOHlNREl3TFRBM0xUQXhWREU0T2pNM09qTXlMakl6TXpFeU56RTFOMW89In19"
    }
}
"##
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/channel_points/custom_rewards/redemptions?broadcaster_id=274637212&reward_id=92af127c-7326-4483-a52b-b0da0be61c01&status=CANCELED"
        );

    dbg!(GetCustomRewardRedemptionRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

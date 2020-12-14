//! Endpoints regarding channel points/redeems
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient};
//! # use twitch_api2::helix::points::{GetCustomRewardRedemptionRequest, CustomRewardRedemptionStatus};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let request = GetCustomRewardRedemptionRequest::builder()
//!     .broadcaster_id("274637212".to_string())
//!     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
//!     .status(CustomRewardRedemptionStatus::Cancelled)
//!     .build();
//!
//!
//! println!("{:?}", &client.req_get(request, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```
use crate::{helix, types};
use serde::{Deserialize, Serialize};

#[doc(inline)]
pub use get_custom_reward_redemption::{CustomRewardRedemption, GetCustomRewardRedemptionRequest};

#[doc(inline)]
pub use update_redemption_status::{
    UpdateRedemptionStatusBody, UpdateRedemptionStatusInformation, UpdateRedemptionStatusRequest,
};

/// Custom reward redemption statuses: UNFULFILLED, FULFILLED or CANCELED
#[derive(PartialEq, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CustomRewardRedemptionStatus {
    /// Unfulfilled reward - the user has claimed it but it is still pending.
    #[serde(rename = "UNFULFILLED")]
    Unfulfilled,
    /// Fulfilled reward - the user has claimed it and the reward has been granted.
    #[serde(rename = "FULFILLED")]
    Fulfilled,
    /// Cancelled reward - the reward has been cancelled before fulfillment, and any spent points have been refunded.
    #[serde(rename = "CANCELED")]
    Cancelled,
}

/// Returns Custom Reward Redemption objects for a Custom Reward on a channel that was created by the same client_id.
///
/// Developers only have access to get and update redemptions for the rewards they created.
/// [`get-custom-reward-redemption`](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
///
/// # Accessing the endpoint
///
/// ## Request: [GetCustomRewardRedemptionRequest]
///
/// To use this endpoint, construct a [`GetCustomRewardRedemptionRequest`] with the [`GetCustomRewardRedemptionRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::points::{CustomRewardRedemptionStatus, GetCustomRewardRedemptionRequest};
/// let request = GetCustomRewardRedemptionRequest::builder()
///     .broadcaster_id("274637212".to_string())
///     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
///     .status(CustomRewardRedemptionStatus::Cancelled)
///     .build();
/// ```
///
/// ## Response: [CustomRewardRedemption]
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix;
/// use twitch_api2::helix::points::{CustomRewardRedemptionStatus, CustomRewardRedemption, GetCustomRewardRedemptionRequest};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
/// let request = GetCustomRewardRedemptionRequest::builder()
///     .broadcaster_id("274637212".to_string())
///     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
///     .status(CustomRewardRedemptionStatus::Cancelled)
///     .build();
/// let response: Vec<CustomRewardRedemption> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_custom_reward_redemption {
    use super::*;

    /// Query Parameters for [Get Custom Reward Redemption](super::get_custom_reward_redemption)
    ///
    /// [`get-custom-reward-redemption`](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetCustomRewardRedemptionRequest {
        /// Provided broadcaster_id must match the user_id in the auth token
        #[builder(default, setter(into))]
        pub broadcaster_id: types::UserId,

        /// When ID is not provided, this parameter returns paginated Custom Reward Redemption objects for redemptions of the Custom Reward with ID reward_id
        #[builder(default, setter(into))]
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
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct CustomRewardRedemption {
        /// The id of the broadcaster that the reward belongs to.
        pub broadcaster_id: types::UserId,

        /// The display name of the broadcaster that the reward belongs to.
        pub broadcaster_name: types::UserName,

        /// The ID of the redemption.
        pub id: types::RedemptionId,

        /// The ID of the user that redeemed the reward
        pub user_id: types::UserId,

        /// The display name of the user that redeemed the reward.
        pub user_name: types::UserName,

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
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
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

    impl helix::Request for GetCustomRewardRedemptionRequest {
        type Response = Vec<CustomRewardRedemption>;

        const PATH: &'static str = "channel_points/custom_rewards/redemptions";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] =
            &[twitch_oauth2::scopes::Scope::ChannelReadRedemptions];
    }

    impl helix::RequestGet for GetCustomRewardRedemptionRequest {}

    impl helix::Paginated for GetCustomRewardRedemptionRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetCustomRewardRedemptionRequest::builder()
            .broadcaster_id("274637212".to_string())
            .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
            .status(types::CustomRewardRedemptionStatus::Cancelled)
            .build();

        // From twitch docs
        let data = br##"
 {
    "data": [
          {
            "broadcaster_name": "torpedo09",
            "broadcaster_id": "274637212",
            "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
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

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Updates the status of Custom Reward Redemption objects on a channel that are in the UNFULFILLED status.
///
/// Only redemptions for a reward created by the same client_id as attached to the access token can be updated.
/// [`update-redemption-status`](https://dev.twitch.tv/docs/api/reference#update-redemption-status)
///
/// # Accessing the endpoint
///
/// ## Request: [UpdateRedemptionStatusRequest]
///
/// To use this endpoint, construct a [`UpdateRedemptionStatusRequest`] with the [`UpdateRedemptionStatusRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::points::UpdateRedemptionStatusRequest;
/// let request = UpdateRedemptionStatusRequest::builder()
///     .broadcaster_id("274637212".to_string())
///     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
///     .id("17fa2df1-ad76-4804-bfa5-a40ef63efe63".to_string())
///     .build();
/// ```
///
/// ## Body: [UpdateRedemptionStatusBody]
///
/// We also need to provide a body to the request containing what we want to change.
///
/// ```
/// use twitch_api2::helix::points::{CustomRewardRedemptionStatus, UpdateRedemptionStatusBody};
/// let body = UpdateRedemptionStatusBody::builder()
///     .status(CustomRewardRedemptionStatus::Cancelled)
///     .build();
/// ```
///
/// ## Response: [UpdateRedemptionStatusInformation]
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix;
/// use twitch_api2::helix::points::{CustomRewardRedemptionStatus, UpdateRedemptionStatusRequest, UpdateRedemptionStatusBody, UpdateRedemptionStatusInformation};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
/// let request = UpdateRedemptionStatusRequest::builder()
///     .broadcaster_id("274637212".to_string())
///     .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
///     .id("17fa2df1-ad76-4804-bfa5-a40ef63efe63".to_string())
///     .build();
/// let body = UpdateRedemptionStatusBody::builder()
///     .status(CustomRewardRedemptionStatus::Cancelled)
///     .build();
/// let response: UpdateRedemptionStatusInformation = client.req_patch(request, body, &token).await?;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod update_redemption_status {
    use super::*;

    /// Query Parameters for [Update Redemption Status](super::update_redemption_status)
    ///
    /// [`update-redemption-status`](https://dev.twitch.tv/docs/api/reference#update-redemption-status)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct UpdateRedemptionStatusRequest {
        /// Provided broadcaster_id must match the user_id in the auth token.
        #[builder(default, setter(into))]
        pub broadcaster_id: types::UserId,

        /// ID of the Custom Reward the redemptions to be updated are for.
        #[builder(default, setter(into))]
        pub reward_id: types::RewardId,

        /// ID of the Custom Reward Redemption to update, must match a Custom Reward Redemption on broadcaster_id’s channel
        #[builder(default, setter(into))]
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

    /// Return Values for [Update Redemption Status](super::update_redemption_status)
    ///
    /// [`update-redemption-status`](https://dev.twitch.tv/docs/api/reference#update-redemption-status)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub enum UpdateRedemptionStatusInformation {
        /// 200 - OK
        Success,
        /// 400 - Bad Request: Query Parameter missing or invalid
        MissingQuery,
        /// 403 - Forbidden: The Custom Reward was created by a different client_id or Channel Points are not available for the broadcaster
        Forbidden,
        /// 404 - Not Found: No Custom Reward Redemptions with the specified IDs were found with a status of UNFULFILLED.
        NotFound,
        /// Internal Server Error; Failed to update channel
        InternalServerError,
    }

    impl std::convert::TryFrom<http::StatusCode> for UpdateRedemptionStatusInformation {
        type Error = std::borrow::Cow<'static, str>;

        fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
            match s {
                http::StatusCode::OK => Ok(UpdateRedemptionStatusInformation::Success),
                http::StatusCode::BAD_REQUEST => {
                    Ok(UpdateRedemptionStatusInformation::MissingQuery)
                }
                http::StatusCode::FORBIDDEN => Ok(UpdateRedemptionStatusInformation::Forbidden),
                http::StatusCode::NOT_FOUND => Ok(UpdateRedemptionStatusInformation::NotFound),
                other => Err(other.canonical_reason().unwrap_or("").into()),
            }
        }
    }

    impl helix::Request for UpdateRedemptionStatusRequest {
        type Response = UpdateRedemptionStatusInformation;

        const PATH: &'static str = "channel_points/custom_rewards/redemptions";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] =
            &[twitch_oauth2::scopes::Scope::ChannelManageBroadcast];
    }

    impl helix::RequestPatch for UpdateRedemptionStatusRequest {
        type Body = UpdateRedemptionStatusBody;
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = UpdateRedemptionStatusRequest::builder()
            .broadcaster_id("274637212".to_string())
            .reward_id("92af127c-7326-4483-a52b-b0da0be61c01".to_string())
            .id("17fa2df1-ad76-4804-bfa5-a40ef63efe63".to_string())
            .build();

        // From twitch docs
        let data = br##"
 {
    "data": [
        {
            "broadcaster_name": "torpedo09",
            "broadcaster_id": "274637212",
            "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
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

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

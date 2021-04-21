//! Deletes a Custom Reward on a channel.
//! [`delete-custom-reward`](https://dev.twitch.tv/docs/api/reference#delete-custom-reward)
//!
//! # Accessing the endpoint
//!
//! ## Request: [DeleteCustomRewardRequest]
//!
//! To use this endpoint, construct a [`DeleteCustomRewardRequest`] with the [`DeleteCustomRewardRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::points::delete_custom_reward;
//! let request = delete_custom_reward::DeleteCustomRewardRequest::builder()
//!     .broadcaster_id("274637212")
//!     .id("1234")
//!     .build();
//! ```
//!
//! ## Response: [DeleteCustomReward]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, points::delete_custom_reward};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = delete_custom_reward::DeleteCustomRewardRequest::builder()
//!     .broadcaster_id("274637212")
//!     .id("1234")
//!     .build();
//! let response: delete_custom_reward::DeleteCustomReward = client.req_delete(request, &token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`DeleteCustomRewardRequest::parse_response(&request.get_uri(), response)`](DeleteCustomRewardRequest::parse_response)

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Delete CustomReward](super::delete_custom_reward)
///
/// [`delete-custom-reward`](https://dev.twitch.tv/docs/api/reference#delete-custom-reward)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct DeleteCustomRewardRequest {
    /// Provided broadcaster_id must match the user_id in the auth token
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// ID of the Custom Reward to delete, must match a Custom Reward on broadcaster_id’s channel.
    #[builder(setter(into))]
    pub id: types::RewardId,
}
// FIXME: Should return VideoIds
/// Return Values for [Delete CustomReward](super::delete_custom_reward)
///
/// [`delete-custom-reward`](https://dev.twitch.tv/docs/api/reference#delete-custom-reward)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum DeleteCustomReward {
    /// Reward deleted
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

impl std::convert::TryFrom<http::StatusCode> for DeleteCustomReward {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
        match s {
            http::StatusCode::NO_CONTENT => Ok(DeleteCustomReward::Success),
            http::StatusCode::BAD_REQUEST => Ok(DeleteCustomReward::BadRequest),
            http::StatusCode::UNAUTHORIZED => Ok(DeleteCustomReward::AuthFailed),
            http::StatusCode::FORBIDDEN => Ok(DeleteCustomReward::Forbidden),
            http::StatusCode::NOT_FOUND => Ok(DeleteCustomReward::NotFound),
            // http::StatusCode::INTERNAL_SERVER_ERROR => Ok(DeleteCustomReward::InternalServerError),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}

impl Request for DeleteCustomRewardRequest {
    type Response = DeleteCustomReward;

    const PATH: &'static str = "channel_points/custom_rewards";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelManageRedemptions];
}

impl RequestDelete for DeleteCustomRewardRequest {}

#[test]
fn test_request() {
    use helix::*;
    let req = DeleteCustomRewardRequest::builder()
        .broadcaster_id("274637212")
        .id("b045196d-9ce7-4a27-a9b9-279ed341ab28")
        .build();

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212&id=b045196d-9ce7-4a27-a9b9-279ed341ab28"
    );

    dbg!(DeleteCustomRewardRequest::parse_response(&uri, http_response).unwrap());
}

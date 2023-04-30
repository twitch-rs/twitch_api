//! Deletes a Custom Reward on a channel.
//! [`delete-custom-reward`](https://dev.twitch.tv/docs/api/reference#delete-custom-reward)
//!
//! # Accessing the endpoint
//!
//! ## Request: [DeleteCustomRewardRequest]
//!
//! To use this endpoint, construct a [`DeleteCustomRewardRequest`] with the [`DeleteCustomRewardRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::points::delete_custom_reward;
//! let request = delete_custom_reward::DeleteCustomRewardRequest::new(
//!     "274637212",
//!     "1234",
//! );
//! ```
//!
//! ## Response: [DeleteCustomReward]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, points::delete_custom_reward};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = delete_custom_reward::DeleteCustomRewardRequest::new("274637212", "1234");
//! let response: delete_custom_reward::DeleteCustomReward = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`DeleteCustomRewardRequest::parse_response(None, &request.get_uri(), response)`](DeleteCustomRewardRequest::parse_response)

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Delete CustomReward](super::delete_custom_reward)
///
/// [`delete-custom-reward`](https://dev.twitch.tv/docs/api/reference#delete-custom-reward)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct DeleteCustomRewardRequest<'a> {
    /// Provided broadcaster_id must match the user_id in the auth token
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// ID of the Custom Reward to delete, must match a Custom Reward on broadcaster_id’s channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub id: Cow<'a, types::RewardIdRef>,
}

impl<'a> DeleteCustomRewardRequest<'a> {
    /// Reward to delete
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

// FIXME: Should return VideoIds
/// Return Values for [Delete CustomReward](super::delete_custom_reward)
///
/// [`delete-custom-reward`](https://dev.twitch.tv/docs/api/reference#delete-custom-reward)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum DeleteCustomReward {
    /// Reward deleted
    Success,
}

impl Request for DeleteCustomRewardRequest<'_> {
    type Response = DeleteCustomReward;

    const PATH: &'static str = "channel_points/custom_rewards";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelManageRedemptions];
}

impl RequestDelete for DeleteCustomRewardRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(helix::Response {
                data: DeleteCustomReward::Success,
                pagination: None,
                request,
                total: None,
                other: None,
            }),
            _ => Err(helix::HelixRequestDeleteError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = DeleteCustomRewardRequest::new("274637212", "b045196d-9ce7-4a27-a9b9-279ed341ab28");

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212&id=b045196d-9ce7-4a27-a9b9-279ed341ab28"
    );

    dbg!(DeleteCustomRewardRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

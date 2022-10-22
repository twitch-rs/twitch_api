//! Removes a moderator from the broadcaster’s chat room.
//! [`remove-channel-moderator`](https://dev.twitch.tv/docs/api/reference#remove-channel-moderator)
//!
//! # Accessing the endpoint
//!
//! ## Request: [RemoveChannelModeratorRequest]
//!
//! To use this endpoint, construct a [`RemoveChannelModeratorRequest`] with the [`RemoveChannelModeratorRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::remove_channel_moderator;
//! let request = remove_channel_moderator::RemoveChannelModeratorRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! ```
//!
//! ## Response: [RemoveChannelModeratorResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::remove_channel_moderator};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = remove_channel_moderator::RemoveChannelModeratorRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! let response: helix::moderation::RemoveChannelModeratorResponse = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`RemoveChannelModeratorRequest::parse_response(None, &request.get_uri(), response)`](RemoveChannelModeratorRequest::parse_response)

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Remove Channel Moderator](super::remove_channel_moderator)
///
/// [`remove-channel-moderator`](https://dev.twitch.tv/docs/api/reference#remove-channel-moderator)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct RemoveChannelModeratorRequest {
    /// The ID of the broadcaster that owns the chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_id: types::UserId,
    /// The ID of the user to remove as a moderator from the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_id: types::UserId,
}

impl RemoveChannelModeratorRequest {
    /// Remove moderator
    pub fn new(
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into(),
            moderator_id: moderator_id.into(),
        }
    }
}

/// Return Values for [Remove Channel Moderator](super::remove_channel_moderator)
///
/// [`remove-channel-moderator`](https://dev.twitch.tv/docs/api/reference#remove-channel-moderator)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum RemoveChannelModeratorResponse {
    /// Successfully removed the moderator.
    Success,
}

impl Request for RemoveChannelModeratorRequest {
    type Response = RemoveChannelModeratorResponse;

    const PATH: &'static str = "moderation/moderators";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageModerators];
}

impl RequestDelete for RemoveChannelModeratorRequest {
    fn parse_inner_response<'d>(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response {
                data: RemoveChannelModeratorResponse::Success,
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
    let req = RemoveChannelModeratorRequest::new("1234", "5678");

    dbg!(req.create_request("token", "clientid").unwrap());

    // From twitch docs
    let data = b"".to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/moderators?broadcaster_id=1234&moderator_id=5678"
    );

    dbg!(RemoveChannelModeratorRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

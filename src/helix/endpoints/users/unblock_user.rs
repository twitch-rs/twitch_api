//! Unblocks the specified user on behalf of the authenticated user.
//! [`unblock-user`](https://dev.twitch.tv/docs/api/reference#unblock-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UnblockUserRequest]
//!
//! To use this endpoint, construct a [`UnblockUserRequest`] with the [`UnblockUserRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::unblock_user;
//! let request = unblock_user::UnblockUserRequest::builder()
//!     .target_user_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [UnblockUser]
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::unblock_user};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = unblock_user::UnblockUserRequest::builder()
//!     .target_user_id("1234")
//!     .build();
//! let response: unblock_user::UnblockUser = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`UnblockUserRequest::parse_response(None, &request.get_uri(), response)`](UnblockUserRequest::parse_response)

use super::*;
use helix::RequestDelete;
/// Query Parameters for [Unblock User](super::unblock_user)
///
/// [`unblock-user`](https://dev.twitch.tv/docs/api/reference#unblock-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UnblockUserRequest<'a> {
    /// User ID of the follower
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub target_user_id: &'a types::UserIdRef,
}

impl<'a> UnblockUserRequest<'a> {
    /// Create a new unblock request
    pub fn unblock_user(target_user_id: impl Into<&'a types::UserIdRef>) -> Self {
        Self {
            target_user_id: target_user_id.into(),
        }
    }
}

/// Return Values for [Unblock User](super::unblock_user)
///
/// [`unblock-user`](https://dev.twitch.tv/docs/api/reference#unblock-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum UnblockUser {
    /// 204 - User unblocked successfully.
    Success,
}

impl Request for UnblockUserRequest<'_> {
    type Response = UnblockUser;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "users/blocks";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserManageBlockedUsers];
}

impl RequestDelete for UnblockUserRequest<'_> {
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
            http::StatusCode::NO_CONTENT => Ok(helix::Response {
                data: UnblockUser::Success,
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
    let req = UnblockUserRequest::unblock_user("41245071");

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();
    // FIXME: I have not tested this in production

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/blocks?target_user_id=41245071"
    );

    dbg!(UnblockUserRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

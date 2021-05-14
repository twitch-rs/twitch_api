//! Unblocks the specified user on behalf of the authenticated user.
//! [`unblock-user`](https://dev.twitch.tv/docs/api/reference#unblock-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UnblockUserRequest]
//!
//! To use this endpoint, construct a [`UnblockUserRequest`] with the [`UnblockUserRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::users::unblock_user;
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
//! use twitch_api2::helix::{self, users::unblock_user};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = unblock_user::UnblockUserRequest::builder()
//!     .target_user_id("1234")
//!     .build();
//! let response: unblock_user::UnblockUser = client.req_delete(request, &token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`UnblockUserRequest::parse_response(&request.get_uri(), response)`](UnblockUserRequest::parse_response)

use super::*;
use helix::RequestDelete;
/// Query Parameters for [Unblock User](super::unblock_user)
///
/// [`unblock-user`](https://dev.twitch.tv/docs/api/reference#unblock-user)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UnblockUserRequest {
    /// User ID of the follower
    #[builder(setter(into))]
    pub target_user_id: types::UserId,
}

/// Return Values for [Unblock User](super::unblock_user)
///
/// [`unblock-user`](https://dev.twitch.tv/docs/api/reference#unblock-user)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum UnblockUser {
    /// 204 - User unblocked successfully.
    Success,
    /// 400 - Request was invalid.
    InvalidRequest,
    /// 401 - Authorization failed.
    AuthFailed,
}

impl std::convert::TryFrom<http::StatusCode> for UnblockUser {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
        match s {
            http::StatusCode::NO_CONTENT => Ok(UnblockUser::Success),
            http::StatusCode::BAD_REQUEST => Ok(UnblockUser::InvalidRequest),
            http::StatusCode::UNAUTHORIZED => Ok(UnblockUser::AuthFailed),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}

impl Request for UnblockUserRequest {
    type Response = UnblockUser;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "users/blocks";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserManageBlockedUsers];
}

impl RequestDelete for UnblockUserRequest {}

#[test]
fn test_request() {
    use helix::*;
    let req = UnblockUserRequest::builder()
        .target_user_id("41245071".to_string())
        .build();

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();
    // FIXME: I have not tested this in production

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/blocks?target_user_id=41245071"
    );

    dbg!(UnblockUserRequest::parse_response(&uri, http_response).unwrap());
}

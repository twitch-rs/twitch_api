//! Deletes a specified user from the followers of a specified channel.
//! [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
//!
//! # Notes
//!
//! This doesn't seem to work for removing people who follow owner of token. Use [Block User](crate::helix::users::block_user) for that
//!
//! # Accessing the endpoint
//!
//! ## Request: [DeleteUserFollowsRequest]
//!
//! To use this endpoint, construct a [`DeleteUserFollowsRequest`] with the [`DeleteUserFollowsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::users::delete_user_follows;
//! let request = delete_user_follows::DeleteUserFollowsRequest::builder()
//!     .from_id("1234").to_id("4321")
//!     .build();
//! ```
//!
//! ## Response: [DeleteUserFollow]
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, users::delete_user_follows};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = delete_user_follows::DeleteUserFollowsRequest::builder()
//!     .from_id("1234").to_id("4321")
//!     .build();
//! let response: delete_user_follows::DeleteUserFollow = client.req_delete(request, &token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`DeleteUserFollowsRequest::parse_response(&request.get_uri(), response)`](DeleteUserFollowsRequest::parse_response)

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Delete Users Follows](super::delete_user_follows)
///
/// [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct DeleteUserFollowsRequest {
    /// User ID of the follower
    #[builder(setter(into))]
    pub from_id: types::UserId,
    /// Channel to be unfollowed by the user
    #[builder(setter(into))]
    pub to_id: types::UserId,
}
/// Return Values for [Delete Users Follows](super::delete_user_follows)
///
/// [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum DeleteUserFollow {
    /// 204 - User successfully deleted from list of channel followers
    Success,
    /// 400 - Missing Query Parameter
    MissingQuery,
    /// 422 - Entity cannot be processed
    ProcessingError,
}

impl std::convert::TryFrom<http::StatusCode> for DeleteUserFollow {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
        match s {
            http::StatusCode::NO_CONTENT => Ok(DeleteUserFollow::Success),
            http::StatusCode::BAD_REQUEST => Ok(DeleteUserFollow::MissingQuery),
            http::StatusCode::UNPROCESSABLE_ENTITY => Ok(DeleteUserFollow::ProcessingError),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}

impl Request for DeleteUserFollowsRequest {
    type Response = DeleteUserFollow;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "users/follows";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditFollows];
}

impl RequestDelete for DeleteUserFollowsRequest {}

#[test]
fn test_request() {
    use helix::*;
    let req = DeleteUserFollowsRequest::builder()
        .to_id("41245072".to_string())
        .from_id("41245071".to_string())
        .build();

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();
    // FIXME: I have not tested this in production

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/follows?from_id=41245071&to_id=41245072"
    );

    dbg!(DeleteUserFollowsRequest::parse_response(&uri, http_response).unwrap());
}

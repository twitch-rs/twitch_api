//! Blocks the specified user on behalf of the authenticated user.
//! [`block-user`](https://dev.twitch.tv/docs/api/reference#block-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [BlockUserRequest]
//!
//! To use this endpoint, construct a [`BlockUserRequest`] with the [`BlockUserRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::users::block_user::{self, SourceContext, Reason};
//! let request = block_user::BlockUserRequest::builder()
//!     .target_user_id("1234")
//!     .build();
//! // Or, specifying a reason for the block
//! let request = block_user::BlockUserRequest::builder()
//!     .target_user_id("1234")
//!     .source_context(SourceContext::Chat)
//!     .reason(Reason::Spam)
//!     .build();
//! ```
//!
//! ## Response: [BlockUser]
//!
//! Send the request to receive the response with [`HelixClient::req_put()`](helix::HelixClient::req_put).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, users::block_user};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = block_user::BlockUserRequest::builder()
//!     .target_user_id("1234")
//!     .build();
//! let response: block_user::BlockUser = client.req_put(request, helix::EmptyBody, &token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPut::create_request)
//! and parse the [`http::Response`] with [`BlockUserRequest::parse_response(&request.get_uri(), response)`](BlockUserRequest::parse_response)
use super::*;
use helix::RequestPut;

/// Query Parameters for [Block User](super::block_user)
///
/// [`block-user`](https://dev.twitch.tv/docs/api/reference#block-user)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct BlockUserRequest {
    /// User ID of the follower
    #[builder(default, setter(into))]
    pub target_user_id: types::UserId,
    /// Source context for blocking the user. Valid values: "chat", "whisper".
    #[builder(default, setter(into))]
    pub source_context: Option<SourceContext>,
    /// Reason for blocking the user. Valid values: "spam", "harassment", or "other".
    #[builder(default, setter(into))]
    pub reason: Option<Reason>,
}

/// Source context for blocking the user.
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SourceContext {
    /// Chat
    Chat,
    /// Whisper
    Whispher,
}

/// Reason for blocking the user.
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Reason {
    /// Spam
    Spam,
    /// Harassment
    Harassment,
    /// Other
    Other,
}
/// Return Values for [Block User](super::block_user)
///
/// [`block-user`](https://dev.twitch.tv/docs/api/reference#block-user)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum BlockUser {
    /// 204 - User blocked successfully.
    Success,
    /// 400 - Request was invalid.
    InvalidRequest,
    /// 401 - Authorization failed.
    AuthFailed,
}

impl std::convert::TryFrom<http::StatusCode> for BlockUser {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
        match s {
            http::StatusCode::NO_CONTENT => Ok(BlockUser::Success),
            http::StatusCode::BAD_REQUEST => Ok(BlockUser::InvalidRequest),
            http::StatusCode::UNAUTHORIZED => Ok(BlockUser::AuthFailed),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}

impl Request for BlockUserRequest {
    type Response = BlockUser;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "users/blocks";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserManageBlockedUsers];
}

impl RequestPut for BlockUserRequest {
    type Body = helix::EmptyBody;
}

#[test]
fn test_request() {
    use helix::*;
    let req = BlockUserRequest::builder()
        .target_user_id("41245071".to_string())
        .build();

    dbg!(req.create_request(EmptyBody, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();
    // FIXME: I have not tested this in production

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/blocks?target_user_id=41245071"
    );

    dbg!(BlockUserRequest::parse_response(&uri, http_response).unwrap());
}

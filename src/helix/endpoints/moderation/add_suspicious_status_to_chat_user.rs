//! Adds a suspicious user status to a chatter on the broadcaster’s channel.
//! [`add-suspicious-status-to-chat-user`](https://dev.twitch.tv/docs/api/reference#add-suspicious-status-to-chat-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [AddSuspiciousStatusToChatUserRequest]
//!
//! To use this endpoint, construct a [`AddSuspiciousStatusToChatUserRequest`] with the [`AddSuspiciousStatusToChatUserRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::add_suspicious_status_to_chat_user;
//! let request = add_suspicious_status_to_chat_user::AddSuspiciousStatusToChatUserRequest::new("1234", "5678");
//! ```
//!
//! ## Body: [AddSuspiciousStatusToChatUserBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::moderation::{add_suspicious_status_to_chat_user, AddedSuspiciousUserStatus};
//! let body = add_suspicious_status_to_chat_user::AddSuspiciousStatusToChatUserBody::new("9876", AddedSuspiciousUserStatus::Restricted);
//! ```
//!
//! ## Response: [SuspiciousUserInfo]
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::{add_suspicious_status_to_chat_user, AddedSuspiciousUserStatus, SuspiciousUserInfo}};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = add_suspicious_status_to_chat_user::AddSuspiciousStatusToChatUserRequest::new("1234", "5678");
//! let body = add_suspicious_status_to_chat_user::AddSuspiciousStatusToChatUserBody::new("9876", AddedSuspiciousUserStatus::Restricted);
//! let response: SuspiciousUserInfo = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`AddSuspiciousStatusToChatUserRequest::parse_response(None, &request.get_uri(), response)`](AddSuspiciousStatusToChatUserRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Add Suspicious Status to Chat User](super::add_suspicious_status_to_chat_user)
///
/// [`add-suspicious-status-to-chat-user`](https://dev.twitch.tv/docs/api/reference#add-suspicious-status-to-chat-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct AddSuspiciousStatusToChatUserRequest<'a> {
    /// The user ID of the broadcaster, indicating the channel where the status is being applied.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The user ID of the moderator who is applying the status.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> AddSuspiciousStatusToChatUserRequest<'a> {
    /// Add a suspicious user status to a chatter on the broadcaster’s channel.
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
        }
    }
}

/// Body Parameters for [Add Suspicious Status to Chat User](super::add_suspicious_status_to_chat_user)
///
/// [`add-suspicious-status-to-chat-user`](https://dev.twitch.tv/docs/api/reference#add-suspicious-status-to-chat-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct AddSuspiciousStatusToChatUserBody<'a> {
    /// The ID of the user being given the suspicious status.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// The type of suspicious status.
    pub status: AddedSuspiciousUserStatus,
}

/// A user's suspicious status when adding.
///
/// This is the subset of valid values of [SuspiciousUserStatus] that can be specified when adding a status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddedSuspiciousUserStatus {
    /// The user is actively monitored.
    ActiveMonitoring,
    /// The user is restricted.
    Restricted,
}

impl<'a> AddSuspiciousStatusToChatUserBody<'a> {
    /// Create a new [`AddSuspiciousStatusToChatUserBody`]
    pub fn new(
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        status: AddedSuspiciousUserStatus,
    ) -> Self {
        Self {
            user_id: user_id.into_cow(),
            status,
        }
    }
}

impl helix::private::SealedSerialize for AddSuspiciousStatusToChatUserBody<'_> {}

impl Request for AddSuspiciousStatusToChatUserRequest<'_> {
    type PaginationData = ();
    type Response = SuspiciousUserInfo;

    const PATH: &'static str = "moderation/suspicious_users";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageSuspiciousUsers];
}

impl<'a> RequestPost for AddSuspiciousStatusToChatUserRequest<'a> {
    type Body = AddSuspiciousStatusToChatUserBody<'a>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        helix::parse_single_return(request, uri, response, status)
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = AddSuspiciousStatusToChatUserRequest::new("141981764", "12826");

    let body =
        AddSuspiciousStatusToChatUserBody::new("9876", AddedSuspiciousUserStatus::Restricted);

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"user_id":"9876","status":"RESTRICTED"}"#
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "user_id": "9876",
                "broadcaster_id": "141981764",
                "moderator_id": "12826",
                "updated_at": "2025-12-01T23:08:18+00:00",
                "status": "RESTRICTED",
                "types": [
                    "MANUALLY_ADDED"
                ]
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/suspicious_users?broadcaster_id=141981764&moderator_id=12826"
    );

    dbg!(
        AddSuspiciousStatusToChatUserRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
    );
}

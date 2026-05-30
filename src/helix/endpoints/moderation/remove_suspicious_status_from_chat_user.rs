//! Remove a suspicious user status from a chatter on broadcaster’s channel.
//! [`remove-suspicious-status-from-chat-user`](https://dev.twitch.tv/docs/api/reference#remove-suspicious-status-from-chat-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [RemoveSuspiciousStatusFromChatUserRequest]
//!
//! To use this endpoint, construct a [`RemoveSuspiciousStatusFromChatUserRequest`] with the [`RemoveSuspiciousStatusFromChatUserRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::remove_suspicious_status_from_chat_user;
//! let request = remove_suspicious_status_from_chat_user::RemoveSuspiciousStatusFromChatUserRequest::new("1234", "5678", "9876");
//! ```
//!
//! ## Response: [SuspiciousUserInfo]
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::{remove_suspicious_status_from_chat_user, SuspiciousUserInfo}};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = remove_suspicious_status_from_chat_user::RemoveSuspiciousStatusFromChatUserRequest::new("1234", "5678", "9876");
//! let response: SuspiciousUserInfo = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`RemoveSuspiciousStatusFromChatUserRequest::parse_response(None, &request.get_uri(), response)`](RemoveSuspiciousStatusFromChatUserRequest::parse_response)

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Remove Suspicious Status From Chat User](super::remove_suspicious_status_from_chat_user)
///
/// [`remove-suspicious-status-from-chat-user`](https://dev.twitch.tv/docs/api/reference#remove-suspicious-status-from-chat-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct RemoveSuspiciousStatusFromChatUserRequest<'a> {
    /// The user ID of the broadcaster, indicating the channel where the status is being removed.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The user ID of the moderator who is removing the status.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The ID of the user having the suspicious status removed.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
}

impl<'a> RemoveSuspiciousStatusFromChatUserRequest<'a> {
    /// Remove a suspicious user status from a chatter on broadcaster’s channel.
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            user_id: user_id.into_cow(),
        }
    }
}

impl Request for RemoveSuspiciousStatusFromChatUserRequest<'_> {
    type PaginationData = ();
    type Response = SuspiciousUserInfo;

    const PATH: &'static str = "moderation/suspicious_users";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageSuspiciousUsers];
}

impl<'a> RequestDelete for RemoveSuspiciousStatusFromChatUserRequest<'a> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
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
    let req = RemoveSuspiciousStatusFromChatUserRequest::new("141981764", "12826", "9876");

    dbg!(req.create_request("token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "user_id": "9876",
                "broadcaster_id": "141981764",
                "moderator_id": "12826",
                "updated_at": "2025-12-01T23:08:18+00:00",
                "status": "NO_TREATMENT",
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
        "https://api.twitch.tv/helix/moderation/suspicious_users?broadcaster_id=141981764&moderator_id=12826&user_id=9876"
    );

    dbg!(
        RemoveSuspiciousStatusFromChatUserRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
    );
}

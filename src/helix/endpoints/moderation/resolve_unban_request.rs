//! Resolves an unban request by approving or denying it.
//!
//! [`resolve-unban-requests`](https://dev.twitch.tv/docs/api/reference#resolve-unban-requests)
//!
//! # Accessing the endpoint
//!
//! ## Request: [ResolveUnbanRequest]
//!
//! To use this endpoint, construct an [`ResolveUnbanRequest`] with the [`ResolveUnbanRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::resolve_unban_request;
//! let request = resolve_unban_request::ResolveUnbanRequest::approve(
//!     "123",
//!     "456",
//!     "123-456-789",
//! )
//! .resolution_text("something");
//! ```
//!
//! ## Response: [UnbanRequest]
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::resolve_unban_request};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = resolve_unban_request::ResolveUnbanRequest::approve(
//!     "123",
//!     "456",
//!     "123-456-789",
//! );
//! let response: helix::moderation::UnbanRequest = client.req_patch(request, helix::EmptyBody, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPut::create_request)
//! and parse the [`http::Response`] with [`ResolveUnbanRequest::parse_response(None, &request.get_uri(), response)`](ResolveUnbanRequest::parse_response)

use super::*;
use helix::RequestPatch;

pub use super::{UnbanRequest, UnbanRequestStatus};

/// Query Parameters for [Resolve Unban Request](super::resolve_unban_request)
///
/// [`resolve-unban-requests`](https://dev.twitch.tv/docs/api/reference#resolve-unban-requests)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ResolveUnbanRequest<'a> {
    /// The ID of the broadcaster whose channel is approving or denying the unban request.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that has permission to moderate the broadcaster’s unban requests. This ID must match the user ID in the user access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The ID of the Unban Request to resolve.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub unban_request_id: Cow<'a, types::UnbanRequestIdRef>,
    /// Resolution status.
    ///
    /// Only [Approved](UnbanRequestStatus::Approved) and [Denied](UnbanRequestStatus::Denied) are accepted.
    pub status: UnbanRequestStatus,
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    /// Message supplied by the unban request resolver. The message is limited to a maximum of 500 characters.
    pub resolution_text: Option<Cow<'a, str>>,
}

impl<'a> ResolveUnbanRequest<'a> {
    /// Approve an unban request
    pub fn approve(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        unban_request_id: impl types::IntoCow<'a, types::UnbanRequestIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            unban_request_id: unban_request_id.into_cow(),
            status: UnbanRequestStatus::Approved,
            resolution_text: None,
        }
    }

    /// Deny an unban request
    pub fn deny(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        unban_request_id: impl types::IntoCow<'a, types::UnbanRequestIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            unban_request_id: unban_request_id.into_cow(),
            status: UnbanRequestStatus::Denied,
            resolution_text: None,
        }
    }

    /// Resolve an unban request
    ///
    /// Only [Approved](UnbanRequestStatus::Approved) and [Denied](UnbanRequestStatus::Denied) are accepted as the status.
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        unban_request_id: impl types::IntoCow<'a, types::UnbanRequestIdRef> + 'a,
        status: UnbanRequestStatus,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            unban_request_id: unban_request_id.into_cow(),
            status,
            resolution_text: None,
        }
    }

    /// Set the resolution text for a request
    pub fn resolution_text(mut self, text: impl types::IntoCow<'a, str> + 'a) -> Self {
        self.resolution_text = Some(text.into_cow());
        self
    }
}

impl Request for ResolveUnbanRequest<'_> {
    type PaginationData = ();
    type Response = super::UnbanRequest;

    const PATH: &'static str = "moderation/unban_requests";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageUnbanRequests];
}

impl RequestPatch for ResolveUnbanRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestPatchError>
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
    // modified from twitch example (they specified no resolution and the wrong moderator)
    let req = ResolveUnbanRequest::approve(
        "274637212",
        "141981764",
        "92af127c-7326-4483-a52b-b0da0be61c01",
    )
    .resolution_text("okay");

    req.create_request(helix::EmptyBody, "token", "clientid")
        .unwrap();

    // Own response because the docs are wrong
    let data = br#"
    {
        "data": [
            {
                "broadcaster_id": "129546453",
                "broadcaster_login": "nerixyz",
                "broadcaster_name": "nerixyz",
                "created_at": "2024-10-12T18:34:38Z",
                "id": "1dfff107-17fc-44cb-9f64-570a33757ac0",
                "moderator_id": "129546453",
                "moderator_login": "nerixyz",
                "moderator_name": "nerixyz",
                "resolution_text": "",
                "resolved_at": "2024-10-12T18:45:47Z",
                "status": "approved",
                "text": "My unban request text",
                "user_id": "489584266",
                "user_login": "uint128",
                "user_name": "uint128"
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/unban_requests?broadcaster_id=274637212&moderator_id=141981764&unban_request_id=92af127c-7326-4483-a52b-b0da0be61c01&status=approved&resolution_text=okay"
    );

    let res = ResolveUnbanRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.status, UnbanRequestStatus::Approved);
    assert_eq!(res.resolution_text.as_deref(), Some(""));
}

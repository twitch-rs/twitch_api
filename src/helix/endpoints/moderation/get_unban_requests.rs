//! Gets a list of unban requests for a broadcaster’s channel.
//! [`get-unban-requests`](https://dev.twitch.tv/docs/api/reference#get-unban-requests)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetUnbanRequestsRequest]
//!
//! To use this endpoint, construct a [`GetUnbanRequestsRequest`] with the [`GetUnbanRequestsRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::get_unban_requests;
//! // get pending unban requests
//! let request = get_unban_requests::GetUnbanRequestsRequest::new(
//!     "1234",
//!     "5678",
//!     get_unban_requests::UnbanRequestStatus::Pending,
//! );
//! ```
//!
//! ## Response: [UnbanRequest]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::get_unban_requests};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_unban_requests::GetUnbanRequestsRequest::new("1234", "5678", get_unban_requests::UnbanRequestStatus::Pending);
//! let response: Vec<get_unban_requests::UnbanRequest> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUnbanRequestsRequest::parse_response(None, &request.get_uri(), response)`](GetUnbanRequestsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// The status of an unban request
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UnbanRequestStatus {
    /// The request has been created, but not yet resolved
    Pending,
    /// The request has been approved by a moderator/broadcaster
    Approved,
    /// The request has been denied by a moderator/broadcaster
    Denied,
    /// The request has been approved and the user acknowledged the resolution
    Acknowledged,
    /// The user cancelled the request
    Canceled,
}

/// Query Parameters for [Get Unban Requests](super::get_unban_requests)
///
/// [`get-unban-requests`](https://dev.twitch.tv/docs/api/reference#get-unban-requests)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetUnbanRequestsRequest<'a> {
    /// The ID of the broadcaster whose channel is receiving unban requests.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that has permission to moderate the broadcaster’s unban requests. This ID must match the user ID in the user access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// Filter by a status.
    pub status: UnbanRequestStatus,
    /// The ID used to filter what unban requests are returned.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Option<Cow<'a, types::UserIdRef>>,
    /// Cursor used to get next page of results. Pagination object in response contains cursor value.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// The maximum number of items to return per page in response
    #[cfg_attr(feature = "typed-builder", builder(setter(into), default))]
    pub first: Option<usize>,
}

impl<'a> GetUnbanRequestsRequest<'a> {
    /// Get Unban Requests in a broadcasters channel filtered by a status
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        status: UnbanRequestStatus,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            status,
            user_id: None,
            after: None,
            first: None,
        }
    }

    /// Filter for unban requests from a specific user
    pub fn user(mut self, user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        self.user_id = Some(user_id.into_cow());
        self
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Unban Requests](super::get_unban_requests)
///
/// [`get-unban-requests`](https://dev.twitch.tv/docs/api/reference#get-unban-requests)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UnbanRequest {
    /// Unban request ID.
    pub id: types::UnbanRequestId,
    /// User ID of broadcaster whose channel is receiving the unban request.
    pub broadcaster_id: types::UserId,
    /// The broadcaster's display name.
    pub broadcaster_name: types::DisplayName,
    /// The broadcaster's login name.
    pub broadcaster_login: types::UserName,
    /// User ID of moderator who approved/denied the request.
    pub moderator_id: Option<types::UserId>,
    /// The moderator's display name.
    pub moderator_name: Option<types::DisplayName>,
    /// The moderator's login name.
    pub moderator_login: Option<types::UserName>,
    /// User ID of the requestor who is asking for an unban.
    pub user_id: types::UserId,
    /// The user's display name.
    pub user_name: types::DisplayName,
    /// The user's login name.
    pub user_login: types::UserName,
    /// Text of the request from the requesting user.
    pub text: String,
    /// Status of the request.
    pub status: UnbanRequestStatus,
    /// Timestamp of when the unban request was created.
    pub created_at: types::Timestamp,
    /// Timestamp of when moderator/broadcaster approved or denied the request.
    pub resolved_at: Option<types::Timestamp>,
    /// Text input by the resolver (moderator) of the unban. request
    pub resolution_text: Option<String>,
}

impl Request for GetUnbanRequestsRequest<'_> {
    type Response = Vec<UnbanRequest>;

    const PATH: &'static str = "moderation/unban_requests";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ModeratorReadUnbanRequests,
        twitch_oauth2::Scope::ModeratorManageUnbanRequests
    )];
}

impl RequestGet for GetUnbanRequestsRequest<'_> {}

impl helix::Paginated for GetUnbanRequestsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request_pending() {
    use helix::*;
    let req = GetUnbanRequestsRequest::new("274637212", "274637212", UnbanRequestStatus::Pending);

    let data = br#"
{
  "data": [
    {
      "broadcaster_id": "129546453",
      "broadcaster_login": "nerixyz",
      "broadcaster_name": "nerixyz",
      "created_at": "2024-10-12T18:34:38Z",
      "id": "1dfff107-17fc-44cb-9f64-570a33757ac0",
      "moderator_id": null,
      "moderator_login": null,
      "moderator_name": null,
      "resolution_text": null,
      "resolved_at": null,
      "status": "pending",
      "text": "My unban request text",
      "user_id": "489584266",
      "user_login": "uint128",
      "user_name": "uint128"
    }
  ],
  "pagination": {
    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
  }
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/unban_requests?broadcaster_id=274637212&moderator_id=274637212&status=pending"
    );

    let data = GetUnbanRequestsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(data.len(), 1);
    let pending = &data[0];

    assert_eq!(pending.status, UnbanRequestStatus::Pending);
    assert_eq!(pending.id.as_str(), "1dfff107-17fc-44cb-9f64-570a33757ac0");
    assert_eq!(pending.broadcaster_login.as_str(), "nerixyz");
    assert_eq!(pending.moderator_login.as_deref(), None);
    assert_eq!(pending.user_login.as_str(), "uint128");
    assert_eq!(pending.resolution_text.as_deref(), None);
    assert_eq!(pending.resolved_at.as_deref(), None);
}

#[cfg(test)]
#[test]
fn test_request_approved() {
    use helix::*;
    let req = GetUnbanRequestsRequest::new("274637212", "274637212", UnbanRequestStatus::Approved);

    // slightly modified from twitch docs
    let data = br#"
{
  "data": [
    {
      "id": "92af127c-7326-4483-a52b-b0da0be61c01",
      "broadcaster_name": "torpedo09",
      "broadcaster_login": "torpedo09",
      "broadcaster_id": "274637212",
      "moderator_id": "141981764",
      "moderator_login": "twitchdev",
      "moderator_name": "TwitchDev",
      "user_id": "424596340",
      "user_login": "quotrok",
      "user_name": "quotrok",
      "text": "Please unban me from the channel?",
      "status": "approved",
      "created_at": "2022-08-07T02:07:55Z",
      "resolved_at": "2022-08-09T02:07:55Z",
      "resolution_text": ""
    }
  ],
  "pagination": {
    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
  }
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/unban_requests?broadcaster_id=274637212&moderator_id=274637212&status=approved"
    );

    let data = GetUnbanRequestsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(data.len(), 1);
    let approved = &data[0];

    assert_eq!(approved.status, UnbanRequestStatus::Approved);
    assert_eq!(approved.id.as_str(), "92af127c-7326-4483-a52b-b0da0be61c01");
    assert_eq!(approved.broadcaster_login.as_str(), "torpedo09");
    assert_eq!(
        approved.moderator_login.as_ref().unwrap().as_str(),
        "twitchdev"
    );
    assert_eq!(approved.user_login.as_str(), "quotrok");
    assert_eq!(approved.resolution_text.as_deref(), Some(""));
    assert_eq!(
        approved.resolved_at.as_ref().unwrap().as_str(),
        "2022-08-09T02:07:55Z"
    );
}

//! Gets the broadcaster’s list of non-private, blocked words or phrases. These are the terms that the broadcaster or moderator added manually, or that were denied by AutoMod.
//!
//! [`get-blocked-terms`](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetBlockedTermsRequest]
//!
//! To use this endpoint, construct a [`GetBlockedTermsRequest`] with the [`GetBlockedTermsRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::get_blocked_terms;
//! let request =
//!     get_blocked_terms::GetBlockedTermsRequest::new("1234", "5678");
//! ```
//!
//! ## Response: [BlockedTerm]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::get_blocked_terms};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_blocked_terms::GetBlockedTermsRequest::new("1234", "5678");
//! let response: Vec<helix::moderation::BlockedTerm> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetBlockedTermsRequest::parse_response(None, &request.get_uri(), response)`](GetBlockedTermsRequest::parse_response)

use super::*;
use helix::{PaginationState, RequestGet};

/// Query Parameters for [Get Blocked Terms](super::get_blocked_terms)
///
/// [`get-blocked-terms`](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
#[must_use]
pub struct GetBlockedTermsRequest<'a> {
    /// The ID of the broadcaster whose blocked terms you’re getting.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID associated with the user OAuth token.
    /// If the broadcaster wants to get their own block terms (instead of having the moderator do it), set this parameter to the broadcaster’s ID, too.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The maximum number of blocked terms to return per page in the response. The minimum page size is 1 blocked term per page and the maximum is 100. The default is 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<u32>,
    /// The cursor used to get the next page of results. The Pagination object in the response contains the cursor’s value.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
}

impl<'a> GetBlockedTermsRequest<'a> {
    /// Get blocked terms in a broadcasters channel as specified moderator
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
            after: Default::default(),
            first: Default::default(),
        }
    }

    /// Set amount of results returned per page.
    pub const fn first(mut self, first: u32) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Blocked Terms](super::get_blocked_terms)
///
/// [`get-blocked-terms`](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
pub type GetBlockedTermsResponse = BlockedTerm;

impl Request for GetBlockedTermsRequest<'_> {
    type PaginationData = PaginationState<Self>;
    type Response = Vec<BlockedTerm>;

    const PATH: &'static str = "moderation/blocked_terms";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ModeratorReadBlockedTerms,
        twitch_oauth2::Scope::ModeratorManageBlockedTerms
    )];
}

impl RequestGet for GetBlockedTermsRequest<'_> {}

impl helix::Paginated for GetBlockedTermsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetBlockedTermsRequest::new("1234", "5678").first(10);

    // From twitch docs, FIXME: has ... and a "bad" comma
    let data = br#"
    {
      "data": [
        {
          "broadcaster_id": "1234",
          "moderator_id": "5678",
          "id": "520e4d4e-0cda-49c7-821e-e5ef4f88c2f2",
          "text": "A phrase I'm not fond of",
          "created_at": "2021-09-29T19:45:37Z",
          "updated_at": "2021-09-29T19:45:37Z",
          "expires_at": null
        }
      ],
      "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6I..."
      }
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/blocked_terms?broadcaster_id=1234&moderator_id=5678&first=10"
    );

    dbg!(GetBlockedTermsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

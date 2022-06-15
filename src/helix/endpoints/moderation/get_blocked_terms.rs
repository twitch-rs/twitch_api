//! Gets the broadcaster’s list of non-private, blocked words or phrases. These are the terms that the broadcaster or moderator added manually, or that were denied by AutoMod.
//! [`get-blocked-terms`](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetBlockedTerms]
//!
//! To use this endpoint, construct a [`GetBlockedTerms`] with the [`GetBlockedTerms::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::get_blocked_terms;
//! let request = get_blocked_terms::GetBlockedTerms::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
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
//! let request = get_blocked_terms::GetBlockedTerms::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! let response: Vec<helix::moderation::BlockedTerm> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetBlockedTerms::parse_response(None, &request.get_uri(), response)`](GetBlockedTerms::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Blocked Terms](super::get_blocked_terms)
///
/// [`get-blocked-terms`](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetBlockedTerms {
    /// The ID of the broadcaster whose blocked terms you’re getting.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID associated with the user OAuth token.
    /// If the broadcaster wants to get their own block terms (instead of having the moderator do it), set this parameter to the broadcaster’s ID, too.
    #[builder(setter(into))]
    pub moderator_id: types::UserId,
    /// The maximum number of blocked terms to return per page in the response. The minimum page size is 1 blocked term per page and the maximum is 100. The default is 20.
    #[builder(default, setter(into))]
    pub first: Option<u32>,
    /// The cursor used to get the next page of results. The Pagination object in the response contains the cursor’s value.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
}

/// Return Values for [Get Blocked Terms](super::get_blocked_terms)
///
/// [`get-blocked-terms`](https://dev.twitch.tv/docs/api/reference#get-blocked-terms)
pub type GetBlockedTermsResponse = BlockedTerm;

impl Request for GetBlockedTerms {
    type Response = Vec<BlockedTerm>;

    const PATH: &'static str = "moderation/blocked_terms";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorReadBlockedTerms];
}

impl RequestGet for GetBlockedTerms {}

impl helix::Paginated for GetBlockedTerms {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetBlockedTerms::builder()
        .broadcaster_id("1234")
        .moderator_id("5678")
        .first(10)
        .build();

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

    dbg!(GetBlockedTerms::parse_response(Some(req), &uri, http_response).unwrap());
}

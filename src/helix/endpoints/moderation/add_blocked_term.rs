//! Adds a word or phrase to the broadcaster’s list of blocked terms. These are the terms that broadcasters don’t want used in their chat room.
//! [`add-blocked-term`](https://dev.twitch.tv/docs/api/reference#add-blocked-term)
//!
//! # Accessing the endpoint
//!
//! ## Request: [AddBlockedTermRequest]
//!
//! To use this endpoint, construct a [`AddBlockedTermRequest`] with the [`AddBlockedTermRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api2::helix::moderation::add_blocked_term;
//! let request = add_blocked_term::AddBlockedTermRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! ```
//!
//! ## Body: [AddBlockedTermBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::moderation::add_blocked_term;
//! let body = add_blocked_term::AddBlockedTermBody::new("A phrase I'm not fond of".to_string());
//! ```
//!
//! ## Response: [BlockedTerm]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::add_blocked_term};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = add_blocked_term::AddBlockedTermRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! let body = add_blocked_term::AddBlockedTermBody::new("A phrase I'm not fond of".to_string());
//! let response: &helix::moderation::BlockedTerm = client.req_post(request, body, &token).await?.data.first().unwrap();
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`AddBlockedTermRequest::parse_response(None, &request.get_uri(), response)`](AddBlockedTermRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Add Blocked Term](super::add_blocked_term)
///
/// [`add-blocked-term`](https://dev.twitch.tv/docs/api/reference#add-blocked-term)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct AddBlockedTermRequest {
    /// The ID of the broadcaster that owns the list of blocked terms.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID associated with the user OAuth token.
    ///
    /// If the broadcaster wants to add the blocked term (instead of having the moderator do it), set this parameter to the broadcaster’s ID, too.
    #[builder(setter(into))]
    pub moderator_id: types::UserId,
}

/// Body Parameters for [Add Blocked Term](super::add_blocked_term)
///
/// [`add-blocked-term`](https://dev.twitch.tv/docs/api/reference#add-blocked-term)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct AddBlockedTermBody {
    ///The word or phrase to block from being used in the broadcaster’s chat room.
    ///
    /// The term must contain a minimum of 2 characters and may contain up to a maximum of 500 characters.
    /// Terms can use a wildcard character (*). The wildcard character must appear at the beginning or end of a word, or set of characters. For example, *foo or foo*.
    pub text: String,
}

impl AddBlockedTermBody {
    /// Create a new [`AddBlockedTermBody`]
    pub fn new(text: String) -> Self { Self { text } }
}

impl helix::private::SealedSerialize for AddBlockedTermBody {}

impl helix::HelixRequestBody for Vec<AddBlockedTermBody> {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a Vec<AddBlockedTermBody>,
        }

        serde_json::to_vec(&InnerBody { data: self })
            .map_err(Into::into)
            .map(Into::into)
    }
}

/// Return Values for [Add Blocked Term](super::add_blocked_term)
///
/// [`add-blocked-term`](https://dev.twitch.tv/docs/api/reference#add-blocked-term)
pub type AddBlockedTermResponse = BlockedTerm;

impl Request for AddBlockedTermRequest {
    // FIXME: this is a single entry
    type Response = Vec<BlockedTerm>;

    const PATH: &'static str = "moderation/blocked_terms";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorManageBlockedTerms];
}

impl RequestPost for AddBlockedTermRequest {
    type Body = AddBlockedTermBody;
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = AddBlockedTermRequest::builder()
        .broadcaster_id("1234")
        .moderator_id("5678")
        .build();

    let body = AddBlockedTermBody::new("A phrase I'm not fond of".to_string());

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "broadcaster_id": "713936733",
            "moderator_id": "713936733",
            "id": "3bb6e5d3-afb1-416c-ad4e-21af970ccfe7",
            "text": "A phrase I'm not fond of",
            "created_at": "2021-09-29T15:36:45Z",
            "updated_at": "2021-09-29T15:36:45Z",
            "expires_at": null
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/blocked_terms?broadcaster_id=1234&moderator_id=5678"
    );

    dbg!(AddBlockedTermRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

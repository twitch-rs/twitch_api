//! Gets a specified user’s block list. The list is sorted by when the block occurred in descending order (i.e. most recent block first).
//!
//! [`get-user-block-list`](https://dev.twitch.tv/docs/api/reference#get-user-block-list)
//!
//! ## Request: [GetUserBlockListRequest]
//!
//! To use this endpoint, construct a [`GetUserBlockListRequest`] with the [`GetUserBlockListRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::get_user_block_list;
//! let request =
//!     get_user_block_list::GetUserBlockListRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [UserBlock]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::get_user_block_list};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_user_block_list::GetUserBlockListRequest::broadcaster_id("1234");
//! let response: Vec<get_user_block_list::UserBlock> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUserBlockListRequest::parse_response(None, &request.get_uri(), response)`](GetUserBlockListRequest::parse_response)

use super::*;
use helix::{PaginationState, RequestGet};

/// Query Parameters for [Get Users Block List](super::get_user_block_list)
///
/// [`get-user-block-list`](https://dev.twitch.tv/docs/api/reference#get-user-block-list)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetUserBlockListRequest<'a> {
    /// User ID for a Twitch user.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
}

impl<'a> GetUserBlockListRequest<'a> {
    /// Get a specified user’s block list
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            after: Default::default(),
            first: Default::default(),
        }
    }
}

/// Return Values for [Get Users Block List](super::get_user_block_list)
///
/// [`get-user-block-list`](https://dev.twitch.tv/docs/api/reference#get-user-block-list)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserBlock {
    /// User ID of the blocked user.
    pub user_id: types::UserId,
    /// Login of the blocked user.
    pub user_login: types::UserName,
    /// Display name of the blocked user.
    pub display_name: types::DisplayName,
}

impl Request for GetUserBlockListRequest<'_> {
    type PaginationData = PaginationState<Self>;
    type Response = Vec<UserBlock>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::UserReadBlockedUsers];
    const PATH: &'static str = "users/blocks";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadBlockedUsers];
}

impl RequestGet for GetUserBlockListRequest<'_> {}

impl helix::Paginated for GetUserBlockListRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetUserBlockListRequest::broadcaster_id("23161357");

    // From twitch docs // FIXME: twitch docs say id, not user_id
    let data = br#"
{
    "data": [
        {
        "user_id": "135093069",
        "user_login": "bluelava",
        "display_name": "BlueLava"
        },
        {
        "user_id": "27419011",
        "user_login": "travistyoj",
        "display_name": "TravistyOJ"
        }
    ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/blocks?broadcaster_id=23161357"
    );

    dbg!(GetUserBlockListRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

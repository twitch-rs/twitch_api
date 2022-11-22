//! Gets the list of users that are connected to the specified broadcaster’s chat session.
//! [`get-chatters`](https://dev.twitch.tv/docs/api/reference#get-chatters)
//!
//! # Notes
//!
//! there is a delay between when users join and leave a chat and when the list is updated accordingly.
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetChattersRequest]
//!
//! To use this endpoint, construct a [`GetChattersRequest`] with the [`GetChattersRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::get_chatters;
//! let request = get_chatters::GetChattersRequest::new("1234", "4321");
//! ```
//!
//! ## Response: [Chatter]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::get_chatters};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_chatters::GetChattersRequest::new("1234", "4321");
//! let response: Vec<helix::chat::Chatter> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChattersRequest::parse_response(None, &request.get_uri(), response)`](GetChattersRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Chatters](super::get_chatters)
///
/// [`get-chatters`](https://dev.twitch.tv/docs/api/reference#get-chatters)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetChattersRequest<'a> {
    /// The ID of the broadcaster whose list of chatters you want to get.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the moderator or the specified broadcaster that’s requesting the list of chatters. This ID must match the user ID associated with the user access token.
    ///
    /// The moderator must have permission to moderate the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
    /// The maximum number of items to return per page in the response. The minimum page size is 1 item per page and the maximum is 1,000. The default is 100.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// The cursor used to get the next page of results. The Pagination object in the response contains the cursor’s value.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
}

impl<'a> GetChattersRequest<'a> {
    /// Get chatters in broadcasters channel
    ///
    /// # Notes
    ///
    /// The moderator has to be the token owner and can moderate the chat
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
            moderator_id: moderator_id.to_cow(),
            first: None,
            after: None,
        }
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

impl helix::Paginated for GetChattersRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

/// Return Values for [Get Chatters](super::get_chatters)
///
/// [`get-chatters`](https://dev.twitch.tv/docs/api/reference#get-chatters)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Chatter {
    /// The ID of a user that’s connected to the broadcaster’s chat room.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,
}

impl Request for GetChattersRequest<'_> {
    type Response = Vec<Chatter>;

    const PATH: &'static str = "chat/chatters";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetChattersRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChattersRequest::new("123456", "654321");

    // From twitch docs
    // FIXME: Example has ...
    let data = br#"
    {
        "data": [{
                "user_id": "128393656",
                "user_login": "smittysmithers",
                "user_name": "smittysmithers"
            }],
        "pagination": {
            "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
        },
        "total": 8
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321"
    );

    dbg!(GetChattersRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

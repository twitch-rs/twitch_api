//! Returns all moderators in a channel.
//! [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetModeratorsRequest]
//!
//! To use this endpoint, construct a [`GetModeratorsRequest`] with the [`GetModeratorsRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::get_moderators;
//! let request = get_moderators::GetModeratorsRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [Moderator]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::get_moderators};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_moderators::GetModeratorsRequest::broadcaster_id("1234");
//! let response: Vec<get_moderators::Moderator> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetModeratorsRequest::parse_response(None, &request.get_uri(), response)`](GetModeratorsRequest::parse_response)
use super::*;
use helix::RequestGet;

// Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
// Maximum: 100
/// Query Parameters for [Get Moderators](super::get_moderators)
///
/// [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetModeratorsRequest<'a> {
    /// Must match the User ID in the Bearer token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Filters the results and only returns a status object for users who are moderators in this channel and have a matching user_id.
    #[cfg_attr(feature = "typed-builder", builder(setter(into), default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub user_id: types::Collection<'a, types::UserId>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Number of values to be returned per page. Limit: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(setter(into), default))]
    pub first: Option<usize>,
}

impl<'a> GetModeratorsRequest<'a> {
    /// Get moderators in a broadcasters channel.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            user_id: types::Collection::default(),
            after: Default::default(),
            first: Default::default(),
        }
    }

    /// Filter the results for specific users.
    pub fn user_ids(mut self, user_ids: impl Into<types::Collection<'a, types::UserId>>) -> Self {
        self.user_id = user_ids.into();
        self
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Moderators](super::get_moderators)
///
/// [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Moderator {
    /// User ID of a moderator in the channel.
    pub user_id: types::UserId,
    /// Display name of a moderator in the channel.
    pub user_name: types::DisplayName,
    /// Login of a moderator in the channel.
    pub user_login: types::UserName,
}

impl Request for GetModeratorsRequest<'_> {
    type Response = Vec<Moderator>;

    const PATH: &'static str = "moderation/moderators";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ModerationRead,
        twitch_oauth2::Scope::ChannelManageModerators
    )];
}

impl RequestGet for GetModeratorsRequest<'_> {}

impl helix::Paginated for GetModeratorsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetModeratorsRequest::broadcaster_id("198704263");

    // From twitch docs
    let data = br#"
{
    "data": [
        {
            "user_id": "424596340",
            "user_login": "quotrok",
            "user_name": "quotrok"
        },
        {
            "user_id": "424596340",
            "user_login": "quotrok",
            "user_name": "quotrok"
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
        "https://api.twitch.tv/helix/moderation/moderators?broadcaster_id=198704263"
    );

    dbg!(GetModeratorsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

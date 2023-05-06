//! Gets a list of broadcasters that the specified user follows. You can also use this endpoint to see whether a user follows a specific broadcaster.
//! [`get-followed-channels`](https://dev.twitch.tv/docs/api/reference#get-followed-channels)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetFollowedChannels]
//!
//! To use this endpoint, construct a [`GetFollowedChannels`] with the [`GetFollowedChannels::user_id()`] or [`GetFollowedChannels::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::get_followed_channels;
//! let request = get_followed_channels::GetFollowedChannels::user_id("1234");
//! ```
//!
//! ## Response: [FollowedBroadcaster]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::get_followed_channels};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_followed_channels::GetFollowedChannels::user_id("1234");
//! let response: Vec<get_followed_channels::FollowedBroadcaster> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetFollowedChannels::parse_response(None, &request.get_uri(), response)`](GetFollowedChannels::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Followed Channels](super::get_followed_channels)
///
/// [`get-followed-channels`](https://dev.twitch.tv/docs/api/reference#get-followed-channels)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetFollowedChannels<'a> {
    /// A user’s ID. Use this parameter to see whether the user follows this broadcaster.
    /// If specified, the response contains this user if they follow the broadcaster.
    /// If not specified, the response contains all users that follow the broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// FollowedBroadcaster’s user ID associated with the channel.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Option<Cow<'a, types::UserIdRef>>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
}

impl<'a> GetFollowedChannels<'a> {
    /// Get specified broadcasters channel editors
    pub fn user_id(user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: user_id.into_cow(),
            broadcaster_id: None,
            after: None,
            first: None,
        }
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }

    /// Check if the user is following this broadcaster
    pub fn broadcaster_id(
        self,
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: Some(broadcaster_id.into_cow()),
            ..self
        }
    }
}

/// Return Values for [Get Followed Channels](super::get_followed_channels)
///
/// [`get-followed-channels`](https://dev.twitch.tv/docs/api/reference#get-followed-channels)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct FollowedBroadcaster {
    /// An ID that uniquely identifies the broadcaster that this user is following.
    pub broadcaster_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_name: types::DisplayName,
    /// The UTC timestamp when the user started following the broadcaster.
    pub followed_at: types::UserId,
}

impl Request for GetFollowedChannels<'_> {
    type Response = Vec<FollowedBroadcaster>;

    const PATH: &'static str = "channels/followed";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadFollows];
}

impl RequestGet for GetFollowedChannels<'_> {}

impl helix::Paginated for GetFollowedChannels<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetFollowedChannels::user_id("123456");

    // From twitch docs
    // FIXME: example has trailing `,`, is missing a `,` and has ...
    let data = br#"
    {
        "total": 8,
        "data": [
            {
                "broadcaster_id": "11111",
                "broadcaster_login": "userloginname",
                "broadcaster_name": "UserDisplayName",
                "followed_at": "2022-05-24T22:22:08Z"
            },
            {
                "broadcaster_id": "11111",
                "broadcaster_login": "userloginname",
                "broadcaster_name": "UserDisplayName",
                "followed_at": "2022-05-24T22:22:08Z"
            }
        ],
        "pagination": {
            "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
        }
    }
      "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/followed?user_id=123456"
    );

    dbg!(GetFollowedChannels::parse_response(Some(req), &uri, http_response).unwrap());
}

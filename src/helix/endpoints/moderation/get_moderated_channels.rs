//! Gets a list of channels that the specified user has moderator privileges in.
//! [`get-moderated-channels`](https://dev.twitch.tv/docs/api/reference#get-moderated-channels)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetModeratedChannelsRequest]
//!
//! To use this endpoint, construct a [`GetModeratedChannelsRequest`] with the [`GetModeratedChannelsRequest::user_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::get_moderated_channels;
//! let request =
//!     get_moderated_channels::GetModeratedChannelsRequest::user_id("1234");
//! ```
//!
//! ## Response: [ModeratedChannel]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::get_moderated_channels};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_moderated_channels::GetModeratedChannelsRequest::user_id("1234");
//! let response: Vec<get_moderated_channels::ModeratedChannel> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetModeratedChannelsRequest::parse_response(None, &request.get_uri(), response)`](GetModeratedChannelsRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Moderated Channels](super::get_moderated_channels)
///
/// [`get-moderated-channels`](https://dev.twitch.tv/docs/api/reference#get-moderated-channels)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetModeratedChannelsRequest<'a> {
    /// A user’s ID. Returns the list of channels that this user has moderator privileges in. This ID must match the user ID in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// The cursor used to get the next page of results. The Pagination object in the response contains the cursor’s value.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// The maximum number of items to return per page in the response. Limit: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(setter(into), default))]
    pub first: Option<usize>,
}

impl<'a> GetModeratedChannelsRequest<'a> {
    /// Get Moderated Channels for an authenticated user.
    pub fn user_id(user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: user_id.into_cow(),
            after: Default::default(),
            first: Default::default(),
        }
    }

    /// Set amount of results returned per page.
    pub const fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Moderated Channels](super::get_moderated_channels)
///
/// [`get-moderated-channels`](https://dev.twitch.tv/docs/api/reference#get-moderated-channels)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ModeratedChannel {
    /// An ID that uniquely identifies the channel this user can moderate.
    pub broadcaster_id: types::UserId,
    /// The channel’s login name.
    pub broadcaster_login: types::UserName,
    /// The channels’ display name.
    pub broadcaster_name: types::DisplayName,
}

impl Request for GetModeratedChannelsRequest<'_> {
    type Response = Vec<ModeratedChannel>;

    const PATH: &'static str = "moderation/channels";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadModeratedChannels];
}

impl RequestGet for GetModeratedChannelsRequest<'_> {}

impl helix::Paginated for GetModeratedChannelsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetModeratedChannelsRequest::user_id("931931");

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "broadcaster_id" : "12345",
                "broadcaster_login" : "grateful_broadcaster",
                "broadcaster_name" : "Grateful_Broadcaster"
            },
            {
                "broadcaster_id" : "98765",
                "broadcaster_login" : "bashfulgamer",
                "broadcaster_name" : "BashfulGamer"
            }
        ],
        "pagination" : {
            "cursor" : "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
        }
    }
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/channels?user_id=931931"
    );

    let res = GetModeratedChannelsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.len(), 2);

    let (first, second) = (&res[0], &res[1]);
    assert_eq!(first.broadcaster_id.as_str(), "12345");
    assert_eq!(first.broadcaster_login.as_str(), "grateful_broadcaster");
    assert_eq!(first.broadcaster_name.as_str(), "Grateful_Broadcaster");
    assert_eq!(second.broadcaster_id.as_str(), "98765");
    assert_eq!(second.broadcaster_login.as_str(), "bashfulgamer");
    assert_eq!(second.broadcaster_name.as_str(), "BashfulGamer");
}

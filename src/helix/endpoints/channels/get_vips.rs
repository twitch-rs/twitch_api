//! Gets a list of the channel’s VIPs.
//! [`get-vips`](https://dev.twitch.tv/docs/api/reference#get-vips)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetVipsRequest]
//!
//! To use this endpoint, construct a [`GetVipsRequest`] with the [`GetVipsRequest::broadcaster_id()`] or [`GetVipsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::get_vips;
//! let request = get_vips::GetVipsRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [Vip]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::get_vips};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_vips::GetVipsRequest::broadcaster_id("1234");
//! let response: Vec<get_vips::Vip> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetVipsRequest::parse_response(None, &request.get_uri(), response)`](GetVipsRequest::parse_response)
use super::*;
use helix::{PaginationState, RequestGet};

/// Query Parameters for [Get VIPs](super::get_vips)
///
/// [`get-vips`](https://dev.twitch.tv/docs/api/reference#get-vips)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetVipsRequest<'a> {
    /// The ID of the broadcaster whose list of VIPs you want to get.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Filters the list for specific VIPs. To specify more than one user, include the user_id parameter for each user to get. For example, &user_id=1234&user_id=5678. The maximum number of IDs that you may specify is 100. Ignores those users in the list that aren’t VIPs.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub user_id: types::Collection<'a, types::UserId>,
    /// The maximum number of items to return per page in the response. The minimum page size is 1 item per page and the maximum is 100. The default is 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// The cursor used to get the next page of results. The Pagination object in the response contains the cursor’s value. Read more.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
}

impl<'a> GetVipsRequest<'a> {
    /// Get channel VIPs in channel
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::channels::GetVipsRequest;
    /// let request = GetVipsRequest::broadcaster_id("1337");
    /// ```
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            user_id: types::Collection::default(),
            first: None,
            after: None,
        }
    }

    /// Set amount of results returned per page.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::channels::GetVipsRequest;
    /// let request = GetVipsRequest::broadcaster_id("1234").first(100);
    /// ```
    pub const fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }

    /// Filter response with these IDs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::channels::GetVipsRequest;
    ///
    /// let user_ids: &[&twitch_types::UserIdRef] = &["1234".into()];
    /// let request = GetVipsRequest::broadcaster_id("1337").user_ids(user_ids);
    /// ```
    pub fn user_ids(self, user_ids: impl Into<types::Collection<'a, types::UserId>>) -> Self {
        Self {
            user_id: user_ids.into(),
            ..self
        }
    }
}

/// Return Values for [Get VIPs](super::get_vips)
///
/// [`get-vips`](https://dev.twitch.tv/docs/api/reference#get-vips)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Vip {
    /// An ID that uniquely identifies the VIP user.
    pub user_id: types::UserId,
    /// The user’s display name.
    pub user_name: types::DisplayName,
    /// The user’s login name.
    pub user_login: types::UserName,
}

impl Request for GetVipsRequest<'_> {
    type PaginationData = PaginationState<Self>;
    type Response = Vec<Vip>;

    const PATH: &'static str = "channels/vips";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadVips,
        twitch_oauth2::Scope::ChannelManageVips
    )];
}

impl helix::Paginated for GetVipsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

impl RequestGet for GetVipsRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request_all() {
    use helix::*;
    let req = GetVipsRequest::broadcaster_id("123");

    // From twitch docs
    // FIXME: Example has ...
    let data = br#"
      {
        "data": [
          {
            "user_id": "11111",
            "user_name": "UserDisplayName",
            "user_login": "userloginname"
          }
        ],
        "pagination": {
          "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
        }
      }"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/vips?broadcaster_id=123"
    );

    dbg!(GetVipsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_multiple() {
    use helix::*;

    let ids: &[&types::UserIdRef] = &["456".into(), "678".into()];
    let req = GetVipsRequest::broadcaster_id("123").user_ids(ids);

    // From twitch docs
    // FIXME: Example has ...
    let data = br#"
      {
        "data": [
          {
            "user_id": "11111",
            "user_name": "UserDisplayName",
            "user_login": "userloginname"
          }
        ],
        "pagination": {
          "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
        }
      }"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/vips?broadcaster_id=123&user_id=456&user_id=678"
    );

    dbg!(GetVipsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

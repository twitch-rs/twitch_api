//! Gets a list of the channel’s VIPs.
//! [`get-vips`](https://dev.twitch.tv/docs/api/reference#get-vips)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetVipsRequest]
//!
//! To use this endpoint, construct a [`GetVipsRequest`] with the [`GetVipsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::get_vips;
//! let request = get_vips::GetVipsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
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
//! let request = get_vips::GetVipsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_vips::Vip> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetVipsRequest::parse_response(None, &request.get_uri(), response)`](GetVipsRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get VIPs](super::get_vips)
///
/// [`get-vips`](https://dev.twitch.tv/docs/api/reference#get-vips)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetVipsRequest {
    /// The ID of the broadcaster whose list of VIPs you want to get.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_id: types::UserId,
    /// Filters the list for specific VIPs. To specify more than one user, include the user_id parameter for each user to get. For example, &user_id=1234&user_id=5678. The maximum number of IDs that you may specify is 100. Ignores those users in the list that aren’t VIPs.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub user_id: Vec<types::UserId>,
    /// The maximum number of items to return per page in the response. The minimum page size is 1 item per page and the maximum is 100. The default is 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// The cursor used to get the next page of results. The Pagination object in the response contains the cursor’s value. Read more.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub after: Option<helix::Cursor>,
}

impl GetVipsRequest {
    /// Get channel VIPs in channel
    pub fn broadcaster_id(broadcaster_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into(),
            user_id: vec![],
            first: None,
            after: None,
        }
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }

    /// Filter response with this ID
    pub fn user_id(self, user_id: impl Into<types::UserId>) -> Self {
        Self {
            user_id: vec![user_id.into()],
            ..self
        }
    }

    /// Filter response with these IDs
    pub fn user_ids(self, user_ids: impl IntoIterator<Item = impl Into<types::UserId>>) -> Self {
        Self {
            user_id: user_ids.into_iter().map(Into::into).collect(),
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

impl Request for GetVipsRequest {
    type Response = Vec<Vip>;

    const PATH: &'static str = "channels/vips";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadVips];
}

impl helix::Paginated for GetVipsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor; }
}

impl RequestGet for GetVipsRequest {}

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
    let req = GetVipsRequest::broadcaster_id("123").user_ids(["456", "678"]);

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

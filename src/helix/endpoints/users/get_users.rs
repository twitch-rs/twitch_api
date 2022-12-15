//! Gets information about one or more specified Twitch users.
//! [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
//!
//! ## Request: [GetUsersRequest]
//!
//! To use this endpoint, construct a [`GetUsersRequest`] with the [`GetUsersRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::get_users;
//! let request = get_users::GetUsersRequest::builder()
//!     .id(&["1234".into()][..])
//!     .login(&["justintvfan".into()][..])
//!     .build();
//! ```
//!
//! ## Response: [User]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::get_users};
//! # use twitch_api::{client, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let ids: &[&types::UserIdRef] = &["1234".into()];
//! let logins: &[&types::UserNameRef] = &["justintvfan".into()];
//! let request = get_users::GetUsersRequest::builder()
//!     .id(ids)
//!     .login(logins)
//!     .build();
//! let response: Vec<get_users::User> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUsersRequest::parse_response(None, &request.get_uri(), response)`](GetUsersRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Users](super::get_users)
///
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetUsersRequest<'a> {
    /// User ID. Multiple user IDs can be specified. Limit: 100.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "Cow::Borrowed(&[])", setter(into))
    )]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub id: Cow<'a, [&'a types::UserIdRef]>,
    /// User login name. Multiple login names can be specified. Limit: 100.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "Cow::Borrowed(&[])", setter(into))
    )]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub login: Cow<'a, [&'a types::UserNameRef]>,
}

impl<'a> GetUsersRequest<'a> {
    /// Get multiple user by their [`UserName`](types::UserName)
    ///
    /// ```rust
    /// use twitch_api::helix::users::get_users::GetUsersRequest;
    /// GetUsersRequest::logins(&["twitch".into(), "justintv".into()][..]);
    /// ```
    pub fn logins(login: impl Into<Cow<'a, [&'a types::UserNameRef]>>) -> Self {
        Self {
            id: Cow::Borrowed(&[]),
            login: login.into(),
        }
    }

    /// Get multiple user by their [`UserId`](types::UserId)
    pub fn ids(ids: impl Into<Cow<'a, [&'a types::UserIdRef]>>) -> Self {
        Self {
            id: ids.into(),
            login: Cow::Borrowed(&[]),
        }
    }

    /// Returns an empty [`GetUsersRequest`]
    ///
    /// # Notes
    ///
    /// This is not a valid request, it needs to be filled out with other fields.
    pub fn new() -> Self {
        Self {
            id: Cow::Borrowed(&[]),
            login: Cow::Borrowed(&[]),
        }
    }
}

/// Return Values for [Get Users](super::get_users)
///
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone, yoke::Yokeable)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct User<'a> {
    /// User’s broadcaster type: "partner", "affiliate", or "".
    pub broadcaster_type: Option<types::BroadcasterType>,
    /// Date when the user was created.
    pub created_at: &'a str,
    /// User’s channel description.
    pub description: Option<String>,
    /// User’s display name.
    pub display_name: Cow<'a, types::DisplayNameRef>,
    /// User’s email address. Returned if the request includes the [`user:read:email` scope](twitch_oauth2::Scope::UserReadEmail).
    pub email: Option<String>,
    /// User’s ID.
    pub id: Cow<'a, types::UserId>,
    /// User’s login name.
    pub login: Cow<'a, types::UserNameRef>,
    /// URL of the user’s offline image.
    pub offline_image_url: Option<String>,
    /// URL of the user’s profile image.
    pub profile_image_url: Option<String>,
    /// User’s type: "staff", "admin", "global_mod", or "".
    #[serde(rename = "type")]
    pub type_: Option<types::UserType>,
    #[deprecated(
        since = "0.7.0",
        note = "removed, see https://discuss.dev.twitch.tv/t/get-users-api-endpoint-view-count-deprecation/37777"
    )]
    #[serde(default)]
    /// Total number of views of the user’s channel.
    pub view_count: usize,
}

impl Request for GetUsersRequest<'_> {
    type Response<'de> = Vec<User<'de>>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
    const PATH: &'static str = "users";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetUsersRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;

    let ids: &[&types::UserIdRef] = &["44322889".into()];
    let req = GetUsersRequest::ids(ids);

    // From twitch docs
    // FIXME: This is not valid anymore. Twitch....
    let data = br#"
{
    "data": [
        {
        "id": "141981764",
        "login": "twitchdev",
        "display_name": "TwitchDev",
        "type": "",
        "broadcaster_type": "partner",
        "description": "Supporting third-party developers building Twitch integrations from chatbots to game integrations.",
        "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png",
        "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png",
        "view_count": 5980557,
        "email": "not-real@email.com",
        "created_at": "2016-12-14T20:32:28.894263Z"
        }
    ]
    }
"#
        .to_vec();

    let http_response = http::Response::builder().body(data.as_slice()).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users?id=44322889"
    );

    dbg!(GetUsersRequest::parse_response(Some(req), &uri, &http_response).unwrap());
}

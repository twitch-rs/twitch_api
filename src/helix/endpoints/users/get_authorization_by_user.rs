//! Gets the authorization scopes that the specified user has granted the application.
//! [`get-authorization-by-user`](https://dev.twitch.tv/docs/api/reference#get-authorization-by-user)
//!
//! ## Request: [GetAuthorizationByUserRequest]
//!
//! To use this endpoint, construct a [`GetAuthorizationByUserRequest`] with the [`GetAuthorizationByUserRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::get_authorization_by_user;
//! let request =
//!     get_authorization_by_user::GetAuthorizationByUserRequest::new(&[
//!         "12345", "5678",
//!     ]);
//! ```
//!
//! ## Response: [AuthorizedUser]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::get_authorization_by_user};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_authorization_by_user::GetAuthorizationByUserRequest::new(&["12345", "5678"]);
//! let response: Vec<get_authorization_by_user::AuthorizedUser> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetAuthorizationByUserRequest::parse_response(None, &request.get_uri(), response)`](GetAuthorizationByUserRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Authorization By User](super::get_authorization_by_user)
///
/// [`get-authorization-by-user`](https://dev.twitch.tv/docs/api/reference#get-authorization-by-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetAuthorizationByUserRequest<'a> {
    /// The ID of the user(s) you want to check authorization for.
    ///
    /// The maximum number of IDs you may specify is 10.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "types::Collection::default()", setter(into))
    )]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub user_id: types::Collection<'a, types::UserId>,
}

impl<'a> GetAuthorizationByUserRequest<'a> {
    /// Get multiple user authorizations by their [`UserId`](types::UserId)
    pub fn new(ids: impl Into<types::Collection<'a, types::UserId>>) -> Self {
        Self {
            user_id: ids.into(),
        }
    }
}

/// Return Values for [Get Authorization By User](super::get_authorization_by_user)
///
/// [`get-authorization-by-user`](https://dev.twitch.tv/docs/api/reference#get-authorization-by-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AuthorizedUser {
    /// The user’s ID.
    pub user_id: types::UserId,
    /// The user’s display name.
    pub user_name: types::DisplayName,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// An array of all the scopes the user has granted to the client ID.
    #[cfg(feature = "twitch_oauth2")]
    pub scopes: Vec<twitch_oauth2::Scope>,
    /// An array of all the scopes the user has granted to the client ID.
    #[cfg(not(feature = "twitch_oauth2"))]
    pub scopes: Vec<String>,
}

impl Request for GetAuthorizationByUserRequest<'_> {
    type PaginationData = ();
    type Response = Vec<AuthorizedUser>;

    const PATH: &'static str = "authorization/users";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetAuthorizationByUserRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetAuthorizationByUserRequest::new(&["141981764", "197886470"]);

    let data = br#"
    {
        "data": [
            {
                "user_id": "141981764",
                "user_name": "TwitchDev",
                "user_login": "twitchdev",
                "scopes": [
                    "bits:read", 
                    "channel:bot", 
                    "channel:manage:predictions"
                ]
            },
            {
                "user_id": "197886470",
                "user_name": "TwitchRivals",
                "user_login": "twitchrivals",
                "scopes": [
                    "channel:manage:predictions"
                ]
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/authorization/users?user_id=141981764&user_id=197886470"
    );

    let res = GetAuthorizationByUserRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;

    assert_eq!(res.len(), 2);

    let dev = &res[0];
    let rivals = &res[1];

    assert_eq!(dev.user_login.as_str(), "twitchdev");
    assert_eq!(dev.scopes.len(), 3);
    assert_eq!(rivals.user_login.as_str(), "twitchrivals");
    assert_eq!(rivals.scopes.len(), 1);

    #[cfg(feature = "twitch_oauth2")]
    {
        assert_eq!(dev.scopes[0], twitch_oauth2::Scope::BitsRead);
        assert_eq!(dev.scopes[1], twitch_oauth2::Scope::ChannelBot);
        assert_eq!(
            dev.scopes[2],
            twitch_oauth2::Scope::ChannelManagePredictions
        );
        assert_eq!(
            rivals.scopes[0],
            twitch_oauth2::Scope::ChannelManagePredictions
        );
    }
    #[cfg(not(feature = "twitch_oauth2"))]
    {
        assert_eq!(dev.scopes[0], "bits:read");
        assert_eq!(dev.scopes[1], "channel:bot");
        assert_eq!(dev.scopes[2], "channel:manage:predictions");
        assert_eq!(rivals.scopes[0], "channel:manage:predictions");
    }

    assert_eq!(
        res,
        serde_json::from_str::<Vec<AuthorizedUser>>(&serde_json::to_string(&res).unwrap()).unwrap()
    );
}

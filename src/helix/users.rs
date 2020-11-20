//! Endpoints regarding users
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, users::GetUsersRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetUsersRequest::builder()
//!     .login(vec!["justinfan1337".to_string()])
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use get_users::{GetUsersRequest, User};

#[doc(inline)]
pub use get_users_follows::{GetUsersFollowsRequest, UsersFollow};

#[doc(inline)]
pub use delete_user_follows::{DeleteUserFollow, DeleteUserFollowsRequest};

#[doc(inline)]
pub use create_user_follows::{CreateUserFollows, CreateUserFollowsBody, CreateUserFollowsRequest};

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Gets information about one or more specified Twitch users.
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
///
/// ## Request: [GetUsersRequest]
///
/// To use this endpoint, construct a [`GetUsersRequest`] with the [`GetUsersRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::users::get_users;
/// let request = get_users::GetUsersRequest::builder()
///     .id(vec!["1234".to_string()])
///     .login(vec!["justintvfan".to_string()])
///     .build();
/// ```
///
/// ## Response: [User]
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, users::get_users};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
/// let request = get_users::GetUsersRequest::builder()
///     .id(vec!["1234".to_string()])
///     .login(vec!["justintvfan".to_string()])
///     .build();
/// let response: Vec<get_users::User> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_users {
    use super::*;
    /// Query Parameters for [Get Users](super::get_users)
    ///
    /// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetUsersRequest {
        /// User ID. Multiple user IDs can be specified. Limit: 100.
        #[builder(default)]
        pub id: Vec<types::UserId>,
        /// User login name. Multiple login names can be specified. Limit: 100.
        #[builder(default)]
        pub login: Vec<types::UserName>,
    }

    /// Return Values for [Get Users](super::get_users)
    ///
    /// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct User {
        /// User’s broadcaster type: "partner", "affiliate", or "".
        pub broadcaster_type: Option<types::BroadcasterType>,
        /// User’s channel description.
        pub description: Option<String>,
        /// User’s display name.
        pub display_name: types::DisplayName,
        /// User’s email address. Returned if the request includes the [`user:read:email` scope](twitch_oauth2::Scope::UserReadEmail).
        pub email: Option<String>,
        /// User’s ID.
        pub id: types::UserId,
        /// User’s login name.
        pub login: types::UserName,
        /// URL of the user’s offline image.
        pub offline_image_url: Option<String>,
        /// URL of the user’s profile image.
        pub profile_image_url: Option<String>,
        /// User’s type: "staff", "admin", "global_mod", or "".
        #[serde(rename = "type")]
        pub type_: Option<types::UserType>,
        /// Total number of views of the user’s channel.
        pub view_count: usize,
    }

    impl helix::Request for GetUsersRequest {
        type Response = Vec<User>;

        #[cfg(feature = "twitch_oauth2")]
        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
        const PATH: &'static str = "users";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetUsersRequest {}

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetUsersRequest::builder()
            .id(vec!["44322889".to_string()])
            .build();

        // From twitch docs
        let data = br#"
{
    "data": [{
        "id": "44322889",
        "login": "dallas",
        "display_name": "dallas",
        "type": "staff",
        "broadcaster_type": "",
        "description": "Just a gamer playing games and chatting. :)",
        "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-profile_image-1a2c906ee2c35f12-300x300.png",
        "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-channel_offline_image-1a2c906ee2c35f12-1920x1080.png",
        "view_count": 191836881,
        "email": "login@provider.com"
    }]
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/users?id=44322889"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Gets information on follow relationships between two Twitch users.
/// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
///
/// ## Request: [GetUsersFollowsRequest]
///
/// To use this endpoint, construct a [`GetUsersFollowsRequest`] with the [`GetUsersFollowsRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::users::get_users_follows;
/// let request = get_users_follows::GetUsersFollowsRequest::builder()
///     .to_id("1234".to_string())
///     .build();
/// ```
///
/// ## Response: [UsersFollow]
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, users::get_users_follows};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
/// let request = get_users_follows::GetUsersFollowsRequest::builder()
///     .to_id("1234".to_string())
///     .build();
/// let response: Vec<get_users_follows::UsersFollow> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_users_follows {
    use super::*;
    /// Query Parameters for [Get Users Follows](super::get_users_follows)
    ///
    /// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetUsersFollowsRequest {
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub after: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        pub first: Option<usize>,
        /// User ID. The request returns information about users who are being followed by the from_id user.
        #[builder(default, setter(into))]
        pub from_id: Option<String>,
        /// User ID. The request returns information about users who are following the to_id user.
        #[builder(default, setter(into))]
        pub to_id: Option<String>,
    }

    /// Return Values for [Get Users Follows](super::get_users_follows)
    ///
    /// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct UsersFollow {
        ///Date and time when the from_id user followed the to_id user.
        pub followed_at: types::Timestamp,
        ///ID of the user following the to_id user.
        pub from_id: types::UserId,
        ///Display name corresponding to from_id.
        pub from_name: types::DisplayName,
        ///ID of the user being followed by the from_id user.
        pub to_id: types::UserId,
        ///Display name corresponding to to_id.
        pub to_name: types::DisplayName,
        // FIXME: This never seems to be returned.
        /// Total number of items returned.
        ///
        /// * If only `from_id` was in the request, this is the total number of followed users.
        /// * If only `to_id` was in the request, this is the total number of followers.
        /// * If both `from_id` and to_id were in the request, this is 1 (if the "from" user follows the "to" user) or 0.
        pub total: Option<usize>,
    }

    impl helix::Request for GetUsersFollowsRequest {
        type Response = Vec<UsersFollow>;

        #[cfg(feature = "twitch_oauth2")]
        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
        const PATH: &'static str = "users/follows";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetUsersFollowsRequest {}

    impl helix::Paginated for GetUsersFollowsRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetUsersFollowsRequest::builder()
            .to_id("23161357".to_string())
            .build();

        // From twitch docs
        let data = br#"
{
    "total": 12345,
    "data":
    [
        {
            "from_id": "171003792",
            "from_name": "IIIsutha067III",
            "to_id": "23161357",
            "to_name": "LIRIK",
            "followed_at": "2017-08-22T22:55:24Z"
        },
        {
            "from_id": "113627897",
            "from_name": "Birdman616",
            "to_id": "23161357",
            "to_name": "LIRIK",
            "followed_at": "2017-08-22T22:55:04Z"
        }
    ],
    "pagination":{
        "cursor": "eyJiIjpudWxsLCJhIjoiMTUwMzQ0MTc3NjQyNDQyMjAwMCJ9"
    }
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/users/follows?to_id=23161357"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Deletes a specified user from the followers of a specified channel.
/// [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
///
/// # Notes
///
/// This doesn't seem to work for removing people who follow owner of token. Use twitch web chat `/block <user_login>` for that
///
/// # Accessing the endpoint
///
/// ## Request: [DeleteUserFollowsRequest]
///
/// To use this endpoint, construct a [`DeleteUserFollowsRequest`] with the [`DeleteUserFollowsRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::users::delete_user_follows;
/// let request = delete_user_follows::DeleteUserFollowsRequest::builder()
///     .from_id("1234").to_id("4321")
///     .build();
/// ```
///
/// ## Response: [DeleteUserFollow]
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, users::delete_user_follows};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
/// let request = delete_user_follows::DeleteUserFollowsRequest::builder()
///     .from_id("1234").to_id("4321")
///     .build();
/// let response: delete_user_follows::DeleteUserFollow = client.req_delete(request, &token).await?;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestDelete::parse_response())
pub mod delete_user_follows {
    use super::*;
    /// Query Parameters for [Delete Users Follows](super::delete_user_follows)
    ///
    /// [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct DeleteUserFollowsRequest {
        /// User ID of the follower
        #[builder(default, setter(into))]
        pub from_id: types::UserId,
        /// Channel to be unfollowed by the user
        #[builder(default, setter(into))]
        pub to_id: types::UserId,
    }
    /// Return Values for [[Delete Users Follows](super::delete_user_follows)
    ///
    /// [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub enum DeleteUserFollow {
        /// 204 - User successfully deleted from list of channel followers
        Success,
        /// 400 - Missing Query Parameter
        MissingQuery,
        /// 422 - Entity cannot be processed
        ProcessingError,
    }

    impl std::convert::TryFrom<http::StatusCode> for DeleteUserFollow {
        type Error = std::borrow::Cow<'static, str>;

        fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
            match s {
                http::StatusCode::NO_CONTENT => Ok(DeleteUserFollow::Success),
                http::StatusCode::BAD_REQUEST => Ok(DeleteUserFollow::MissingQuery),
                http::StatusCode::UNPROCESSABLE_ENTITY => Ok(DeleteUserFollow::Success),
                other => Err(other.canonical_reason().unwrap_or("").into()),
            }
        }
    }

    impl helix::Request for DeleteUserFollowsRequest {
        type Response = DeleteUserFollow;

        #[cfg(feature = "twitch_oauth2")]
        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
        const PATH: &'static str = "users/follows";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditFollows];
    }

    impl helix::RequestDelete for DeleteUserFollowsRequest {}

    #[test]
    fn test_request() {
        use helix::*;
        let req = DeleteUserFollowsRequest::builder()
            .to_id("41245072".to_string())
            .from_id("41245071".to_string())
            .build();

        // From twitch docs
        let data = br#""#.to_vec();

        let http_response = http::Response::builder().status(204).body(data).unwrap();
        // FIXME: I have not tested this in production

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/users/follows?from_id=41245071&to_id=41245072"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Adds a specified user to the followers of a specified channel.
/// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
///
/// # Accessing the endpoint
///
/// ## Request: [CreateUserFollowsRequest]
///
/// To use this endpoint, construct a [`CreateUserFollowsRequest`] with the [`CreateUserFollowsRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::users::create_user_follows;
/// let request = create_user_follows::CreateUserFollowsRequest::builder()
///     .build();
/// ```
///
/// ## Body: [CreateUserFollowsBody]
///
/// We also need to provide a body to the request containing what we want to change.
///
/// ```
/// # use twitch_api2::helix::users::create_user_follows;
/// let body = create_user_follows::CreateUserFollowsBody::builder()
///     .build();
/// ```
///
/// ## Response: [CreateUserFollows]
///
///
/// Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
///
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, users::create_user_follows};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
/// let request = create_user_follows::CreateUserFollowsRequest::builder()
///     .build();
/// let body = create_user_follows::CreateUserFollowsBody::builder()
///     .build();
/// let response: create_user_follows::CreateUserFollows = client.req_post(request, body, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestPost::parse_response())
pub mod create_user_follows {
    use std::convert::TryInto;

    use super::*;
    /// Query Parameters for [Create User Follows](super::create_user_follows)
    ///
    /// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
    #[derive(
        PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default,
    )]
    #[non_exhaustive]
    pub struct CreateUserFollowsRequest {}

    /// Body Parameters for [Create User Follows](super::create_user_follows)
    ///
    /// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
    #[derive(
        PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default,
    )]
    #[non_exhaustive]
    pub struct CreateUserFollowsBody {
        /// User ID of the follower
        #[builder(default, setter(into))]
        pub from_id: Option<types::UserId>,
        /// ID of the channel to be followed by the user
        #[builder(default, setter(into))]
        pub to_id: Option<types::UserId>,
    }

    /// Return Values for [Create User Follows](super::create_user_follows)
    ///
    /// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub enum CreateUserFollows {
        // FIXME: Twitch docs....
        /// 204 or 200 - Successfully created follows
        Success,
        /// 400 - Missing Query Parameter
        MissingQuery,
        /// 422 - Entity cannot be processed
        ProcessingError,
    }

    impl std::convert::TryFrom<http::StatusCode> for CreateUserFollows {
        type Error = std::borrow::Cow<'static, str>;

        fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
            match s {
                http::StatusCode::NO_CONTENT | http::StatusCode::OK => {
                    Ok(CreateUserFollows::Success)
                }
                http::StatusCode::BAD_REQUEST => Ok(CreateUserFollows::MissingQuery),
                http::StatusCode::UNPROCESSABLE_ENTITY => Ok(CreateUserFollows::Success),
                other => Err(other.canonical_reason().unwrap_or("").into()),
            }
        }
    }

    impl helix::Request for CreateUserFollowsRequest {
        type Response = CreateUserFollows;

        #[cfg(feature = "twitch_oauth2")]
        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
        const PATH: &'static str = "users/follows";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditFollows];
    }

    impl helix::RequestPost for CreateUserFollowsRequest {
        type Body = CreateUserFollowsBody;

        fn parse_response(
            self,
            uri: &http::Uri,
            response: http::Response<Vec<u8>>,
        ) -> Result<
            helix::Response<Self, <Self as helix::Request>::Response>,
            helix::HelixRequestPostError,
        >
        where
            Self: Sized,
        {
            let text = std::str::from_utf8(&response.body())
                .map_err(|e| helix::HelixRequestPostError::Utf8Error(response.body().clone(), e))?;
            if let Ok(helix::HelixRequestError {
                error,
                status,
                message,
            }) = serde_json::from_str::<helix::HelixRequestError>(&text)
            {
                return Err(helix::HelixRequestPostError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                });
            }

            let response = response.status().try_into().map_err(|_| {
                // This path should never be taken, but just to be sure we do this
                helix::HelixRequestPostError::Error {
                    status: response.status(),
                    uri: uri.clone(),
                    body: response.body().clone(),
                    message: String::new(), // FIXME: None, but this branch should really never be hit
                    error: String::new(),
                }
            })?;
            Ok(helix::Response {
                data: response, // FIXME: This should be a bit better...
                pagination: <_>::default(),
                request: self,
            })
        }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = CreateUserFollowsRequest::builder().build();

        // From twitch docs
        let data = br#""#.to_vec();

        let http_response = http::Response::builder().status(200).body(data).unwrap();
        // This is marked as 204 in twitch docs, but in reality it's 200

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/users/follows?"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

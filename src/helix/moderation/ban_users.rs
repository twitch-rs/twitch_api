//! Bans one or more users from participating in a broadcaster’s chat room, or puts them in a timeout.
//! [`ban-users`](https://dev.twitch.tv/docs/api/reference#ban-users)
//!
//! # Accessing the endpoint
//!
//! ## Request: [BanUsersRequest]
//!
//! To use this endpoint, construct a [`BanUsersRequest`] with the [`BanUsersRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api2::helix::moderation::ban_users;
//! let request = ban_users::BanUsersRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! ```
//!
//! ## Body: [BanUsersBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::moderation::ban_users;
//! let body = ban_users::BanUsersBody::builder()
//!     .msg_id("test1")
//!     .msg_text("automod please approve this!")
//!     .user_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [BanUsersResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::ban_users};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = ban_users::BanUsersRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! let body = vec![ban_users::BanUsersBody::builder()
//!     .msg_id("test1")
//!     .msg_text("automod please approve this!")
//!     .user_id("1234")
//!     .build()];
//! let response: ban_users::BanUsersResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`BanUsersRequest::parse_response(None, &request.get_uri(), response)`](BanUsersRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Ban Users](super::ban_users)
///
/// [`ban-users`](https://dev.twitch.tv/docs/api/reference#ban-users)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct BanUsersRequest {
    /// The ID of the broadcaster whose chat room the user is being banned from.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID associated with the user OAuth token.
    #[builder(setter(into))]
    pub moderator_id: types::UserId,
}

/// Body Parameters for [Ban Users](super::ban_users)
///
/// [`ban-users`](https://dev.twitch.tv/docs/api/reference#ban-users)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct BanUsersBody {
    /// Duration of the (optional) timeout in seconds.
    ///
    /// To ban a user indefinitely, don’t include this field.
    ///
    /// To put a user in a timeout, include this field and specify the timeout period, in seconds.
    /// The minimum timeout is 1 second and the maximum is 1,209,600 seconds (2 weeks).
    /// To end a user’s timeout early, set this field to 1, or send an Unban user request.
    pub duration: Option<u32>,
    /// The reason the user is being banned or put in a timeout. The text is user defined and limited to a maximum of 500 characters.
    pub reason: String,
    /// The ID of the user to ban or put in a timeout.
    pub user_id: types::UserId,
}

impl BanUsersBody {
    /// Create a new [`BanUsersBody`]
    pub fn new(user_id: types::UserId, reason: String, duration: impl Into<Option<u32>>) -> Self {
        Self {
            duration: duration.into(),
            reason,
            user_id,
        }
    }
}

impl helix::HelixRequestBody for Vec<BanUsersBody> {
    fn try_to_body(&self) -> Result<Vec<u8>, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a Vec<BanUsersBody>,
        }

        serde_json::to_vec(&InnerBody { data: self }).map_err(Into::into)
    }
}

/// Return Values for [Ban Users](super::ban_users)
///
/// [`ban-users`](https://dev.twitch.tv/docs/api/reference#ban-users)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BanUsersResponse {
    /// List of sucessfully banned or timedout users and their [related](BanUser) information.
    pub banned: Vec<BanUser>,
    /// List of [user IDs](types::UserId) that raised an error.
    pub errors: Vec<BanUserError>,
}

/// Describes a user that was banned. See also [`get_banned_users::BannedUser`](super::BannedUser)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BanUser {
    /// The broadcaster whose chat room the user was banned from chatting in.
    pub broadcaster_id: types::UserId,
    /// The UTC date and time (in RFC3339 format) that the timeout will end. Is null if the user was banned instead of put in a timeout.
    pub end_time: Option<types::Timestamp>,
    /// The moderator that banned or put the user in the timeout.
    pub moderator_id: types::UserId,
    /// The user that was banned or was put in a timeout.
    pub user_id: types::UserId,
}

/// An error that occurred when banning or timing out a [user id](types::UserId) using [`BanUserRequest`].
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct BanUserError(String);

impl BanUserError {
    fn inner(&self) -> Option<(&types::UserIdRef, &str)> {
        self.as_str()
            .split_once(':')
            .map(|t| (t.0.into(), t.1.trim_start()))
    }

    /// Return the [`UserId`](types::UserId) of the user that raised this [error](self::error).
    pub fn user_id(&self) -> Option<&types::UserIdRef> { self.inner().map(|t| t.0) }

    /// Return the error that was raised this when doing a ban or timeout.
    pub fn error(&self) -> Option<&str> { self.inner().map(|t| t.1) }

    /// Get the error formatted as given by twitch: `9876: user is already banned`
    pub fn as_str(&self) -> &str { &self.0 }
}

impl Request for BanUsersRequest {
    type Response = BanUsersResponse;

    const PATH: &'static str = "moderation/bans";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModeratorManageBannedUsers];
}

impl RequestPost for BanUsersRequest {
    type Body = Vec<BanUsersBody>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug, Clone)]
        struct InnerResponse {
            data: Vec<BanUser>,
            errors: Vec<BanUserError>,
        }
        let InnerResponse { data, errors } = helix::parse_json(response, true).map_err(|e| {
            helix::HelixRequestPostError::DeserializeError(
                response.to_string(),
                e,
                uri.clone(),
                status,
            )
        })?;
        Ok(helix::Response {
            data: BanUsersResponse {
                banned: data,
                errors,
            },
            pagination: None,
            request,
            total: None,
            other: None,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = BanUsersRequest::builder()
        .broadcaster_id("1234")
        .moderator_id("5678")
        .build();

    let body = vec![BanUsersBody::new(
        "9876".into(),
        "no reason".to_string(),
        300,
    )];

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "broadcaster_id": "1234",
            "moderator_id": "5678",
            "user_id": "5432",
            "end_time": "2021-09-28T16:19:11Z"
          }
        ],
        "errors": [
          "9876: user is already banned"
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/bans?broadcaster_id=1234&moderator_id=5678"
    );

    dbg!(BanUsersRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

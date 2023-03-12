//! Bans a user from participating in a broadcaster’s chat room, or puts them in a timeout.
//! [`ban-user`](https://dev.twitch.tv/docs/api/reference#ban-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [BanUserRequest]
//!
//! To use this endpoint, construct a [`BanUserRequest`] with the [`BanUserRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::ban_user;
//! let request = ban_user::BanUserRequest::new("1234", "5678");
//! ```
//!
//! ## Body: [BanUserBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::moderation::ban_user;
//! let body = ban_user::BanUserBody::new("9876", "no reason", 120);
//! ```
//!
//! ## Response: [BanUser]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::ban_user};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = ban_user::BanUserRequest::new("1234", "5678");
//! let body = ban_user::BanUserBody::new("9876", "no reason", 120);
//! let response: ban_user::BanUser = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`BanUserRequest::parse_response(None, &request.get_uri(), response)`](BanUserRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Ban User](super::ban_user)
///
/// [`ban-user`](https://dev.twitch.tv/docs/api/reference#ban-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct BanUserRequest<'a> {
    /// The ID of the broadcaster whose chat room the user is being banned from.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room.
    /// This ID must match the user ID associated with the user OAuth token.
    ///
    /// If the broadcaster wants to ban the user (instead of having the moderator do it),
    /// set this parameter to the broadcaster’s ID, too.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> BanUserRequest<'a> {
    /// Ban a user on this channel
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        moderator_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            moderator_id: moderator_id.into_cow(),
        }
    }
}

/// Body Parameters for [Ban User](super::ban_user)
///
/// [`ban-user`](https://dev.twitch.tv/docs/api/reference#ban-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct BanUserBody<'a> {
    /// The ID of the user to ban or put in a timeout.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
    /// Duration of the (optional) timeout in seconds.
    ///
    /// To ban a user indefinitely, don’t include this field.
    ///
    /// To put a user in a timeout, include this field and specify the timeout period, in seconds.
    /// The minimum timeout is 1 second and the maximum is 1,209,600 seconds (2 weeks).
    /// To end a user’s timeout early, set this field to 1, or send an Unban user request.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// The reason the user is being banned or put in a timeout. The text is user defined and limited to a maximum of 500 characters.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub reason: Cow<'a, str>,
}

impl<'a> BanUserBody<'a> {
    /// Create a new [`BanUserBody`]
    pub fn new(
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        reason: impl Into<Cow<'a, str>>,
        duration: impl Into<Option<u32>>,
    ) -> Self {
        Self {
            user_id: user_id.into_cow(),
            reason: reason.into(),
            duration: duration.into(),
        }
    }
}

impl helix::HelixRequestBody for BanUserBody<'_> {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a BanUserBody<'a>,
        }
        serde_json::to_vec(&InnerBody { data: self })
            .map_err(Into::into)
            .map(Into::into)
    }
}

/// Return Values for [Ban User](super::ban_user)
///
/// [`ban-user`](https://dev.twitch.tv/docs/api/reference#ban-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BanUser {
    /// The broadcaster whose chat room the user was banned from chatting in.
    pub broadcaster_id: types::UserId,
    /// The UTC date and time (in RFC3999 format) when the ban was created.
    pub created_at: types::Timestamp,
    /// The UTC date and time (in RFC3339 format) that the timeout will end. Is null if the user was banned instead of put in a timeout.
    pub end_time: Option<types::Timestamp>,
    /// The moderator that banned or put the user in the timeout.
    pub moderator_id: types::UserId,
    /// The user that was banned or was put in a timeout.
    pub user_id: types::UserId,
}

impl Request for BanUserRequest<'_> {
    type Response = BanUser;

    const PATH: &'static str = "moderation/bans";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorManageBannedUsers];
}

impl<'a> RequestPost for BanUserRequest<'a> {
    type Body = BanUserBody<'a>;

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
        }
        let InnerResponse { data } = helix::parse_json(response, true).map_err(|e| {
            helix::HelixRequestPostError::DeserializeError(
                response.to_string(),
                e,
                uri.clone(),
                status,
            )
        })?;
        Ok(helix::Response {
            data: data.into_iter().next().ok_or_else(|| {
                helix::HelixRequestPostError::InvalidResponse {
                    reason: "missing response data",
                    response: response.to_string(),
                    status,
                    uri: uri.clone(),
                }
            })?,
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
    let req = BanUserRequest::new("1234", "5678");

    let body = BanUserBody::new("9876", "no reason", 300);

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"data":{"user_id":"9876","duration":300,"reason":"no reason"}}"#
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "broadcaster_id": "1234",
            "moderator_id": "5678",
            "user_id": "9876",
            "created_at": "2021-09-28T19:27:31Z",
            "end_time": "2021-09-28T19:22:31Z"
          }
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

    dbg!(BanUserRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_error() {
    use helix::*;
    let req = BanUserRequest::new("1234", "5678");

    let body = BanUserBody::new("9876", "no reason", 300);

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "error": "Bad Request",
        "status": 400,
        "message": "user is already banned"
    }
"#
    .to_vec();

    let http_response = http::Response::builder().status(400).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/bans?broadcaster_id=1234&moderator_id=5678"
    );

    dbg!(BanUserRequest::parse_response(Some(req), &uri, http_response).unwrap_err());
}

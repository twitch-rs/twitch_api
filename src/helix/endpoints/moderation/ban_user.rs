//! Bans a user from participating in a broadcaster’s chat room, or puts them in a timeout.
//! [`ban-user`](https://dev.twitch.tv/docs/api/reference#ban-user)
//!
//! # Accessing the endpoint
//!
//! ## Request: [BanUserRequest]
//!
//! To use this endpoint, construct a [`BanUserRequest`] with the [`BanUserRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api2::helix::moderation::ban_user;
//! let request = ban_user::BanUserRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! ```
//!
//! ## Body: [BanUserBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::moderation::ban_user;
//! let body = ban_user::BanUserBody::new("9876", "no reason".to_string(), 120);
//! ```
//!
//! ## Response: [BanUser]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::ban_user};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = ban_user::BanUserRequest::builder()
//!     .broadcaster_id("1234")
//!     .moderator_id("5678")
//!     .build();
//! let body = ban_user::BanUserBody::new("9876", "no reason".to_string(), 120);
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
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct BanUserRequest {
    /// The ID of the broadcaster whose chat room the user is being banned from.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// The ID of a user that has permission to moderate the broadcaster’s chat room. This ID must match the user ID associated with the user OAuth token.
    #[builder(setter(into))]
    pub moderator_id: types::UserId,
}

/// Body Parameters for [Ban User](super::ban_user)
///
/// [`ban-user`](https://dev.twitch.tv/docs/api/reference#ban-user)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct BanUserBody {
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

impl BanUserBody {
    /// Create a new [`BanUserBody`]
    pub fn new(
        user_id: impl Into<types::UserId>,
        reason: String,
        duration: impl Into<Option<u32>>,
    ) -> Self {
        Self {
            duration: duration.into(),
            reason,
            user_id: user_id.into(),
        }
    }
}

impl helix::HelixRequestBody for BanUserBody {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a Vec<&'a BanUserBody>,
        }
        let v = vec![self];
        serde_json::to_vec(&InnerBody { data: &v })
            .map_err(Into::into)
            .map(Into::into)
    }
}

/// Return Values for [Ban User](super::ban_user)
///
/// [`ban-user`](https://dev.twitch.tv/docs/api/reference#ban-user)
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

impl Request for BanUserRequest {
    type Response = BanUser;

    const PATH: &'static str = "moderation/bans";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ModeratorManageBannedUsers];
}

impl RequestPost for BanUserRequest {
    type Body = BanUserBody;

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
    let req = BanUserRequest::builder()
        .broadcaster_id("1234")
        .moderator_id("5678")
        .build();

    let body = BanUserBody::new("9876", "no reason".to_string(), 300);

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "broadcaster_id": "1234",
            "moderator_id": "5678",
            "user_id": "9876",
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
    let req = BanUserRequest::builder()
        .broadcaster_id("1234")
        .moderator_id("5678")
        .build();

    let body = BanUserBody::new("9876", "no reason".to_string(), 300);

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

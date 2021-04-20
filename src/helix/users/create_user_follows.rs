//! Adds a specified user to the followers of a specified channel.
//! [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreateUserFollowsRequest]
//!
//! To use this endpoint, construct a [`CreateUserFollowsRequest`] with the [`CreateUserFollowsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::users::create_user_follows;
//! let request = create_user_follows::CreateUserFollowsRequest::builder()
//!     .build();
//! ```
//!
//! ## Body: [CreateUserFollowsBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::users::create_user_follows;
//! let body = create_user_follows::CreateUserFollowsBody::builder()
//!     .from_id("1337")
//!     .to_id("7331")
//!     .build();
//! ```
//!
//! ## Response: [CreateUserFollows]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, users::create_user_follows};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = create_user_follows::CreateUserFollowsRequest::builder()
//!     .build();
//! let body = create_user_follows::CreateUserFollowsBody::builder()
//!     .from_id("1337")
//!     .to_id("7331")
//!     .build();
//! let response: create_user_follows::CreateUserFollows = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreateUserFollowsRequest::parse_response(None, &request.get_uri(), response)`](CreateUserFollowsRequest::parse_response)

use std::convert::TryInto;

use super::*;
use helix::RequestPost;
/// Query Parameters for [Create User Follows](super::create_user_follows)
///
/// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreateUserFollowsRequest {}

/// Body Parameters for [Create User Follows](super::create_user_follows)
///
/// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CreateUserFollowsBody {
    /// User ID of the follower
    #[builder(setter(into))]
    pub from_id: types::UserId,
    /// ID of the channel to be followed by the user
    #[builder(setter(into))]
    pub to_id: types::UserId,
    /// If `true`, the user gets email or push notifications (depending on the user’s notification settings) when the channel goes live. Default value is `false`.
    #[builder(default)]
    pub allow_notifications: Option<bool>,
}

impl helix::private::SealedSerialize for CreateUserFollowsBody {}

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
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(CreateUserFollows::Success),
            http::StatusCode::BAD_REQUEST => Ok(CreateUserFollows::MissingQuery),
            http::StatusCode::UNPROCESSABLE_ENTITY => Ok(CreateUserFollows::Success),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}

impl Request for CreateUserFollowsRequest {
    type Response = CreateUserFollows;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "users/follows";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditFollows];
}

impl RequestPost for CreateUserFollowsRequest {
    type Body = CreateUserFollowsBody;

    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<
        helix::Response<Self, <Self as helix::Request>::Response>,
        helix::HelixRequestPostError,
    >
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            helix::HelixRequestPostError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
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
            request,
        })
    }

    fn parse_inner_response(
        _: Option<Self>,
        _: &http::Uri,
        _: &str,
        _: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        unimplemented!("Create User Follows does not implement `parse_inner_response`")
    }
}

#[test]
fn test_request() {
    use helix::*;
    let req = CreateUserFollowsRequest::builder().build();

    let body = CreateUserFollowsBody::builder()
        .from_id("1234")
        .to_id("4321")
        .build();

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();
    // This is marked as 204 in twitch docs, but in reality it's 200

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/follows?"
    );

    dbg!(CreateUserFollowsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

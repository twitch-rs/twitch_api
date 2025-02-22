//! Updates the specified user’s information.
//!
//! [`update-user`](https://dev.twitch.tv/docs/api/reference/#update-user)
//!
//! The user ID in the OAuth token identifies the user whose information you want to update.
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateUserRequest]
//!
//! To use this endpoint, construct an [`UpdateUserRequest`] with the [`UpdateUserRequest::description()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::update_user;
//! let mut request =
//!     update_user::UpdateUserRequest::description("my description");
//! ```
//!
//! ## Response: [User]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::update_user};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let mut request = update_user::UpdateUserRequest::description("my description");
//! let body = helix::EmptyBody;
//! let response: helix::users::User = client.req_put(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`UpdateUserRequest::parse_response(None, &request.get_uri(), response)`](UpdateUserRequest::parse_response)
use super::*;
use helix::RequestPut;

/// Query Parameters for [Update User](super::update_user)
///
/// [`update-user`](https://dev.twitch.tv/docs/api/reference#update-user)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UpdateUserRequest<'a> {
    /// The string to update the channel’s description to. The description is limited to a maximum of 300 characters.
    ///
    /// To remove the description, specify this parameter but don’t set it’s value (specify an empty string).
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub description: Option<Cow<'a, str>>,
}

impl<'a> UpdateUserRequest<'a> {
    /// Update nothing (returns the current user)
    pub const fn empty() -> Self { Self { description: None } }

    /// Update the description of the current user
    pub fn description(description: impl Into<Cow<'a, str>>) -> Self {
        Self {
            description: Some(description.into()),
        }
    }
}

impl Request for UpdateUserRequest<'_> {
    type Response = User;

    const PATH: &'static str = "users";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserEdit];
}

impl RequestPut for UpdateUserRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPutError>
    where
        Self: Sized,
    {
        helix::parse_single_return(request, uri, response, status)
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    let req = UpdateUserRequest::description("my description");

    let body = helix::EmptyBody;

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs (slightly modified to include a space)
    let data = br#"
        {
          "data":[{
            "id": "44322889",
            "login": "dallas",
            "display_name": "dallas",
            "type": "staff",
            "broadcaster_type": "affiliate",
            "description": "my description",
            "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/4d1f36cbf1f0072d-profile_image-300x300.png",
            "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-channel_offline_image-2e82c1df2a464df7-1920x1080.jpeg",
            "view_count": 6995,
            "email": "not-real@email.com",
            "created_at": "2013-06-03T19:12:02.580593Z"
          }]
        }
    "#.to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users?description=my+description"
    );

    let res = UpdateUserRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;

    assert_eq!(res.id.as_str(), "44322889");
    assert_eq!(res.type_, Some(types::UserType::Staff));
    assert_eq!(
        res.broadcaster_type,
        Some(types::BroadcasterType::Affiliate)
    );
    assert_eq!(res.login.as_str(), "dallas");
    assert_eq!(res.description.unwrap(), "my description");
}

#[cfg(test)]
#[test]
fn test_request_empty() {
    let req = UpdateUserRequest::empty();

    let body = helix::EmptyBody;

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    let data = br#"
        {
          "data": [
            {
              "broadcaster_type": "",
              "created_at": "2016-07-14T16:13:40Z",
              "description": "hi",
              "display_name": "nerixyz",
              "id": "129546453",
              "login": "nerixyz",
              "offline_image_url": "",
              "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/e065218b-49df-459d-afd3-c6557870f551-profile_image-300x300.png",
              "type": "",
              "view_count": 0
            }
          ]
        }
    "#.to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/users?");

    let res = UpdateUserRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;

    assert_eq!(res.id.as_str(), "129546453");
    assert_eq!(res.type_, Some(types::UserType::None));
    assert_eq!(res.broadcaster_type, Some(types::BroadcasterType::None));
}

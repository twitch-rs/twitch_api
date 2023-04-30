//! Adds a moderator to the broadcaster’s moderation room.
//! [`add-channel-moderator`](https://dev.twitch.tv/docs/api/reference#add-channel-moderator)
//!
//! # Accessing the endpoint
//!
//! ## Request: [AddChannelModeratorRequest]
//!
//! To use this endpoint, construct a [`AddChannelModeratorRequest`] with the [`AddChannelModeratorRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::add_channel_moderator;
//! let request =
//!     add_channel_moderator::AddChannelModeratorRequest::new("1234", "5678");
//! ```
//!
//! ## Response: [AddChannelModeratorResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::add_channel_moderator};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = add_channel_moderator::AddChannelModeratorRequest::new("1234", "5678");
//! let response: helix::moderation::AddChannelModeratorResponse = client.req_post(request, helix::EmptyBody, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`AddChannelModeratorRequest::parse_response(None, &request.get_uri(), response)`](AddChannelModeratorRequest::parse_response)

use super::*;
use helix::RequestPost;

/// Query Parameters for [Add Channel Moderator](super::add_channel_moderator)
///
/// [`add-channel-moderator`](https://dev.twitch.tv/docs/api/reference#add-channel-moderator)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct AddChannelModeratorRequest<'a> {
    /// The ID of the broadcaster that owns the chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the user to add as a moderator in the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> AddChannelModeratorRequest<'a> {
    /// Add moderator on channel
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

/// Return Values for [Add Channel Moderator](super::add_channel_moderator)
///
/// [`add-channel-moderator`](https://dev.twitch.tv/docs/api/reference#add-channel-moderator)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum AddChannelModeratorResponse {
    /// Successfully added the moderator.
    Success,
}

impl Request for AddChannelModeratorRequest<'_> {
    type Response = AddChannelModeratorResponse;

    const PATH: &'static str = "moderation/moderators";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageModerators];
}

impl RequestPost for AddChannelModeratorRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response<'d>(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response {
                data: AddChannelModeratorResponse::Success,
                pagination: None,
                request,
                total: None,
                other: None,
            }),
            _ => Err(helix::HelixRequestPostError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = AddChannelModeratorRequest::new("1234", "5678");

    dbg!(req
        .create_request(helix::EmptyBody, "token", "clientid")
        .unwrap());

    // From twitch docs
    let data = b"".to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/moderators?broadcaster_id=1234&moderator_id=5678"
    );

    dbg!(AddChannelModeratorRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

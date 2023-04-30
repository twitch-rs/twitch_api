//! Adds a VIP to the broadcaster’s chat room.
//! [`add-channel-vip`](https://dev.twitch.tv/docs/api/reference#add-channel-vip)
//!
//! # Accessing the endpoint
//!
//! ## Request: [AddChannelVipRequest]
//!
//! To use this endpoint, construct a [`AddChannelVipRequest`] with the [`AddChannelVipRequest::new()`] or[`AddChannelVipRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::channels::add_channel_vip;
//! let request = add_channel_vip::AddChannelVipRequest::new("123", "456");
//! ```
//!
//! ## Response: [AddChannelVipResponse]
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, channels::add_channel_vip};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = add_channel_vip::AddChannelVipRequest::new("123", "456");
//! let response: add_channel_vip::AddChannelVipResponse = client.req_post(request, helix::EmptyBody, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(body, &token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`AddChannelVipRequest::parse_response(None, &request.get_uri(), response)`](AddChannelVipRequest::parse_response)
use super::*;
use helix::RequestPost;

/// Query Parameters for [Add Channel Vip](super::add_channel_vip)
///
/// [`add-channel-vip`](https://dev.twitch.tv/docs/api/reference#add-channel-vip)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct AddChannelVipRequest<'a> {
    /// The ID of the broadcaster that’s granting VIP status to the user.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the user to add as a VIP in the broadcaster’s chat room.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, types::UserIdRef>,
}

impl<'a> AddChannelVipRequest<'a> {
    /// Add a channel VIP
    pub fn new(
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            user_id: user_id.into_cow(),
        }
    }
}

/// Return Values for [Add Channel Vip](super::add_channel_vip)
///
/// [`add-channel-vip`](https://dev.twitch.tv/docs/api/reference#add-channel-vip)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum AddChannelVipResponse {
    /// Successfully added the VIP.
    Success,
}

impl Request for AddChannelVipRequest<'_> {
    type Response = AddChannelVipResponse;

    const PATH: &'static str = "channels/vips";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageVips];
}

impl RequestPost for AddChannelVipRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response {
                data: AddChannelVipResponse::Success,
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
    let req = AddChannelVipRequest::new("123", "456");

    dbg!(req
        .create_request(helix::EmptyBody, "token", "clientid")
        .unwrap());

    let data = vec![];

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/channels/vips?broadcaster_id=123&user_id=456"
    );

    dbg!(AddChannelVipRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

//! Activates or deactivates the broadcaster’s Shield Mode.
//!
//! [`update-shield-mode-status`](https://dev.twitch.tv/docs/api/reference#update-shield-mode-status)
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateShieldModeStatusRequest]
//!
//! To use this endpoint, construct an [`UpdateShieldModeStatusRequest`] with the [`UpdateShieldModeStatusRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::update_shield_mode_status;
//! let request = update_shield_mode_status::UpdateShieldModeStatusRequest::new(
//!     "123", "456",
//! );
//! ```
//!
//! //! ## Body: [UpdateShieldModeStatusBody]
//! We also need to provide a body to the request.
//!
//! ```
//! # use twitch_api::helix::moderation::update_shield_mode_status;
//! let body =
//!     update_shield_mode_status::UpdateShieldModeStatusBody::is_active(false);
//! ```
//!
//!
//! ## Response: [ShieldModeStatus]
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::update_shield_mode_status};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = update_shield_mode_status::UpdateShieldModeStatusRequest::new(
//!     "123",
//!     "456"
//! );
//! let body =
//!     update_shield_mode_status::UpdateShieldModeStatusBody::is_active(false);
//! let response: helix::moderation::ShieldModeStatus = client.req_put(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPut::create_request)
//! and parse the [`http::Response`] with [`UpdateShieldModeStatusRequest::parse_response(None, &request.get_uri(), response)`](UpdateShieldModeStatusRequest::parse_response)

use super::*;
use helix::RequestPut;

pub use super::ShieldModeStatus;

/// Query Parameters for [Update Shield Mode Status](super::update_shield_mode_status)
///
/// [`update-shield-mode-status`](https://dev.twitch.tv/docs/api/reference#update-shield-mode-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateShieldModeStatusRequest<'a> {
    /// The ID of the broadcaster whose Shield Mode you want to activate or deactivate.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that is one of the broadcaster’s moderators. This ID must match the user ID in the access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> UpdateShieldModeStatusRequest<'a> {
    /// Set the shield mode status on specified channel as the specified moderator
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

/// Body Parameters for [Update Shield Mode Status](super::update_shield_mode_status)
///
/// [`update-shield-mode-status`](https://dev.twitch.tv/docs/api/reference#update-shield-mode-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateShieldModeStatusBody<'a> {
    /// A Boolean value that determines whether to activate Shield Mode.
    pub is_active: bool,
    #[serde(skip)]
    _ph: std::marker::PhantomData<&'a ()>,
}

impl helix::private::SealedSerialize for UpdateShieldModeStatusBody<'_> {}

impl<'a> UpdateShieldModeStatusBody<'a> {
    /// Set status of shield mode
    pub fn is_active(is_active: bool) -> Self {
        Self {
            is_active,
            _ph: std::marker::PhantomData,
        }
    }
}

impl Request for UpdateShieldModeStatusRequest<'_> {
    type Response = super::ShieldModeStatus;

    const PATH: &'static str = "moderation/shield_mode";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageShieldMode];
}

impl<'a> RequestPut for UpdateShieldModeStatusRequest<'a> {
    type Body = UpdateShieldModeStatusBody<'a>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestPutError>
    where
        Self: Sized,
    {
        let inner_response: helix::InnerResponse<Vec<_>> = crate::parse_json(response, true)
            .map_err(|e| {
                helix::HelixRequestPutError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response::new(
            inner_response.data.into_iter().next().ok_or(
                helix::HelixRequestPutError::InvalidResponse {
                    reason: "expected an entry in `data`",
                    response: response.to_string(),
                    status,
                    uri: uri.clone(),
                },
            )?,
            inner_response.pagination.cursor,
            request,
            inner_response.total,
            inner_response.other,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = UpdateShieldModeStatusRequest::new("12345", "98765");
    let body = UpdateShieldModeStatusBody::is_active(false);

    assert_eq!(
        std::str::from_utf8(&body.try_to_body().unwrap()).unwrap(),
        r#"{"is_active":false}"#
    );

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "is_active": false,
            "moderator_id": "98765",
            "moderator_name": "SimplySimple",
            "moderator_login": "simplysimple",
            "last_activated_at": "2022-07-26T17:16:03.123Z"
          }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/shield_mode?broadcaster_id=12345&moderator_id=98765"
    );

    dbg!(UpdateShieldModeStatusRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

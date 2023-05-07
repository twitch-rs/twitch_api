//! Gets the broadcaster’s Shield Mode activation status.
//! [`get-shield-mode-status`](https://dev.twitch.tv/docs/api/reference#get-shield-mode-status)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetShieldModeStatusRequest]
//!
//! To use this endpoint, construct a [`GetShieldModeStatusRequest`] with the [`GetShieldModeStatusRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::get_shield_mode_status;
//! let request =
//!     get_shield_mode_status::GetShieldModeStatusRequest::new("1234", "5678");
//! ```
//!
//! ## Response: [ShieldModeStatus]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::get_shield_mode_status};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_shield_mode_status::GetShieldModeStatusRequest::new("1234", "5678");
//! let response: helix::moderation::ShieldModeStatus = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetShieldModeStatusRequest::parse_response(None, &request.get_uri(), response)`](GetShieldModeStatusRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Shield Mode Status](super::get_shield_mode_status)
///
/// [`get-shield-mode-status`](https://dev.twitch.tv/docs/api/reference#get-shield-mode-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetShieldModeStatusRequest<'a> {
    /// The ID of the broadcaster whose Shield Mode activation status you want to get.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster or a user that is one of the broadcaster’s moderators. This ID must match the user ID in the access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub moderator_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetShieldModeStatusRequest<'a> {
    /// Get shield mode status in a broadcasters channel as specified moderator
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

/// Return Values for [Get Shield Mode Status](super::get_shield_mode_status)
///
/// [`get-shield-mode-status`](https://dev.twitch.tv/docs/api/reference#get-shield-mode-status)
#[derive(PartialEq, Eq, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ShieldModeStatus {
    /// A Boolean value that determines whether Shield Mode is active. Is true if the broadcaster activated Shield Mode; otherwise, false.
    pub is_active: bool,
    #[serde(flatten)]
    /// Information about the last activated shield mode
    pub last_shield_mode: Option<LastShieldMode>,
}

impl<'de> Deserialize<'de> for ShieldModeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct InnerShieldModeStatus {
            is_active: bool,
            #[serde(deserialize_with = "helix::deserialize_none_from_empty_string")]
            moderator_id: Option<types::UserId>,
            #[serde(deserialize_with = "helix::deserialize_none_from_empty_string")]
            moderator_login: Option<types::UserName>,
            #[serde(deserialize_with = "helix::deserialize_none_from_empty_string")]
            moderator_name: Option<types::DisplayName>,
            #[serde(deserialize_with = "helix::deserialize_none_from_empty_string")]
            last_activated_at: Option<types::Timestamp>,
        }

        let s = InnerShieldModeStatus::deserialize(deserializer)?;
        let last = match (
            s.moderator_id,
            s.moderator_login,
            s.moderator_name,
            s.last_activated_at,
        ) {
            (
                Some(moderator_id),
                Some(moderator_login),
                Some(moderator_name),
                Some(last_activated_at),
            ) => Some(LastShieldMode {
                moderator_id,
                moderator_login,
                moderator_name,
                last_activated_at,
            }),
            _ => None,
        };
        Ok(ShieldModeStatus {
            is_active: s.is_active,
            last_shield_mode: last,
        })
    }
}

/// Information about a shield mode
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct LastShieldMode {
    /// An ID that identifies the moderator that last activated Shield Mode.
    pub moderator_id: types::UserId,
    /// The moderator’s login name.
    pub moderator_login: types::UserName,
    /// The moderator’s display name.
    pub moderator_name: types::DisplayName,
    /// The UTC timestamp (in RFC3339 format) of when Shield Mode was last activated.
    pub last_activated_at: types::Timestamp,
}

impl LastShieldMode {}
impl Request for GetShieldModeStatusRequest<'_> {
    type Response = ShieldModeStatus;

    const PATH: &'static str = "moderation/shield_mode";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModeratorReadShieldMode];
}

impl RequestGet for GetShieldModeStatusRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        let inner_response: helix::InnerResponse<Vec<_>> = crate::parse_json(response, true)
            .map_err(|e| {
                helix::HelixRequestGetError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response::new(
            inner_response.data.into_iter().next().ok_or(
                helix::HelixRequestGetError::InvalidResponse {
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
    let req = GetShieldModeStatusRequest::new("12345", "98765");

    // From twitch docs, FIXME: has ... and a "bad" comma
    let data = br#"
    {
        "data": [
          {
            "is_active": true,
            "moderator_id": "98765",
            "moderator_name": "SimplySimple",
            "moderator_login": "simplysimple",
            "last_activated_at": "2022-07-26T17:16:03.123Z"
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/shield_mode?broadcaster_id=12345&moderator_id=98765"
    );

    dbg!(GetShieldModeStatusRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_empty() {
    use helix::*;
    let req = GetShieldModeStatusRequest::new("12345", "98765");

    // From twitch docs, FIXME: has ... and a "bad" comma
    let data = br#"
    {
        "data": [
          {
            "is_active": false,
            "moderator_id": "",
            "moderator_name": "",
            "moderator_login": "",
            "last_activated_at": ""
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/shield_mode?broadcaster_id=12345&moderator_id=98765"
    );
    let status =
        GetShieldModeStatusRequest::parse_response(Some(req), &uri, http_response).unwrap();
    dbg!(&status);
    assert!(status.data.last_shield_mode.is_none());
}

//! Gets a list of all extensions (both active and inactive) that the broadcaster has installed.
//! [`get-user-extensions`](https://dev.twitch.tv/docs/api/reference#get-user-extensions)
//!
//! The user ID in the access token identifies the broadcaster.
//!
//! ## Request: [GetUserExtensionsRequest]
//!
//! To use this endpoint, construct a [`GetUserExtensionsRequest`] with the [`GetUserExtensionsRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::get_user_extensions;
//! let request = get_user_extensions::GetUserExtensionsRequest::new();
//! ```
//!
//! ## Response: [Extension]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::get_user_extensions};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_user_extensions::GetUserExtensionsRequest::new();
//! let response: Vec<get_user_extensions::Extension> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUserExtensionsRequest::parse_response(None, &request.get_uri(), response)`](GetUserExtensionsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get User Extensions](super::get_user_extensions)
///
/// [`get-user-extensions`](https://dev.twitch.tv/docs/api/reference#get-user-extensions)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetUserExtensionsRequest {}

impl GetUserExtensionsRequest {
    /// Get a list of all extensions (both active and inactive) that the broadcaster has installed.
    pub fn new() -> Self { Self::default() }
}

/// Return Values for [Get User Extensions](super::get_user_extensions)
///
/// [`get-user-extensions`](https://dev.twitch.tv/docs/api/reference#get-user-extensions)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Extension {
    /// An ID that identifies the extension.
    pub id: types::ExtensionId,
    /// The extension's version.
    pub version: String,
    /// The extension's name.
    pub name: String,
    /// A Boolean value that determines whether the extension is configured and can be activated.
    ///
    /// Is true if the extension is configured and can be activated.
    pub can_activate: bool,
    /// The extension types that you can activate for this extension.
    #[serde(rename = "type")]
    pub type_: Vec<ExtensionType>,
}

/// Where an extension can appear.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ExtensionType {
    /// Displays as part of the video, taking up part of the screen. Component Extensions can be hidden by viewers.
    Component,
    /// Displays on mobile
    Mobile,
    /// Displays on top of the whole video as a transparent overlay.
    Overlay,
    /// Displays in a box under the video.
    Panel,
    /// An unknown type, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

impl Request for GetUserExtensionsRequest {
    type Response = Vec<Extension>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::UserReadBlockedUsers];
    const PATH: &'static str = "users/extensions/list";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::UserReadBroadcast,
        twitch_oauth2::Scope::UserEditBroadcast
    )];
}

impl RequestGet for GetUserExtensionsRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetUserExtensionsRequest::new();

    let data = br#"
    {
        "data": [
            {
                "id": "wi08ebtatdc7oj83wtl9uxwz807l8b",
                "version": "1.1.8",
                "name": "Streamlabs Leaderboard",
                "can_activate": true,
                "type": [
                    "panel"
                ]
            },
            {
                "id": "d4uvtfdr04uq6raoenvj7m86gdk16v",
                "version": "2.0.2",
                "name": "Prime Subscription and Loot Reminder",
                "can_activate": true,
                "type": [
                    "overlay"
                ]
            },
            {
                "id": "rh6jq1q334hqc2rr1qlzqbvwlfl3x0",
                "version": "1.1.0",
                "name": "TopClip",
                "can_activate": true,
                "type": [
                    "mobile",
                    "panel"
                ]
            },
            {
                "id": "zfh2irvx2jb4s60f02jq0ajm8vwgka",
                "version": "1.0.19",
                "name": "Streamlabs",
                "can_activate": true,
                "type": [
                    "mobile",
                    "overlay"
                ]
            },
            {
                "id": "lqnf3zxk0rv0g7gq92mtmnirjz2cjj",
                "version": "0.0.1",
                "name": "Dev Experience Test",
                "can_activate": true,
                "type": [
                    "component",
                    "mobile",
                    "panel",
                    "overlay"
                ]
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/extensions/list?"
    );

    let res = GetUserExtensionsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.len(), 5);
    assert_eq!(res[4].id.as_str(), "lqnf3zxk0rv0g7gq92mtmnirjz2cjj");
    assert_eq!(res[4].version, "0.0.1");
    assert!(res[4].can_activate);
    assert_eq!(res[4].type_.len(), 4);
    assert_eq!(res[4].type_[0], ExtensionType::Component);
    assert_eq!(res[4].type_[1], ExtensionType::Mobile);
    assert_eq!(res[4].type_[2], ExtensionType::Panel);
    assert_eq!(res[4].type_[3], ExtensionType::Overlay);
}

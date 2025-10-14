//! Gets the active extensions that the broadcaster has installed for each configuration.
//! [`get-user-active-extensions`](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions)
//!
//! ## Request: [GetUserActiveExtensionsRequest]
//!
//! To use this endpoint, construct a [`GetUserActiveExtensionsRequest`] with the [`GetUserActiveExtensionsRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::get_user_active_extensions;
//! let request =
//!     get_user_active_extensions::GetUserActiveExtensionsRequest::new();
//! ```
//!
//! ## Response: [ExtensionConfiguration]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::get_user_active_extensions};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_user_active_extensions::GetUserActiveExtensionsRequest::new();
//! let response: get_user_active_extensions::ExtensionConfiguration = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUserActiveExtensionsRequest::parse_response(None, &request.get_uri(), response)`](GetUserActiveExtensionsRequest::parse_response)

use std::collections::HashMap;

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get User Active Extensions](super::get_user_active_extensions)
///
/// [`get-user-active-extensions`](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetUserActiveExtensionsRequest<'a> {
    /// The ID of the broadcaster whose active extensions you want to get.
    ///
    /// This parameter is required if you specify an app access token and is optional if you specify a user access token. If you specify a user access token and don’t specify this parameter, the API uses the user ID from the access token.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Option<Cow<'a, types::UserIdRef>>,
}

impl<'a> GetUserActiveExtensionsRequest<'a> {
    /// Gets the active extensions that the broadcaster has installed for each configuration.
    ///
    /// Requires a user access token.
    pub fn new() -> Self { Self::default() }

    /// Gets the active extensions that the user has installed for each configuration.
    ///
    /// Requires an app access token.
    pub fn user_id(user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            user_id: Some(user_id.into_cow()),
        }
    }
}

/// Return Values for [Get User Active Extensions](super::get_user_active_extensions)
///
/// [`get-user-active-extensions`](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ExtensionConfiguration {
    /// A dictionary that contains the data for a panel extension.
    ///
    /// The dictionary’s key is a sequential number beginning with 1.
    pub panel: HashMap<String, ExtensionSlot<ActiveExtension>>,
    /// A dictionary that contains the data for a video-overlay extension.
    ///
    /// The dictionary’s key is a sequential number beginning with 1.
    pub overlay: HashMap<String, ExtensionSlot<ActiveExtension>>,
    /// A dictionary that contains the data for a video-component extension.
    ///
    /// The dictionary’s key is a sequential number beginning with 1.
    pub component: HashMap<String, ExtensionSlot<ActivePositionedExtension>>,
}

/// An active extension slot
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ActiveExtension {
    /// An ID that identifies the extension.
    pub id: types::ExtensionId,
    /// The extension’s version.
    pub version: String,
    /// The extension’s name.
    pub name: String,
}

/// An active extension slot where the extension can be positioned
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ActivePositionedExtension {
    /// An ID that identifies the extension.
    pub id: types::ExtensionId,
    /// The extension’s version.
    pub version: String,
    /// The extension’s name.
    pub name: String,
    /// The x-coordinate where the extension is placed.
    pub x: i32,
    /// The y-coordinate where the extension is placed.
    pub y: i32,
}

impl Request for GetUserActiveExtensionsRequest<'_> {
    type PaginationData = ();
    type Response = ExtensionConfiguration;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::UserReadBlockedUsers];
    const PATH: &'static str = "users/extensions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::UserReadBroadcast,
        twitch_oauth2::Scope::UserEditBroadcast
    )];
}

impl RequestGet for GetUserActiveExtensionsRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetUserActiveExtensionsRequest::new();

    let data = br#"
    {
        "data": {
            "panel": {
                "1": {
                    "active": true,
                    "id": "rh6jq1q334hqc2rr1qlzqbvwlfl3x0",
                    "version": "1.1.0",
                    "name": "TopClip"
                },
                "2": {
                    "active": true,
                    "id": "wi08ebtatdc7oj83wtl9uxwz807l8b",
                    "version": "1.1.8",
                    "name": "Streamlabs Leaderboard"
                },
                "3": {
                    "active": true,
                    "id": "naty2zwfp7vecaivuve8ef1hohh6bo",
                    "version": "1.0.9",
                    "name": "Streamlabs Stream Schedule & Countdown"
                }
            },
            "overlay": {
                "1": {
                    "active": true,
                    "id": "zfh2irvx2jb4s60f02jq0ajm8vwgka",
                    "version": "1.0.19",
                    "name": "Streamlabs"
                }
            },
            "component": {
                "1": {
                    "active": true,
                    "id": "lqnf3zxk0rv0g7gq92mtmnirjz2cjj",
                    "version": "0.0.1",
                    "name": "Dev Experience Test",
                    "x": 0,
                    "y": 0
                },
                "2": {
                    "active": false
                }
            }
        }
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/extensions?"
    );

    let res = GetUserActiveExtensionsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.panel.len(), 3);
    assert_eq!(res.overlay.len(), 1);
    assert_eq!(res.component.len(), 2);

    assert_eq!(
        *res.overlay.get("1").unwrap(),
        ExtensionSlot::Active(ActiveExtension {
            id: "zfh2irvx2jb4s60f02jq0ajm8vwgka".into(),
            version: "1.0.19".to_owned(),
            name: "Streamlabs".to_owned(),
        })
    );
    assert_eq!(
        *res.component.get("1").unwrap(),
        ExtensionSlot::Active(ActivePositionedExtension {
            id: "lqnf3zxk0rv0g7gq92mtmnirjz2cjj".into(),
            version: "0.0.1".to_owned(),
            name: "Dev Experience Test".to_owned(),
            x: 0,
            y: 0,
        })
    );
    assert_eq!(*res.component.get("2").unwrap(), ExtensionSlot::Inactive);

    assert_eq!(
        res,
        serde_json::from_str(&serde_json::to_string(&res).unwrap()).unwrap()
    );
}

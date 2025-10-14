//! Updates the specified user’s information.
//!
//! [`update-user-extensions`](https://dev.twitch.tv/docs/api/reference/#update-user-extensions)
//!
//! The user ID in the OAuth token identifies the user whose information you want to update.
//!
//! # Accessing the endpoint
//!
//! ## Request: [UpdateUserExtensionsRequest]
//!
//! To use this endpoint, construct an [`UpdateUserExtensionsRequest`] with the [`UpdateUserExtensionsRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::users::update_user_extensions;
//! let mut request =
//!     update_user_extensions::UpdateUserExtensionsRequest::new();
//! ```
//!
//! ## Body: [UpdateUserExtensionsBody]
//!
//! We also need to provide a body to the request.
//!
//! ```
//! # use twitch_api::helix::users::{self, update_user_extensions};
//! # use std::collections::HashMap;
//! # use std::iter::FromIterator;
//! # use std::borrow::Cow;
//! let body = update_user_extensions::UpdateUserExtensionsBody::new(
//!     update_user_extensions::ExtensionSpecification::new().panel(
//!         HashMap::from_iter([(
//!             Cow::Borrowed("1"),
//!             users::ExtensionSlot::Inactive,
//!         )]),
//!     ),
//! );
//! ```
//!
//! ## Response: [ExtensionConfiguration]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_patch()`](helix::HelixClient::req_patch).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::update_user_extensions};
//! # use twitch_api::client;
//! # use std::{iter::FromIterator, borrow::Cow, collections::HashMap};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let mut request = update_user_extensions::UpdateUserExtensionsRequest::new();
//! let body = update_user_extensions::UpdateUserExtensionsBody::new(
//!     update_user_extensions::ExtensionSpecification::new()
//!         .panel(HashMap::from_iter([(Cow::Borrowed("1"), helix::users::ExtensionSlot::Inactive)]))
//! );
//! let response: helix::users::ExtensionConfiguration = client.req_put(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`UpdateUserExtensionsRequest::parse_response(None, &request.get_uri(), response)`](UpdateUserExtensionsRequest::parse_response)
use std::collections::HashMap;

use super::*;
use helix::RequestPut;

/// Query Parameters for [Update User Extensions](super::update_user_extensions)
///
/// [`update-user-extensions`](https://dev.twitch.tv/docs/api/reference#update-user-extensions)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateUserExtensionsRequest<'a> {
    #[serde(skip)]
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl UpdateUserExtensionsRequest<'_> {
    /// Create a new update request
    pub fn new() -> Self { Self::default() }
}

impl Request for UpdateUserExtensionsRequest<'_> {
    type PaginationData = ();
    type Response = ExtensionConfiguration;

    const PATH: &'static str = "users/extensions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserEditBroadcast];
}

/// Body for [Update User Extensions](super::update_user_extensions)
///
/// [`update-user-extensions`](https://dev.twitch.tv/docs/api/reference#update-user-extensions)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateUserExtensionsBody<'a> {
    /// The specification for the user's extensions (which extensions to update)
    pub data: ExtensionSpecification<'a>,
}

impl helix::private::SealedSerialize for UpdateUserExtensionsBody<'_> {}

impl<'a> UpdateUserExtensionsBody<'a> {
    /// Create a new specificaton for the user's extensions
    pub const fn new(data: ExtensionSpecification<'a>) -> Self { Self { data } }
}

/// Inner body for [Update User Extensions](super::update_user_extensions)
///
/// [`update-user-extensions`](https://dev.twitch.tv/docs/api/reference#update-user-extensions)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ExtensionSpecification<'a> {
    /// A dictionary that contains the data for a panel extension.
    ///
    /// The dictionary’s key is a sequential number beginning with 1.
    pub panel: Option<HashMap<Cow<'a, str>, ExtensionSlot<ActiveExtension<'a>>>>,
    /// A dictionary that contains the data for a video-overlay extension.
    ///
    /// The dictionary’s key is a sequential number beginning with 1.
    pub overlay: Option<HashMap<Cow<'a, str>, ExtensionSlot<ActiveExtension<'a>>>>,
    /// A dictionary that contains the data for a video-component extension.
    ///
    /// The dictionary’s key is a sequential number beginning with 1.
    pub component: Option<HashMap<Cow<'a, str>, ExtensionSlot<ActivePositionedExtension<'a>>>>,
}

impl<'a> ExtensionSpecification<'a> {
    /// Create an empty specification
    pub fn new() -> Self { Self::default() }

    /// Set the panel extensions
    pub fn panel(
        mut self,
        panel: HashMap<Cow<'a, str>, ExtensionSlot<ActiveExtension<'a>>>,
    ) -> Self {
        self.panel = Some(panel);
        self
    }

    /// Set the overlay extensions
    pub fn overlay(
        mut self,
        overlay: HashMap<Cow<'a, str>, ExtensionSlot<ActiveExtension<'a>>>,
    ) -> Self {
        self.overlay = Some(overlay);
        self
    }

    /// Set the component extensions
    pub fn component(
        mut self,
        component: HashMap<Cow<'a, str>, ExtensionSlot<ActivePositionedExtension<'a>>>,
    ) -> Self {
        self.component = Some(component);
        self
    }
}

/// An active extension slot
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ActiveExtension<'a> {
    /// An ID that identifies the extension.
    pub id: Cow<'a, types::ExtensionIdRef>,
    /// The extension’s version.
    pub version: Cow<'a, str>,
}

impl<'a> ActiveExtension<'a> {
    /// Create an active extension with an ID and a version
    pub fn new(
        id: impl types::IntoCow<'a, types::ExtensionIdRef> + 'a,
        version: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            id: id.into_cow(),
            version: version.into(),
        }
    }
}

/// An active extension slot where the extension can be positioned
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct ActivePositionedExtension<'a> {
    /// An ID that identifies the extension.
    pub id: Cow<'a, types::ExtensionIdRef>,
    /// The extension’s version.
    pub version: Cow<'a, str>,
    /// The x-coordinate where the extension is placed.
    pub x: i32,
    /// The y-coordinate where the extension is placed.
    pub y: i32,
}

impl<'a> ActivePositionedExtension<'a> {
    /// Create an active positioned extension with an ID, a version, and a position
    pub fn new(
        id: impl types::IntoCow<'a, types::ExtensionIdRef> + 'a,
        version: impl Into<Cow<'a, str>>,
        x: i32,
        y: i32,
    ) -> Self {
        Self {
            id: id.into_cow(),
            version: version.into(),
            x,
            y,
        }
    }
}

impl<'a> RequestPut for UpdateUserExtensionsRequest<'a> {
    type Body = UpdateUserExtensionsBody<'a>;

    fn parse_inner_response(
        _request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestPutError>
    where
        Self: Sized,
    {
        let inner_response: helix::InnerResponse<<Self as Request>::Response> =
            crate::parse_json(response, true).map_err(|e| {
                helix::HelixRequestPutError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response::new(
            inner_response.data,
            (),
            inner_response.other,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    use std::iter::FromIterator;

    let req = UpdateUserExtensionsRequest::new();

    let spec = ExtensionSpecification::new()
        .panel(HashMap::from_iter([
            (
                Cow::Borrowed("1"),
                ExtensionSlot::Active(ActiveExtension::new(
                    "rh6jq1q334hqc2rr1qlzqbvwlfl3x0",
                    "1.1.0",
                )),
            ),
            (
                Cow::Borrowed("2"),
                ExtensionSlot::Active(ActiveExtension::new(
                    "wi08ebtatdc7oj83wtl9uxwz807l8b",
                    "1.1.8",
                )),
            ),
            (
                Cow::Borrowed("3"),
                ExtensionSlot::Active(ActiveExtension::new(
                    "naty2zwfp7vecaivuve8ef1hohh6bo",
                    "1.0.9",
                )),
            ),
        ]))
        .overlay(HashMap::from_iter([(
            Cow::Borrowed("1"),
            ExtensionSlot::Active(ActiveExtension::new(
                "zfh2irvx2jb4s60f02jq0ajm8vwgka",
                "1.0.19",
            )),
        )]))
        .component(HashMap::from_iter([
            (
                Cow::Borrowed("1"),
                ExtensionSlot::Active(ActivePositionedExtension::new(
                    "lqnf3zxk0rv0g7gq92mtmnirjz2cjj",
                    "0.0.1",
                    0,
                    0,
                )),
            ),
            (Cow::Borrowed("2"), ExtensionSlot::Inactive),
        ]));
    let body = UpdateUserExtensionsBody::new(spec);

    assert_eq!(
        body,
        serde_json::from_str(&serde_json::to_string(&body).unwrap()).unwrap()
    );

    // XXX: can't test the serialized body as the order of HashMap is unspecified

    dbg!(req.create_request(body, "token", "clientid").unwrap());

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

    let res = UpdateUserExtensionsRequest::parse_response(Some(req), &uri, http_response)
        .unwrap()
        .data;
    assert_eq!(res.panel.len(), 3);
    assert_eq!(res.overlay.len(), 1);
    assert_eq!(res.component.len(), 2);
    assert_eq!(*res.component.get("2").unwrap(), ExtensionSlot::Inactive);
}

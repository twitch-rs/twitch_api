//! Helix endpoints or the [New Twitch API](https://dev.twitch.tv/docs/api)
//!
//!
//! Aside from using [`HelixClient`] as described on [the crate documentation](crate),
//! you can decide to use this library without any specific client implementation.
//!
//! ```rust
//! use twitch_api2::helix::{self, Request, RequestGet, users::{GetUsersRequest, User}};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//!
//! let request = GetUsersRequest::builder()
//!     .login(vec!["justintv123".into()])
//!     .build();
//!
//! // Send it however you want
//! // Create a [`http::Response<hyper::body::Bytes>`] with RequestGet::create_request, which takes an access token and a client_id
//! let response = send_http_request(request.create_request("accesstoken", "client_id")?)?;
//!
//! // then parse the response
//! let uri = request.get_uri()?;
//! let user: helix::Response<_, Vec<User>> = GetUsersRequest::parse_response(Some(request), &uri, response)?;
//! println!("{:#?}", user);
//! # Ok(())
//! # }
//! # fn send_http_request(_: http::Request<hyper::body::Bytes>) -> Result<http::Response<hyper::body::Bytes>,&'static str> {
//! # Ok(http::Response::builder().body(r#"{"data":[{"id":"141981764","login":"twitchdev","display_name":"TwitchDev","type":"","broadcaster_type":"partner","description":"Supportingthird-partydevelopersbuildingTwitchintegrationsfromchatbotstogameintegrations.","profile_image_url":"https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png","offline_image_url":"https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png","view_count":5980557,"email":"not-real@email.com","created_at":"2016-12-14T20:32:28.894263Z"}]}"#.as_bytes().to_owned().into()).unwrap())
//! # }
//! ```

use serde::Deserialize;
#[doc(no_inline)]
#[cfg(feature = "twitch_oauth2")]
pub use twitch_oauth2::Scope;
#[cfg(feature = "twitch_oauth2")]
use twitch_oauth2::TwitchToken;

#[cfg(feature = "client")]
pub mod client;
mod endpoints;
pub mod request;
pub mod response;

#[cfg(feature = "client")]
#[doc(inline)]
pub use client::{client_ext::make_stream, *};
pub use endpoints::*;
#[cfg(feature = "client")]
#[doc(inline)]
pub use request::errors::ClientRequestError;
#[doc(inline)]
pub use request::errors::{
    CreateRequestError, HelixRequestDeleteError, HelixRequestGetError, HelixRequestPatchError,
    HelixRequestPostError, HelixRequestPutError, InvalidUri, SerializeError,
};
#[doc(inline)]
pub use request::{Request, RequestDelete, RequestGet, RequestPatch, RequestPost, RequestPut};
#[doc(inline)]
pub use response::Response;

pub(crate) mod ser;
pub(crate) use crate::deserialize_default_from_null;
use crate::{parse_json, parse_json_value};

#[derive(PartialEq, Deserialize, Debug)]
struct InnerResponse<D> {
    data: D,
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    #[serde(default)]
    pagination: Pagination,
    #[serde(default)]
    total: Option<i64>,
    #[serde(default, flatten)]
    other: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Deserialize, Debug)]
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
struct CustomInnerResponse<'a> {
    #[serde(borrow)]
    data: &'a serde_json::value::RawValue,
    #[serde(default)]
    pagination: Pagination,
    #[serde(default)]
    total: Option<i64>,
    // FIXME: There is an issue with RawValue on flatten maps. https://github.com/serde-rs/json/issues/599
    #[serde(flatten, default)]
    other: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize, Clone, Debug)]
struct HelixRequestError {
    error: String,
    status: u16,
    message: String,
}

/// Deserialize "" as <T as Default>::Default
fn deserialize_none_from_empty_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: serde::de::DeserializeOwned, {
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::String(string) if string.is_empty() => Ok(None),
        other => Ok(parse_json_value(other, true).map_err(serde::de::Error::custom)?),
    }
}

/// A request that can be paginated.
pub trait Paginated: Request {
    /// Should returns the current pagination cursor.
    ///
    /// # Notes
    ///
    /// Pass [`Option::None`] if no cursor is found.
    fn set_pagination(&mut self, cursor: Option<Cursor>);
}

/// A cursor for pagination. This is needed because of how pagination is represented in the [New Twitch API](https://dev.twitch.tv/docs/api)
#[derive(PartialEq, Deserialize, Debug, Clone, Default)]
struct Pagination {
    #[serde(default)]
    cursor: Option<Cursor>,
}

/// A cursor is a pointer to the current "page" in the twitch api pagination
#[aliri_braid::braid(serde)]
pub struct Cursor;

/// Errors that can happen when creating a body
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum BodyError {
    /// could not serialize as json
    JsonError(#[from] serde_json::Error),
    /// could not serialize to query
    QuerySerializeError(#[from] ser::Error),
    /// uri is invalid
    InvalidUri(#[from] InvalidUri),
}

/// Create a body. Used for specializing request bodies
pub trait HelixRequestBody {
    /// Create the body
    fn try_to_body(&self) -> Result<hyper::body::Bytes, BodyError>;
}

/// An empty body.
///
/// Implements [`HelixRequestBody::try_to_body`], returning an empty vector
#[derive(Default, Clone, Copy)]
pub struct EmptyBody;

impl HelixRequestBody for EmptyBody {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, BodyError> { Ok(<_>::default()) }
}

// TODO: I would want specialization for this. For now, to override this behavior for a body, we specify a sealed trait
impl<T> HelixRequestBody for T
where T: serde::Serialize + private::SealedSerialize
{
    fn try_to_body(&self) -> Result<hyper::body::Bytes, BodyError> {
        serde_json::to_vec(&self)
            .map_err(Into::into)
            .map(Into::into)
    }
}

pub(crate) mod private {
    pub trait SealedSerialize {}
}

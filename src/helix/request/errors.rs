//! Errors that can occur during request processing.
pub use super::ser::Error as SerializeError;
use crate::helix::BodyError;

#[cfg(feature = "client")]
#[cfg_attr(nightly, doc(cfg(feature = "client")))]
/// Errors for [`HelixClient::req_get`](super::super::HelixClient::req_get) and similar functions.
#[derive(thiserror::Error, Debug)]
// #[derive(displaydoc::Display)] https://github.com/yaahc/displaydoc/issues/15
#[non_exhaustive]
pub enum ClientRequestError<RE: std::error::Error + Send + Sync + 'static> {
    /// Request failed from reqwests side
    #[error("request failed")]
    RequestError(#[source] RE),
    /// Request failed from reqwests side
    #[error("body conversion failed")]
    HyperError(#[from] hyper::Error),
    /// No pagination found
    #[error("no pagination found")]
    NoPage,
    /// Could not create request
    #[error("could not create request")]
    CreateRequestError(#[from] CreateRequestError),
    /// Got error from GET response
    #[error(transparent)]
    HelixRequestGetError(#[from] HelixRequestGetError),
    /// Got error from PUT response
    #[error(transparent)]
    HelixRequestPutError(#[from] HelixRequestPutError),
    /// Got error from POST response
    #[error(transparent)]
    HelixRequestPostError(#[from] HelixRequestPostError),
    /// Got error from PATCH response
    #[error(transparent)]
    HelixRequestPatchError(#[from] HelixRequestPatchError),
    /// Got error from DELETE response
    #[error(transparent)]
    HelixRequestDeleteError(#[from] HelixRequestDeleteError),
    /// Custom error
    #[error("{0}")]
    Custom(std::borrow::Cow<'static, str>),
}
/// Could not create request
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum CreateRequestError {
    /// http crate returned an error
    HttpError(#[from] http::Error),
    /// serialization of body failed
    SerializeError(#[from] BodyError),
    /// could not assemble URI for request
    InvalidUri(#[from] InvalidUri),
    /// {0}
    Custom(std::borrow::Cow<'static, str>),
}

/// Errors that can happen when creating [`http::Uri`] for [`Request`](super::Request)
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum InvalidUri {
    /// URI could not be parsed
    UriParseError(#[from] http::uri::InvalidUri),
    /// could not assemble URI for request
    UrlError(#[from] url::ParseError),
    /// could not serialize request to query
    QuerySerializeError(#[from] SerializeError),
}

/// Could not parse GET response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum HelixRequestGetError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `GET {uri}`
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
    },
    /// could not parse response as utf8 when calling `GET {2}`
    Utf8Error(hyper::body::Bytes, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `GET {2}` with response: {3} - {0:?}
    DeserializeError(
        String,
        #[source] crate::DeserError,
        http::Uri,
        http::StatusCode,
    ),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse PUT response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum HelixRequestPutError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `PUT {uri}` with a body
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent to PUT response
        body: hyper::body::Bytes,
    },
    /// could not parse response as utf8 when calling `PUT {2}`
    Utf8Error(hyper::body::Bytes, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `PUT {2}` with response: {3} - {0:?}
    DeserializeError(
        String,
        #[source] crate::DeserError,
        http::Uri,
        http::StatusCode,
    ),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse POST response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum HelixRequestPostError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `POST {uri}` with a body
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent to POST response
        body: hyper::body::Bytes,
    },
    /// could not parse response as utf8 when calling `POST {2}`
    Utf8Error(hyper::body::Bytes, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `POST {2}` with response: {3} - {0:?}
    DeserializeError(
        String,
        #[source] crate::DeserError,
        http::Uri,
        http::StatusCode,
    ),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse PATCH response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum HelixRequestPatchError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `PATCH {uri}` with a body
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent to POST response
        body: hyper::body::Bytes,
    },
    /// could not parse response as utf8 when calling `POST {2}`
    Utf8Error(hyper::body::Bytes, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `POST {2}` with response: {3} - {0:?}
    DeserializeError(
        String,
        #[source] crate::DeserError,
        http::Uri,
        http::StatusCode,
    ),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse DELETE response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum HelixRequestDeleteError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `DELETE {uri}`
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent to DELETE response
        body: hyper::body::Bytes,
    },
    /// could not parse response as utf8 when calling `DELETE {2}`
    Utf8Error(hyper::body::Bytes, #[source] std::str::Utf8Error, http::Uri),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Helper trait to allow construction of any error for an invalid response
pub(crate) trait HelixRequestError {
    fn invalid_response(
        reason: &'static str,
        response: String,
        status: http::StatusCode,
        uri: http::Uri,
    ) -> Self;
}

/// Helper trait to allow construction of any error for a deserailization error (not available for DELETE requests)
pub(crate) trait HelixRequestDeserError: HelixRequestError {
    fn deserialize_error(
        body: String,
        err: crate::DeserError,
        uri: http::Uri,
        status: http::StatusCode,
    ) -> Self;
}

macro_rules! impl_request_error {
    ($($t:ty),*) => {
        $(impl HelixRequestError for $t {
            fn invalid_response(
                reason: &'static str,
                response: String,
                status: http::StatusCode,
                uri: http::Uri,
            ) -> Self {
                Self::InvalidResponse {
                    reason,
                    response,
                    status,
                    uri,
                }
            }
        })*
    };
}

macro_rules! impl_request_deser_error {
    ($($t:ty),*) => {
        $(impl HelixRequestDeserError for $t {
            fn deserialize_error(
                body: String,
                err: crate::DeserError,
                uri: http::Uri,
                status: http::StatusCode,
            ) -> Self {
                Self::DeserializeError(body, err, uri, status)
            }
        })*
    };
}

impl_request_error!(
    HelixRequestGetError,
    HelixRequestPatchError,
    HelixRequestPostError,
    HelixRequestPutError,
    HelixRequestDeleteError
);

impl_request_deser_error!(
    HelixRequestGetError,
    HelixRequestPatchError,
    HelixRequestPostError,
    HelixRequestPutError
);

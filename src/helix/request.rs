//! Requests for driving the API
pub mod errors;
use std::{convert::TryInto, marker::PhantomData, str::FromStr};

use crate::parse_json;

use super::{ser, HelixRequestBody, HelixRequestError, InnerResponse, Response};
use errors::*;
/// A request is a Twitch endpoint, see [New Twitch API](https://dev.twitch.tv/docs/api/reference) reference
#[async_trait::async_trait]
pub trait Request: serde::Serialize {
    /// The path to the endpoint relative to the helix root. eg. `channels` for [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    const PATH: &'static str;
    /// Scopes needed for this endpoint
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator;
    /// Optional scopes needed by this endpoint
    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    /// Response type. twitch's response will  deserialize to this.
    type Response: serde::de::DeserializeOwned + PartialEq;
    /// Defines layout of the url parameters.
    fn query(&self) -> Result<String, errors::SerializeError> { ser::to_string(self) }
    /// Returns full URI for the request, including query parameters.
    fn get_uri(&self) -> Result<http::Uri, InvalidUri> {
        let query = self.query()?;
        let url = crate::TWITCH_HELIX_URL
            .join(<Self as Request>::PATH)
            .map(|mut u| {
                u.set_query(Some(&query));
                u
            })?;
        http::Uri::from_str(url.as_str()).map_err(Into::into)
    }
    /// Returns bare URI for the request, NOT including query parameters.
    fn get_bare_uri() -> Result<http::Uri, InvalidUri> {
        let url = crate::TWITCH_HELIX_URL.join(<Self as Request>::PATH)?;
        http::Uri::from_str(url.as_str()).map_err(Into::into)
    }
}

/// Helix endpoint POSTs information
pub trait RequestPost: Request {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<hyper::body::Bytes>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        // eprintln!("\n\nbody is ------------ {:?} ------------", body);

        let mut bearer = http::HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| {
            CreateRequestError::Custom("Could not make token into headervalue".into())
        })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::POST)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(body)
            .map_err(Into::into)
    }

    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response<B: Into<hyper::body::Bytes>>(
        // FIXME: Is this really needed? Its currently only used for error reporting.
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<B>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPostError>
    where
        Self: Sized,
    {
        let response: http::Response<hyper::body::Bytes> = response.map(|b| b.into());
        let text = std::str::from_utf8(response.body().as_ref()).map_err(|e| {
            HelixRequestPostError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestPostError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        <Self as RequestPost>::parse_inner_response(request, uri, text, response.status())
    }

    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPostError>
    where
        Self: Sized,
    {
        let response: InnerResponse<<Self as Request>::Response> = parse_json(response, true)
            .map_err(|e| {
                HelixRequestPostError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(Response::new(
            response.data,
            response.pagination.cursor,
            request,
            response.total,
            response.other,
        ))
    }
}

/// Helix endpoint PATCHs information
pub trait RequestPatch: Request {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<hyper::body::Bytes>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        // eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer = http::HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| {
            CreateRequestError::Custom("Could not make token into headervalue".into())
        })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::PATCH)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(body)
            .map_err(Into::into)
    }

    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response<B: Into<hyper::body::Bytes>>(
        // FIXME: Is this really needed? Its currently only used for error reporting.
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<B>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPatchError>
    where
        Self: Sized,
    {
        let response: http::Response<hyper::body::Bytes> = response.map(|b| b.into());
        let text = std::str::from_utf8(response.body().as_ref()).map_err(|e| {
            HelixRequestPatchError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestPatchError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        <Self as RequestPatch>::parse_inner_response(request, uri, text, response.status())
    }

    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPatchError>
    where
        Self: Sized;
}

/// Helix endpoint DELETEs information
pub trait RequestDelete: Request {
    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<hyper::body::Bytes>, CreateRequestError> {
        let uri = self.get_uri()?;

        let mut bearer = http::HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| {
            CreateRequestError::Custom("Could not make token into headervalue".into())
        })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::DELETE)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(Vec::with_capacity(0).into())
            .map_err(Into::into)
    }
    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response<B: Into<hyper::body::Bytes>>(
        // FIXME: Is this really needed? Its currently only used for error reporting.
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<B>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestDeleteError>
    where
        Self: Sized,
    {
        let response: http::Response<hyper::body::Bytes> = response.map(|b| b.into());
        let text = std::str::from_utf8(response.body().as_ref()).map_err(|e| {
            HelixRequestDeleteError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestDeleteError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        <Self as RequestDelete>::parse_inner_response(request, uri, text, response.status())
    }
    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestDeleteError>
    where
        Self: Sized;
}

/// Helix endpoint PUTs information
pub trait RequestPut: Request {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<hyper::body::Bytes>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        // eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer = http::HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| {
            CreateRequestError::Custom("Could not make token into headervalue".into())
        })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::PUT)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(body)
            .map_err(Into::into)
    }

    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response<B: Into<hyper::body::Bytes>>(
        // FIXME: Is this really needed? Its currently only used for error reporting.
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<B>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPutError>
    where
        Self: Sized,
    {
        let response: http::Response<hyper::body::Bytes> = response.map(|b| b.into());
        let text = std::str::from_utf8(response.body().as_ref()).map_err(|e| {
            HelixRequestPutError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestPutError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        <Self as RequestPut>::parse_inner_response(request, uri, text, response.status())
    }

    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPutError>
    where
        Self: Sized;
}

/// Helix endpoint GETs information
pub trait RequestGet: Request {
    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<hyper::body::Bytes>, CreateRequestError> {
        let uri = self.get_uri()?;

        let mut bearer = http::HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| {
            CreateRequestError::Custom("Could not make token into headervalue".into())
        })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::GET)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(Vec::with_capacity(0).into())
            .map_err(Into::into)
    }

    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response<B: Into<hyper::body::Bytes>>(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<B>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestGetError>
    where
        Self: Sized,
    {
        let response: http::Response<hyper::body::Bytes> = response.map(|b| b.into());
        let text = std::str::from_utf8(response.body().as_ref()).map_err(|e| {
            HelixRequestGetError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        //eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestGetError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }
        <Self as RequestGet>::parse_inner_response(request, uri, text, response.status())
    }

    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestGetError>
    where
        Self: Sized,
    {
        let response: InnerResponse<_> = parse_json(response, true).map_err(|e| {
            HelixRequestGetError::DeserializeError(response.to_string(), e, uri.clone(), status)
        })?;
        Ok(Response::new(
            response.data,
            response.pagination.cursor,
            request,
            response.total,
            response.other,
        ))
    }
}

/// Parses a response where Helix responds with a single datum inside `data`.
///
/// An example response is `{ "data": [ { "foo": 1 } ]`.
pub(crate) fn parse_single_return<T, E>(
    request: Option<T>,
    uri: &http::Uri,
    response: &str,
    status: http::StatusCode,
) -> Result<Response<T, T::Response>, E>
where
    T: Request,
    E: errors::HelixRequestDeserError,
{
    let resp = match status {
        http::StatusCode::OK => {
            let resp: InnerResponse<[T::Response; 1]> = parse_json(response, true)
                .map_err(|e| E::deserialize_error(response.to_string(), e, uri.clone(), status))?;
            let [s] = resp.data;
            s
        }
        _ => {
            return Err(E::invalid_response(
                "unexpected status code",
                response.to_string(),
                status,
                uri.clone(),
            ));
        }
    };
    Ok(Response::with_data(resp, request))
}

/// A helper type to parse responses that contain either zero or one element in an array
pub(crate) struct ZeroOrOne<T>(pub Option<T>);

impl<'de, T> serde::Deserialize<'de> for ZeroOrOne<T>
where T: serde::Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct ZeroOrOneVisitor<T> {
            _marker: PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for ZeroOrOneVisitor<T>
        where T: serde::Deserialize<'de>
        {
            type Value = ZeroOrOne<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an array of length zero or one")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where A: serde::de::SeqAccess<'de> {
                match seq.next_element() {
                    Ok(None) => Ok(ZeroOrOne(None)),
                    Ok(Some(it)) => match seq.next_element::<T>() {
                        Ok(None) => Ok(ZeroOrOne(Some(it))),
                        Ok(Some(_)) => Err(serde::de::Error::invalid_length(2, &self)),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }
        }

        deserializer.deserialize_seq(ZeroOrOneVisitor::<T> {
            _marker: Default::default(),
        })
    }
}

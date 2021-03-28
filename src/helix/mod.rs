//! Helix endpoints or the [New Twitch API](https://dev.twitch.tv/docs/api)
//!
//!
//! Aside from using [`HelixClient`] as described on [the crate documentation](crate),
//! you can decide to use this library without any specific client implementation.
//!
//! ```rust,no_run
//! use twitch_api2::helix::{self, Request, RequestGet, users::{GetUsersRequest, User}};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//!
//! let request = GetUsersRequest::builder()
//!     .login(vec!["justintv123".to_string()])
//!     .build();
//!
//! // Send it however you want
//! // Create a [`http::Response<Vec<u8>>`] with RequestGet::create_request, which takes an access token and a client_id
//! let response = send_http_request(request.create_request("accesstoken", "client_id")?)?;
//!
//! // then parse the response
//! let uri = request.get_uri()?;
//! let user: helix::Response<_, Vec<User>> = GetUsersRequest::parse_response(Some(request), &uri,response)?;
//! println!("{:#?}", user);
//! # Ok(())
//! # }
//! # fn send_http_request(_: http::Request<Vec<u8>>) -> Result<http::Response<Vec<u8>>,&'static str> {todo!()}
//! ```
//!
use serde::Deserialize;
use std::{convert::TryInto, str::FromStr};
#[cfg(feature = "twitch_oauth2")]
use twitch_oauth2::TwitchToken;
#[cfg(all(feature = "client"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "helix"))))]
mod client_ext;

pub mod bits;
pub mod channels;
pub mod clips;
#[cfg(feature = "eventsub")]
#[cfg_attr(nightly, doc(cfg(feature = "eventsub")))]
pub mod eventsub;
pub mod games;
pub mod hypetrain;
pub mod moderation;
pub mod points;
pub mod search;
pub mod streams;
pub mod subscriptions;
pub mod tags;
pub mod teams;
pub mod users;
pub mod videos;
pub mod webhooks;

pub(crate) mod ser;
pub use ser::Error as SerializeError;

#[doc(no_inline)]
#[cfg(feature = "twitch_oauth2")]
pub use twitch_oauth2::Scope;

/// Client for Helix or the [New Twitch API](https://dev.twitch.tv/docs/api)
///
/// Provides [`HelixClient::req_get`] for requesting endpoints which uses [GET method][RequestGet].
///
///
/// Most [clients][crate::HttpClient] will be able to use the `'static` lifetime
///
/// ```rust,no_run
/// # use twitch_api2::{HelixClient}; pub mod reqwest {pub type Client = twitch_api2::client::DummyHttpClient;}
/// pub struct MyStruct {
///     twitch: HelixClient<'static, reqwest::Client>,
///     token: twitch_oauth2::AppAccessToken,
/// }
/// // etc
/// ```
///
/// See [`HttpClient`][crate::HttpClient] for implemented http clients, you can also define your own if needed.
///
/// # Examples
///
/// Get a [user](users::User) from their login name.
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # pub mod reqwest {pub type Client = twitch_api2::client::DummyHttpClient;}
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
/// use twitch_api2::helix::{HelixClient, users::User};
/// let helix: HelixClient<'static, reqwest::Client> = HelixClient::default();
/// let user: Option<User> = helix
///     .get_user_from_login("justintv".to_string(), &token).await.unwrap();
/// # Ok(()) }
/// ```
#[cfg(all(feature = "client"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "helix"))))]
#[derive(Clone)]
pub struct HelixClient<'a, C>
where C: crate::HttpClient<'a> {
    client: C,
    _pd: std::marker::PhantomData<&'a ()>, // TODO: Implement rate limiter...
}

#[derive(PartialEq, Deserialize, Debug)]
struct InnerResponse<D> {
    data: D,
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    #[serde(default)]
    pagination: Pagination,
}
#[derive(Deserialize, Clone, Debug)]
struct HelixRequestError {
    error: String,
    status: u16,
    message: String,
}

#[cfg(feature = "client")]
impl<'a, C: crate::HttpClient<'a>> HelixClient<'a, C> {
    /// Create a new client with an existing client
    pub fn with_client(client: C) -> HelixClient<'a, C> {
        HelixClient {
            client,
            _pd: std::marker::PhantomData::default(),
        }
    }

    /// Create a new [`HelixClient`] with a default [`HttpClient`][crate::HttpClient]
    pub fn new() -> HelixClient<'a, C>
    where C: Default {
        let client = C::default();
        HelixClient::with_client(client)
    }

    /// Retrieve a clone of the [`HttpClient`][crate::HttpClient] inside this [`HelixClient`]
    pub fn clone_client(&self) -> C
    where C: Clone {
        self.client.clone()
    }

    /// Request on a valid [`RequestGet`] endpoint
    ///
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// #   use twitch_api2::helix::{HelixClient, channels};
    /// #   let token = Box::new(twitch_oauth2::UserToken::from_existing_unchecked(
    /// #       twitch_oauth2::AccessToken::new("totallyvalidtoken".to_string()), None,
    /// #       twitch_oauth2::ClientId::new("validclientid".to_string()), None, "justintv".to_string(), "1337".to_string(), None, None));
    ///     let req = channels::GetChannelInformationRequest::builder().broadcaster_id("123456").build();
    ///     let client = HelixClient::new();
    /// # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
    ///
    ///     let response = client.req_get(req, &token).await;
    /// # }
    /// # // fn main() {run()}
    /// ```
    pub async fn req_get<R, D, T>(
        &'a self,
        request: R,
        token: &T,
    ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestGet,
        D: serde::de::DeserializeOwned + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    }

    /// Request on a valid [`RequestPost`] endpoint
    pub async fn req_post<R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestPost<Body = B>,
        B: HelixRequestBody,
        D: serde::de::DeserializeOwned + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    }

    /// Request on a valid [`RequestPatch`] endpoint
    pub async fn req_patch<R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<D, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestPatch<Body = B>,
        B: HelixRequestBody,
        D: std::convert::TryFrom<http::StatusCode, Error = std::borrow::Cow<'static, str>>
            + serde::de::DeserializeOwned
            + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(&uri, response).map_err(Into::into)
    }

    /// Request on a valid [`RequestDelete`] endpoint
    pub async fn req_delete<R, D, T>(
        &'a self,
        request: R,
        token: &T,
    ) -> Result<D, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestDelete,
        D: std::convert::TryFrom<http::StatusCode, Error = std::borrow::Cow<'static, str>>
            + serde::de::DeserializeOwned
            + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(&uri, response).map_err(Into::into)
    }

    /// Request on a valid [`RequestPut`] endpoint
    pub async fn req_put<R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<D, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestPut<Body = B>,
        B: HelixRequestBody,
        D: std::convert::TryFrom<http::StatusCode, Error = std::borrow::Cow<'static, str>>
            + serde::de::DeserializeOwned
            + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(&uri, response).map_err(Into::into)
    }
}

#[cfg(feature = "client")]
impl<'a, C> Default for HelixClient<'a, C>
where C: crate::HttpClient<'a> + Default
{
    fn default() -> HelixClient<'a, C> { HelixClient::new() }
}

/// Deserialize 'null' as <T as Default>::Default
fn deserialize_default_from_empty_string<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: serde::de::DeserializeOwned + Default, {
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::String(string) if string.is_empty() => Ok(None),
        other => Ok(serde_json::from_value(other).map_err(serde::de::Error::custom)?),
    }
}

/// A request is a Twitch endpoint, see [New Twitch API](https://dev.twitch.tv/docs/api/reference) reference
#[async_trait::async_trait]
#[cfg_attr(nightly, doc(spotlight))]
pub trait Request: serde::Serialize {
    /// The path to the endpoint relative to the helix root. eg. `channels` for [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    const PATH: &'static str;
    /// Scopes needed by this endpoint
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope];
    /// Optional scopes needed by this endpoint
    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    /// Response type. twitch's response will  deserialize to this.
    type Response: serde::de::DeserializeOwned + PartialEq;
    /// Defines layout of the url parameters.
    fn query(&self) -> Result<String, ser::Error> { ser::to_string(&self) }
    /// Returns full URI for the request, including query parameters.
    fn get_uri(&self) -> Result<http::Uri, InvalidUri> {
        http::Uri::from_str(&format!(
            "{}{}?{}",
            crate::TWITCH_HELIX_URL,
            <Self as Request>::PATH,
            self.query()?
        ))
        .map_err(Into::into)
    }
    /// Returns bare URI for the request, NOT including query parameters.
    fn get_bare_uri() -> Result<http::Uri, InvalidUri> {
        http::Uri::from_str(&format!(
            "{}{}?",
            crate::TWITCH_HELIX_URL,
            <Self as Request>::PATH,
        ))
        .map_err(Into::into)
    }
}

/// Helix endpoint POSTs information
#[cfg_attr(nightly, doc(spotlight))]
pub trait RequestPost: Request {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        //eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
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

    /// Parse response. Override for different behavior
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPostError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            HelixRequestPostError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<HelixRequestError>(&text)
        {
            return Err(HelixRequestPostError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        let response: InnerResponse<<Self as Request>::Response> = serde_json::from_str(&text)
            .map_err(|e| {
                HelixRequestPostError::DeserializeError(text.to_string(), e, uri.clone())
            })?;
        Ok(Response {
            data: response.data,
            pagination: response.pagination.cursor,
            request,
        })
    }
}

/// Helix endpoint PATCHs information
#[cfg_attr(nightly, doc(spotlight))]
pub trait RequestPatch: Request
where <Self as Request>::Response:
        std::convert::TryFrom<http::StatusCode, Error = std::borrow::Cow<'static, str>> {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        // eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
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

    /// Parse response. Override for different behavior
    fn parse_response(
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<<Self as Request>::Response, HelixRequestPatchError>
    where
        Self: Sized,
    {
        match response.status().try_into() {
            Ok(result) => Ok(result),
            Err(err) => Err(HelixRequestPatchError {
                status: response.status(),
                message: err.to_string(),
                uri: uri.clone(),
                body: response.body().clone(),
            }),
        }
    }
}

/// Helix endpoint DELETEs information
#[cfg_attr(nightly, doc(spotlight))]
pub trait RequestDelete: Request {
    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
                CreateRequestError::Custom("Could not make token into headervalue".into())
            })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::DELETE)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(Vec::with_capacity(0))
            .map_err(Into::into)
    }

    /// Parse response. Override for different behavior
    fn parse_response(
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<<Self as Request>::Response, HelixRequestDeleteError>
    where
        <Self as Request>::Response:
            std::convert::TryFrom<http::StatusCode, Error = std::borrow::Cow<'static, str>>,
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            HelixRequestDeleteError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        // eprintln!("\n\nmessage is ------------ {} ------------", text);

        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<HelixRequestError>(&text)
        {
            return Err(HelixRequestDeleteError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }

        match response.status().try_into() {
            Ok(result) => Ok(result),
            Err(err) => Err(HelixRequestDeleteError::Error {
                error: String::new(),
                status: response.status(),
                message: err.to_string(),
                uri: uri.clone(),
            }),
        }
    }
}

/// Helix endpoint PUTs information
#[cfg_attr(nightly, doc(spotlight))]
pub trait RequestPut: Request
where <Self as Request>::Response:
        std::convert::TryFrom<http::StatusCode, Error = std::borrow::Cow<'static, str>> {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        // eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
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

    /// Parse response. Override for different behavior
    fn parse_response(
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<<Self as Request>::Response, HelixRequestPutError> {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            HelixRequestPutError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        // eprintln!("\n\nmessage is ------------ {} ------------", text);

        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<HelixRequestError>(&text)
        {
            return Err(HelixRequestPutError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }

        match response.status().try_into() {
            Ok(result) => Ok(result),
            Err(err) => Err(HelixRequestPutError::Error {
                error: String::new(),
                status: response.status(),
                message: err.to_string(),
                uri: uri.clone(),
                body: response.body().clone(),
            }),
        }
    }
}

/// Helix endpoint GETs information
#[cfg_attr(nightly, doc(spotlight))]
pub trait RequestGet: Request {
    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
                CreateRequestError::Custom("Could not make token into headervalue".into())
            })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::GET)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(Vec::with_capacity(0))
            .map_err(Into::into)
    }

    /// Parse response. Override for different behavior
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestGetError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            HelixRequestGetError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        //eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<HelixRequestError>(&text)
        {
            return Err(HelixRequestGetError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }
        let response: InnerResponse<_> = serde_json::from_str(&text).map_err(|e| {
            HelixRequestGetError::DeserializeError(text.to_string(), e, uri.clone())
        })?;
        Ok(Response {
            data: response.data,
            pagination: response.pagination.cursor,
            request,
        })
    }
}

/// Response retrieved from endpoint. Data is the type in [`Request::Response`]
#[derive(PartialEq, Debug)]
pub struct Response<R, D>
where
    R: Request<Response = D>,
    D: serde::de::DeserializeOwned + PartialEq, {
    /// Twitch's response field for `data`.
    pub data: D,
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    pub pagination: Option<Cursor>,
    /// The request that was sent, used for [pagination](Paginated).
    pub request: Option<R>,
}

impl<R, D, T> Response<R, D>
where
    R: Request<Response = D>,
    D: IntoIterator<Item = T> + PartialEq + serde::de::DeserializeOwned,
{
    /// Get first result of this response.
    pub fn first(self) -> Option<T> { self.data.into_iter().next() }
}

#[cfg(feature = "client")]
impl<R, D> Response<R, D>
where
    R: Request<Response = D> + Clone + Paginated + RequestGet + std::fmt::Debug,
    D: serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    /// Get the next page in the responses.
    pub async fn get_next<'a, C: crate::HttpClient<'a>>(
        self,
        client: &'a HelixClient<'a, C>,
        token: &impl TwitchToken,
    ) -> Result<Option<Response<R, D>>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    {
        if let Some(mut req) = self.request.clone() {
            if self.pagination.is_some() {
                req.set_pagination(self.pagination);
                let res = client.req_get(req, token).await.map(Some);
                if let Ok(Some(r)) = res {
                    // FIXME: Workaround for https://github.com/twitchdev/issues/issues/18
                    if r.data == self.data {
                        Ok(None)
                    } else {
                        Ok(Some(r))
                    }
                } else {
                    res
                }
            } else {
                Ok(None)
            }
        } else {
            // TODO: Make into proper error
            Err(ClientRequestError::Custom(
                "no source request attached".into(),
            ))
        }
    }
}

/// Request can be paginated with a cursor
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
pub type Cursor = String;

/// Errors for [`HelixClient::req_get`] and similar functions.
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum ClientRequestError<RE: std::error::Error + Send + Sync + 'static> {
    /// request failed from reqwests side
    RequestError(RE),
    /// no pagination found
    NoPage,
    /// could not create request
    CreateRequestError(#[from] CreateRequestError),
    /// could not parse GET response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestGetError(#[from] HelixRequestGetError),
    /// could not parse PUT response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestPutError(#[from] HelixRequestPutError),
    /// could not parse POST response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestPostError(#[from] HelixRequestPostError),
    /// could not parse PATCH response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestPatchError(#[from] HelixRequestPatchError),
    /// could not parse DELETE response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestDeleteError(#[from] HelixRequestDeleteError),
    /// {0}
    Custom(std::borrow::Cow<'static, str>),
}
/// Could not create request
#[derive(thiserror::Error, Debug, displaydoc::Display)]
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

/// Errors that can happen when creating [`http::Uri`] for [`Request`]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum InvalidUri {
    /// URI could not be parsed
    UriParseError(#[from] http::uri::InvalidUri),
    /// could not serialize request to query
    QuerySerializeError(#[from] ser::Error),
}

/// Could not parse GET response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
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
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `GET {2}` with response: {0:?}
    DeserializeError(String, #[source] serde_json::Error, http::Uri),
    // FIXME: Only used in webhooks parse_payload
    /// could not get URI for request
    InvalidUri(#[from] InvalidUri),
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
        /// Body sent with PUT
        body: Vec<u8>,
    },
    /// could not parse response as utf8 when calling `PUT {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `PUT {2}` with response: {0:?}
    DeserializeError(String, #[source] serde_json::Error, http::Uri),
}

/// Could not parse POST response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
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
        /// Body sent with POST
        body: Vec<u8>,
    },
    /// could not parse response as utf8 when calling `POST {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `POST {2}` with response: {0:?}
    DeserializeError(String, #[source] serde_json::Error, http::Uri),
}

/// helix returned error {status:?}: {message:?} when calling `PATCH {uri}` with a body
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub struct HelixRequestPatchError {
    /// Status code of error, usually 400-499
    status: http::StatusCode,
    /// Error message from Twitch
    message: String,
    /// URI to the endpoint
    uri: http::Uri,
    /// Body sent with PATCH
    body: Vec<u8>,
}

/// Could not parse DELETE response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum HelixRequestDeleteError {
    /// helix returned error {status:?}- {error}: {message:?} when calling `DELETE {uri}`
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
    /// could not parse response as utf8 when calling `DELETE {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
}

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
    fn try_to_body(&self) -> Result<Vec<u8>, BodyError>;
}

/// An empty body.
///
/// Implements [`HelixRequestBody::try_to_body`], returning an empty vector
#[derive(Default, Clone, Copy)]
pub struct EmptyBody;

impl HelixRequestBody for EmptyBody {
    fn try_to_body(&self) -> Result<Vec<u8>, BodyError> { Ok(vec![]) }
}

// TODO: I would want specialization for this. For now, to override this behavior for a body, we specify a sealed trait
impl<T> HelixRequestBody for T
where T: serde::Serialize + private::SealedSerialize
{
    fn try_to_body(&self) -> Result<Vec<u8>, BodyError> {
        serde_json::to_vec(&self).map_err(Into::into)
    }
}

pub(crate) mod private {
    pub trait SealedSerialize {}
}

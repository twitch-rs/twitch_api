//! Helix endpoints or the [New Twitch API](https://dev.twitch.tv/docs/api)
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, io};
use twitch_oauth2::TwitchToken;

pub mod channels;
pub mod clips;
pub mod moderation;
pub mod streams;
pub mod subscriptions;
pub mod users;

pub(crate) mod ser;
pub use ser::Error;

#[doc(no_inline)]
pub use twitch_oauth2::Scope;

/// Client for Helix or the [New Twitch API](https://dev.twitch.tv/docs/api)
///
/// Provides [HelixClient::req_get] for requesting endpoints which uses [GET method][RequestGet].
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
/// See [HttpClient][crate::HttpClient] for implemented http clients, you can also define your own if needed.
#[cfg(feature = "helix")]
#[cfg_attr(nightly, doc(cfg(feature = "helix")))]
#[derive(Clone)]
pub struct HelixClient<'a, C>
where C: crate::HttpClient<'a> {
    client: C,
    _pd: std::marker::PhantomData<&'a ()>, // TODO: Implement rate limiter...
}

#[derive(PartialEq, Deserialize, Debug)]
struct InnerResponse<D> {
    data: Vec<D>,
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

impl<'a, C: crate::HttpClient<'a>> HelixClient<'a, C> {
    /// Create a new client with with an existing client
    pub fn with_client(client: C) -> HelixClient<'a, C> {
        HelixClient {
            client,
            _pd: std::marker::PhantomData::default(),
        }
    }

    /// Create a new [HelixClient] with a default [HttpClient][crate::HttpClient]
    pub fn new() -> HelixClient<'a, C>
    where C: Default {
        let client = C::default();
        HelixClient::with_client(client)
    }

    /// Retrieve a clone of the [HttpClient][crate::HttpClient] inside this [HelixClient]
    pub fn clone_client(&self) -> C
    where C: Clone {
        self.client.clone()
    }

    /// Request on a valid [RequestGet] endpoint
    ///
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// #   use twitch_api2::helix::{HelixClient, channels};
    /// #   let token = Box::new(twitch_oauth2::UserToken::from_existing_unchecked(
    /// #       twitch_oauth2::AccessToken::new("totallyvalidtoken".to_string()), None,
    /// #       twitch_oauth2::ClientId::new("validclientid".to_string()), None, None));
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
    ) -> Result<Response<R, D>, RequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestGet,
        D: serde::de::DeserializeOwned,
        T: TwitchToken + ?Sized,
    {
        let url = url::Url::parse(&format!(
            "{}{}?{}",
            crate::TWITCH_HELIX_URL,
            <R as Request>::PATH,
            request.query()?
        ))?;
        let mut bearer = http::HeaderValue::from_str(&format!("Bearer {}", token.token().secret()))
            .map_err(|_| RequestError::Custom("Could not make token into headervalue".into()))?;
        bearer.set_sensitive(true);
        let req = http::Request::builder()
            .method(http::Method::GET)
            .uri(url.to_string())
            .header("Client-ID", token.client_id().as_str())
            .header(http::header::AUTHORIZATION, bearer)
            .body(Vec::with_capacity(0))?;
        let response = self
            .client
            .req(req)
            .await
            .map_err(RequestError::RequestError)?;
        let text = std::str::from_utf8(&response.body())
            .map_err(|e| RequestError::Utf8Error(response.body().clone(), e))?;
        //eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<HelixRequestError>(&text)
        {
            return Err(RequestError::HelixRequestGetError {
                error,
                status: status
                    .try_into()
                    .unwrap_or_else(|_| http::StatusCode::BAD_REQUEST),
                message,
                url,
            });
        }
        let response: InnerResponse<D> = serde_json::from_str(&text)?;
        Ok(Response {
            data: response.data,
            pagination: response.pagination,
            request,
        })
    }

    /// Request on a valid [RequestPost] endpoint
    pub async fn req_post<R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<Response<R, D>, RequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestPost<Body = B>,
        B: serde::Serialize,
        D: serde::de::DeserializeOwned,
        T: TwitchToken + ?Sized,
    {
        let url = url::Url::parse(&format!(
            "{}{}?{}",
            crate::TWITCH_HELIX_URL,
            <R as Request>::PATH,
            request.query()?
        ))?;

        let body = request.body(&body)?;
        // eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer = http::HeaderValue::from_str(&format!("Bearer {}", token.token().secret()))
            .map_err(|_| RequestError::Custom("Could not make token into headervalue".into()))?;
        bearer.set_sensitive(true);
        let req = http::Request::builder()
            .method(http::Method::POST)
            .uri(url.to_string())
            .header("Client-ID", token.client_id().as_str())
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(body.clone().into_bytes())?;
        let response = self
            .client
            .req(req)
            .await
            .map_err(RequestError::RequestError)?;
        let text = std::str::from_utf8(&response.body())
            .map_err(|e| RequestError::Utf8Error(response.body().clone(), e))?;
        // eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<HelixRequestError>(&text)
        {
            return Err(RequestError::HelixRequestPutError {
                error,
                status: status
                    .try_into()
                    .unwrap_or_else(|_| http::StatusCode::BAD_REQUEST),
                message,
                url,
                body,
            });
        }
        let response: InnerResponse<D> = serde_json::from_str(&text)?;
        Ok(Response {
            data: response.data,
            pagination: response.pagination,
            request,
        })
    }

    /// Request on a valid [RequestPatch] endpoint
    pub async fn req_patch<R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<D, RequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestPatch<Body = B>,
        B: serde::Serialize,
        D: std::convert::TryFrom<http::StatusCode, Error = std::borrow::Cow<'static, str>>,
        T: TwitchToken + ?Sized,
    {
        let url = url::Url::parse(&format!(
            "{}{}?{}",
            crate::TWITCH_HELIX_URL,
            <R as Request>::PATH,
            request.query()?
        ))?;

        let body = request.body(&body)?;
        // eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer = http::HeaderValue::from_str(&format!("Bearer {}", token.token().secret()))
            .map_err(|_| RequestError::Custom("Could not make token into headervalue".into()))?;
        bearer.set_sensitive(true);
        let req = http::Request::builder()
            .method(http::Method::PATCH)
            .uri(url.to_string())
            .header("Client-ID", token.client_id().as_str())
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(body.clone().into_bytes())?;
        let response = self
            .client
            .req(req)
            .await
            .map_err(RequestError::RequestError)?;
        //let text = std::str::from_utf8(&response.body())
        //    .map_err(|e| RequestError::Utf8Error(response.body().clone(), e))?;
        // eprintln!("\n\nmessage is ------------ {} ------------", text);

        match response.status().try_into() {
            Ok(result) => Ok(result),
            Err(err) => Err(RequestError::HelixRequestPatchError {
                status: response.status(),
                message: err.to_string(),
                url,
                body,
            }),
        }
    }
}

impl<'a, C> Default for HelixClient<'a, C>
where C: crate::HttpClient<'a> + Default
{
    fn default() -> HelixClient<'a, C> { HelixClient::new() }
}

/// A request is a Twitch endpoint, see [New Twitch API](https://dev.twitch.tv/docs/api/reference) reference
#[async_trait::async_trait]
pub trait Request: serde::Serialize {
    /// The path to the endpoint relative to the helix root. eg. `channels` for [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    const PATH: &'static str;
    /// Scopes needed by this endpoint
    const SCOPE: &'static [twitch_oauth2::Scope];
    /// Optional scopes needed by this endpoint
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    /// Response type. twitch's response will  deserialize to this.
    type Response;
    /// Defines layout of the url parameters.
    fn query(&self) -> Result<String, ser::Error> { ser::to_string(&self) }
}

/// Helix endpoint POSTs information
pub trait RequestPost: Request {
    /// Body parameters
    type Body: serde::Serialize;

    /// Create body text from [RequestPost::Body]
    fn body(&self, body: &Self::Body) -> Result<String, serde_json::Error> {
        serde_json::to_string(body)
    }
}

/// Helix endpoint PATCHs information
pub trait RequestPatch: Request {
    /// Body parameters
    type Body: serde::Serialize;

    /// Create body text from [RequestPost::Body]
    fn body(&self, body: &Self::Body) -> Result<String, serde_json::Error> {
        serde_json::to_string(body)
    }
}

/// Helix endpoint GETs information
pub trait RequestGet: Request {}

/// Response retrieved from endpoint. Data is the type in [Request::Response]
#[derive(PartialEq, Debug)]
pub struct Response<R, D>
where R: Request<Response = D> {
    ///  Twitch's response field for `data`.
    pub data: Vec<D>,
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    pub pagination: Pagination,
    /// The request that was sent, used for [Paginated]
    pub request: R,
}

impl<R, D> Response<R, D>
where
    R: Request<Response = D> + Clone + Paginated + RequestGet,
    D: serde::de::DeserializeOwned,
{
    /// Get the next page in the responses.
    pub async fn get_next<'a, C: crate::HttpClient<'a>>(
        self,
        client: &'a HelixClient<'a, C>,
        token: &impl TwitchToken,
    ) -> Result<Option<Response<R, D>>, RequestError<<C as crate::HttpClient<'a>>::Error>>
    {
        let mut req = self.request.clone();
        if let Some(ref cursor) = self.pagination.cursor {
            req.set_pagination(cursor.clone());
            client.req_get(req, token).await.map(Some)
        } else {
            Ok(None)
        }
    }
}

/// Request can be paginated with a cursor
pub trait Paginated: Request {
    /// Should returns the current pagination cursor.
    ///
    /// # Notes
    ///
    /// Pass [Option::None] if no cursor is found.
    fn set_pagination(&mut self, cursor: Cursor);
}
/// A cursor for pagination. This is needed because of how pagination is represented in the [New Twitch API](https://dev.twitch.tv/docs/api)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Default)]
pub struct Pagination {
    #[serde(default)]
    cursor: Option<Cursor>,
}

/// A cursor is a pointer to the current "page" in the twitch api pagination
pub type Cursor = String;

/// Errors for [HelixClient::req_get] and similar functions.
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum RequestError<RE: std::error::Error + 'static> {
    /// http crate returned an error
    HttpError(#[from] http::Error),
    /// could not parse body as utf8: {1}
    Utf8Error(Vec<u8>, std::str::Utf8Error),
    /// url could not be parsed
    UrlParseError(#[from] url::ParseError),
    /// io error
    IOError(#[from] io::Error),
    /// deserialization failed when processing request result
    DeserializeError(#[from] serde_json::Error),
    /// Could not serialize request to query
    QuerySerializeError(#[from] ser::Error),
    /// request failed from reqwests side
    RequestError(RE),
    /// no pagination found
    NoPage,
    /// could not parse response from patch:  {0}
    PatchParseError(std::borrow::Cow<'static, str>),
    /// {0}
    Custom(std::borrow::Cow<'static, str>),
    /// helix returned error {status:?} - {error}: {message:?} when calling `GET {url}`
    HelixRequestGetError {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URL to the endpoint
        url: url::Url,
    },
    /// helix returned error {status:?} - {error}: {message:?} when calling `PUT {url}: "{body}"`
    HelixRequestPutError {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URL to the endpoint
        url: url::Url,
        /// Body sent with PUT
        body: String,
    },
    /// helix returned error {status:?}: {message:?} when calling `PATCH {url}: "{body}"`
    HelixRequestPatchError {
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URL to the endpoint
        url: url::Url,
        /// Body sent with PUT
        body: String,
    },
}

/// Repeat url query items with name
///
/// ```rust
/// let users = &["emilgardis", "jtv", "tmi"].iter().map(<_>::to_string).collect::<Vec<_>>();
///  assert_eq!(&twitch_api2::helix::repeat_query("user", users), "user=emilgardis&user=jtv&user=tmi")
/// ```
pub fn repeat_query(name: &str, items: &[String]) -> String {
    let mut s = String::new();
    for (idx, item) in items.iter().enumerate() {
        s.push_str(&format!("{}={}", name, item));
        if idx + 1 != items.len() {
            s.push_str("&")
        }
    }
    s
}

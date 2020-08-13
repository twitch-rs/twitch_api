//! Helix endpoints or the [New Twitch API](https://dev.twitch.tv/docs/api)
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, io};
use twitch_oauth2::TwitchToken;

pub mod channel;
pub mod clips;
pub mod moderation;
pub mod streams;
pub mod users;

pub use twitch_oauth2::Scope;

/// Client for Helix or the [New Twitch API](https://dev.twitch.tv/docs/api)
///
/// Provides [HelixClient::req_get] for requesting endpoints which uses [GET method][RequestGet].
#[derive(Clone)]
pub struct HelixClient {
    client: reqwest::Client,
    // TODO: Implement rate limiter...
}

impl HelixClient {
    /// Create a new client with with an existing [reqwest::Client]
    pub fn with_client(client: reqwest::Client) -> HelixClient { HelixClient { client } }

    /// Create a new client with a default [reqwest::Client]
    pub fn new() -> HelixClient {
        let client = reqwest::Client::new();
        HelixClient::with_client(client)
    }

    /// Retrieve a clone of the [reqwest::Client] inside this [HelixClient]
    pub fn clone_client(&self) -> reqwest::Client { self.client.clone() }

    /// Request on a valid [Request] endpoint
    ///
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// #   use twitch_api2::helix::{HelixClient, channel};
    /// #   let token = Box::new(twitch_oauth2::UserToken::from_existing_unchecked(
    /// #       twitch_oauth2::AccessToken::new("totallyvalidtoken".to_string()), None,
    /// #       twitch_oauth2::ClientId::new("validclientid".to_string()), None, None));
    ///     let req = channel::GetChannelRequest::builder().broadcaster_id("123456").build();
    ///     let client = HelixClient::new();
    ///     let response = client.req_get(req, &token).await;
    /// # }
    /// # // fn main() {run()}
    /// ```
    #[allow(clippy::needless_doctest_main)]
    pub async fn req_get<R, D>(
        &self,
        request: R,
        token: &impl TwitchToken,
    ) -> Result<Response<R, D>, RequestError>
    where
        R: Request<Response = D> + Request + RequestGet,
        D: serde::de::DeserializeOwned,
    {
        #[derive(PartialEq, Deserialize, Debug)]
        struct InnerResponse<D> {
            data: Vec<D>,
            /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
            #[serde(default)]
            pagination: Pagination,
        }
        #[derive(Deserialize, Clone, Debug)]
        pub struct HelixRequestError {
            error: String,
            status: u16,
            message: String,
        }

        let url = url::Url::parse(&format!(
            "{}{}?{}",
            crate::TWITCH_HELIX_URL,
            <R as Request>::PATH,
            request.query()?
        ))?;

        let req = self
            .client
            .get(url.clone())
            .header("Client-ID", token.client_id().as_str())
            .bearer_auth(token.token().secret())
            .send()
            .await?;
        let text = req.text().await?;
        //eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<HelixRequestError>(&text)
        {
            return Err(RequestError::HelixRequestError {
                error,
                status: status
                    .try_into()
                    .unwrap_or_else(|_| reqwest::StatusCode::BAD_REQUEST),
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
}

impl Default for HelixClient {
    fn default() -> Self { HelixClient::new() }
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
    /// Defines layout of the url parameters. By default uses [serde_urlencoded]
    fn query(&self) -> Result<String, serde_urlencoded::ser::Error> {
        serde_urlencoded::to_string(&self)
    }
}

/// Helix endpoint PUTs information
pub trait RequestPut: Request {}

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
    pub async fn get_next(
        self,
        client: &HelixClient,
        token: &impl TwitchToken,
    ) -> Result<Option<Response<R, D>>, RequestError>
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
pub enum RequestError {
    /// url could not be parsed
    UrlParseError(#[from] url::ParseError),
    /// io error
    IOError(#[from] io::Error),
    /// deserialization failed
    DeserializeError(#[from] serde_json::Error),
    /// Could not serialize request to query
    QuerySerializeError(#[from] serde_urlencoded::ser::Error),
    /// request failed from reqwests side
    RequestError(#[from] reqwest::Error),
    /// no pagination found
    NoPage,
    /// something happened
    Other,
    /// helix returned error {status:?} - {error} when calling `{url}` with message: {message}
    HelixRequestError {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: reqwest::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URL to the endpoint
        url: url::Url,
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

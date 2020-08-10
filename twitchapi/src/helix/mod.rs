//! Helix endpoints or the [New Twitch API](https://dev.twitch.tv/docs/api)
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, io, sync::Arc};
use tokio::sync;
use twitch_oauth2::TwitchToken;

pub mod channel;
pub mod clips;
pub mod streams;
pub mod users;

pub use twitch_oauth2::Scope;

/// Client for Helix or the [New Twitch API](https://dev.twitch.tv/docs/api)
#[derive(Clone)]
pub struct HelixClient {
    token: Arc<sync::RwLock<Box<dyn TwitchToken + Send + Sync>>>,
    client: reqwest::Client,
    // TODO: Implement rate limiter...
}

impl HelixClient {
    /// Create a new client with a default [reqwest::Client]
    pub fn new<T>(token: Box<T>) -> HelixClient
    where T: TwitchToken + Sized + Send + Sync + 'static {
        let client = reqwest::Client::new();
        HelixClient {
            token: Arc::new(sync::RwLock::new(token)),
            client,
        }
    }

    /// Retrieve a clone of the [reqwest::Client] inside this [HelixClient]
    pub fn clone_client(&self) -> reqwest::Client { self.client.clone() }

    /// Get a [tokio::time::Delay] that will return when the token attached to this client expires
    pub async fn monitor_expire(&self) -> Option<tokio::time::Delay> {
        self.token()
            .await
            .as_ref()
            .expires()
            .map(tokio::time::Instant::from_std)
            .map(tokio::time::delay_until)
    }

    /// Access the underlying [TwitchToken] from this client
    pub async fn token(&self) -> sync::RwLockReadGuard<'_, Box<dyn TwitchToken + Send + Sync>> {
        self.token.read().await
    }

    /// Refresh the underlying [TwitchToken]
    pub async fn refresh_token(&self) -> Result<(), twitch_oauth2::RefreshTokenError> {
        let mut token = self.token.write().await;
        token.as_mut().refresh_token().await?;
        Ok(())
    }

    // FIXME: allow multiple?
    /// Get information about users channel with specific id
    pub async fn get_channel_information(
        &self,
        broadcaster_id: String,
    ) -> Result<Option<channel::GetChannel>, RequestError>
    {
        let req = channel::GetChannelRequest { broadcaster_id };
        let response = self.req_get(req).await?;
        Ok(response.data.into_iter().next())
    }

    /// Access GetStreams builder.
    pub async fn get_streams<F>(
        &self,
        builder: F,
    ) -> Result<Response<streams::GetStreamsRequest, streams::GetStreams>, RequestError>
    where
        F: FnOnce(
            streams::get_streams::GetStreamsRequestBuilder<((), (), (), (), (), (), ())>,
        ) -> streams::GetStreamsRequest,
    {
        let req = builder(streams::GetStreamsRequest::builder());
        let response = self.req_get(req).await?;
        Ok(response)
    }

    /// Get user information. See [users::get_users]
    pub async fn get_users<F>(
        &self,
        builder: F,
    ) -> Result<Response<users::GetUsersRequest, users::GetUsers>, RequestError>
    where
        F: FnOnce(users::get_users::GetUsersRequestBuilder<((), ())>) -> users::GetUsersRequest,
    {
        let req = builder(users::GetUsersRequest::builder());
        let response = self.req_get(req).await?;
        Ok(response)
    }

    /// Get clip information. See [clips::get_clips]
    pub async fn get_clips<F>(
        &self,
        builder: F,
    ) -> Result<Response<clips::GetClipsRequest, clips::GetClips>, RequestError>
    where
        F: FnOnce(
            clips::get_clips::GetClipsRequestBuilder<((), (), (), (), (), (), (), ())>,
        ) -> clips::GetClipsRequest,
    {
        let req = builder(clips::GetClipsRequest::builder());
        let response = self.req_get(req).await?;
        Ok(response)
    }

    /// Request on a valid [Request] endpoint
    pub async fn req_get<R, D>(&self, request: R) -> Result<Response<R, D>, RequestError>
    where
        R: Request<Response = D> + Request,
        D: serde::de::DeserializeOwned, {
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

        let token = self.token().await;
        let req = self
            .client
            .get(url.clone())
            .header("Client-ID", token.client_id())
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
    R: Request<Response = D> + Clone + Paginated,
    D: serde::de::DeserializeOwned,
{
    /// Get the next page in the responses.
    pub async fn get_next(
        &self,
        client: &HelixClient,
    ) -> Result<Option<Response<R, D>>, RequestError>
    {
        let mut req = self.request.clone();
        if let Some(ref cursor) = self.pagination.cursor {
            req.set_pagination(cursor.clone());
            client.req_get(req).await.map(Some)
        } else {
            Ok(None)
        }
    }
}

/// Request can be paginated with a cursor
pub trait Paginated {
    /// Should returns the current pagination cursor.
    /// 
    /// # Notes
    /// 
    /// Use [Cursor.cursor] as [Option::None] if no cursor is found.
    fn set_pagination(&mut self, cursor: Cursor);
}
/// A cursor for pagination. This is needed because of how pagination is represented in the [New Twitch API](https://dev.twitch.tv/docs/api)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Default)]
pub struct Pagination {
    #[serde(default)]
    cursor: Option<Cursor>,
}

/// A cursor is a pointer to the current "page" in thje twitch api pagination
pub type Cursor = String;

/// Errors for [HelixClient::req_get] and similar functions.
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("url could not be parsed")]
    UrlParseError(#[from] url::ParseError),
    #[error("io error")]
    IOError(#[from] io::Error),
    #[error("deserialization failed")]
    DeserializeError(#[from] serde_json::Error),
    #[error("Could not serialize request to query")]
    QuerySerializeError(#[from] serde_urlencoded::ser::Error),
    #[error("request failed from reqwests side")]
    RequestError(#[from] reqwest::Error),
    #[error("no pagination found")]
    NoPage,
    #[error("something happened")]
    Other,
    #[error(
        "helix returned error {status:?} - {error} when calling `{url}` with message: {message}"
    )]
    HelixRequestError {
        error: String,
        status: reqwest::StatusCode,
        message: String,
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

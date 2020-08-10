use serde::{Deserialize, Serialize};
use std::{convert::TryInto, io, sync::Arc};
use tokio::sync;
use twitch_oauth2::TwitchToken;

pub mod channel;
pub mod clips;
pub mod streams;
pub mod users;

pub use twitch_oauth2::Scope;
#[derive(Clone)]
pub struct HelixClient {
    token: Arc<sync::RwLock<Box<dyn TwitchToken + Send + Sync>>>,
    client: reqwest::Client,
    // TODO: Implement rate limiter...
}

impl HelixClient {
    pub fn new<T>(token: Box<T>) -> HelixClient
    where T: TwitchToken + Sized + Send + Sync + 'static {
        let client = reqwest::Client::new();
        HelixClient {
            token: Arc::new(sync::RwLock::new(token)),
            client,
        }
    }

    pub fn clone_client(&self) -> reqwest::Client { self.client.clone() }

    pub async fn monitor_expire(&self) -> Option<tokio::time::Delay> {
        self.token()
            .await
            .as_ref()
            .expires()
            .map(tokio::time::Instant::from_std)
            .map(tokio::time::delay_until)
    }

    pub async fn token(&self) -> sync::RwLockReadGuard<'_, Box<dyn TwitchToken + Send + Sync>> {
        self.token.read().await
    }

    pub async fn refresh_token(&self) -> Result<(), twitch_oauth2::RefreshTokenError> {
        let mut token = self.token.write().await;
        token.as_mut().refresh_token().await?;
        Ok(())
    }

    // FIXME: allow multiple?
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

#[async_trait::async_trait]
pub trait Request: serde::Serialize {
    const PATH: &'static str;
    const SCOPE: &'static [twitch_oauth2::Scope];
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    type Response;
    fn query(&self) -> Result<String, serde_urlencoded::ser::Error> {
        serde_urlencoded::to_string(&self)
    }
}

pub trait RequestPut: Request {}

pub trait RequestGet: Request {}

#[derive(PartialEq, Debug)]
pub struct Response<R, D>
where R: Request<Response = D> {
    pub data: Vec<D>,
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    pub pagination: Pagination,
    pub request: R,
}

impl<R, D> Response<R, D>
where
    R: Request<Response = D> + Clone + Paginated,
    D: serde::de::DeserializeOwned,
{
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

pub trait Paginated {
    fn set_pagination(&mut self, cursor: Cursor);
}

#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Default)]
pub struct Pagination {
    #[serde(default)]
    cursor: Option<Cursor>,
}

pub type Cursor = String;

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

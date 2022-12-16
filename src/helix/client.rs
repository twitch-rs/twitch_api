//! Client for Helix endpoints
use crate::client::ResponseExt;

use super::*;

pub(crate) mod client_ext;
#[cfg(feature = "unsupported")]
mod custom;

#[cfg(feature = "client")]
impl<C: crate::HttpClient<'static> + crate::client::ClientDefault<'static>> Default
    for HelixClient<'static, C>
{
    fn default() -> Self { Self::new() }
}

/// Client for Helix or the [New Twitch API](https://dev.twitch.tv/docs/api)
///
/// Use [`HelixClient::new`] or [`HelixClient::with_client`] to create a new client.
///
/// ```rust
/// use twitch_api::HelixClient;
/// # pub mod reqwest {pub type Client = twitch_api::client::DummyHttpClient;}
/// let helix: HelixClient<reqwest::Client> = HelixClient::new();
/// ```
///
/// See [`req_get`](HelixClient::req_get) for [`GET`](RequestGet),
/// [`req_put`](HelixClient::req_put) for [`PUT`](RequestPut),
/// [`req_post`](HelixClient::req_post) for [`POST`](RequestPost),
/// [`req_patch`](HelixClient::req_patch) for [`PATCH`](RequestPatch) and
/// [`req_delete`](HelixClient::req_delete) for [`DELETE`](RequestDelete)
///
/// Most [clients][crate::HttpClient] will be able to use the `'static` lifetime
///
/// ```rust,no_run
/// # use twitch_api::{HelixClient}; pub mod reqwest {pub type Client = twitch_api::client::DummyHttpClient;}
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
/// # pub mod reqwest {pub type Client = twitch_api::client::DummyHttpClient;}
/// use twitch_api::helix::{users::User, HelixClient};
/// let client: HelixClient<'static, reqwest::Client> = HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
/// let user: Option<User> = client
///     .get_user_from_login("justintv", &token)
///     .await
///     .unwrap();
/// # Ok(()) }
/// ```
#[derive(Clone)]
#[cfg(all(feature = "client", feature = "helix"))] // this is needed due to a bug?
pub struct HelixClient<'a, C>
where C: crate::HttpClient<'a> {
    pub(crate) client: C,
    pub(crate) _pd: std::marker::PhantomData<&'a ()>, // TODO: Implement rate limiter...
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
    where C: crate::client::ClientDefault<'a> {
        let client = C::default_client();
        HelixClient::with_client(client)
    }

    /// Retrieve a clone of the [`HttpClient`][crate::HttpClient] inside this [`HelixClient`]
    pub fn clone_client(&self) -> C
    where C: Clone {
        self.client.clone()
    }

    /// Retrieve a reference of the [`HttpClient`][crate::HttpClient] inside this [`HelixClient`]
    pub fn get_client(&self) -> &C { &self.client }

    /// Request on a valid [`RequestGet`] endpoint
    ///
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// #   use twitch_api::helix::{HelixClient, channels};
    /// #   let token = Box::new(twitch_oauth2::UserToken::from_existing_unchecked(
    /// #       twitch_oauth2::AccessToken::new("totallyvalidtoken".to_string()), None,
    /// #       twitch_oauth2::ClientId::new("validclientid".to_string()), None, "justintv".into(), "1337".into(), None, None));
    /// let ids: &[&twitch_types::UserIdRef] = &["123456".into()];
    /// let req = channels::GetChannelInformationRequest::broadcaster_ids(ids);
    /// let client = HelixClient::new();
    /// # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
    ///
    /// let response = client.req_get(req, &token).await;
    /// # }
    /// # // fn main() {run()}
    /// ```
    pub async fn req_get<'req, R, T>(
        &'a self,
        request: R,
        token: &T,
    ) -> Result<
        Response<R, yoke::Yoke<R::Response, Vec<u8>>>,
        ClientRequestError<<C as crate::HttpClient<'a>>::Error>,
    >
    where
        R: Request + RequestGet,
        for<'y> R::Response: yoke::Yokeable<'y>,
        for<'y> yoke::trait_hack::YokeTraitHack<<R::Response as yoke::Yokeable<'y>>::Output>:
            serde::Deserialize<'y>,
        T: TwitchToken + ?Sized,
        C: Send,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?
            .into_response_vec()
            .await?;
        let (parts, body) = response.into_parts();
        let mut pagination = None;
        let mut request_opt = None;
        let mut total = None;
        let mut other = None;
        let resp: yoke::Yoke<_, _> = yoke::Yoke::try_attach_to_cart::<_, _>(
            body,
            |body| -> Result<_, ClientRequestError<<C as crate::HttpClient<'a>>::Error>> {
                let response = http::Response::from_parts(parts, body);
                let Response {
                    data,
                    pagination: pagination_inner,
                    request: request_inner,
                    total: total_inner,
                    other: other_inner,
                }: Response<_, _> = <R>::parse_response(Some(request), &uri, &response).unwrap();
                pagination = pagination_inner;
                request_opt = request_inner;
                total = total_inner;
                other = other_inner;
                Ok(data)
            },
        )?;

        Ok(Response {
            data: resp,
            pagination,
            request: request_opt,
            total,
            other,
        })
    }

    // /// Request on a valid [`RequestPost`] endpoint
    // pub async fn req_post<R, B, D, T>(
    //     &'a self,
    //     request: R,
    //     body: B,
    //     token: &T,
    // ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    // where
    //     R: Request<Response = D> + Request + RequestPost<Body = B>,
    //     B: HelixRequestBody,
    //     D: serde::de::DeserializeOwned + PartialEq,
    //     T: TwitchToken + ?Sized,
    // {
    //     let req =
    //         request.create_request(body, token.token().secret(), token.client_id().as_str())?;
    //     let uri = req.uri().clone();
    //     let response = self
    //         .client
    //         .req(req)
    //         .await
    //         .map_err(ClientRequestError::RequestError)?
    //         .into_response_vec()
    //         .await?;
    //     <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    // }

    // /// Request on a valid [`RequestPatch`] endpoint
    // pub async fn req_patch<R, B, D, T>(
    //     &'a self,
    //     request: R,
    //     body: B,
    //     token: &T,
    // ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    // where
    //     R: Request<Response = D> + Request + RequestPatch<Body = B>,
    //     B: HelixRequestBody,
    //     D: serde::de::DeserializeOwned + PartialEq,
    //     T: TwitchToken + ?Sized,
    // {
    //     let req =
    //         request.create_request(body, token.token().secret(), token.client_id().as_str())?;
    //     let uri = req.uri().clone();
    //     let response = self
    //         .client
    //         .req(req)
    //         .await
    //         .map_err(ClientRequestError::RequestError)?
    //         .into_response_vec()
    //         .await?;
    //     <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    // }

    // /// Request on a valid [`RequestDelete`] endpoint
    // pub async fn req_delete<R, D, T>(
    //     &'a self,
    //     request: R,
    //     token: &T,
    // ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    // where
    //     R: Request<Response = D> + Request + RequestDelete,
    //     D: serde::de::DeserializeOwned + PartialEq,
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
    //     let uri = req.uri().clone();
    //     let response = self
    //         .client
    //         .req(req)
    //         .await
    //         .map_err(ClientRequestError::RequestError)?
    //         .into_response_vec()
    //         .await?;
    //     <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    // }

    // /// Request on a valid [`RequestPut`] endpoint
    // pub async fn req_put<R, B, D, T>(
    //     &'a self,
    //     request: R,
    //     body: B,
    //     token: &T,
    // ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    // where
    //     R: Request<Response = D> + Request + RequestPut<Body = B>,
    //     B: HelixRequestBody,
    //     D: serde::de::DeserializeOwned + PartialEq,
    //     T: TwitchToken + ?Sized,
    // {
    //     let req =
    //         request.create_request(body, token.token().secret(), token.client_id().as_str())?;
    //     let uri = req.uri().clone();
    //     let response = self
    //         .client
    //         .req(req)
    //         .await
    //         .map_err(ClientRequestError::RequestError)?
    //         .into_response_vec()
    //         .await?;
    //     <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    // }
}

//! Different clients you can use with this crate to call the auth server.
//!
//! This is enables you to use your own http implementation.
//! For example, say you have a http client that has a "client" named `foo::Client`.
//!
//! Our client has a function `call` which looks something like this
//! ```rust,no_run
//! # struct Client;type ClientError = std::io::Error; impl Client {
//! fn call(&self, req: http::Request<Vec<u8>>) -> futures::future::BoxFuture<'static, Result<http::Response<Vec<u8>>, ClientError>> {
//! # stringify!(
//!     ...
//! # ); todo!()
//! }
//! # }
//! ```
//! To use that for requests we do the following.
//!
//! ```no_run
//! use twitch_api2::client::{BoxedFuture, Req, Response};
//! # mod foo { use twitch_api2::client::{BoxedFuture, Req, Response}; pub struct Client; impl Client{pub fn call(&self, req: http::Request<Vec<u8>>) -> futures::future::BoxFuture<'static, Result<http::Response<Vec<u8>>, ClientError>> {unimplemented!()}} pub type ClientError = std::io::Error;}
//! impl<'a> twitch_api2::HttpClient<'a> for foo::Client {
//!     type Error = foo::ClientError;
//!
//!     fn req(&'a self, request: Req) -> BoxedFuture<'static, Result<Response, Self::Error>> {
//!         let fut = self.call(request);
//!         Box::pin(async {fut.await})
//!     }
//! }
//! ```
//! We can then use it like usual.
//!
//!  ```rust,no_run
//! # use twitch_api2::client::{BoxedFuture, Req, Response};
//! # mod foo { use twitch_api2::client::{BoxedFuture, Req, Response}; pub struct Client; impl Client{pub fn call(&self, req: http::Request<Vec<u8>>) -> futures::future::BoxFuture<'static, Result<http::Response<Vec<u8>>, ClientError>> {unimplemented!()}} pub type ClientError = std::io::Error;}
//! # impl<'a> twitch_api2::HttpClient<'a> for foo::Client {
//! #     type Error = foo::ClientError;
//! #    fn req(&'a self, request: Req) -> BoxedFuture<'static, Result<Response, Self::Error>> {
//! #        let fut = self.call(request);
//! #        Box::pin(async {fut.await})
//! #    }
//! # }
//! # use twitch_api2::{TwitchClient};
//! pub struct MyStruct {
//!     twitch: TwitchClient<'static, foo::Client>,
//!     token: twitch_oauth2::AppAccessToken,
//! }
//!
//! ```
//!
//! If your client is from a remote crate, you can use [the newtype pattern](https://github.com/rust-unofficial/patterns/blob/607fcb00c4ecb9c6317e4e101e16dc15717758bd/patterns/newtype.md)
//!
//! Of course, sometimes the clients use different types for their responses and requests. but simply translate them into [http] types and it will work.
//!
//! See the source of this module for the implementation of [Client] for [surf](https://crates.io/crates/surf) and [reqwest](https://crates.io/crates/reqwest) if you need inspiration.
//!

use std::error::Error;
use std::future::Future;

/// A boxed future, mimics `futures::future::BoxFuture`
pub type BoxedFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// The request type we're expecting with body.
pub type Req = http::Request<Vec<u8>>;
/// The response type we're expecting with body
pub type Response = http::Response<Vec<u8>>;
/// A client that can do requests
pub trait Client<'a>: Send + 'a {
    /// Error returned by the client
    type Error: Error + Send + Sync + 'static;
    /// Send a request
    fn req(&'a self, request: Req) -> BoxedFuture<'a, Result<Response, <Self as Client>::Error>>;
}

// This makes errors very muddy, preferably we'd actually use rustc_on_unimplemented, but that is highly not recommended (and doesn't work 100% for me at least)
// impl<'a, F, R, E> Client<'a> for F
// where
//     F: Fn(Req) -> R + Send + Sync + 'a,
//     R: Future<Output = Result<Response, E>> + Send + Sync + 'a,
//     E: Error + Send + Sync + 'static,
// {
//     type Error = E;
//
//     fn req(&'a self, request: Req) -> BoxedFuture<'a, Result<Response, Self::Error>> {
//         Box::pin((self)(request))
//     }
// }

#[cfg(all(feature = "reqwest", feature = "client"))]
use reqwest::Client as ReqwestClient;

#[cfg(all(feature = "reqwest", feature = "client"))]
#[cfg_attr(nightly, doc(cfg(feature = "reqwest_client")))] // FIXME: This doc_cfg does nothing
impl<'a> Client<'a> for ReqwestClient {
    type Error = reqwest::Error;

    fn req(&'a self, request: Req) -> BoxedFuture<'static, Result<Response, Self::Error>> {
        // Reqwest plays really nice here and has a try_from on `http::Request` -> `reqwest::Request`
        use std::convert::TryFrom;
        let req = match reqwest::Request::try_from(request) {
            Ok(req) => req,
            Err(e) => return Box::pin(async { Err(e) }),
        };
        // We need to "call" the execute outside the async closure to not capture self.
        let fut = self.execute(req);
        Box::pin(async move {
            // Await the request and translate to `http::Response`
            let mut response = fut.await?;
            let mut result = http::Response::builder();
            let headers = result
                .headers_mut()
                // This should not fail, we just created the response.
                .expect("expected to get headers mut when building response");
            std::mem::swap(headers, response.headers_mut());
            let result = result.version(response.version());
            Ok(result
                .body(response.bytes().await?.as_ref().to_vec())
                .expect("mismatch reqwest -> http conversion should not fail"))
        })
    }
}

/// Possible errors from [Client::req()] when using [surf::Client]
#[cfg(all(feature = "surf", feature = "client"))]
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum SurfError {
    /// surf failed to do the request: {0}
    Surf(surf::Error),
    /// could not construct header value
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    /// could not construct header name
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    /// uri could not be translated into an url.
    UrlError(#[from] url::ParseError),
}
#[cfg(all(feature = "surf", feature = "client"))]
use surf::Client as SurfClient;

#[cfg(all(feature = "surf", feature = "client"))]
#[cfg_attr(nightly, doc(cfg(feature = "surf_client")))] // FIXME: This doc_cfg does nothing
impl<'a> Client<'a> for SurfClient {
    type Error = SurfError;

    fn req(&'a self, request: Req) -> BoxedFuture<'static, Result<Response, Self::Error>> {
        // First we translate the `http::Request` method and uri into types that surf understands.

        let method: surf::http::Method = request.method().clone().into();

        let url = match url::Url::parse(&request.uri().to_string()) {
            Ok(url) => url,
            Err(err) => return Box::pin(async move { Err(err.into()) }),
        };
        // Construct the request
        let mut req = surf::Request::new(method, url);

        // move the headers into the surf request
        for (name, value) in request.headers().iter() {
            let value =
                match surf::http::headers::HeaderValue::from_bytes(value.as_bytes().to_vec())
                    .map_err(SurfError::Surf)
                {
                    Ok(val) => val,
                    Err(err) => return Box::pin(async { Err(err) }),
                };
            req.append_header(name.as_str(), value);
        }

        // assembly the request, now we can send that to our `surf::Client`
        req.body_bytes(&request.body());

        let client = self.clone();
        Box::pin(async move {
            // Send the request and translate the response into a `http::Response`
            let mut response = client.send(req).await.map_err(SurfError::Surf)?;
            let mut result = http::Response::builder();

            let mut response_headers: http::header::HeaderMap = response
                .iter()
                .map(|(k, v)| {
                    Ok((
                        http::header::HeaderName::from_bytes(k.as_str().as_bytes())?,
                        http::HeaderValue::from_str(v.as_str())?,
                    ))
                })
                .collect::<Result<_, SurfError>>()?;

            let _ = std::mem::replace(&mut result.headers_mut(), Some(&mut response_headers));
            let result = if let Some(v) = response.version() {
                result.version(match v {
                    surf::http::Version::Http0_9 => http::Version::HTTP_09,
                    surf::http::Version::Http1_0 => http::Version::HTTP_10,
                    surf::http::Version::Http1_1 => http::Version::HTTP_11,
                    surf::http::Version::Http2_0 => http::Version::HTTP_2,
                    surf::http::Version::Http3_0 => http::Version::HTTP_3,
                    // TODO: Log this somewhere...
                    _ => http::Version::HTTP_3,
                })
            } else {
                result
            };
            Ok(result
                .body(response.body_bytes().await.map_err(SurfError::Surf)?)
                .expect("mismatch surf -> http conversion should not fail"))
        })
    }
}

#[derive(Debug, Default, thiserror::Error, Clone)]
/// A client that will never work, used to trick documentation tests
#[error("this client does not do anything, only used for documentation test that only checks")]
pub struct DummyHttpClient;

impl<'a> Client<'a> for DummyHttpClient {
    type Error = DummyHttpClient;

    fn req(&'a self, _: Req) -> BoxedFuture<'a, Result<Response, Self::Error>> {
        Box::pin(async { Err(DummyHttpClient) })
    }
}

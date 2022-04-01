//! Different clients you can use with this crate to call endpoints.
//!
//! This enables you to use your own http client/implementation.
//! For example, say you have a http client that has a "client" named `foo::Client`.
//!
//! That client has a function `call` which looks something like this
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
//! Of course, sometimes the clients use different types for their responses and requests. but simply translate them into [`http`] types and it will work.
//!
//! See the source of this module for the implementation of [`Client`] for [surf](https://crates.io/crates/surf) and [reqwest](https://crates.io/crates/reqwest) if you need inspiration.

use std::error::Error;
use std::future::Future;

#[cfg(feature = "ureq")]
mod ureq_impl;
#[cfg(feature = "ureq")]
pub use ureq_impl::UreqError;

#[cfg(feature = "surf")]
mod surf_impl;
#[cfg(feature = "surf")]
pub use surf_impl::SurfError;

#[cfg(feature = "reqwest")]
mod reqwest_impl;
#[cfg(feature = "reqwest")]
pub use reqwest_impl::ReqwestClientDefaultError;

/// The User-Agent `product` of this crate.
pub static TWITCH_API2_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// A boxed future, mimics `futures::future::BoxFuture`
pub type BoxedFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// The request type we're expecting with body.
pub type Request = http::Request<Vec<u8>>;
/// The response type we're expecting with body
pub type Response = http::Response<Vec<u8>>;
/// A client that can do requests
pub trait Client<'a>: Send + 'a {
    /// Error returned by the client
    type Error: Error + Send + Sync + 'static;
    /// Send a request
    fn req(&'a self, request: Request) -> BoxedFuture<'a, Result<Response, <Self as Client>::Error>>;
}

/// A specific client default for setting some sane defaults for API calls and oauth2 usage
pub trait ClientDefault<'a>: Clone + Sized {
    /// Errors that can happen when assembling the client
    type Error: std::error::Error + Send + Sync + 'static;
    /// Construct [`Self`] with sane defaults for API calls and oauth2.
    fn default_client() -> Self {
        Self::default_client_with_name(None)
            .expect("a new twitch_api2 client without an extra product should never fail")
    }

    /// Constructs [`Self`] with sane defaults for API calls and oauth2 and setting user-agent to include another product
    ///
    /// Specifically, one should
    ///
    /// * Set User-Agent to `{product} twitch_api2/{version_of_twitch_api2}` (According to RFC7231)
    ///   See [`TWITCH_API2_USER_AGENT`] for the product of this crate
    /// * Disallow redirects
    ///
    /// # Notes
    ///
    /// When the product name is none, this function should never fail. This should be ensured with tests.
    fn default_client_with_name(product: Option<http::HeaderValue>) -> Result<Self, Self::Error>;
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

#[derive(Debug, Default, thiserror::Error, Clone)]
/// A client that will never work, used to trick documentation tests
#[error("this client does not do anything, only used for documentation test that only checks")]
pub struct DummyHttpClient;

impl<'a> Client<'a> for DummyHttpClient {
    type Error = DummyHttpClient;

    fn req(&'a self, _: Request) -> BoxedFuture<'a, Result<Response, Self::Error>> {
        Box::pin(async { Err(DummyHttpClient) })
    }
}

impl<'a> Client<'a> for twitch_oauth2::client::DummyClient {
    type Error = twitch_oauth2::client::DummyClient;

    fn req(&'a self, _: Request) -> BoxedFuture<'a, Result<Response, Self::Error>> {
        Box::pin(async { Err(twitch_oauth2::client::DummyClient) })
    }
}

#[cfg(feature = "surf")]
impl ClientDefault<'static> for DummyHttpClient
where Self: Default
{
    type Error = DummyHttpClient;

    fn default_client_with_name(_: Option<http::HeaderValue>) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

#[cfg(feature = "helix")]
impl<'a, C: Client<'a> + Sync> twitch_oauth2::client::Client<'a> for crate::HelixClient<'a, C> {
    type Error = <C as Client<'a>>::Error;

    fn req(
        &'a self,
        request: Request,
    ) -> BoxedFuture<'a, Result<Response, <Self as twitch_oauth2::client::Client>::Error>> {
        self.get_client().req(request)
    }
}

#[cfg(feature = "tmi")]
impl<'a, C: Client<'a> + Sync> twitch_oauth2::client::Client<'a> for crate::TmiClient<'a, C> {
    type Error = <C as Client<'a>>::Error;

    fn req(
        &'a self,
        request: Request,
    ) -> BoxedFuture<'a, Result<Response, <Self as twitch_oauth2::client::Client>::Error>> {
        self.get_client().req(request)
    }
}

#[cfg(any(feature = "tmi", feature = "helix"))]
impl<'a, C: Client<'a> + Sync> twitch_oauth2::client::Client<'a> for crate::TwitchClient<'a, C> {
    type Error = <C as Client<'a>>::Error;

    fn req(
        &'a self,
        request: Request,
    ) -> BoxedFuture<'a, Result<Response, <Self as twitch_oauth2::client::Client>::Error>> {
        self.get_client().req(request)
    }
}

/// Gives the User-Agent header value for a client annotated with an added `twitch_api2` product
pub fn user_agent(
    product: Option<http::HeaderValue>,
) -> Result<http::HeaderValue, http::header::InvalidHeaderValue> {
    use std::convert::TryInto;

    if let Some(product) = product {
        let mut user_agent = product.as_bytes().to_owned();
        user_agent.push(b' ');
        user_agent.extend(TWITCH_API2_USER_AGENT.as_bytes());
        user_agent.as_slice().try_into()
    } else {
        http::HeaderValue::from_str(TWITCH_API2_USER_AGENT)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "surf_client")]
    fn surf() {
        use super::ClientDefault;
        use std::convert::TryInto;

        super::SurfClient::default_client_with_name(Some("test/123".try_into().unwrap())).unwrap();
        super::SurfClient::default_client();
    }

    #[test]
    #[cfg(feature = "reqwest_client")]
    fn reqwest() {
        use super::ClientDefault;
        use std::convert::TryInto;

        super::ReqwestClient::default_client_with_name(Some("test/123".try_into().unwrap()))
            .unwrap();
        super::ReqwestClient::default_client();
    }
}

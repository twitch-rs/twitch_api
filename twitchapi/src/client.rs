//! Different clients you can use with this crate.
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

//impl<'a, F, R, E> Client<'a> for F
//where
//    F: Fn(Req) -> R + Send + Sync + 'a,
//    R: Future<Output = Result<Response,E>> + Send + Sync + 'a,
//    E: Error + Send + Sync + 'static,
//{
//    type Error = E;
//
//    fn req(&'a self, request: Req) -> BoxedFuture<'a, Result<Response,Self::Error>> {
//        Box::pin((self)(request))
//    }
//}

#[cfg(feature = "reqwest")]
impl Client<'a> for reqwest::Client {
    type Error = reqwest::Error;

    fn req(&'a self, request: Req) -> BoxedFuture<'a, Result<Response, Self::Error>> {
        use std::convert::TryFrom;
        let req = match reqwest::Request::try_from(request) {
            Ok(req) => req,
            Err(e) => return Box::pin(async { Err(e) }),
        };
        Box::pin(async move {
            let mut response = self.execute(req).await?;
            let mut result = http::Response::builder();
            let headers = result
                .headers_mut()
                .expect("expected to get headers mut when building response");
            std::mem::swap(headers, response.headers_mut());
            let result = result.version(response.version());
            Ok(result
                .body(response.bytes().await?.as_ref().to_vec())
                .expect("mismatch reqwest -> http conversion should not fail"))
        })
    }
}

/// Possible errors from [surf_http_client]
#[cfg(feature = "surf")]
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

#[cfg(all(feature = "surf", feature = "http-types"))]
impl Client<'a> for surf::Client {
    type Error = SurfError;

    fn req(&'a self, request: Req) -> BoxedFuture<'a, Result<Response, Self::Error>> {
        let method: surf::http::Method = request.method().clone().into();
        let url = match url::Url::parse(&request.uri().to_string()) {
            Ok(url) => url,
            Err(err) => return Box::pin(async move { Err(err.into()) }),
        };
        let mut req = surf::Request::new(method, url);

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

        req.body_bytes(&request.body());

        Box::pin(async move {
            let mut response = self.send(req).await.map_err(SurfError::Surf)?;
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
                .expect("mismatch reqwest -> http conversion should not fail"))
        })
    }
}

#[doc(hidden)]
#[derive(Debug, Default, thiserror::Error, Clone)]
/// A client that will never work, used to trick documentation tests
#[error("this client does not do anything, only used for documentation test that only checks")]
pub struct DummyHttpClient;

impl Client<'a> for DummyHttpClient {
    type Error = DummyHttpClient;

    fn req(&'a self, _: Req) -> BoxedFuture<'a, Result<Response, Self::Error>> {
        Box::pin(async { Err(DummyHttpClient) })
    }
}

use super::*;

use ureq::Agent as UreqAgent;

/// Possible errors from [`Client::req()`] when using the [ureq](https://crates.io/crates/ureq) client
///
/// Also returned by [`ClientDefault::default_client_with_name`]
#[cfg_attr(nightly, doc(cfg(feature = "ureq_client")))]
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum UreqError {
    /// Ureq failed to do the request
    Ureq(#[from] ureq::Error),
    /// Http failed
    Http(#[from] http::Error),
    /// The response could not be collected
    Io(#[from] std::io::Error),
    /// could not construct header value
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    /// could not construct header name
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    /// uri could not be translated into an url.
    UrlError(#[from] url::ParseError),
}

#[cfg_attr(nightly, doc(cfg(feature = "ureq_client")))] // FIXME: This doc_cfg does nothing
impl<'a> Client<'a> for UreqAgent {
    type Error = UreqError;

    fn req(&'a self, request: Request) -> BoxedFuture<'static, Result<Response, Self::Error>> {
        use std::io::Read;

        let method = request.method().to_string();
        let url = request.uri().to_string();
        let mut req = self.request(&method, &url);

        for (header, value) in request.headers() {
            if let Ok(value) = value.to_str() {
                req = req.set(header.as_str(), value);
            }
        }
        Box::pin(async move {
            let body = request.into_body();
            let response = match req.send_bytes(&body).map_err(UreqError::Ureq) {
                Ok(val) => val,
                Err(err) => return Err(err),
            };

            let mut result = http::Response::builder().status(response.status());
            let headers = result
                .headers_mut()
                // This should not fail, we just created the response.
                .expect("expected to get headers mut when building response");
            for name in response.headers_names() {
                if let Some(value) = response.header(&name) {
                    let value = match http::header::HeaderValue::from_bytes(value.as_bytes())
                        .map_err(UreqError::InvalidHeaderValue)
                    {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                    };
                    let header = match http::header::HeaderName::from_bytes(name.as_bytes())
                        .map_err(UreqError::InvalidHeaderName)
                    {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                    };
                    headers.append(header, value);
                }
            }
            result = result.version(match response.http_version() {
                "HTTP/0.9" => http::Version::HTTP_09,
                "HTTP/1.0" => http::Version::HTTP_10,
                "HTTP/1.1" => http::Version::HTTP_11,
                "HTTP/2.0" => http::Version::HTTP_2,
                "HTTP/3.0" => http::Version::HTTP_3,
                // TODO: Log this somewhere...
                _ => http::Version::HTTP_11,
            });
            match response
                .into_reader()
                .take(10_000_000)
                .bytes()
                .collect()
            {
                Ok(v) => result.body(v).map_err(Into::into),
                Err(e) => Err(e.into()),
            }
        })
    }
}

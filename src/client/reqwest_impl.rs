use super::*;

use reqwest::Client as ReqwestClient;

#[cfg_attr(nightly, doc(cfg(feature = "reqwest_client")))] // FIXME: This doc_cfg does nothing
impl<'a> Client<'a> for ReqwestClient {
    type Error = reqwest::Error;

    fn req(&'a self, request: Request) -> BoxedFuture<'static, Result<Response, Self::Error>> {
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
            let mut result = http::Response::builder().status(response.status());
            let headers = result
                .headers_mut()
                // This should not fail, we just created the response.
                .expect("expected to get headers mut when building response");
            std::mem::swap(headers, response.headers_mut());
            let result = result.version(response.version());
            Ok(result
                .body(response.bytes().await?.into())
                .expect("mismatch reqwest -> http conversion should not fail"))
        })
    }
}

/// Possible errors from [`ClientDefault::default_client_with_name`] for [reqwest](https://crates.io/crates/reqwest)
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum ReqwestClientDefaultError {
    /// could not construct header value for User-Agent
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    /// reqwest returned an error
    ReqwestError(#[from] reqwest::Error),
}

impl ClientDefault<'static> for ReqwestClient {
    type Error = ReqwestClientDefaultError;

    fn default_client_with_name(product: Option<http::HeaderValue>) -> Result<Self, Self::Error> {
        let builder = Self::builder();
        let user_agent = user_agent(product)?;
        let builder = builder.user_agent(user_agent);
        let builder = builder.redirect(reqwest::redirect::Policy::none());
        builder.build().map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reqwest() {
        use super::ClientDefault;
        use std::convert::TryInto;

        super::ReqwestClient::default_client_with_name(Some("test/123".try_into().unwrap()))
            .unwrap();
        super::ReqwestClient::default_client();
    }
}

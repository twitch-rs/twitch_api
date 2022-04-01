use super::*;
/// Possible errors from [`Client::req()`] when using the [surf](https://crates.io/crates/surf) client
///
/// Also returned by [`ClientDefault::default_client_with_name`]
#[cfg_attr(nightly, doc(cfg(feature = "surf_client")))]
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

use surf::Client as SurfClient;

#[cfg_attr(nightly, doc(cfg(feature = "surf_client")))] // FIXME: This doc_cfg does nothing
impl<'a> Client<'a> for SurfClient {
    type Error = SurfError;

    fn req(&'a self, request: Request) -> BoxedFuture<'static, Result<Response, Self::Error>> {
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

        let client = self.clone();
        Box::pin(async move {
            // assembly the request, now we can send that to our `surf::Client`
            req.body_bytes(request.into_body());
            // Send the request and translate the response into a `http::Response`
            let mut response = client.send(req).await.map_err(SurfError::Surf)?;
            let mut result = http::Response::builder().status(response.status());

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
                    _ => http::Version::HTTP_11,
                })
            } else {
                result
            };
            Ok(result
                .body(
                    response
                        .take_body()
                        .into_bytes()
                        .await
                        .map_err(SurfError::Surf)
                        .map(Into::into)?,
                )
                .expect("mismatch surf -> http conversion should not fail"))
        })
    }
}

/// Possible errors from [`ClientDefault::default_client_with_name`] for [surf](https://crates.io/crates/surf)
#[cfg_attr(nightly, doc(cfg(feature = "surf_client")))]
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum SurfClientDefaultError {
    /// surf returned an error: {0}
    SurfError(surf::Error),
}

impl ClientDefault<'static> for SurfClient
where Self: Default
{
    type Error = SurfClientDefaultError;

    fn default_client_with_name(product: Option<http::HeaderValue>) -> Result<Self, Self::Error> {
        use std::str::FromStr as _;

        struct SurfAgentMiddleware {
            user_agent: surf::http::headers::HeaderValue,
        }

        #[async_trait::async_trait]
        impl surf::middleware::Middleware for SurfAgentMiddleware {
            async fn handle(
                &self,
                req: surf::Request,
                client: SurfClient,
                next: surf::middleware::Next<'_>,
            ) -> surf::Result<surf::Response> {
                let mut req = req;
                // if let Some(header) = req.header_mut(surf::http::headers::USER_AGENT) {
                //     let mut user_agent = self.user_agent.as_str().as_bytes().to_owned();
                //     user_agent.push(b' ');
                //     user_agent.extend(header.as_str().as_bytes());
                //     req.set_header(
                //         surf::http::headers::USER_AGENT,
                //         surf::http::headers::HeaderValue::from_bytes(user_agent).expect(
                //             "product User-Agent + existing User-Agent is expected to be valid ASCII",
                //         ),
                //     );
                // } else {
                req.set_header(surf::http::headers::USER_AGENT, self.user_agent.clone());
                // }
                next.run(req, client).await
            }
        }

        let client = surf::Client::default();
        let user_agent = if let Some(product) = product {
            let mut user_agent = product.as_bytes().to_owned();
            user_agent.push(b' ');
            user_agent.extend(TWITCH_API2_USER_AGENT.as_bytes());
            surf::http::headers::HeaderValue::from_bytes(user_agent)
                .map_err(SurfClientDefaultError::SurfError)?
        } else {
            surf::http::headers::HeaderValue::from_str(TWITCH_API2_USER_AGENT)
                .map_err(SurfClientDefaultError::SurfError)?
        };
        let middleware = SurfAgentMiddleware { user_agent };
        Ok(client.with(middleware))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn surf() {
        use super::ClientDefault;
        use std::convert::TryInto;

        super::SurfClient::default_client_with_name(Some("test/123".try_into().unwrap())).unwrap();
        super::SurfClient::default_client();
    }
}

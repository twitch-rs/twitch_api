//! Provides different http clients
#[doc(inline)]
#[cfg(feature = "reqwest_client")]
#[cfg_attr(nightly, doc(cfg(feature = "reqwest_client")))]
pub use oauth2::reqwest::async_http_client as reqwest_http_client;

#[doc(inline)]
#[cfg(feature = "surf_client")]
pub use surf_client::http_client as surf_http_client;

#[doc(inline)]
#[cfg(feature = "surf_client")]
pub use surf_client::Error as SurfError;

#[cfg(feature = "surf_client")]
mod surf_client {
    use oauth2::{HttpRequest, HttpResponse};

    /// Possible errors for [surf_http_client][http_client]
    #[derive(Debug, displaydoc::Display, thiserror::Error)]
    pub enum Error {
        /// surf failed to do the request: {0}
        Surf(surf::Error),
        /// could not construct header value
        InvalidHeaderValue(#[from] oauth2::http::header::InvalidHeaderValue),
        /// could not construct header name
        InvalidHeaderName(#[from] oauth2::http::header::InvalidHeaderName),
    }

    ///  Asynchronous HTTP client using [Surf][surf::Client]
    #[cfg_attr(nightly, doc(cfg(feature = "surf_client")))]
    pub async fn http_client(request: HttpRequest) -> Result<HttpResponse, Error> {
        let client = surf::Client::new();
        let method: http_types::Method = request.method.into();
        let mut req = surf::Request::new(method, request.url);

        for (name, value) in &request.headers {
            let value = surf::http::headers::HeaderValue::from_bytes(value.as_bytes().to_vec())
                .map_err(Error::Surf)?;
            req.append_header(name.as_str(), value);
        }

        req.body_bytes(&request.body);

        let mut response = client.send(req).await.map_err(Error::Surf)?;
        let headers = response
            .iter()
            .map(|(k, v)| {
                Ok((
                    oauth2::http::header::HeaderName::from_bytes(k.as_str().as_bytes())?,
                    oauth2::http::HeaderValue::from_str(v.as_str())?,
                ))
            })
            .collect::<Result<_, Error>>()?;
        Ok(HttpResponse {
            body: response.body_bytes().await.map_err(Error::Surf)?,
            status_code: response.status().into(),
            headers,
        })
    }
}

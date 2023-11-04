use super::*;

use tower_service::Service;

/// A wrapped [tower](tower_service::Service) service
pub struct TowerService<S, ReqBody>(pub S, std::marker::PhantomData<fn(ReqBody)>);

#[derive(Debug, thiserror::Error, displaydoc::Display)]
#[non_exhaustive]
/// Errors that can occur when using a [`TowerService`]
pub enum TowerError {
    /// service errored
    ServiceError(#[source] Box<dyn std::error::Error + Send + Sync>),
    /// couldn't create body from service response
    BodyError(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl<S, ReqBody, ResBody> Client for TowerService<S, ReqBody>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>>,
    S: Clone + Send + Sync + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    S::Future: Future<Output = Result<http::Response<ResBody>, S::Error>> + Send + 'static,
    ResBody: hyper::body::HttpBody + Sync + Send + 'static,
    ResBody::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    ResBody::Data: Sync + Send,
    ReqBody: From<hyper::body::Bytes> + Sync + Send + 'static,
{
    type Error = TowerError;

    fn req(&self, request: Request) -> BoxedFuture<'static, Result<Response, Self::Error>> {
        let mut service = self.0.clone();
        Box::pin(async move {
            futures::future::poll_fn(|cx| service.poll_ready(cx))
                .await
                .map_err(|e| TowerError::ServiceError(e.into()))?;

            let fut = service.call(request.map(|b| b.into()));
            let (parts, body) = fut
                .await
                .map_err(|e| TowerError::ServiceError(e.into()))?
                .into_parts();

            let b = hyper::body::to_bytes(body)
                .await
                .map_err(|e| TowerError::BodyError(e.into()))?;

            Ok(http::Response::from_parts(parts, b.into()))
        })
    }
}

impl<S, ReqBody> TowerService<S, ReqBody> {
    // TODO: needs tait: https://github.com/rust-lang/rust/issues/63063
    /// Create a new wrapped tower service
    ///
    /// # Notes
    ///
    /// Make sure your service stack can be cloned, one easy way to ensure this is to use [`tower::buffer`](https://docs.rs/tower/*/tower/builder/struct.ServiceBuilder.html#method.buffer)
    pub fn new(s: S) -> Self
    where S: Clone {
        TowerService(s, std::marker::PhantomData)
    }
}

impl<S: Default, ReqBody> Default for TowerService<S, ReqBody> {
    fn default() -> Self { TowerService(<_>::default(), std::marker::PhantomData) }
}

impl<S: Clone, ReqBody> Clone for TowerService<S, ReqBody> {
    fn clone(&self) -> Self { TowerService(self.0.clone(), std::marker::PhantomData) }
}

//! Deletes a specified conduit.
//! [`delete-conduit`](https://dev.twitch.tv/docs/api/reference/#delete-conduit)
//!
//! Note that it may take some time for Eventsub subscriptions on a deleted conduit to show as disabled when calling [Get Eventsub Subscriptions](super::get_eventsub_subscriptions).

use super::*;
use helix::RequestDelete;

/// Query Parameters for [Delete Conduit](super::delete_conduit)
///
/// [`delete-conduit`](https://dev.twitch.tv/docs/api/reference/#delete-conduit)
#[derive(PartialEq, Eq, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct DeleteConduitRequest<'a> {
    /// Conduit ID.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub id: Cow<'a, types::ConduitIdRef>,
}

impl<'a> DeleteConduitRequest<'a> {
    /// Delete a specific conduit
    pub fn new(id: impl types::IntoCow<'a, types::ConduitIdRef> + 'a) -> Self {
        Self { id: id.into_cow() }
    }
}

/// Return Values for [Delete Conduit](super::delete_conduit)
///
/// [`delete-conduit`](https://dev.twitch.tv/docs/api/reference/#delete-conduit)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum DeleteConduitResponse {
    /// 204 - Conduit deleted
    Success,
}

impl Request for DeleteConduitRequest<'_> {
    type PaginationData = ();
    type Response = DeleteConduitResponse;

    const PATH: &'static str = "eventsub/conduits";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestDelete for DeleteConduitRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(helix::Response::with_data(
                DeleteConduitResponse::Success,
                request,
            )),
            _ => Err(helix::HelixRequestDeleteError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_uri() {
    use helix::*;
    let req = DeleteConduitRequest::new("bfcfc993-26b1-b876-44d9-afe75a379dac");

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/conduits?id=bfcfc993-26b1-b876-44d9-afe75a379dac"
    );
}

#[cfg(test)]
#[test]
fn test_successful_response() {
    use helix::*;
    let req = DeleteConduitRequest::new("bfcfc993-26b1-b876-44d9-afe75a379dac");

    let http_response = http::Response::builder().status(204).body(vec![]).unwrap();

    let uri = req.get_uri().unwrap();
    let response = DeleteConduitRequest::parse_response(Some(req), &uri, http_response).unwrap();

    assert_eq!(response.data, DeleteConduitResponse::Success);
}

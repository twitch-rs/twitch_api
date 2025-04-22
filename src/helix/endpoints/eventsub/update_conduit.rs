//!  Updates a conduit’s shard count.
//! [`update-conduit`](https://dev.twitch.tv/docs/api/reference/#update-conduits)
//!
//! To delete shards, update the count to a lower number, and the shards above the count will be deleted.
//! For example, if the existing shard count is 100, by resetting shard count to 50, shards 50-99 are disabled.

use super::*;
use crate::eventsub;
use helix::RequestPatch;

/// Query Parameters for [Update Conduit](super::update_conduit)
///
/// [`update-conduit`](https://dev.twitch.tv/docs/api/reference/#update-conduits)
#[derive(PartialEq, Eq, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UpdateConduitRequest<'a> {
    #[serde(skip)]
    #[cfg_attr(feature = "typed-builder", builder(default))]
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl Request for UpdateConduitRequest<'_> {
    type Response = eventsub::Conduit;

    const PATH: &'static str = "eventsub/conduits";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

/// Body Parameters for [Update Conduit](super::update_conduit)
///
/// [`update-conduit`](https://dev.twitch.tv/docs/api/reference/#update-conduits)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateConduitBody<'a> {
    /// Conduit ID.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub id: Cow<'a, types::ConduitIdRef>,
    /// The new number of shards for this conduit.
    pub shard_count: usize,
}

impl helix::private::SealedSerialize for UpdateConduitBody<'_> {}

impl<'a> UpdateConduitBody<'a> {
    /// Conduit body settings
    pub fn new(id: impl types::IntoCow<'a, types::ConduitIdRef> + 'a, shard_count: usize) -> Self {
        Self {
            id: id.into_cow(),
            shard_count,
        }
    }
}

impl<'a> RequestPatch for UpdateConduitRequest<'a> {
    type Body = UpdateConduitBody<'a>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPatchError>
    where
        Self: Sized,
    {
        helix::parse_single_return(request, uri, response, status)
    }
}

#[cfg(test)]
#[test]
fn test_successful_response() {
    helix::assert_helix_snapshot!(
        UpdateConduitRequest:
        req = UpdateConduitRequest::default(),
        body = UpdateConduitBody::new("bfcfc993-26b1-b876-44d9-afe75a379dac", 5),
        res = br#"{
            "data": [
                {
                    "id": "bfcfc993-26b1-b876-44d9-afe75a379dac",
                    "shard_count": 5
                }
            ]
        }"#,
        @r#"
    uri
    ----
    https://api.twitch.tv/helix/eventsub/conduits?

    body
    ----
    {"id":"bfcfc993-26b1-b876-44d9-afe75a379dac","shard_count":5}

    response
    ----
    Response {
        data: Conduit {
            id: "bfcfc993-26b1-b876-44d9-afe75a379dac",
            shard_count: 5,
        },
        pagination: None,
        request: Some(
            UpdateConduitRequest {
                _phantom: PhantomData<&()>,
            },
        ),
        total: None,
        other: None,
    }
    "#,
    );
}

#[cfg(test)]
#[test]
fn test_successful_unexpected_response() {
    use helix::*;
    let req: UpdateConduitRequest = UpdateConduitRequest::default();

    let data = br#"{
      "data": []
    }
    "#
    .to_vec();
    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    let response = UpdateConduitRequest::parse_response(Some(req), &uri, http_response);
    assert!(response.is_err());
}

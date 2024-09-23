//! Creates a new conduit for your Client.
//! [`create-conduit`](https://dev.twitch.tv/docs/api/reference/#create-conduits)

use super::*;
use crate::eventsub;
use helix::RequestPost;

/// Query Parameters for [Create Conduit](super::create_conduit)
///
/// [`create-conduit`](https://dev.twitch.tv/docs/api/reference/#create-conduits)
#[derive(PartialEq, Eq, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct CreateConduitRequest {}

impl Request for CreateConduitRequest {
    type Response = eventsub::Conduit;

    const PATH: &'static str = "eventsub/conduits";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

/// Body Parameters for [Create Conduit](super::create_conduit)
///
/// [`create-conduit`](https://dev.twitch.tv/docs/api/reference/#create-conduits)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreateConduitBody {
    /// The number of shards to create for this conduit.
    pub shard_count: usize,
}

impl CreateConduitBody {
    /// Conduit body settings
    pub fn new(shard_count: usize) -> Self { Self { shard_count } }
}

impl helix::private::SealedSerialize for CreateConduitBody {}

impl RequestPost for CreateConduitRequest {
    type Body = CreateConduitBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct InnerResponse {
            data: [eventsub::Conduit; 1],
        }

        let inner_response: InnerResponse = helix::parse_json(response, true).map_err(|e| {
            helix::HelixRequestPostError::DeserializeError(
                response.to_string(),
                e,
                uri.clone(),
                status,
            )
        })?;

        let [conduit] = inner_response.data;

        Ok(helix::Response::new(conduit, None, request, None, None))
    }
}

#[cfg(test)]
#[test]
fn test_uri() {
    use helix::*;
    let req: CreateConduitRequest = CreateConduitRequest::default();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/conduits?"
    );
}

#[cfg(test)]
#[test]
fn test_successful_response() {
    use helix::*;
    let req: CreateConduitRequest = CreateConduitRequest::default();

    let data = br#"{
      "data": [
        {
          "id": "bfcfc993-26b1-b876-44d9-afe75a379dac",
          "shard_count": 5
        }
      ]
    }
    "#
    .to_vec();
    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    let response = CreateConduitRequest::parse_response(Some(req), &uri, http_response).unwrap();

    assert_eq!(
        response.data,
        crate::eventsub::Conduit {
            id: "bfcfc993-26b1-b876-44d9-afe75a379dac".to_string(),
            shard_count: 5,
        },
    );

    dbg!("{:#?}", response);
}

#[cfg(test)]
#[test]
fn test_successful_unexpected_response() {
    use helix::*;
    let req: CreateConduitRequest = CreateConduitRequest::default();

    let data = br#"{
      "data": []
    }
    "#
    .to_vec();
    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    let response = CreateConduitRequest::parse_response(Some(req), &uri, http_response);
    assert!(response.is_err());
}

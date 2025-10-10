//! Get the conduits for your Client.
//! [`get-conduits`](https://dev.twitch.tv/docs/api/reference/#get-conduits)

use super::*;
use crate::eventsub;
use helix::RequestGet;

/// Query Parameters for [Get Conduits](super::get_conduits)
///
/// [`get-conduits`](https://dev.twitch.tv/docs/api/reference/#get-conduits)
#[derive(PartialEq, Eq, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetConduitsRequest {}

impl Request for GetConduitsRequest {
    type PaginationData = ();
    type Response = Vec<eventsub::Conduit>;

    const PATH: &'static str = "eventsub/conduits";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetConduitsRequest {}

#[cfg(test)]
#[test]
fn test_uri() {
    use helix::*;
    let req: GetConduitsRequest = GetConduitsRequest::default();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/conduits?"
    );
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req: GetConduitsRequest = GetConduitsRequest::default();

    let data = br#"{
      "data": [
        {
          "id": "26b1c993-bfcf-44d9-b876-379dacafe75a",
          "shard_count": 15
        },
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
    let response = GetConduitsRequest::parse_response(Some(req), &uri, http_response).unwrap();

    assert_eq!(
        response.data,
        vec![
            crate::eventsub::Conduit {
                id: "26b1c993-bfcf-44d9-b876-379dacafe75a".into(),
                shard_count: 15,
            },
            crate::eventsub::Conduit {
                id: "bfcfc993-26b1-b876-44d9-afe75a379dac".into(),
                shard_count: 5,
            },
        ]
    );

    dbg!("{:#?}", response);
}

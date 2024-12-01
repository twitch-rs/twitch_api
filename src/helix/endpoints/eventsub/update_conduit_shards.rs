//! Updates shard(s) for a [conduit](https://dev.twitch.tv/docs/eventsub/handling-conduit-events).
//! [`update-conduit-shards`](https://dev.twitch.tv/docs/api/reference/#update-conduit-shards)

use super::*;
use crate::eventsub;
use helix::RequestPatch;

/// Query Parameters for [Update Conduit Shards](super::update_conduit_shards)
///
/// [`update-conduit-shards`](https://dev.twitch.tv/docs/api/reference/#update-conduit-shards)
#[derive(PartialEq, Eq, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct UpdateConduitShardsRequest<'a> {
    #[serde(skip)]
    #[cfg_attr(feature = "typed-builder", builder(default))]
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl Request for UpdateConduitShardsRequest<'_> {
    type Response = UpdateConduitShardsResponse;

    const PATH: &'static str = "eventsub/conduits/shards";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

/// The structured response for [Update Conduit Shards](super::update_conduit_shards)
///
/// [`update-conduit-shards`](https://dev.twitch.tv/docs/api/reference/#update-conduit-shards)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct UpdateConduitShardsResponse {
    /// List of successful shard updates.
    pub shards: Vec<eventsub::ShardResponse>,

    /// List of unsuccessful updates.
    pub errors: Vec<eventsub::ShardError>,
}

/// Body Parameters for [Update Conduit Shards](super::update_conduit_shards)
///
/// [`update-conduit-shards`](https://dev.twitch.tv/docs/api/reference/#update-conduit-shards)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct UpdateConduitShardsBody<'a> {
    /// Conduit ID.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub conduit_id: Cow<'a, types::ConduitIdRef>,

    /// List of shards to update.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub shards: Cow<'a, [eventsub::Shard]>,
}

impl<'a> UpdateConduitShardsBody<'a> {
    /// Conduit body settings
    pub fn new(
        conduit_id: impl types::IntoCow<'a, types::ConduitIdRef> + 'a,
        shards: impl Into<Cow<'a, [eventsub::Shard]>>,
    ) -> Self {
        Self {
            conduit_id: conduit_id.into_cow(),
            shards: shards.into(),
        }
    }
}

impl helix::private::SealedSerialize for UpdateConduitShardsBody<'_> {}

impl<'a> RequestPatch for UpdateConduitShardsRequest<'a> {
    type Body = UpdateConduitShardsBody<'a>;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPatchError>
    where
        Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct InnerResponse {
            data: Vec<eventsub::ShardResponse>,
            errors: Vec<eventsub::ShardError>,
        }

        let inner_response: InnerResponse = helix::parse_json(response, true).map_err(|e| {
            helix::HelixRequestPatchError::DeserializeError(
                response.to_string(),
                e,
                uri.clone(),
                status,
            )
        })?;

        Ok(helix::Response::new(
            UpdateConduitShardsResponse {
                shards: inner_response.data,
                errors: inner_response.errors,
            },
            None,
            request,
            None,
            None,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_uri() {
    use helix::*;
    let req: UpdateConduitShardsRequest = UpdateConduitShardsRequest::default();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/conduits/shards?"
    );
}

#[cfg(test)]
#[test]
fn test_successful_response() {
    use helix::*;
    let req: UpdateConduitShardsRequest = UpdateConduitShardsRequest::default();

    let data = br#"{
  "data": [
    {
      "id": "0",
      "status": "enabled",
      "transport": {
        "method": "webhook",
        "callback": "https://this-is-a-callback.com"
      }
    },
    {
      "id": "1",
      "status": "webhook_callback_verification_pending",
      "transport": {
        "method": "webhook",
        "callback": "https://this-is-a-callback-2.com"
      }
    }
  ],
  "errors": [
    {
      "id": "3",
      "message": "The shard id is outside the conduit's range",
      "code": "invalid_parameter"
    }
  ]
}"#
    .to_vec();
    let http_response = http::Response::builder().status(202).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    let response =
        UpdateConduitShardsRequest::parse_response(Some(req), &uri, http_response).unwrap();

    assert_eq!(
        response.data.shards,
        vec![
            crate::eventsub::ShardResponse {
                id: "0".into(),
                status: crate::eventsub::ShardStatus::Enabled,
                transport: crate::eventsub::TransportResponse::Webhook(
                    crate::eventsub::WebhookTransportResponse {
                        callback: "https://this-is-a-callback.com".to_string(),
                    }
                ),
            },
            crate::eventsub::ShardResponse {
                id: "1".into(),
                status: crate::eventsub::ShardStatus::WebhookCallbackVerificationPending,
                transport: crate::eventsub::TransportResponse::Webhook(
                    crate::eventsub::WebhookTransportResponse {
                        callback: "https://this-is-a-callback-2.com".to_string(),
                    }
                ),
            }
        ]
    );

    assert_eq!(
        response.data.errors,
        vec![crate::eventsub::ShardError {
            id: "3".into(),
            message: "The shard id is outside the conduit's range".to_string(),
            code: "invalid_parameter".to_string(),
        },]
    );

    dbg!("{:#?}", response);
}

#[cfg(test)]
#[test]
fn test_successful_unexpected_response() {
    use helix::*;
    let req: UpdateConduitShardsRequest = UpdateConduitShardsRequest::default();

    let data = br#"{
      "data": []
    }
    "#
    .to_vec();
    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    let response = UpdateConduitShardsRequest::parse_response(Some(req), &uri, http_response);
    assert!(response.is_err());
}

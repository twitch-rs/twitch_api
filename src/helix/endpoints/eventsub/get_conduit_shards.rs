//! Gets a list of all shards for a conduit.
//! [`get-conduit-shards`](https://dev.twitch.tv/docs/api/reference/#get-conduit-shards)

use super::*;
use crate::eventsub;
use helix::{PaginationState, RequestGet};

/// Query Parameters for [Get Conduit Shards](super::get_conduit_shards)
///
/// [`get-conduit-shards`](https://dev.twitch.tv/docs/api/reference/#get-conduit-shards)
#[derive(PartialEq, Eq, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetConduitShardsRequest<'a> {
    /// Conduit ID.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub conduit_id: Cow<'a, types::ConduitIdRef>,

    /// Status to filter by.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub status: Option<eventsub::ShardStatus>,

    /// The cursor used to get the next page of results. The pagination object in the response contains the cursor’s value.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
}

impl<'a> GetConduitShardsRequest<'a> {
    /// Request the shards of a conduit
    pub fn new(conduit_id: impl types::IntoCow<'a, types::ConduitIdRef> + 'a) -> Self {
        Self {
            conduit_id: conduit_id.into_cow(),
            status: None,
            after: None,
        }
    }

    /// Filter shards by a specific status
    pub const fn status(mut self, status: eventsub::ShardStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the cursor to get a page of results
    pub fn after(mut self, after: impl types::IntoCow<'a, helix::CursorRef> + 'a) -> Self {
        self.after = Some(after.into_cow());
        self
    }
}

/// Return Values for [Get Conduit Shards](super::get_conduit_shards)
///
/// [`get-conduit-shards`](https://dev.twitch.tv/docs/api/reference/#get-conduit-shards)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ConduitShards {
    /// List of information about a conduit's shards.
    pub shards: Vec<eventsub::ShardResponse>,
}

impl Request for GetConduitShardsRequest<'_> {
    type PaginationData = PaginationState<Self>;
    type Response = Vec<eventsub::ShardResponse>;

    const PATH: &'static str = "eventsub/conduits/shards";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetConduitShardsRequest<'_> {}

impl helix::Paginated for GetConduitShardsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_uri() {
    use helix::*;
    let req = GetConduitShardsRequest::new("12345");

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/eventsub/conduits/shards?conduit_id=12345"
    );
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    use types::Timestamp;

    use crate::eventsub::{
        ShardStatus, TransportResponse, WebhookTransportResponse, WebsocketTransportResponse,
    };
    let req = GetConduitShardsRequest::new("12345");

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
    },
    {
      "id": "2",
      "status": "enabled",
      "transport": {
        "method": "websocket",
        "session_id": "9fd5164a-a958-4c60-b7f4-6a7202506ca0",
        "connected_at": "2020-11-10T14:32:18.730260295Z"
      }
    },
    {
      "id": "3",
      "status": "enabled",
      "transport": {
        "method": "websocket",
        "session_id": "238b4b08-13f1-4b8f-8d31-56665a7a9d9f",
        "connected_at": "2020-11-10T14:32:18.730260295Z"
      }
    },
    {
      "id": "4",
      "status": "websocket_disconnected",
      "transport": {
        "method": "websocket",
        "session_id": "ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9",
        "connected_at": "2020-11-10T14:32:18.730260295Z",
        "disconnected_at": "2020-11-11T14:32:18.730260295Z"
      }
    }
  ],
  "pagination": {}
}"#
    .to_vec();
    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    let response = GetConduitShardsRequest::parse_response(Some(req), &uri, http_response).unwrap();

    assert_eq!(
        response.data,
        vec![
            crate::eventsub::ShardResponse {
                id: "0".into(),
                status: ShardStatus::Enabled,
                transport: TransportResponse::Webhook(WebhookTransportResponse {
                    callback: "https://this-is-a-callback.com".to_string(),
                }),
            },
            crate::eventsub::ShardResponse {
                id: "1".into(),
                status: ShardStatus::WebhookCallbackVerificationPending,
                transport: TransportResponse::Webhook(WebhookTransportResponse {
                    callback: "https://this-is-a-callback-2.com".to_string(),
                }),
            },
            crate::eventsub::ShardResponse {
                id: "2".into(),
                status: ShardStatus::Enabled,
                transport: TransportResponse::Websocket(WebsocketTransportResponse {
                    session_id: "9fd5164a-a958-4c60-b7f4-6a7202506ca0".to_string(),
                    connected_at: Some(Timestamp::from_static("2020-11-10T14:32:18.730260295Z")),
                    disconnected_at: None,
                }),
            },
            crate::eventsub::ShardResponse {
                id: "3".into(),
                status: ShardStatus::Enabled,
                transport: TransportResponse::Websocket(WebsocketTransportResponse {
                    session_id: "238b4b08-13f1-4b8f-8d31-56665a7a9d9f".to_string(),
                    connected_at: Some(Timestamp::from_static("2020-11-10T14:32:18.730260295Z")),
                    disconnected_at: None,
                }),
            },
            crate::eventsub::ShardResponse {
                id: "4".into(),
                status: ShardStatus::WebsocketDisconnected,
                transport: TransportResponse::Websocket(WebsocketTransportResponse {
                    session_id: "ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9".to_string(),
                    connected_at: Some(Timestamp::from_static("2020-11-10T14:32:18.730260295Z")),
                    disconnected_at: Some(Timestamp::from_static("2020-11-11T14:32:18.730260295Z")),
                }),
            },
        ]
    );

    dbg!("{:#?}", response);
}

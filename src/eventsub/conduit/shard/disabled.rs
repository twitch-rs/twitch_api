#![doc(alias = "conduit.shard.disabled")]
//! A conduit shard is disabled by twitch.

use super::*;

/// [`conduit.shard.disabled`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#conduitsharddisabled)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ConduitShardDisabledV1 {
    /// Your application’s client id. The provided client_id must match the client ID in the application access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub client_id: String,
    /// Optional. The conduit ID to receive events for. If omitted, events for all of this client’s conduits are sent.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub conduit_id: Option<String>,
}

impl ConduitShardDisabledV1 {
    /// Your application’s client id. The provided client_id must match the client ID in the application access token.
    pub fn client_id(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            conduit_id: None,
        }
    }

    /// The conduit ID to receive events for. If omitted, events for all of this client’s conduits are sent.
    pub fn conduit_id(mut self, conduit_id: impl Into<String>) -> Self {
        self.conduit_id = Some(conduit_id.into());
        self
    }
}

impl EventSubscription for ConduitShardDisabledV1 {
    type Payload = ConduitShardDisabledV1Payload;

    const EVENT_TYPE: EventType = EventType::ConduitShardDisabled;
    #[cfg(feature = "twitch_oauth2")]
    /// App access token where the client ID matches the client ID in the condition.
    /// If conduit_id is specified, the client must be the owner of the conduit.
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
    const VERSION: &'static str = "1";
}

/// [`conduit.shard.disabled`](ConduitShardDisabledV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ConduitShardDisabledV1Payload {
    /// The conduit ID.
    pub conduit_id: String,
    /// The shard ID within the conduit.
    pub shard_id: String,
    /// The status of the disabled shard.
    pub status: eventsub::ShardStatus,
    /// The transport details about the disable shard.
    pub transport: eventsub::TransportResponse,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "conduit.shard.disabled",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "client_id": "uo6dggojyb8d6soh92zknwmi5ej1q2"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2023-04-11T10:11:12.123Z"
        },
        "event": {
            "conduit_id": "bfcfc993-26b1-b876-44d9-afe75a379dac",
            "shard_id": "4",
            "status": "websocket_disconnected",
            "transport": {
                "method": "websocket",
                "session_id": "ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9",
                "connected_at": "2020-11-10T14:32:18.730260295Z",
                "disconnected_at": "2020-11-11T14:32:18.730260295Z"
            }
        }
    }
    "##;

    let val = dbg!(eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val);
}

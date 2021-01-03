//! Notifies when a follows event occurs.

use crate::types;

use super::*;

use serde::{Deserialize, Serialize};

impl Topic for HypeTrainEventTopic {
    type Helix = crate::helix::hypetrain::GetHypeTrainEventsRequest;

    const PATH: &'static str = "hypetrain/events";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadHypeTrain];
}

/// Notifies when a stream changes [Topic: Stream Changed](https://dev.twitch.tv/docs/api/webhooks-reference#topic-stream-changed)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
pub struct HypeTrainEventTopic {
    /// Specifies the user whose stream is monitored.
    #[builder(setter(into))]
    pub user_id: types::UserId,
}

#[test]
fn test_topic() {
    use crate::helix::webhooks::hub::*;
    use crate::helix::*;

    let req = WebhookHubRequest::<HypeTrainEventTopic>::builder().build();
    let body = WebhookHubBody::builder()
        .callback("https://example.com/this-is-a-callback")
        .lease_seconds(864000)
        .mode(WebhookSubscriptionMode::Subscribe)
        .secret("12233213890390".to_string())
        .topic(
            HypeTrainEventTopic::builder()
                .user_id(5678.to_string())
                .build(),
        )
        .build();

    req.create_request(body, "token", "clientid").unwrap();

    // From twitch docs, stream offline
    let data = br#"
    {
        "data": []
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    dbg!(HypeTrainEventTopic::parse_payload(http_response).unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
            "event_type": "hypetrain.progression",
            "event_timestamp": "2020-04-24T20:07:24Z",
            "version": "1.0",
            "event_data": {
              "broadcaster_id": "270954519",
              "cooldown_end_time": "2020-04-24T20:13:21.003802269Z",
              "expires_at": "2020-04-24T20:12:21.003802269Z",
              "goal": 1800,
              "id": "70f0c7d8-ff60-4c50-b138-f3a352833b50",
              "last_contribution": {
                "total": 200,
                "type": "BITS",
                "user": "134247454"
              },
              "level": 2,
              "started_at": "2020-04-24T20:05:47.30473127Z",
              "top_contributions": [
                {
                  "total": 600,
                  "type": "BITS",
                  "user": "134247450"
                }
              ],
              "total": 600
            }
          }
        ],
        "pagination": {
          "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjI3MDk1NDUxOToxNTg3NzU4ODQ0OjFiMEFzYkluQ0haVzJTUUZRa0N6cU4wN0liMiJ9fQ"
        }
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    dbg!(HypeTrainEventTopic::parse_payload(http_response).unwrap());
}

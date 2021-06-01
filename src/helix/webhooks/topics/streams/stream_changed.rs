//! Notifies when a stream changes.

use crate::types;

use super::*;

use serde::{Deserialize, Serialize};

impl Topic for StreamChangedTopic {
    type Helix = crate::helix::streams::GetStreamsRequest;

    const PATH: &'static str = "streams";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Notifies when a stream changes [Topic: Stream Changed](https://dev.twitch.tv/docs/api/webhooks-reference#topic-stream-changed)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
pub struct StreamChangedTopic {
    /// Specifies the user whose stream is monitored.
    #[builder(setter(into))]
    pub user_id: types::UserId,
}

#[cfg(test)]
#[test]
fn test_topic() {
    use crate::helix::webhooks::hub::*;
    use crate::helix::*;

    let req = WebhookHubRequest::<StreamChangedTopic>::builder().build();
    let body = WebhookHubBody::builder()
        .callback("https://example.com/this-is-a-callback")
        .lease_seconds(864000)
        .mode(WebhookSubscriptionMode::Subscribe)
        .secret("12233213890390".to_string())
        .topic(
            StreamChangedTopic::builder()
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

    dbg!(StreamChangedTopic::parse_payload(http_response).unwrap());

    // From twitch docs, stream online, mentions deprecated `community_ids`, and missing tag_ids
    let data = br#"
    {
        "data": [
          {
            "id": "0123456789",
            "user_id": "5678",
            "user_name": "wjdtkdqhs",
            "user_login": "wjdtkdqhs",
            "game_id": "21779",
            "type": "live",
            "title": "Best Stream Ever",
            "viewer_count": 417,
            "started_at": "2017-12-01T10:09:45Z",
            "language": "en",
            "thumbnail_url": "https://link/to/thumbnail.jpg",
            "tag_ids": [],
            "is_mature": true
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    dbg!(StreamChangedTopic::parse_payload(http_response).unwrap());
}

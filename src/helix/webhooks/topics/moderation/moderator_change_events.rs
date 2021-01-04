//! Notifies when a broadcaster adds or removes moderators.

use crate::types;

use super::*;

use serde::{Deserialize, Serialize};

impl Topic for ModeratorChangedTopic {
    type Helix = crate::helix::moderation::GetModeratorEventsRequest;

    const PATH: &'static str = "streams";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Notifies when a broadcaster adds or removes moderators. [Topic: Moderator Change Events](https://dev.twitch.tv/docs/api/webhooks-reference#topic-moderator-change-events)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
pub struct ModeratorChangedTopic {
    /// Must be 1.
    #[serde(default = "one")]
    #[builder(setter(skip), default = 1)]
    first: u32,
    /// User ID of the broadcaster. Must match the User ID in the Bearer token
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
}

#[inline(always)]
const fn one() -> u32 { 1 }

#[test]
fn test_topic() {
    use crate::helix::webhooks::hub::*;
    use crate::helix::*;

    let req = WebhookHubRequest::<ModeratorChangedTopic>::builder().build();
    let body = WebhookHubBody::builder()
        .callback("https://example.com/this-is-a-callback")
        .lease_seconds(864000)
        .mode(WebhookSubscriptionMode::Subscribe)
        .secret("12233213890390".to_string())
        .topic(
            ModeratorChangedTopic::builder()
                .broadcaster_id(5678.to_string())
                .build(),
        )
        .build();

    req.create_request(body, "token", "clientid").unwrap();

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "id": "1IVKZGWSqf45QIgf6WFKtYpd0Or",
            "event_type": "moderation.moderator.add",
            "event_timestamp": "2019-03-15T19:32:58Z",
            "version": "v1",
            "event_data": {
              "broadcaster_id": "198704263",
              "broadcaster_name": "aan22209",
              "user_id": "423374343",
              "user_name": "glowillig"
            }
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    dbg!(ModeratorChangedTopic::parse_payload(http_response).unwrap());
}

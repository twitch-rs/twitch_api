//! Notifies on subscription changes.

use crate::types;

use super::*;

use serde::{Deserialize, Serialize};

impl Topic for SubscriptionEventsTopic {
    type Helix = crate::helix::subscriptions::GetBroadcasterSubscriptionsEventsRequest;

    const PATH: &'static str = "streams";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Notifies on subscription changes. [Topic: Subscription Events](https://dev.twitch.tv/docs/api/webhooks-reference#topic-subscription-events)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
pub struct SubscriptionEventsTopic {
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

#[cfg(test)]
#[test]
fn test_topic() {
    use crate::helix::webhooks::hub::*;
    use crate::helix::*;

    let req = WebhookHubRequest::<SubscriptionEventsTopic>::builder().build();
    let body = WebhookHubBody::builder()
        .callback("https://example.com/this-is-a-callback")
        .lease_seconds(864000)
        .mode(WebhookSubscriptionMode::Subscribe)
        .secret("12233213890390".to_string())
        .topic(
            SubscriptionEventsTopic::builder()
                .broadcaster_id(5678.to_string())
                .build(),
        )
        .build();

    req.create_request(body, "token", "clientid").unwrap();

    // From twitch docs, at 2020-03-16T16:31:42 hami0315 subscribed to the broadcaster meka:
    let data = br#"
    {
        "data": [
          {
            "id": "3Wba7BrK0NQuEX9BO8emK8aHfpK",
            "event_type": "subscriptions.subscribe",
            "event_timestamp": "2020-03-16T16:31:42Z",
            "version": "1.0",
            "event_data": {
              "broadcaster_id": "158038007",
              "broadcaster_name": "meka",
              "is_gift": true,
              "plan_name": "Channel Subscription (meka)",
              "tier": "1000",
              "user_id": "505037911",
              "user_name": "hami0315",
              "gifter_id": "156900877",
              "gifter_name": "baxter4343"
            }
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    dbg!(SubscriptionEventsTopic::parse_payload(http_response).unwrap());

    // From twitch docs, at 2019-02-03T08:14:19 dallas notified broadcaster Birdman616 of their subscription in chat:
    // FIXME: Twitch docs mentions message, no such thing anymore.
    let data = br#"
    {
        "data": [
          {
            "id": "1Gf161qjQtk0mOMD4VeIMjTGPUk",
            "event_type": "subscriptions.notification",
            "event_timestamp": "2019-02-03T08:14:19Z",
            "version": "1.0",
            "event_data": {
              "broadcaster_id": "113627897",
              "broadcaster_name": "Birdman616",
              "is_gift": true,
              "plan_name": "Channel Subscription (Birdman616)",
              "tier": "1000",
              "user_id": "44322889",
              "user_name": "dallas"
            }
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    dbg!(SubscriptionEventsTopic::parse_payload(http_response).unwrap());
}

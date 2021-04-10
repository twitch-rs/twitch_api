//! Notifies when a follows event occurs.

use crate::types;

use super::*;

use serde::{Deserialize, Serialize};

impl Topic for UserFollowsTopic {
    type Helix = crate::helix::users::GetUsersFollowsRequest;

    const PATH: &'static str = "users/follows";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Notifies when a follows event occurs [Topic: User Follows](https://dev.twitch.tv/docs/api/webhooks-reference#topic-user-follows)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
pub struct UserFollowsTopic {
    /// Must be 1.
    #[serde(default = "one")]
    #[builder(setter(skip), default = 1)]
    first: u32,
    /// Specifies the user who starts following someone.
    #[builder(setter(into), default)]
    pub from_id: Option<types::UserId>,
    /// Specifies the user who has a new follower.
    #[builder(setter(into), default)]
    pub to_id: Option<types::UserId>,
}

#[inline(always)]
const fn one() -> u32 { 1 }

#[test]
fn test_topic() {
    use crate::helix::webhooks::hub::*;
    use crate::helix::*;

    let req = WebhookHubRequest::<UserFollowsTopic>::builder().build();
    let body = WebhookHubBody::builder()
        .callback("https://example.com/this-is-a-callback")
        .lease_seconds(864000)
        .mode(WebhookSubscriptionMode::Subscribe)
        .secret("12233213890390".to_string())
        .topic(
            UserFollowsTopic::builder()
                .from_id(1336.to_string())
                .build(),
        )
        .build();
    // UserFollows::builder().from_id(1336).build();
    // Create request

    req.create_request(body, "token", "clientid").unwrap();

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "from_id": "1336",
            "from_name": "ebi",
            "from_login": "EBI",
            "to_id": "1337",
            "to_name": "oliver0823nagy",
            "to_login": "oliver0823nagy",
            "followed_at": "2017-08-22T22:55:24Z"
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    dbg!(UserFollowsTopic::parse_payload(http_response).unwrap());
}

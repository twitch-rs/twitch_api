//! Notifies when a user changes information about their profile.

use crate::types;

use super::*;

use serde::{Deserialize, Serialize};

impl Topic for UserChangedTopic {
    type Helix = crate::helix::users::GetUsersRequest;

    const PATH: &'static str = "users";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
}

/// Notifies when a user changes information about their profile [Topic: User Changed](https://dev.twitch.tv/docs/api/webhooks-reference#topic-user-changed)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
pub struct UserChangedTopic {
    /// Specifies the user whose data is monitored.
    #[builder(setter(into))]
    pub id: types::UserId,
}

#[test]
fn test_topic() {
    use crate::helix::webhooks::hub::*;
    use crate::helix::*;

    let req = WebhookHubRequest::<UserChangedTopic>::builder().build();
    let body = WebhookHubBody::builder()
        .callback("https://example.com/this-is-a-callback")
        .lease_seconds(864000)
        .mode(WebhookSubscriptionMode::Subscribe)
        .secret("12233213890390".to_string())
        .topic(UserChangedTopic::builder().id(1336.to_string()).build())
        .build();

    // Create request
    req.create_request(body, "token", "clientid").unwrap();

    // From twitch docs, missing created_at
    let data = br#"
    {
        "data": [
          {
            "id": "1234",
            "login": "1234login",
            "display_name": "hiiam1234",
            "type": "staff",
            "broadcaster_type": "",
            "description": "1234 is me",
            "profile_image_url": "https://link/to/pic/1234.jpg",
            "offline_image_url": "https://link/to/offline_pic/1234_off.jpg",
            "view_count": 3455,
            "created_at": "2013-06-03T19:12:02.580593Z"
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    dbg!(UserChangedTopic::parse_payload(http_response).unwrap());
}

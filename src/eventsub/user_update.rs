//! Subscription that sends a notification when user updates their account.
use super::*;
/// The `user.update` subscription type sends a notification when user updates their account.
/// [`userupdate`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userupdate)
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct UserUpdateV1 {
    /// The user ID for the user you want update notifications for.
    pub user_id: types::UserId,
}

impl EventSubscription for UserUpdateV1 {
    type Payload = UserUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::UserUpdate;
    const VERSION: &'static str = "1";
}

/// Response payload for [`user.update` version `1`](UserUpdateV1) subscription.
///
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct UserUpdateV1Payload {
    /// The user’s description.
    pub description: String,
    /// The user’s email. Only included if you have the [`user:read:email`](twitch_oauth2::Scope::UserReadEmail) scope for the user.
    pub email: Option<String>,
    /// The user’s user id.
    pub user_id: types::UserId,
    /// The user’s user name.
    pub user_name: types::UserName,
}

#[test]
fn parse_payload() {
    let payload = r#"
{
    "subscription": {
        "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        "type": "user.update",
        "version": "1",
        "condition": {
           "user_id": "1337"
        },
         "transport": {
            "method": "webhook",
            "callback": "https://example.com/webhooks/callback"
        },
        "created_at": "2019-11-16T10:11:12.123Z"
    },
    "event": {
        "user_id": "1337",
        "user_name": "cool_user",
        "email": "user@email.com",
        "description": "cool description"
    }
}
    "#;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

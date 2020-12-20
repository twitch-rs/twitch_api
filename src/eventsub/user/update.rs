//! Specified user updates their account.
use super::*;
/// [`user.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userupdate): user updates their account.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct UserUpdateV1 {
    /// The user ID for the user you want update notifications for.
    pub user_id: types::UserId,
}

impl EventSubscription for UserUpdateV1 {
    type Payload = UserUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::UserUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const VERSION: &'static str = "1";
}

/// [`user.update`](UserUpdateV1) response payload.
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

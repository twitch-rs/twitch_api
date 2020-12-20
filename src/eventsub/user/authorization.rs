//! A user has revoked authorization for your client id.
use super::*;
/// [`user.authorization.revoke`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userauthorizationrevoke): a user has revoked authorization for your client id.  Use this webhook to meet government requirements for handling user data, such as GDPR, LGPD, or CCPA.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct UserAuthorizationRevokeV1 {
    /// Your application’s client id. The provided client_id must match the client id in the application access token
    pub client_id: types::UserId,
}

impl EventSubscription for UserAuthorizationRevokeV1 {
    type Payload = UserAuthorizationRevokeV1Payload;

    const EVENT_TYPE: EventType = EventType::UserAuthorizationRevoke;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const VERSION: &'static str = "1";
}

/// [`user.authorization.revoke`](UserAuthorizationRevokeV1) response payload.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct UserAuthorizationRevokeV1Payload {
    /// The client_id of the application with revoked user access.
    pub client_id: String,
    /// The user id for the user who has revoked authorization for your client id.
    pub user_id: types::UserId,
    /// The user name for the user who has revoked authorization for your client id. This is null if the user no longer exists.
    pub user_name: Option<types::UserName>,
}

#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "user.authorization.revoke",
            "version": "1",
            "condition": {
                "client_id": "crq72vsaoijkc83xx42hz6i37"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "client_id": "crq72vsaoijkc83xx42hz6i37",
            "user_id": "1337",
            "user_name": "cool_user"
        }
    }
    "#;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

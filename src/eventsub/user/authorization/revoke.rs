#![doc(alias = "user.authorization.revoke")]
//! A user has revoked authorization for your client id.
use super::*;
/// [`user.authorization.revoke`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userauthorizationrevoke): a user has revoked authorization for your client id.  Use this webhook to meet government requirements for handling user data, such as GDPR, LGPD, or CCPA.
#[derive(Clone, Debug, typed_builder::TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserAuthorizationRevokeV1 {
    /// Your application’s client id. The provided client_id must match the client id in the application access token
    #[builder(setter(into))]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserAuthorizationRevokeV1Payload {
    /// The client_id of the application with revoked user access.
    pub client_id: String,
    /// The user id for the user who has revoked authorization for your client id.
    pub user_id: types::UserId,
    /// The user login for the user who has revoked authorization for your client id. This is null if the user no longer exists.
    pub user_login: Option<types::UserName>,
    /// The user name for the user who has revoked authorization for your client id. This is null if the user no longer exists.
    pub user_name: Option<types::DisplayName>,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "user.authorization.revoke",
            "version": "1",
            "status": "enabled",
            "cost": 0,
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
            "user_login": "cool_user",
            "user_name": "Cool_User"
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

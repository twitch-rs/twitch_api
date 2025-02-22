#![doc(alias = "user.authorization.grant")]
//! A user’s authorization has been granted to your client id.
use super::*;
/// [`user.authorization.grant`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userauthorizationgrant): a user’s authorization has been granted to your client id.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserAuthorizationGrantV1 {
    /// Your application’s client id. The provided client_id must match the client id in the application access token
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub client_id: String,
}

impl UserAuthorizationGrantV1 {
    /// Your application’s client id. The provided client_id must match the client id in the application access token
    pub const fn new(client_id: String) -> Self { Self { client_id } }
}

impl EventSubscription for UserAuthorizationGrantV1 {
    type Payload = UserAuthorizationGrantV1Payload;

    const EVENT_TYPE: EventType = EventType::UserAuthorizationGrant;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
    const VERSION: &'static str = "1";
}

/// [`user.authorization.grant`](UserAuthorizationGrantV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserAuthorizationGrantV1Payload {
    /// The client_id of the application that was granted user access.
    pub client_id: String,
    /// The user id for the user who has granted authorization for your client id.
    pub user_id: types::UserId,
    /// The user login for the user who has granted authorization for your client id.
    pub user_login: types::UserName,
    /// The user display name for the user who has granted authorization for your client id.
    pub user_name: types::DisplayName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "user.authorization.grant",
            "version": "1",
            "status": "enabled",
            "cost": 1,
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

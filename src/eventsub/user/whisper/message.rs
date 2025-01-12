#![doc(alias = "user.whisper.message")]
//! A user receives a whisper
use super::*;

/// [`user.whisper.message`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userwhispermessage): a user receives a whisper.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserWhisperMessageV1 {
    /// The user ID of the person receiving whispers.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub user_id: types::UserId,
}

impl UserWhisperMessageV1 {
    /// The user ID of the person receiving whispers.
    pub fn new(user_id: impl Into<types::UserId>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
}

impl EventSubscription for UserWhisperMessageV1 {
    type Payload = UserWhisperMessageV1Payload;

    const EVENT_TYPE: EventType = EventType::UserWhisperMessage;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::UserReadWhispers,
        twitch_oauth2::Scope::UserManageWhispers
    )];
    const VERSION: &'static str = "1";
}

/// [`user.whisper.message`](UserWhisperMessageV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserWhisperMessageV1Payload {
    /// The ID of the user sending the message.
    pub from_user_id: types::UserId,
    /// The name of the user sending the message.
    pub from_user_name: types::DisplayName,
    /// The login of the user sending the message.
    pub from_user_login: types::UserName,
    /// The ID of the user receiving the message.
    pub to_user_id: types::UserId,
    /// The name of the user receiving the message.
    pub to_user_name: types::DisplayName,
    /// The login of the user receiving the message.
    pub to_user_login: types::UserName,
    /// The whisper ID.
    pub whisper_id: types::WhisperId,
    /// Object containing whisper information.
    pub whisper: Whisper,
}

/// Object containing whisper information.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Whisper {
    /// The body of the whisper message.
    pub text: String,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "user.whisper.message",
            "version": "1",
            "condition": {
                "user_id": "423374343"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z",
            "cost": 0
        },
        "event": {
            "from_user_id": "423374343",
            "from_user_login": "glowillig",
            "from_user_name": "glowillig",
            "to_user_id": "424596340",
            "to_user_login": "quotrok",
            "to_user_name": "quotrok",
            "whisper_id": "some-whisper-id",
            "whisper": {
                "text": "a secret"
            }
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

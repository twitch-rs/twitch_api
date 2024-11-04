#![doc(alias = "automod.terms.update")]
//! a notification when a broadcaster’s automod terms are updated

use super::*;
/// [`automod.terms.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodtermsupdate): A notification is sent when a broadcaster’s automod terms are updated. Changes to private terms are not sent.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodTermsUpdateV1 {
    /// User ID of the broadcaster (channel). Maximum: 1
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// User ID of the moderator creating the subscription. Maximum: 1
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl AutomodTermsUpdateV1 {
    /// Get automod update notifications for permitted/blocked terms in this channel as a moderator
    pub fn new(
        broadcaster_user_id: impl Into<types::UserId>,
        moderator_user_id: impl Into<types::UserId>,
    ) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
            moderator_user_id: moderator_user_id.into(),
        }
    }
}

impl EventSubscription for AutomodTermsUpdateV1 {
    type Payload = AutomodTermsUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::AutomodTermsUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageAutoMod];
    const VERSION: &'static str = "1";
}

/// [`automod.terms.update`](AutomodTermsUpdateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodTermsUpdateV1Payload {
    /// The ID of the broadcaster specified in the request.
    pub broadcaster_user_id: types::UserId,
    /// The login of the broadcaster specified in the request.
    pub broadcaster_user_login: types::UserName,
    /// The user name of the broadcaster specified in the request.
    pub broadcaster_user_name: types::DisplayName,

    /// The ID of the moderator who changed the channel settings.
    pub moderator_user_id: types::UserId,
    /// The login of the moderator.
    pub moderator_user_login: types::UserName,
    /// The moderator’s user name.
    pub moderator_user_name: types::DisplayName,

    /// The status change applied to the terms.
    pub action: AutomodTermAction,
    /// Indicates whether this term was added due to an Automod message approve/deny action.
    pub from_automod: bool,
    /// The list of terms that had a status change.
    pub terms: Vec<String>,
}

/// An action on an Automod term
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AutomodTermAction {
    /// A term was added to the list of permitted terms
    AddPermitted,
    /// A term was removed from the list of permitted terms
    RemovePermitted,
    /// A term was added to the list of blocked terms
    AddBlocked,
    /// A term was removed to the list of blocked terms
    RemoveBlocked,
    /// An unknown term action, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "480cfe69-7aea-4527-b0a7-9704a0bf7294",
            "status": "enabled",
            "type": "automod.terms.update",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQZ12VWLotRG6u3pudLlbhvhIGY2VsbC1j"
            },
            "created_at": "2024-11-03T11:52:05.699721918Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_login": "nerixyz",
            "broadcaster_user_name": "nerixyz",
            "moderator_user_id": "129546453",
            "moderator_user_login": "nerixyz",
            "moderator_user_name": "nerixyz",
            "action": "add_permitted",
            "from_automod": true,
            "terms": [
                "boobs"
            ]
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodTermsUpdateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid terms type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert!(notif.from_automod);
    assert_eq!(notif.action, AutomodTermAction::AddPermitted);
    assert_eq!(notif.terms.len(), 1);
    assert_eq!(notif.terms[0], "boobs");
}

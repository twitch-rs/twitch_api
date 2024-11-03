#![doc(alias = "automod.settings.update")]
//! a notification is sent when a broadcaster’s automod settings are updated.

use super::*;
/// [`automod.settings.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodsettingsupdate): a notification is sent when a broadcaster’s automod settings are updated.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodSettingsUpdateV1 {
    /// User ID of the broadcaster (channel). Maximum: 1
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// User ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl AutomodSettingsUpdateV1 {
    /// Get notifications for updates on Automod settings in this channel as a moderator
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

impl EventSubscription for AutomodSettingsUpdateV1 {
    type Payload = AutomodSettingsUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::AutomodSettingsUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorReadAutomodSettings];
    const VERSION: &'static str = "1";
}

/// [`automod.settings.update`](AutomodSettingsUpdateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodSettingsUpdateV1Payload {
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

    /// The default AutoMod level for the broadcaster. This field is [None] if the broadcaster has set one or more of the individual settings.
    pub overall_level: Option<u8>,
    /// The Automod level for discrimination against disability.
    pub disability: u8,
    /// The Automod level for hostility involving aggression.
    pub aggression: u8,
    /// The AutoMod level for discrimination based on sexuality, sex, or gender.
    pub sexuality_sex_or_gender: u8,
    /// The Automod level for discrimination against women.
    pub misogyny: u8,
    /// The Automod level for hostility involving name calling or insults.
    pub bullying: u8,
    /// The Automod level for profanity.
    pub swearing: u8,
    /// The Automod level for racial discrimination.
    pub race_ethnicity_or_religion: u8,
    /// The Automod level for sexual content.
    pub sex_based_terms: u8,
}

#[cfg(test)]
#[test]
fn parse_payload_overall() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "ef82080b-fe22-4959-996a-f7a1ab5467ea",
            "status": "enabled",
            "type": "automod.settings.update",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQ_7uykM5qRQGvMF6kjH9xkhIGY2VsbC1j"
            },
            "created_at": "2024-11-03T12:00:36.504376879Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_name": "nerixyz",
            "broadcaster_user_login": "nerixyz",
            "moderator_user_id": "129546453",
            "moderator_user_name": "nerixyz",
            "moderator_user_login": "nerixyz",
            "overall_level": 1,
            "bullying": 0,
            "disability": 0,
            "race_ethnicity_or_religion": 1,
            "misogyny": 0,
            "sexuality_sex_or_gender": 1,
            "aggression": 1,
            "sex_based_terms": 0,
            "swearing": 0
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodSettingsUpdateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.overall_level, Some(1));
    assert_eq!(notif.bullying, 0);
    assert_eq!(notif.disability, 0);
    assert_eq!(notif.race_ethnicity_or_religion, 1);
    assert_eq!(notif.misogyny, 0);
    assert_eq!(notif.sexuality_sex_or_gender, 1);
    assert_eq!(notif.aggression, 1);
    assert_eq!(notif.sex_based_terms, 0);
    assert_eq!(notif.swearing, 0);
}

#[cfg(test)]
#[test]
fn parse_payload_individual() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "ef82080b-fe22-4959-996a-f7a1ab5467ea",
            "status": "enabled",
            "type": "automod.settings.update",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "129546453",
                "moderator_user_id": "129546453"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQ_7uykM5qRQGvMF6kjH9xkhIGY2VsbC1j"
            },
            "created_at": "2024-11-03T12:00:36.504376879Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "129546453",
            "broadcaster_user_name": "nerixyz",
            "broadcaster_user_login": "nerixyz",
            "moderator_user_id": "129546453",
            "moderator_user_name": "nerixyz",
            "moderator_user_login": "nerixyz",
            "overall_level": null,
            "bullying": 2,
            "disability": 0,
            "race_ethnicity_or_religion": 1,
            "misogyny": 0,
            "sexuality_sex_or_gender": 1,
            "aggression": 1,
            "sex_based_terms": 0,
            "swearing": 0
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::AutomodSettingsUpdateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "129546453");
    assert_eq!(notif.overall_level, None);
    assert_eq!(notif.bullying, 2);
    assert_eq!(notif.disability, 0);
    assert_eq!(notif.race_ethnicity_or_religion, 1);
    assert_eq!(notif.misogyny, 0);
    assert_eq!(notif.sexuality_sex_or_gender, 1);
    assert_eq!(notif.aggression, 1);
    assert_eq!(notif.sex_based_terms, 0);
    assert_eq!(notif.swearing, 0);
}

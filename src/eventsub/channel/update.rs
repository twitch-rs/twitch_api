#![doc(alias = "channel.update")]
//! Channel has updated the category, title, mature flag, or broadcast language.
use super::*;

/// [`channel.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelupdate) subscription type sends notifications when a broadcaster updates the category, title, mature flag, or broadcast language for their channel.
#[derive(Clone, Debug, typed_builder::TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelUpdateV1 {
    /// The broadcaster user ID for the channel you want to get updates for.
    #[builder(setter(into))]
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelUpdateV1 {
    type Payload = ChannelUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const VERSION: &'static str = "1";
}

/// [`channel.update`](ChannelUpdateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelUpdateV1Payload {
    /// The broadcaster’s user ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s user display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The channel’s stream title.
    pub title: String,
    /// The channel’s broadcast language.
    pub language: String,
    /// The channel’s category ID.
    pub category_id: types::CategoryId,
    /// The category name.
    pub category_name: String,
    /// A boolean identifying whether the channel is flagged as mature. Valid values are true and false.
    pub is_mature: bool,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    // FIXME: https://github.com/twitchdev/issues/issues/268
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.update",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
               "broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Best Stream Ever",
            "language": "en",
            "category_id": "21779",
            "category_name": "Fortnite",
            "is_mature": false
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

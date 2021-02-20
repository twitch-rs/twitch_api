#![doc(alias = "stream.online")]
//! The specified broadcaster starts a stream
use super::*;

/// [`stream.online`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#streamonline): the specified broadcaster starts a stream
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StreamOnlineV1 {
    /// The broadcaster user ID you want to get stream online notifications for.
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for StreamOnlineV1 {
    type Payload = StreamOnlineV1Payload;

    const EVENT_TYPE: EventType = EventType::StreamOnline;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const VERSION: &'static str = "1";
}

/// [`stream.online`](StreamOnlineV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StreamOnlineV1Payload {
    /// The broadcaster’s user id.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s user login.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s user display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The event id.
    pub id: String,
    /// The stream type. Valid values are: live, playlist, watch_party, premiere, rerun.
    #[serde(rename = "type")]
    pub type_: types::VideoType,
    /// The timestamp at which the stream went online at.
    pub started_at: types::Timestamp,
}

#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "stream.online",
            "version": "1",
            "status": "enabled",
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
            "id": "9001",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "type": "live",
            "started_at": "2020-10-11T10:11:12.123Z"
        }
    }
    "#;

    let val = dbg!(crate::eventsub::Payload::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

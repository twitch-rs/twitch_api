//! A a broadcaster raids another broadcaster’s channel.
use types::UserName;

use super::*;

/// [`channel.raid`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelraid-beta): a a broadcaster raids another broadcaster’s channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelRaidBeta {
    /// The broadcaster user ID that created the channel raid you want to get notifications for. Use this parameter if you want to know when a specific broadcaster raids another broadcaster.
    pub from_broadcaster_user_id: Option<types::UserId>,
    /// The broadcaster user ID that received the channel raid you want to get notifications for. Use this parameter if you want to know when a specific broadcaster is raided by another broadcaster.
    pub to_broadcaster_user_id: Option<types::UserId>,
}

impl EventSubscription for ChannelRaidBeta {
    type Payload = ChannelRaidBetaPayload;

    const EVENT_TYPE: EventType = EventType::ChannelRaid;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelReadSubscriptions];
    const VERSION: &'static str = "beta";
}

/// [`channel.raid`](ChannelRaidBeta) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelRaidBetaPayload {
    /// The broadcaster ID that created the raid.
    pub from_broadcaster_user_id: types::UserId,
    /// The broadcaster login that created the raid.
    pub from_broadcaster_user_login: types::UserName,
    /// The broadcaster display name that created the raid.
    pub from_broadcaster_user_name: types::DisplayName,
    /// The broadcaster ID that received the raid.
    pub to_broadcaster_user_id: types::UserId,
    /// The broadcaster login that received the raid.
    pub to_broadcaster_user_login: types::UserName,
    /// The broadcaster display name that received the raid.
    pub to_broadcaster_user_name: types::DisplayName,
    /// The number of viewers in the raid.
    pub viewers: i64,
}

#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.raid",
            "version": "beta",
            "condition": {
                "to_broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "from_broadcaster_user_id": "1234",
            "from_broadcaster_user_login": "cool_user",
            "from_broadcaster_user_name": "Cool_User",
            "to_broadcaster_user_id": "1337",
            "to_broadcaster_user_login": "cooler_user",
            "to_broadcaster_user_name": "Cooler_User",
            "viewers": 9001
        }
    }
    "#;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

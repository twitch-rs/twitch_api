//! Subscriptions that sends a notification when a hype train ends on the specified channel.

use super::*;
/// The `channel.hype_train.end` subscription type sends a notification when a hype train ends on the specified channel.
/// [`channel.hype_train.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelhype_trainprogress)
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelHypeTrainEndV1 {
    // FIXME: Twitch docs say "want to hype train"
    /// The broadcaster user ID for the channel you want hype train end notifications for.
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelHypeTrainEndV1 {
    type Payload = ChannelHypeTrainEndV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelHypeTrainEnd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadHypeTrain];
    const VERSION: &'static str = "1";
}

/// Response payload for [`channel.hype_train.end` version `1`](ChannelHypeTrainEndV1) subscription.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelHypeTrainEndV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster name.
    pub broadcaster_user_name: types::UserName,
    /// The timestamp at which the hype train cooldown ends so that the next hype train can start.
    pub cooldown_ends_at: types::Timestamp,
    /// The timestamp at which the hype train ended.
    pub ended_at: types::Timestamp,
    /// Current level of hype train event.
    pub level: i64,
    /// The timestamp at which the hype train started.
    pub started_at: types::Timestamp,
    /// The contributors with the most points contributed.
    pub top_contributions: Vec<Contribution>,
    /// Total points contributed to the hype train.
    pub total: i64,
}

#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.hype_train.end",
            "version": "1",
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
            "broadcaster_user_name": "cool_user",
            "level": 2,
            "total": 137,
            "top_contributions": [
                { "user_id": "123", "user_name": "pogchamp", "type": "bits", "total": 50 },
                { "user_id": "456", "user_name": "kappa", "type": "subscription", "total": 45 }
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z",
            "cooldown_ends_at": "2020-07-15T18:16:11.17106713Z"
        }
    }
    "##;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

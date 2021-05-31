#![doc(alias = "channel.poll.begin")]
//! A poll begins on the specified channel.

use super::*;
/// [`channel.poll.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpollbegin-beta): a poll begins on the specified channel.
#[derive(Clone, Debug, typed_builder::TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPollBeginV1 {
    /// The broadcaster user ID of the channel for which “poll begin” notifications will be received.
    #[builder(setter(into))]
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelPollBeginV1 {
    type Payload = ChannelPollBeginV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelPollBegin;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadPolls];
    const VERSION: &'static str = "1";
}

/// [`channel.poll.begin`](ChannelPollBeginV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPollBeginV1Payload {
    /// The Bits voting settings for the poll.
    pub bits_voting: BitsVoting,
    /// The requested broadcaster ID.
    pub broadcaster_user_id: String,
    /// The requested broadcaster login.
    pub broadcaster_user_login: String,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: String,
    /// The Channel Points voting settings for the poll.
    pub channel_points_voting: ChannelPointsVoting,
    /// An array of choices for the poll.
    pub choices: Vec<types::PollChoice>,
    /// The time the poll will end.
    pub ends_at: types::Timestamp,
    /// ID of the poll.
    pub id: types::PollId,
    /// The time the poll started.
    pub started_at: types::Timestamp,
    /// Question displayed for the poll.
    pub title: String,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.poll.begin",
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
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "choices": [
                {"id": "123", "title": "Yeah!"},
                {"id": "124", "title": "No!"},
                {"id": "125", "title": "Maybe!"}
            ],
            "bits_voting": {
                "is_enabled": true,
                "amount_per_vote": 10
            },
            "channel_points_voting": {
                "is_enabled": true,
                "amount_per_vote": 10
            },  
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ends_at": "2020-07-15T17:16:08.17106713Z"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Payload::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

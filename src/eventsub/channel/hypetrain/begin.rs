#![doc(alias = "channel.hype_train.begin")]
//! A hype train begins on the specified channel.

use super::*;
/// [`channel.hype_train.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelhype_trainbegin): a hype train begins on the specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelHypeTrainBeginV1 {
    // FIXME: Twitch docs say "want to hype train"
    /// The broadcaster user ID for the channel you want hype train begin notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelHypeTrainBeginV1 {
    /// The broadcaster user ID for the channel you want hype train begin notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelHypeTrainBeginV1 {
    type Payload = ChannelHypeTrainBeginV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelHypeTrainBegin;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelReadHypeTrain];
    const VERSION: &'static str = "1";
}

/// [`channel.hype_train.begin`](ChannelHypeTrainBeginV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelHypeTrainBeginV1Payload {
    /// The Hype Train ID.
    pub id: types::HypeTrainId,
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The time at which the hype train expires. The expiration is extended when the hype train reaches a new level.
    pub expires_at: types::Timestamp,
    /// The number of points required to reach the next level.
    pub goal: i64,
    /// The most recent contribution.
    pub last_contribution: Contribution,
    /// The number of points contributed to the hype train at the current level.
    pub progress: i64,
    /// The timestamp at which the hype train started.
    pub started_at: types::Timestamp,
    // FIXME: Contains a maximum of two user objects
    /// The contributors with the most points contributed.
    pub top_contributions: Vec<Contribution>,
    /// Total points contributed to the hype train.
    pub total: i64,
    /// The starting level of the Hype Train.
    pub level: i64,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.hype_train.begin",
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
            "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "total": 137,
            "progress": 137,
            "goal": 500,
            "top_contributions": [
                { "user_id": "123", "user_login": "pogchamp", "user_name": "PogChamp", "type": "bits", "total": 50 },
                { "user_id": "456", "user_login": "kappa", "user_name": "Kappa", "type": "subscription", "total": 45 }
            ],
            "last_contribution": { "user_id": "123", "user_login": "pogchamp", "user_name": "PogChamp", "type": "bits", "total": 50 },
            "level": 2,
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "expires_at": "2020-07-15T17:16:11.17106713Z"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

/// [`channel.hype_train.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelhype_trainbegin): a hype train begins on the specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelHypeTrainBeginV2 {
    // FIXME: Twitch docs say "want to hype train"
    /// The broadcaster user ID for the channel you want hype train begin notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelHypeTrainBeginV2 {
    /// The broadcaster user ID for the channel you want hype train begin notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelHypeTrainBeginV2 {
    type Payload = ChannelHypeTrainBeginV2Payload;

    const EVENT_TYPE: EventType = EventType::ChannelHypeTrainBegin;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelReadHypeTrain];
    const VERSION: &'static str = "2";
}

/// [`channel.hype_train.begin`](ChannelHypeTrainBeginV2) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelHypeTrainBeginV2Payload {
    /// The Hype Train ID.
    pub id: types::HypeTrainId,
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// Total points contributed to the hype train.
    pub total: i64,
    /// The number of points contributed to the hype train at the current level.
    pub progress: i64,
    /// The number of points required to reach the next level.
    pub goal: i64,
    // FIXME: Contains a maximum of two user objects
    /// The contributors with the most points contributed.
    pub top_contributions: Vec<Contribution>,
    /// The starting level of the Hype Train.
    pub level: i64,
    /// The all-time high level this type of Hype Train has reached for this broadcaster.
    pub all_time_high_level: i64,
    /// The all-time high total this type of Hype Train has reached for this broadcaster.
    pub all_time_high_total: i64,
    /// Optional. Non-null for a shared Hype Train. Contains the list of broadcasters in the shared Hype Train.
    pub shared_train_participants: Option<Vec<SharedTrainParticipant>>,
    /// The timestamp at which the hype train started.
    pub started_at: types::Timestamp,
    /// The time at which the hype train expires. The expiration is extended when the hype train reaches a new level.
    pub expires_at: types::Timestamp,
    /// The type of the hype train
    #[serde(rename = "type")]
    pub type_: HypeTrainType,
    /// Indicates if the Hype Train is shared. When true, shared_train_participants will contain the list of broadcasters the train is shared with.
    pub is_shared_train: bool,
}

#[cfg(test)]
#[test]
fn parse_payload_v2() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.hype_train.begin",
            "version": "2",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.634234626Z"
        },
        "event": {
            "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "total": 137,
            "progress": 137,
            "goal": 500,
            "top_contributions": [
                {
                    "user_id": "123",
                    "user_login": "pogchamp",
                    "user_name": "PogChamp",
                    "type": "bits",
                    "total": 50
                }
            ],
            "shared_train_participants": null,
            "level": 1,
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "expires_at": "2020-07-15T17:16:11.17106713Z",
            "is_shared_train": false,
            "type": "regular",
            "all_time_high_level": 4,
            "all_time_high_total": 2845
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

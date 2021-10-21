#![doc(alias = "channel.goal.end")]
//! A specified broadcaster ends a goal.

use super::*;
/// [`channel.goal.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#end-goals-event): a specified broadcaster ends a goal.
#[derive(Clone, Debug, typed_builder::TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGoalEndV1 {
    /// The ID of the broadcaster to get notified about.
    #[builder(setter(into))]
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelGoalEndV1 {
    type Payload = ChannelGoalEndV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelGoalEnd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadGoals];
    const VERSION: &'static str = "1";
}

/// [`channel.goal.end`](ChannelGoalEndV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGoalEndV1Payload {
    /// An ID that uniquely identifies this goal.
    pub id: types::CreatorGoalId,
    /// An ID that uniquely identifies the broadcaster.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The broadcaster’s user handle.
    pub broadcaster_user_login: types::UserName,
    /// The type of goal.
    #[serde(rename = "type")]
    pub type_: types::CreatorGoalType,
    /// A description of the goal, if specified. The description may contain a maximum of 40 characters.
    pub description: String,
    /// Indicates whether the broadcaster achieved their goal.
    pub is_achieved: bool,
    /// The current value.
    pub current_amount: i64,
    /// The goal’s target value.
    pub target_amount: i64,
    /// The UTC timestamp in RFC 3339 format, which indicates when the broadcaster created the goal.
    pub started_at: types::Timestamp,
    /// The UTC timestamp in RFC 3339 format, which indicates when the broadcaster reached the goal.
    pub ended_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "status": "enabled",
            "type": "channel.goal.end",
            "version": "1",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "141981764"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2021-07-15T10:11:12.123Z"
        },
        "event": {
            "id": "12345-abc-678-defgh",
            "broadcaster_user_id": "141981764",
            "broadcaster_user_name": "TwitchDev",
            "broadcaster_user_login": "twitchdev",
            "type": "subscription",
            "description": "Help me get partner!",
            "is_achieved": false,
            "current_amount": 100,
            "target_amount": 220,
            "started_at": "2021-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-16T17:16:03.17106713Z"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#![doc(alias = "channel.goal.progress")]
//! Progress is made towards the specified broadcaster’s goal.

use super::*;
/// [`channel.goal.progress`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#progress-goals-event): progress is made towards the specified broadcaster’s goal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGoalProgressV1 {
    /// The ID of the broadcaster to get notified about.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelGoalProgressV1 {
    /// The ID of the broadcaster to get notified about.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelGoalProgressV1 {
    type Payload = ChannelGoalProgressV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelGoalProgress;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelReadGoals];
    const VERSION: &'static str = "1";
}

/// [`channel.goal.progress`](ChannelGoalProgressV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGoalProgressV1Payload {
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
    /// The current value.
    pub current_amount: i64,
    /// The goal’s target value. For example, if the broadcaster has 200 followers before creating the goal, and their goal is to double that number, this field is set to 400.
    pub target_amount: i64,
    /// The UTC timestamp in RFC 3339 format, which indicates when the broadcaster created the goal.
    pub started_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "status": "enabled",
            "type": "channel.goal.progress",
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
            "id": "12345-cool-event",
            "broadcaster_user_id": "141981764",
            "broadcaster_user_name": "TwitchDev",
            "broadcaster_user_login": "twitchdev",
            "type": "subscription",
            "description": "Help me get partner!",
            "current_amount": 100,
            "target_amount": 220,
            "started_at": "2021-07-15T17:16:03.17106713Z"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#![doc(alias = "channel.shoutout.create")]
//! A specified broadcaster sends a Shoutout.

use super::*;
/// [`channel.shoutout.create`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutcreate): a Prediction begins on the specified channel
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelShoutoutCreateV1 {
    /// The ID of the broadcaster that you want to receive notifications about when they send a Shoutout.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The ID of the broadcaster that gave the Shoutout or one of the broadcaster’s moderators.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelShoutoutCreateV1 {
    /// Create a new [`ChannelShoutoutCreateV1`]
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

impl EventSubscription for ChannelShoutoutCreateV1 {
    type Payload = ChannelShoutoutCreateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelShoutoutCreate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ModeratorReadShoutouts,
        twitch_oauth2::Scope::ModeratorManageShoutouts
    )];
    const VERSION: &'static str = "1";
}

/// [`channel.shoutout.create`](ChannelShoutoutCreateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelShoutoutCreateV1Payload {
    /// An ID that identifies the broadcaster that sent the Shoutout.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_user_name: types::DisplayName,
    /// An ID that identifies the broadcaster that received the Shoutout.
    pub to_broadcaster_user_id: types::UserId,
    /// The broadcaster’s login name.
    pub to_broadcaster_user_login: types::UserName,
    /// The broadcaster’s display name.
    pub to_broadcaster_user_name: types::DisplayName,
    /// An ID that identifies the moderator that sent the Shoutout. If the broadcaster sent the Shoutout, this ID is the same as the ID in [`broadcaster_user_id`](Self::broadcaster_user_id).
    pub moderator_user_id: types::UserId,
    /// The moderator’s login name.
    pub moderator_user_login: types::UserName,
    /// The moderator’s display name.
    pub moderator_user_name: types::DisplayName,
    /// The number of users that were watching the broadcaster’s stream at the time of the Shoutout.
    pub viewer_count: i64,
    /// The UTC timestamp (in RFC3339 format) of when the moderator sent the Shoutout.
    pub started_at: types::Timestamp,
    /// The UTC timestamp (in RFC3339 format) of when the broadcaster may send a Shoutout to a different broadcaster.
    pub cooldown_ends_at: types::Timestamp,
    /// The UTC timestamp (in RFC3339 format) of when the broadcaster may send another Shoutout to the broadcaster in [`to_broadcaster_user_id`](Self::to_broadcaster_user_id).
    pub target_cooldown_ends_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
          "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
          "type": "channel.shoutout.create",
          "version": "1",
          "status": "enabled",
          "cost": 0,
          "condition": {
            "broadcaster_user_id": "12345",
            "moderator_user_id": "98765"
          },
          "transport": {
            "method": "webhook",
            "callback": "https://example.com/webhooks/callback"
          },
          "created_at": "2022-07-25T10:11:12.1236739Z"
        },
        "event": {
          "broadcaster_user_id": "12345",
          "broadcaster_user_name": "SimplySimple",
          "broadcaster_user_login": "simplysimple",
          "moderator_user_id": "98765",
          "moderator_user_name": "ParticularlyParticular123",
          "moderator_user_login": "particularlyparticular123",
          "to_broadcaster_user_id": "626262",
          "to_broadcaster_user_name": "SandySanderman",
          "to_broadcaster_user_login": "sandysanderman",
          "started_at": "2022-07-26T17:00:03.17106713Z",
          "viewer_count": 860,
          "cooldown_ends_at": "2022-07-26T17:02:03.17106713Z",
          "target_cooldown_ends_at":"2022-07-26T18:00:03.17106713Z"
        }
      }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#![doc(alias = "channel.shield_mode.begin")]
//! A channel activates Shield Mode

use super::*;
/// [`channel.shield_mode.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modebegin): an user responds to a prediction on the specified channel
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelShieldModeBeginV1 {
    /// The ID of the broadcaster that you want to receive notifications about when they activate Shield Mode.
    pub broadcaster_user_id: types::UserId,
    /// The ID of the broadcaster or one of the broadcaster’s moderators.
    pub moderator_user_id: types::UserId,
}

impl ChannelShieldModeBeginV1 {
    /// Get shield mode begin events for this channel as moderator
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

impl EventSubscription for ChannelShieldModeBeginV1 {
    type Payload = ChannelShieldModeBeginV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelShieldModeBegin;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ModeratorReadShieldMode,
        twitch_oauth2::Scope::ModeratorManageShieldMode
    )];
    const VERSION: &'static str = "1";
}

/// [`channel.shield_mode.begin`](ChannelShieldModeBeginV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelShieldModeBeginV1Payload {
    /// An ID that identifies the broadcaster whose Shield Mode status was updated.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_user_name: types::DisplayName,
    /// An ID that identifies the moderator that updated the Shield Mode’s status. If the broadcaster updated the status, this ID will be the same as broadcaster_user_id.
    pub moderator_user_id: types::UserId,
    /// The moderator’s login name.
    pub moderator_user_login: types::UserName,
    /// The moderator’s display name.
    pub moderator_user_name: types::DisplayName,
    /// The UTC timestamp (in RFC3339 format) of when the moderator activated Shield Mode.
    pub started_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
          "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
          "type": "channel.shield_mode.begin",
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
          "started_at": "2022-07-26T17:00:03.17106713Z"
        }
      }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

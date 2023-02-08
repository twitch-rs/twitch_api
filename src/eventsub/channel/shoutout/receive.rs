#![doc(alias = "channel.shoutout.receive")]
//! A specified broadcaster receives a Shoutout.

use super::*;
/// [`channel.shoutout.receive`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutreceive): a Prediction begins on the specified channel
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelShoutoutReceiveBeta {
    /// The ID of the broadcaster that you want to receive notifications about when they receive a Shoutout.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The ID of the broadcaster that received the Shoutout or one of the broadcaster’s moderators.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelShoutoutReceiveBeta {
    /// Create a new [`ChannelShoutoutReceiveBeta`]
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

impl EventSubscription for ChannelShoutoutReceiveBeta {
    type Payload = ChannelShoutoutReceiveBetaPayload;

    const EVENT_TYPE: EventType = EventType::ChannelShoutoutReceive;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModeratorReadShoutouts];
    const VERSION: &'static str = "beta";
}

/// [`channel.shoutout.receive`](ChannelShoutoutReceiveBeta) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelShoutoutReceiveBetaPayload {
    /// An ID that identifies the broadcaster that received the Shoutout.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_user_name: types::DisplayName,
    /// An ID that identifies the broadcaster that sent the Shoutout.
    pub from_broadcaster_user_id: types::UserId,
    /// The broadcaster’s login name.
    pub from_broadcaster_user_login: types::UserName,
    /// The broadcaster’s display name.
    pub from_broadcaster_user_name: types::DisplayName,
    /// The number of users that were watching the from-broadcaster’s stream at the time of the Shoutout.
    pub viewer_count: i64,
    /// The UTC timestamp (in RFC3339 format) of when the moderator sent the Shoutout.
    pub started_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
          "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
          "type": "channel.shoutout.receive",
          "version": "beta",
          "status": "enabled",
          "cost": 0,
          "condition": {
            "broadcaster_user_id": "626262",
            "moderator_user_id": "98765"
          },
          "transport": {
            "method": "webhook",
            "callback": "https://example.com/webhooks/callback"
          },
          "created_at": "2022-07-25T10:11:12.1236739Z"
        },
        "event": {
          "broadcaster_user_id": "626262",
          "broadcaster_user_name": "SandySanderman",
          "broadcaster_user_login": "sandysanderman",
          "from_broadcaster_user_id": "12345",
          "from_broadcaster_user_name": "SimplySimple",
          "from_broadcaster_user_login": "simplysimple",
          "viewer_count": 860,
          "started_at": "2022-07-26T17:00:03.17106713Z"
        }
      }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

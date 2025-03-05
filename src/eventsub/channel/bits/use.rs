#![doc(alias = "channel.bits.use")]
//! sends a notification whenever Bits are used on a channel

use super::*;

/// [`channel.bits.use`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelbitsuse): sends a notification whenever Bits are used on a channel
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelBitsUseV1 {
    /// The ID of the broadcaster that you want to get Channel Bits notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelBitsUseV1 {
    /// The ID of the broadcaster that you want to get Channel Bits notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelBitsUseV1 {
    type Payload = ChannelBitsUseV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelBitsUse;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![any(twitch_oauth2::Scope::BitsRead)];
    const VERSION: &'static str = "1";
}

/// [`channel.bits.use`](ChannelBitsUseV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelBitsUseV1Payload {
    /// The User ID of the redeeming user.
    pub user_id: types::UserId,
    /// The login name of the redeeming user.
    pub user_login: types::UserName,
    /// The display name of the redeeming user.
    pub user_name: types::DisplayName,
    /// The User ID of the channel where the Bits were redeemed.
    pub broadcaster_user_id: types::UserId,
    /// The login of the channel where the Bits were used.
    pub broadcaster_user_login: types::UserName,
    /// The display name of the channel where the Bits were used.
    pub broadcaster_user_name: types::DisplayName,
    /// The number of Bits used.
    pub bits: usize,
    /// The type of Bits used.
    #[serde(rename = "type")]
    pub type_: BitsType,
    /// Data about Power-up.
    pub power_up: Option<BitsPowerUp>,
    /// An object that contains the user message and emote information needed to recreate the message.
    pub message: Option<crate::eventsub::channel::chat::Message>,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.bits.use",
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
            "created_at": "2019-11-16T10:11:12.634234626Z"
        },
        "event": {
            "user_id": "1234",
            "user_login": "cool_user",
            "user_name": "Cool_User",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cooler_user",
            "broadcaster_user_name": "Cooler_User",
            "bits": 2,
            "type": "cheer",
            "power_up": null,
            "message": {
               "text": "cheer1 hi cheer1",
               "fragments": [{
                  "type": "cheermote",
                  "text": "cheer1",
                  "cheermote": {
                     "prefix": "cheer",
                     "bits": 1,
                     "tier": 1
                  },
                  "emote": null
               }, {
                  "type": "text",
                  "text": " hi ",
                  "cheermote": null,
                  "emote": null

               }, {
                  "type": "cheermote",
                  "text": "cheer1",
                  "cheermote": {
                     "prefix": "cheer",
                     "bits": 1,
                     "tier": 1
                  },
                  "emote": null
                }]
    	   }
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

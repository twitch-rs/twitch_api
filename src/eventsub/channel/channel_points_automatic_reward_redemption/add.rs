#![doc(alias = "channel.channel_points_automatic_reward_redemption.add")]
//! A viewer has redeemed an automatic channel points reward on the specified channel.

use super::*;
/// [`channel.channel_points_automatic_reward_redemption.add`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_automatic_reward_redemptionadd):a viewer has redeemed an automatic channel points reward on the specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPointsAutomaticRewardRedemptionAddV1 {
    /// The broadcaster user ID for the channel you want to receive channel points reward add notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelPointsAutomaticRewardRedemptionAddV1 {
    /// The broadcaster user ID for the channel you want to receive channel points reward add notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelPointsAutomaticRewardRedemptionAddV1 {
    type Payload = ChannelPointsAutomaticRewardRedemptionAddV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelPointsAutomaticRewardRedemptionAdd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadRedemptions,
        twitch_oauth2::Scope::ChannelManageRedemptions
    )];
    const VERSION: &'static str = "1";
}

/// [`channel.channel_points_automatic_reward_redemption.add`](ChannelPointsAutomaticRewardRedemptionAddV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPointsAutomaticRewardRedemptionAddV1Payload {
    /// The ID of the channel where the reward was redeemed.
    pub broadcaster_user_id: types::UserId,
    /// The login of the channel where the reward was redeemed.
    pub broadcaster_user_login: types::UserName,
    /// The display name of the channel where the reward was redeemed.
    pub broadcaster_user_name: types::DisplayName,
    /// The ID of the redeeming user.
    pub user_id: types::UserId,
    /// The login of the redeeming user.
    pub user_login: types::UserName,
    /// The display name of the redeeming user.
    pub user_name: types::DisplayName,
    /// The ID of the Redemption.
    pub id: types::RedemptionId,
    /// An object that contains the reward information.
    pub reward: AutomaticReward,
    /// An object that contains the user message and emote information needed to recreate the message.
    pub message: RedemptionMessage,
    /// A string that the user entered if the reward requires input.
    pub user_input: String,
    /// The UTC date and time (in RFC3339 format) of when the reward was redeemed.
    pub redeemed_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "channel.channel_points_automatic_reward_redemption.add",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "12826"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "12826",
            "broadcaster_user_name": "Twitch",
            "broadcaster_user_login": "twitch",
            "user_id": "141981764",
            "user_name": "TwitchDev",
            "user_login": "twitchdev",
            "id": "f024099a-e0fe-4339-9a0a-a706fb59f353",
            "reward": {
                "type": "send_highlighted_message",
                "cost": 100,
                "unlocked_emote": null
            },
            "message": {
                "text": "Hello world! VoHiYo",
                "emotes": [
                    {
                        "id": "81274",
                        "begin": 13,
                        "end": 18
                    }
                ]
            },
            "user_input": "Hello world! VoHiYo ",
            "redeemed_at": "2024-02-23T21:14:34.260398045Z"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelPointsAutomaticRewardRedemptionAddV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "12826");
    assert_eq!(notif.user_id.as_str(), "141981764");
    assert_eq!(notif.id.as_str(), "f024099a-e0fe-4339-9a0a-a706fb59f353");
    assert_eq!(notif.reward.cost, 100);
    assert!(notif.reward.unlocked_emote.is_none());
    assert_eq!(
        notif.reward.type_,
        AutomaticRewardType::SendHighlightedMessage
    );
    assert_eq!(notif.message.emotes.len(), 1);
}

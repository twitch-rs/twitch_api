//! Subscriptions that sends a notification when a custom channel points reward has been created for the specified channel.

use super::*;
/// The `channel.channel_points_custom_reward.add` subscription type sends a notification when a custom channel points reward has been created for the specified channel.
/// [`channel_points_custom_reward.add`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_rewardadd)
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelPointsCustomRewardAddV1 {
    /// The broadcaster user ID for the channel you want to receive channel points custom reward add notifications for.
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelPointsCustomRewardAddV1 {
    type Payload = ChannelPointsCustomRewardAddV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelPointsCustomRewardAdd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadRedemptions];
    const VERSION: &'static str = "1";
}

/// Response payload for [`channel.channel_points_custom_reward.add` version `1`](ChannelPointsCustomRewardAddV1) subscription.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelPointsCustomRewardAddV1Payload {
    pub background_color: String,
    pub broadcaster_user_id: types::UserId,
    pub broadcaster_user_name: types::UserName,
    pub cooldown_expires_at: Option<types::Timestamp>,
    pub cost: i64,
    pub default_image: Option<types::Image>,
    pub global_cooldown: types::GlobalCooldown,
    pub id: String,
    pub image: Option<types::Image>,
    pub is_enabled: bool,
    pub is_in_stock: bool,
    pub is_paused: bool,
    pub is_user_input_required: bool,
    pub max_per_stream: types::Max,
    pub max_per_user_per_stream: types::Max,
    pub prompt: String,
    pub redemptions_redeemed_current_stream: Option<u32>,
    pub should_redemptions_skip_request_queue: bool,
    pub title: String,
}

#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.channel_points_custom_reward.add",
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
            "id": "9001",
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "cool_user",
            "is_enabled": true,
            "is_paused": false,
            "is_in_stock": true,
            "title": "Cool Reward",
            "cost": 100,
            "prompt": "reward prompt",
            "is_user_input_required": true,
            "should_redemptions_skip_request_queue": false,
            "cooldown_expires_at": null,
            "redemptions_redeemed_current_stream": null,
            "max_per_stream": {
                "is_enabled": true,
                "value": 1000
            },
            "max_per_user_per_stream": {
                "is_enabled": true,
                "value": 1000
            },
            "global_cooldown": {
                "is_enabled": true,
                "seconds": 1000
            },
            "background_color": "#FA1ED2",
            "image": {
                "url_1x": "https://static-cdn.jtvnw.net/image-1.png",
                "url_2x": "https://static-cdn.jtvnw.net/image-2.png",
                "url_4x": "https://static-cdn.jtvnw.net/image-4.png"
            },
            "default_image": {
                "url_1x": "https://static-cdn.jtvnw.net/default-1.png",
                "url_2x": "https://static-cdn.jtvnw.net/default-2.png",
                "url_4x": "https://static-cdn.jtvnw.net/default-4.png"
            }
        }
    }
    "##;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

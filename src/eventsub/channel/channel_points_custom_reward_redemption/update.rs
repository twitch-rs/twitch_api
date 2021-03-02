#![doc(alias = "channel.channel_points_custom_reward_redemption.update")]
//! A redemption of a channel points custom reward has been updated for the specified channel.

use super::*;
/// [`channel.channel_points_custom_reward_redemption.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_reward_redemptionupdate): a redemption of a channel points custom reward has been updated for the specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPointsCustomRewardRedemptionUpdateV1 {
    /// The broadcaster user ID for the channel you want to receive channel points custom reward update notifications for.
    pub broadcaster_user_id: types::UserId,
    /// Optional. Specify a reward id to only receive notifications for a specific reward.
    pub reward_id: Option<types::RewardId>,
}

impl EventSubscription for ChannelPointsCustomRewardRedemptionUpdateV1 {
    type Payload = ChannelPointsCustomRewardRedemptionUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelPointsCustomRewardRedemptionUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadRedemptions];
    const VERSION: &'static str = "1";
}

/// [`channel.channel_points_custom_reward_redemption.update`](ChannelPointsCustomRewardRedemptionUpdateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPointsCustomRewardRedemptionUpdateV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The redemption identifier.
    pub id: types::RewardId,
    /// RFC3339 timestamp of when the reward was redeemed.
    pub redeemed_at: types::Timestamp,
    /// Basic information about the reward that was redeemed, at the time it was redeemed.
    pub reward: Reward,
    /// Will be fulfilled or canceled. Possible values are unknown, unfulfilled, fulfilled, and canceled.
    pub status: RedemptionStatus,
    /// User ID of the user that redeemed the reward.
    pub user_id: types::UserId,
    /// Login of the user that redeemed the reward.
    pub user_login: types::UserName,
    /// Display name of the user that redeemed the reward.
    pub user_name: types::DisplayName,
    /// The user input provided. Empty string if not provided.
    pub user_input: String,
}

#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.channel_points_custom_reward_redemption.update",
            "version": "1",
            "status": "enabled",
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
            "id": "1234",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "user_id": "9001",
            "user_login": "cooler_user",
            "user_name": "Cooler_User",
            "user_input": "pogchamp",
            "status": "fulfilled",
            "reward": {
                "id": "9001",
                "title": "title",
                "cost": 100,
                "prompt": "reward prompt"
            },
            "redeemed_at": "2020-07-15T17:16:03.17106713Z"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Payload::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

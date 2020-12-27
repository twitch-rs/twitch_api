//! PubSub messages for channel points
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A user redeems an reward using channel points.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "String", try_from = "String")]
pub struct ChannelPointsChannelV1 {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    ChannelPointsChannelV1,
    "channel-points-channel-v1",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for ChannelPointsChannelV1 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadRedemptions];
}

/// A redemption users can "buy" with channel points to trigger rewards
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Redemption {
    /// ID of channel where the redemption was triggered
    pub channel_id: types::UserId,
    /// ID of the redemption. Also returned in twitch IRC
    pub id: types::RedemptionId,
    /// Timestamp in which a reward was redeemed
    pub redeemed_at: types::Timestamp,
    /// Data about the reward that was redeemed
    pub reward: Reward,
    /// reward redemption status, will be FULFILLED if a user skips the reward queue, UNFULFILLED otherwise. ACTION_TAKEN is for `redemption-status-update`
    pub status: RedemptionStatus,
    /// User that triggered the reward
    pub user: types::User,
    /// A string that the user entered if the reward requires input
    pub user_input: Option<String>,
    /// A cursor for something
    pub cursor: Option<String>,
}

/// Status for redemption
///
/// # Note
///
/// It is currently not possible to see if a unfullfilled redemption has been fullfilled.
/// Currently only [`ACTION_TAKEN`](RedemptionStatus::ActionTaken) is given on queue changes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum RedemptionStatus {
    /// Redemption was fullfilled, e.g it skipped reward queue
    Fulfilled,
    /// Redemption is not fullfilled, e.g is in reward queue.
    Unfulfilled,
    // FIXME: https://github.com/twitchdev/issues/issues/111
    /// Redemption was updated. Rejected or completed
    #[serde(rename = "ACTION_TAKEN")]
    ActionTaken,
}

/// Reward data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Reward {
    /// Color of background in rewards & challenges screen on client
    pub background_color: String,
    /// ID of channel where the redemption was triggered
    pub channel_id: types::UserId,
    /// Cooldown will expire after this many seconds have passed from pubsub message
    pub cooldown_expires_at: Option<u64>,
    /// Cost of reward.
    pub cost: u32,
    /// Default image of reward in rewards & challenges screen on client
    pub default_image: Option<Image>,
    /// Information about global cooldown
    pub global_cooldown: GlobalCooldown,
    /// ID of reward.
    pub id: types::RewardId,
    /// Set image of reward in rewards & challenges screen on client. If none, see [`Reward::default_image`]
    pub image: Option<Image>,
    /// Reward is enabled or not.
    pub is_enabled: bool,
    /// Reward is in stock
    pub is_in_stock: bool,
    /// Reward is paused
    pub is_paused: bool,
    /// Reward is sub only
    pub is_sub_only: bool,
    /// Reward requires input from user on rewards & challenges screen on client
    pub is_user_input_required: bool,
    /// Maximum redemptions per stream
    pub max_per_stream: Max,
    /// Maximum redemptions per user per stream
    pub max_per_user_per_stream: Max,
    /// Prompt shown when clicking reward on rewards & challenges screen on client
    pub prompt: String,
    // TODO: Is this fullfilled redeemptions or is it x + 1 ? where 1 is this reward redemption
    /// Amount of times this has been redeemed this stream.
    pub redemptions_redeemed_current_stream: Option<u32>,
    /// Does redemption skip queue?
    pub should_redemptions_skip_request_queue: bool,
    /// Template ID
    pub template_id: Option<String>,
    /// Title or Name of reward
    pub title: String,
    /// Unknown
    pub updated_for_indicator_at: Option<types::Timestamp>,
}

/// Links to the same image of different sizes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Image {
    /// URL to png of size 28x28
    pub url_1x: String,
    /// URL to png of size 56x56
    pub url_2x: String,
    /// URL to png of size 112x112
    pub url_4x: String,
}

/// Information about global cooldown
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GlobalCooldown {
    /// Cooldown enabled
    pub is_enabled: bool,
    /// Cooldown amount
    pub global_cooldown_seconds: u32,
}

/// Reward redemption max
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(untagged)]
#[non_exhaustive]
pub enum Max {
    /// Max per stream
    MaxPerStream {
        /// Max per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per stream
        max_per_stream: u32,
    },
    /// Max per user per stream
    MaxPerUserPerStream {
        /// Max per user per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per user per stream
        max_per_user_per_stream: u32,
    },
}

/// `update-redemption-statuses-finished``progress
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Progress {
    /// ID of channel user
    pub channel_id: types::UserId,
    /// BASE64 representation of reward
    pub id: String,
    /// Method by which redemptions were set to new status
    // FIXME: BY_REWARD etc, need to enumify
    pub method: String,
    /// New status of redemptions
    pub new_status: RedemptionStatus,
    /// Total amount of redemptions changed
    pub processed: i64,
    /// ID of reward
    pub reward_id: types::RewardId,
    /// Total redemptions
    // FIXME: What is this?
    pub total: i64,
}

/// Reply from [ChannelPointsChannelV1]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
#[non_exhaustive]
pub enum ChannelPointsChannelV1Reply {
    /// A reward was redeemed
    #[serde(rename = "reward-redeemed")]
    RewardRedeemed {
        /// Time the pubsub message was sent
        timestamp: String,
        /// Data about the redemption, includes unique id and user that redeemed it
        redemption: Redemption,
    },
    /// A custom reward was updated
    #[serde(rename = "custom-reward-updated")]
    CustomRewardUpdated {
        /// Time the pubsub message was sent
        timestamp: String,
        /// Data about the reward that was updated
        updated_reward: Reward,
    },
    /// Status of a redemption was changed
    #[serde(rename = "redemption-status-update")]
    RedemptionStatusUpdate {
        /// Time the pubsub message was sent
        timestamp: String,
        /// Data about the reward that had status updated
        redemption: Redemption,
    },
    /// Status of multiple redemptions were changed
    // TODO: This seems to only be on complete all / reject all
    #[serde(rename = "update-redemption-statuses-finished")]
    UpdateRedemptionStatusesFinished {
        /// Time the pubsub message was sent
        timestamp: String,
        /// Data about the reward that had status updated
        progress: Progress,
    },
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;
    #[test]
    fn channel_point_redeem() {
        let message = r##"
{
    "type": "reward-redeemed",
    "data": {
        "timestamp": "2020-10-10T19:13:30.536153182Z",
        "redemption": {
            "id": "b021f290-bedb-49c2-b90f-e6ceb1c0d4ab",
            "user": {
                "id": "27620241",
                "login": "emilgardis",
                "display_name": "emilgardis"
            },
            "channel_id": "27620241",
            "redeemed_at": "2020-10-10T19:13:30.536153182Z",
            "reward": {
                "id": "252e209d-4f16-4886-a0d1-97f458ad5698",
                "channel_id": "27620241",
                "title": "Hydration",
                "prompt": "Make Emilgardis drink water",
                "cost": 2000,
                "is_user_input_required": true,
                "is_sub_only": false,
                "image": null,
                "default_image": {
                    "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                    "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                    "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
                },
                "background_color": "#81AEFF",
                "is_enabled": true,
                "is_paused": false,
                "is_in_stock": true,
                "max_per_stream": {
                    "is_enabled": false,
                    "max_per_stream": 10
                },
                "should_redemptions_skip_request_queue": false,
                "template_id": null,
                "updated_for_indicator_at": "2020-02-06T17:29:19.737311439Z",
                "max_per_user_per_stream": {
                    "is_enabled": false,
                    "max_per_user_per_stream": 0
                },
                "global_cooldown": {
                    "is_enabled": false,
                    "global_cooldown_seconds": 0
                },
                "redemptions_redeemed_current_stream": 0,
                "cooldown_expires_at": null
            },
            "user_input": "bap",
            "status": "UNFULFILLED"
        }
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE","data": {{ "topic": "channel-points-channel-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelPointsChannelV1 { .. },
            }
        ));
    }

    #[test]
    fn channel_reward_updated() {
        let message = r##"
{
    "type": "custom-reward-updated",
    "data": {
        "timestamp": "2020-10-19T19:40:19.637568468Z",
        "updated_reward": {
            "id": "071397fb-cd09-420d-8d64-f9fd35f5cdfa",
            "channel_id": "27620241",
            "title": "Up the difficulty ",
            "prompt": "stuff.",
            "cost": 20000,
            "is_user_input_required": true,
            "is_sub_only": false,
            "image": null,
            "default_image": {
                "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
            },
            "background_color": "#FF6C00",
            "is_enabled": true,
            "is_paused": true,
            "is_in_stock": true,
            "max_per_stream": {
                "is_enabled": true,
                "max_per_stream": 6
            },
            "should_redemptions_skip_request_queue": false,
            "template_id": null,
            "updated_for_indicator_at": "2020-06-09T16:02:06.943429808Z",
            "max_per_user_per_stream": {
                "is_enabled": false,
                "max_per_user_per_stream": 0
            },
            "global_cooldown": {
                "is_enabled": false,
                "global_cooldown_seconds": 0
            },
            "redemptions_redeemed_current_stream": 0,
            "cooldown_expires_at": null
        }
    }
}
        "##;

        let source = format!(
            r#"{{"type": "MESSAGE","data": {{ "topic": "channel-points-channel-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelPointsChannelV1 { .. },
            }
        ));
    }

    #[test]
    fn redemption_status_update() {
        let message = r##"
{
    "type": "redemption-status-update",
    "data": {
        "timestamp": "2020-10-19T19:41:00.590084358Z",
        "redemption": {
            "id": "844fff0c-6185-44c7-8c30-3d68a565fe1b",
            "user": {
                "id": "27620241",
                "login": "emilgardis",
                "display_name": "emilgardis"
            },
            "channel_id": "27620241",
            "redeemed_at": "2020-10-19T15:01:18.453334233Z",
            "reward": {
                "id": "252e209d-4f16-4886-a0d1-97f458ad5698",
                "channel_id": "27620241",
                "title": "Hydration",
                "prompt": "Make Emilgardis drink water",
                "cost": 2000,
                "is_user_input_required": false,
                "is_sub_only": false,
                "image": null,
                "default_image": null,
                "background_color": "#81AEFF",
                "is_enabled": false,
                "is_paused": false,
                "is_in_stock": false,
                "max_per_stream": {
                    "is_enabled": false,
                    "max_per_stream": 0
                },
                "should_redemptions_skip_request_queue": false,
                "template_id": null,
                "updated_for_indicator_at": null,
                "max_per_user_per_stream": {
                    "is_enabled": false,
                    "max_per_user_per_stream": 0
                },
                "global_cooldown": {
                    "is_enabled": false,
                    "global_cooldown_seconds": 0
                },
                "redemptions_redeemed_current_stream": null,
                "cooldown_expires_at": null
            },
            "user_input": "a",
            "status": "ACTION_TAKEN",
            "cursor": "ODQ0ZmZmMGMtNjE4NS00NGM3LThjMzAtM2Q2OGE1NjVmZTFiX18yMDIwLTEwLTE5VDE1OjAxOjE4LjQ1MzMzNDIzM1o="
        }
    }
}
        "##;

        let source = format!(
            r#"{{"type": "MESSAGE","data": {{ "topic": "channel-points-channel-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelPointsChannelV1 { .. },
            }
        ));
    }

    #[test]
    fn update_redemption_statuses_finished() {
        let message = r##"
{
    "type": "update-redemption-statuses-finished",
    "data": {
        "timestamp": "2020-12-21T02:25:21.717263168Z",
        "progress": {
            "id": "Y29wb0J1bGtFZGl0UmVkZW1wdGlvblN0YXR1c1Byb2dyZXNzOjEzMzc6QlVMS19FRElUX1JFREVNUFRJT05fU1RBVFVTX01FVEhPRF9CWV9SRVdBUkQ6ZGVhZGJlZWY=",
            "channel_id": "1337",
            "reward_id": "deadbeef",
            "method": "BY_REWARD",
            "new_status": "FULFILLED",
            "processed": 5,
            "total": 5
        }
    }
}
        "##;

        let source = format!(
            r#"{{"type": "MESSAGE","data": {{ "topic": "channel-points-channel-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelPointsChannelV1 { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "channel-points-channel-v1.1234";
        assert_eq!(
            ChannelPointsChannelV1 { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "channel-points-channel-v1.1234";
        let right: String = ChannelPointsChannelV1 { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

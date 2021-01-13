#![doc(alias = "hype-train-events-v1")]
//! PubSub messages for hype-trains
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A user redeems an reward using channel points.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(into = "String", try_from = "String")]
pub struct HypeTrainEventsV1 {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    HypeTrainEventsV1,
    "hype-train-events-v1",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for HypeTrainEventsV1 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// A user redeems an reward using channel points.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(into = "String", try_from = "String")]
pub struct HypeTrainEventsV1Rewards {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    HypeTrainEventsV1Rewards,
    "hype-train-events-v1.rewards",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for HypeTrainEventsV1Rewards {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Hype train rewards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainRewards {
    // FIXME: Channel ID is sometimes missing, might be depending on your token
    /// ID of channel where hype-train was initiated
    pub channel_id: Option<types::UserId>,
    /// Level of hype-train that was initiated
    pub completed_level: i64,
    /// Rewards
    pub rewards: Vec<Reward>,
}

/// Hype train started in channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainStart {
    // FIXME: Channel ID is sometimes missing, might be depending on your token
    /// ID of channel where hype-train was initiated
    pub channel_id: Option<types::UserId>,
    /// Current conductors of this hype-train
    #[doc(hidden)]
    pub conductors: Conductors,
    /// Config of this hype-train
    pub config: Box<Config>,
    #[doc(hidden)]
    #[serde(default)]
    pub ended_at: (),
    #[doc(hidden)]
    #[serde(default)]
    pub ending_reason: (),
    /// Server time epoch in milliseconds when hype train ends
    pub expires_at: Option<i64>,
    /// ID of hype train
    pub id: Option<String>,
    /// Participations in hype train
    pub participations: Participations,
    //#[serde(default)]
    /// Progress of hype train
    pub progress: Box<HypeTrainProgress>,
    /// Server time epoch in milliseconds when hype train started
    pub started_at: Option<i64>,
    /// Server time epoch in milliseconds when hype train was updated
    pub updated_at: Option<i64>,
}

/// Hype train ended
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainEnd {
    /// Server time epoch in milliseconds when hype train ended
    pub ended_at: i64,
    /// Reason why hype train ended
    pub ending_reason: EndingReason,
}

/// Hype train conductor updated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainConductorUpdate {
    /// Conductor source
    pub source: SourceType,
    /// User information of conductor
    pub user: types::User,
    /// Participations in hype train
    pub participations: Participations,
}

/// Hype train progression. Akin to [Participations]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainProgression {
    /// Unknown
    pub sequence_id: i64,
    /// Source type of progression
    pub source: SourceType,
    /// Action done to progress
    pub action: ActionType,
    /// Amount of actions done. i.e 500 (five-hundred) bits or 1 (one) tier 3 gift sub
    pub quantity: i64,
    /// Progress of hype train
    pub progress: HypeTrainProgress,
    // FIXME: Should use flatten here on types::User, but https://github.com/serde-rs/serde/issues/1504
    /// Id of the user
    pub user_id: types::UserId,
    /// Login name of the user, not capitalized
    pub user_login: types::UserName,
    /// Display name of user
    pub user_display_name: types::DisplayName,
    // FIXME: 2020-11-05 I suspect this will always be returned
    /// Profile picture of user
    pub user_profile_image_url: Option<String>,
}

/// Hype train leveled up
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HypeTrainLevelUp {
    /// Server time epoch in milliseconds when hype train expires
    pub time_to_expire: i64,
    /// Progress of hype train
    pub progress: HypeTrainProgress,
}

/// Reply from [HypeTrainEventsV1] or [HypeTrainEventsV1Rewards]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(tag = "type", content = "data")]
#[non_exhaustive]
pub enum HypeTrainEventsV1Reply {
    /// Hype train rewards
    #[serde(rename = "hype-train-rewards")]
    HypeTrainRewards(HypeTrainRewards),
    /// Hype train started in channel
    #[serde(rename = "hype-train-start")]
    HypeTrainStart(HypeTrainStart),
    /// Hype train ended
    #[serde(rename = "hype-train-end")]
    HypeTrainEnd(HypeTrainEnd),
    /// Hype train cooldown expired
    #[serde(rename = "hype-train-cooldown-expiration")]
    HypeTrainCooldownExpiration(#[doc(hidden)] Option<()>),
    /// Hype train conductor updated
    #[serde(rename = "hype-train-conductor-update")]
    HypeTrainConductorUpdate(HypeTrainConductorUpdate),
    /// Hype train progression. Akin to [Participations]
    #[serde(rename = "hype-train-progression")]
    HypeTrainProgression(HypeTrainProgression),
    /// Hype train leveled up
    #[serde(rename = "hype-train-level-up")]
    HypeTrainLevelUp(HypeTrainLevelUp),
}

/// Configuration of hype train
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Config {
    /// Hype train public callout emote ID
    pub callout_emote_id: String,
    /// Hype train public callout emote token
    pub callout_emote_token: String,
    // FIXME: Channel ID is sometimes missing, might be depending on your token
    /// ID of channel
    pub channel_id: Option<types::UserId>,
    /// Rewards for conductors
    pub conductor_rewards: ConductorRewards,
    /// Cooldown duration in nanoseconds for hype train
    pub cooldown_duration: i64,
    /// Difficulty of hype train
    pub difficulty: HypeTrainDifficulty,
    /// Difficulty settings
    pub difficulty_settings: std::collections::HashMap<HypeTrainDifficulty, Vec<Level>>,
    #[doc(hidden)]
    #[serde(default = "bool_true")]
    pub has_conductor_badges: bool,
    /// Whether or not Hype Train is enabled in channel
    pub is_enabled: bool,
    #[doc(hidden)]
    pub is_whitelisted: bool,
    /// Support events that must occur within a duration of time to kick off a Hype Train.
    pub kickoff: Kickoff,
    /// Duration in nanoseconds of each level
    pub level_duration: i64,
    /// Thresholds for notifications
    pub notification_thresholds: NotificationThresholds,
    /// Conversion rates for participations
    pub participation_conversion_rates: ParticipationConversionRates,
    #[doc(hidden)]
    pub reward_end_date: (),
    /// Theme color of channel
    ///
    /// None if use_theme_color is set to false
    pub theme_color: Option<String>,
    /// Primary color of hex
    pub primary_hex_color: Option<String>,
    // FIXME: 2020-11-22 I suspect this will always be returned
    /// Uses personalized settings
    pub use_personalized_settings: Option<bool>,
    /// Use theme color or not
    pub use_theme_color: Option<bool>,
    /// Use creator color or not
    pub use_creator_color: Option<bool>,
}

/// Difficulty of Hype Train
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HypeTrainDifficulty {
    /// Easy difficulty
    #[serde(rename = "EASY")]
    Easy,
    /// Medium difficulty
    #[serde(rename = "MEDIUM")]
    Medium,
    /// Hard difficulty
    #[serde(rename = "HARD")]
    Hard,
    /// Super Hard difficulty
    #[serde(rename = "SUPER HARD")]
    SuperHard,
    /// Insane difficulty
    #[serde(rename = "INSANE")]
    Insane,
}

/// How many support events needed to start Hype Train
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Kickoff {
    /// Period in nanoseconds that events must occur
    #[serde(default)]
    pub duration: i64,
    /// Minimum participation points needed to kickoff hypetrain
    pub min_points: i64,
    /// Number of events needed to kickoff hypetrain
    pub num_of_events: i64,
}

/// Conversion table of event to participation points
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ParticipationConversionRates {
    /// Bits
    #[serde(rename = "BITS.CHEER")]
    pub bits_cheer: i64,
    /// Bits by extension
    ///
    /// # Notes
    ///
    /// This seems to be for future usage. Right now, the helix endpoint doesn't have this.
    #[serde(rename = "BITS.EXTENSION")]
    pub bits_extension: i64,
    /// Bits by poll
    ///
    /// # Notes
    ///
    /// This seems to be for future usage. Right now, the helix endpoint doesn't have this.
    #[serde(rename = "BITS.POLL")]
    pub bits_poll: i64,
    /// Tier 1 gifted subs
    #[serde(rename = "SUBS.TIER_1_GIFTED_SUB")]
    pub subs_tier_1_gifted_sub: i64,
    /// Tier 1 sub
    #[serde(rename = "SUBS.TIER_1_SUB")]
    pub subs_tier_1_sub: i64,
    /// Tier 2 gifted subs
    #[serde(rename = "SUBS.TIER_2_GIFTED_SUB")]
    pub subs_tier_2_gifted_sub: i64,
    /// Tier 2 sub
    #[serde(rename = "SUBS.TIER_2_SUB")]
    pub subs_tier_2_sub: i64,
    /// Tier 3 gifted subs
    #[serde(rename = "SUBS.TIER_3_GIFTED_SUB")]
    pub subs_tier_3_gifted_sub: i64,
    /// Tier 3 sub
    #[serde(rename = "SUBS.TIER_3_SUB")]
    pub subs_tier_3_sub: i64,
}

/// Thresholds for notifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct NotificationThresholds {
    /// Bits
    #[serde(rename = "BITS.CHEER")]
    pub bits_cheer: i64,
    /// Bits by extension
    ///
    /// # Notes
    ///
    /// This seems to be for future usage. Right now, the helix endpoint doesn't have this.
    #[serde(rename = "BITS.EXTENSION")]
    pub bits_extension: i64,
    /// Bits by poll
    ///
    /// # Notes
    ///
    /// This seems to be for future usage. Right now, the helix endpoint doesn't have this.
    #[serde(rename = "BITS.POLL")]
    pub bits_poll: i64,
    /// Tier 1 gifted subs
    #[serde(rename = "SUBS.TIER_1_GIFTED_SUB")]
    pub subs_tier_1_gifted_sub: i64,
    /// Tier 1 sub
    #[serde(rename = "SUBS.TIER_1_SUB")]
    pub subs_tier_1_sub: i64,
    /// Tier 2 gifted subs
    #[serde(rename = "SUBS.TIER_2_GIFTED_SUB")]
    pub subs_tier_2_gifted_sub: i64,
    /// Tier 2 sub
    #[serde(rename = "SUBS.TIER_2_SUB")]
    pub subs_tier_2_sub: i64,
    /// Tier 3 gifted subs
    #[serde(rename = "SUBS.TIER_3_GIFTED_SUB")]
    pub subs_tier_3_gifted_sub: i64,
    /// Tier 3 sub
    #[serde(rename = "SUBS.TIER_3_SUB")]
    pub subs_tier_3_sub: i64,
}

/// Reward given to conductors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ConductorRewards {
    /// Reward to conductor of bits
    #[serde(rename = "BITS")]
    pub bits: BitsRewards,
    /// Reward to conductor of subscriptions
    #[serde(rename = "SUBS")]
    pub subs: SubsRewards,
}

/// Rewards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BitsRewards {
    /// Rewards to bits conductor
    #[serde(rename = "CURRENT")]
    pub current: Vec<Reward>,
    /// Rewards to former bits conductor
    #[serde(rename = "FORMER")]
    pub former: Vec<Reward>,
}

/// Rewards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SubsRewards {
    /// Rewards to subscriptions conductor
    #[serde(rename = "CURRENT")]
    pub current: Vec<Reward>,
    /// Rewards to former subscriptions conductor
    #[serde(rename = "FORMER")]
    pub former: Vec<Reward>,
}

/// Participations in hype train
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Participations {
    /// Bits
    #[serde(rename = "BITS.CHEER")]
    pub bits_cheer: Option<i64>,
    /// Bits by extension
    ///
    /// # Notes
    ///
    /// This seems to be for future usage. Right now, the helix endpoint doesn't have this.
    #[serde(rename = "BITS.EXTENSION")]
    pub bits_extension: Option<i64>,
    /// Bits by poll
    ///
    /// # Notes
    ///
    /// This seems to be for future usage. Right now, the helix endpoint doesn't have this.
    #[serde(rename = "BITS.POLL")]
    pub bits_poll: Option<i64>,
    /// Tier 1 gifted subs
    #[serde(rename = "SUBS.TIER_1_GIFTED_SUB")]
    pub subs_tier_1_gifted_sub: Option<i64>,
    /// Tier 1 sub
    #[serde(rename = "SUBS.TIER_1_SUB")]
    pub subs_tier_1_sub: Option<i64>,
    /// Tier 2 gifted subs
    #[serde(rename = "SUBS.TIER_2_GIFTED_SUB")]
    pub subs_tier_2_gifted_sub: Option<i64>,
    /// Tier 2 sub
    #[serde(rename = "SUBS.TIER_2_SUB")]
    pub subs_tier_2_sub: Option<i64>,
    /// Tier 3 gifted subs
    #[serde(rename = "SUBS.TIER_3_GIFTED_SUB")]
    pub subs_tier_3_gifted_sub: Option<i64>,
    /// Tier 3 sub
    #[serde(rename = "SUBS.TIER_3_SUB")]
    pub subs_tier_3_sub: Option<i64>,
}

/// Unknown
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Conductors {}

/// Progress of hype train
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainProgress {
    /// Participation points needed in this level
    pub goal: i64,
    /// Current level
    pub level: Level,
    /// Time left
    #[serde(default)]
    pub remaining_seconds: i64,
    /// Current amassed participation points in this level
    pub total: i64,
    /// Current total amassed participation points in all levels
    pub value: i64,
}

/// Description of a hype-train level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Level {
    /// Participation points needed for this level
    pub goal: i64,
    /// Description of level rewards
    pub rewards: Vec<Reward>,
    // FIXME: Should maybe be an enum
    /// Integer value of reward. 1-5
    pub value: i64,
}

/// A reward
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Reward {
    /// Reward is a emote
    #[serde(rename = "EMOTE")]
    Emote {
        /// Group ID of emote
        group_id: String,
        /// ID of emote
        id: String,
        /// Level that this emote is from
        reward_level: i64,
        // FIXME: Token and set_id seems to only be present on hype-train-start
        /// Token for this emote
        token: Option<String>,
        /// ID of emote set
        set_id: Option<String>,
    },
    /// Reward is a badge
    #[serde(rename = "BADGE")]
    Badge {
        /// ID of badge
        badge_id: String,
        /// Group ID of badge
        group_id: String,
        /// ID of badge
        id: String,
        /// URL to image of badge
        image_url: String,
        /// Level that this badge is from
        reward_level: i64,
    },
}

// FIXME: intradoclink to helix
/// Source type, same as helix hype-train events type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SourceType {
    /// Bits
    Bits,
    /// Subs
    Subs,
}

/// The specific action that was used
// FIXME: Might be the same as channel_bits::BitsContext or channel_cheer::TriggerType
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionType {
    /// A cheer
    Cheer,
    /// Tier 1 sub
    #[serde(rename = "TIER_1_SUB")]
    Tier1,
    /// Tier 2 sub
    #[serde(rename = "TIER_2_SUB")]
    Tier2,
    /// Tier 3 sub
    #[serde(rename = "TIER_3_SUB")]
    Tier3,
    /// Tier 1 gifted sub
    #[serde(rename = "TIER_1_GIFTED_SUB")]
    Tier1GiftedSub,
    /// Tier 2 gifted sub
    #[serde(rename = "TIER_2_GIFTED_SUB")]
    Tier2GiftedSub,
    /// Tier 3 gifted sub
    #[serde(rename = "TIER_3_GIFTED_SUB")]
    Tier3GiftedSub,
}

/// Reason hype-train ended
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EndingReason {
    /// Hype train was completed
    Completed,
    /// Hype train ended
    Expired,
}

pub(crate) fn bool_true() -> bool { true }

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;

    #[test]
    fn hype_train_start1() {
        let message = r##"
{
    "type": "hype-train-start",
    "data": {
        "channel_id": "1234",
        "id": "4375b14c-acae-4ce4-9ef1-800482bb6022",
        "started_at": 1603127828000,
        "expires_at": 1603128128000,
        "updated_at": 1603127828000,
        "ended_at": null,
        "ending_reason": null,
        "config": {
            "channel_id": "1234",
            "is_enabled": true,
            "is_whitelisted": true,
            "kickoff": {
                "num_of_events": 4,
                "min_points": 100,
                "duration": 300000000000
            },
            "cooldown_duration": 7200000000000,
            "level_duration": 300000000000,
            "difficulty": "MEDIUM",
            "reward_end_date": null,
            "participation_conversion_rates": {
                "BITS.CHEER": 1,
                "BITS.EXTENSION": 1,
                "BITS.POLL": 1,
                "SUBS.TIER_1_GIFTED_SUB": 500,
                "SUBS.TIER_1_SUB": 500,
                "SUBS.TIER_2_GIFTED_SUB": 1000,
                "SUBS.TIER_2_SUB": 1000,
                "SUBS.TIER_3_GIFTED_SUB": 2500,
                "SUBS.TIER_3_SUB": 2500
            },
            "notification_thresholds": {
                "BITS.CHEER": 1000,
                "BITS.EXTENSION": 1000,
                "BITS.POLL": 1000,
                "SUBS.TIER_1_GIFTED_SUB": 5,
                "SUBS.TIER_1_SUB": 5,
                "SUBS.TIER_2_GIFTED_SUB": 5,
                "SUBS.TIER_2_SUB": 5,
                "SUBS.TIER_3_GIFTED_SUB": 5,
                "SUBS.TIER_3_SUB": 5
            },
            "difficulty_settings": {
                "MEDIUM": [
                    {
                        "value": 1,
                        "goal": 2000,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739462",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeChimp"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739463",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeGhost"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739465",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeChest"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739466",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeFrog"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739468",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeCherry"
                            }
                        ]
                    },
                    {
                        "value": 2,
                        "goal": 4500,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739479",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeSideeye"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739472",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBrain"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739475",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeZap"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739476",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeShip"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739478",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeSign"
                            }
                        ]
                    },
                    {
                        "value": 3,
                        "goal": 7600,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739481",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeYikes"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739482",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeRacer"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739483",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeCar"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739484",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeFirst"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739485",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeTrophy"
                            }
                        ]
                    },
                    {
                        "value": 4,
                        "goal": 11500,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739489",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBlock"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739490",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeDaze"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739491",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBounce"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739492",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeJewel"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739493",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBlob"
                            }
                        ]
                    },
                    {
                        "value": 5,
                        "goal": 17000,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739495",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeLove"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739496",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypePunk"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739497",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeKO"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739499",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypePunch"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739501",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeFire"
                            }
                        ]
                    }
                ]
            },
            "conductor_rewards": {
                "BITS": {
                    "CURRENT": [
                        {
                            "type": "BADGE",
                            "id": "1",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsxOzgwNTI1Nzk5",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/fae4086c-3190-44d4-83c8-8ef0cbe1a515/2"
                        }
                    ],
                    "FORMER": [
                        {
                            "type": "BADGE",
                            "id": "2",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsyOzgwNTI1Nzk5",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/9c8d038a-3a29-45ea-96d4-5031fb1a7a81/2"
                        }
                    ]
                },
                "SUBS": {
                    "CURRENT": [
                        {
                            "type": "BADGE",
                            "id": "1",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsxOzgwNTI1Nzk5",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/fae4086c-3190-44d4-83c8-8ef0cbe1a515/2"
                        }
                    ],
                    "FORMER": [
                        {
                            "type": "BADGE",
                            "id": "2",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsyOzgwNTI1Nzk5",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/9c8d038a-3a29-45ea-96d4-5031fb1a7a81/2"
                        }
                    ]
                }
            },
            "callout_emote_id": "300640072",
            "callout_emote_token": "PogChamp",
            "theme_color": "#a970ff",
            "has_conductor_badges": true
        },
        "participations": {
            "SUBS.TIER_1_GIFTED_SUB": 2,
            "SUBS.TIER_1_SUB": 1,
            "SUBS.TIER_3_SUB": 1
        },
        "conductors": {},
        "progress": {
            "level": {
                "value": 2,
                "goal": 4500,
                "rewards": [
                    {
                        "type": "EMOTE",
                        "id": "301739479",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeSideeye"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739472",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeBrain"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739475",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeZap"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739476",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeShip"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739478",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeSign"
                    }
                ]
            },
            "value": 2000,
            "goal": 2500,
            "total": 4000,
            "remaining_seconds": 299
        }
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn hype_train_start2() {
        let message = r##"
{
    "type": "hype-train-start",
    "data": {
        "config": {
            "channel_id": "1234",
            "is_enabled": true,
            "is_whitelisted": true,
            "kickoff": {
                "num_of_events": 4,
                "min_points": 100,
                "duration": 300000000000
            },
            "cooldown_duration": 7200000000000,
            "level_duration": 300000000000,
            "difficulty": "MEDIUM",
            "reward_end_date": null,
            "participation_conversion_rates": {
                "BITS.CHEER": 1,
                "BITS.EXTENSION": 1,
                "BITS.POLL": 1,
                "SUBS.TIER_1_GIFTED_SUB": 500,
                "SUBS.TIER_1_SUB": 500,
                "SUBS.TIER_2_GIFTED_SUB": 1000,
                "SUBS.TIER_2_SUB": 1000,
                "SUBS.TIER_3_GIFTED_SUB": 2500,
                "SUBS.TIER_3_SUB": 2500
            },
            "notification_thresholds": {
                "BITS.CHEER": 1000,
                "BITS.EXTENSION": 1000,
                "BITS.POLL": 1000,
                "SUBS.TIER_1_GIFTED_SUB": 5,
                "SUBS.TIER_1_SUB": 5,
                "SUBS.TIER_2_GIFTED_SUB": 5,
                "SUBS.TIER_2_SUB": 5,
                "SUBS.TIER_3_GIFTED_SUB": 5,
                "SUBS.TIER_3_SUB": 5
            },
            "difficulty_settings": {
                "MEDIUM": [
                    {
                        "value": 1,
                        "goal": 2000,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739462",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeChimp"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739463",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeGhost"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739465",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeChest"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739466",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeFrog"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739468",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeCherry"
                            }
                        ]
                    },
                    {
                        "value": 2,
                        "goal": 4500,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739479",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeSideeye"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739472",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBrain"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739475",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeZap"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739476",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeShip"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739478",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeSign"
                            }
                        ]
                    },
                    {
                        "value": 3,
                        "goal": 7600,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739481",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeYikes"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739482",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeRacer"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739483",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeCar"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739484",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeFirst"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739485",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeTrophy"
                            }
                        ]
                    },
                    {
                        "value": 4,
                        "goal": 11500,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739489",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBlock"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739490",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeDaze"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739491",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBounce"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739492",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeJewel"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739493",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBlob"
                            }
                        ]
                    },
                    {
                        "value": 5,
                        "goal": 17000,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739495",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeLove"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739496",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypePunk"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739497",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeKO"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739499",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypePunch"
                            },
                            {
                                "type": "EMOTE",
                                "id": "301739501",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeFire"
                            }
                        ]
                    }
                ]
            },
            "conductor_rewards": {
                "BITS": {
                    "CURRENT": [
                        {
                            "type": "BADGE",
                            "id": "1",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsxOzgwNTI1Nzk5",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/fae4086c-3190-44d4-83c8-8ef0cbe1a515/2"
                        }
                    ],
                    "FORMER": [
                        {
                            "type": "BADGE",
                            "id": "2",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsyOzgwNTI1Nzk5",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/9c8d038a-3a29-45ea-96d4-5031fb1a7a81/2"
                        }
                    ]
                },
                "SUBS": {
                    "CURRENT": [
                        {
                            "type": "BADGE",
                            "id": "1",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsxOzgwNTI1Nzk5",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/fae4086c-3190-44d4-83c8-8ef0cbe1a515/2"
                        }
                    ],
                    "FORMER": [
                        {
                            "type": "BADGE",
                            "id": "2",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsyOzgwNTI1Nzk5",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/9c8d038a-3a29-45ea-96d4-5031fb1a7a81/2"
                        }
                    ]
                }
            },
            "callout_emote_id": "300640072",
            "callout_emote_token": "sessPog",
            "use_theme_color": false,
            "has_conductor_badges": true,
            "primary_hex_color": "5247FF",
            "use_creator_color": true
        },
        "participations": {
            "BITS.CHEER": 419,
            "SUBS.TIER_1_SUB": 2
        },
        "conductors": {},
        "progress": {
            "level": {
                "value": 1,
                "goal": 2000,
                "rewards": [
                    {
                        "type": "EMOTE",
                        "id": "301739462",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeChimp"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739463",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeGhost"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739465",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeChest"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739466",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeFrog"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739468",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeCherry"
                    }
                ]
            },
            "value": 1419,
            "goal": 2000,
            "total": 1419,
            "remaining_seconds": 299
        }
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn hype_train_conductor_update() {
        let message = r##"
{
    "type": "hype-train-conductor-update",
    "data": {
        "source": "BITS",
        "user": {
            "id": "1234",
            "login": "tmi",
            "display_name": "TMI"
        },
        "participations": {
            "BITS.CHEER": 101,
            "SUBS.TIER_1_SUB": 1
        }
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn hype_train_progression1() {
        let message = r##"
{
    "type": "hype-train-progression",
    "data": {
        "user_id": "52309415",
        "user_login": "tmi",
        "user_display_name": "TMI",
        "sequence_id": 4101,
        "action": "TIER_1_GIFTED_SUB",
        "source": "BITS",
        "quantity": 101,
        "progress": {
            "level": {
                "value": 2,
                "goal": 4500,
                "rewards": [
                    {
                        "type": "EMOTE",
                        "id": "301739479",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeSideeye"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739472",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeBrain"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739475",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeZap"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739476",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeShip"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739478",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeSign"
                    }
                ]
            },
            "value": 2101,
            "goal": 2500,
            "total": 4101,
            "remaining_seconds": 252
        }
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn hype_train_progression2() {
        let message = r##"
{
    "type": "hype-train-progression",
    "data": {
        "user_id": "1234",
        "user_login": "tmi",
        "user_display_name": "TMI",
        "user_profile_image_url": "https://static-cdn.jtvnw.net/user-default-pictures-uv/deadbeaf-profile_image-50x50.png",
        "sequence_id": 6500,
        "action": "TIER_1_SUB",
        "source": "SUBS",
        "quantity": 1,
        "progress": {
            "level": {
                "value": 2,
                "goal": 12500,
                "rewards": [
                    {
                        "type": "EMOTE",
                        "id": "301739479",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeSideeye"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739472",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeBrain"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739475",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeZap"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739476",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeShip"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739478",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeSign"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739471",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeBug"
                    }
                ]
            },
            "value": 1500,
            "goal": 7500,
            "total": 6500,
            "remaining_seconds": 237
        }
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn hype_train_level_up() {
        let message = r##"
{
    "type": "hype-train-level-up",
    "data": {
        "time_to_expire": 1603128256000,
        "progress": {
            "level": {
                "value": 3,
                "goal": 7600,
                "rewards": [
                    {
                        "type": "EMOTE",
                        "id": "301739481",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeYikes"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739482",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeRacer"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739483",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeCar"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739484",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeFirst"
                    },
                    {
                        "type": "EMOTE",
                        "id": "301739485",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeTrophy"
                    }
                ]
            },
            "value": 101,
            "goal": 3100,
            "total": 4601,
            "remaining_seconds": 299
        }
    }
}
"##;
        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn hype_train_end_completed() {
        let message = r##"
{
    "type": "hype-train-end",
    "data": {
        "ended_at": 1603128366000,
        "ending_reason": "COMPLETED"
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn hype_train_end_expired() {
        let message = r##"
{
    "type": "hype-train-end",
    "data": {
        "ended_at": 1603314902000,
        "ending_reason": "EXPIRED"
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn hype_train_rewards() {
        let message = r##"
{
    "type": "hype-train-rewards",
    "data": {
        "channel_id": "27620241",
        "completed_level": 4,
        "rewards": [
            {
                "type": "EMOTE",
                "id": "",
                "group_id": "",
                "reward_level": 4
            }
        ]
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.rewards.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1Rewards { .. },
            }
        ));
    }

    #[test]
    fn hype_train_redeemed_settings() {
        let message = r##"
{
    "type": "hype-train-start",
    "data": {
        "channel_id": "233300375",
        "id": "39042897-10cd-4944-a056-e6c7fc6f54d2",
        "started_at": 1605378998000,
        "expires_at": 1605379298000,
        "updated_at": 1605378998000,
        "ended_at": null,
        "ending_reason": null,
        "config": {
            "channel_id": "233300375",
            "is_enabled": true,
            "is_whitelisted": true,
            "kickoff": {
                "num_of_events": 6,
                "min_points": 100,
                "duration": 300000000000
            },
            "cooldown_duration": 7200000000000,
            "level_duration": 300000000000,
            "difficulty": "SUPER HARD",
            "reward_end_date": null,
            "participation_conversion_rates": {
                "BITS.CHEER": 1,
                "BITS.EXTENSION": 1,
                "BITS.POLL": 1,
                "SUBS.TIER_1_GIFTED_SUB": 500,
                "SUBS.TIER_1_SUB": 500,
                "SUBS.TIER_2_GIFTED_SUB": 1000,
                "SUBS.TIER_2_SUB": 1000,
                "SUBS.TIER_3_GIFTED_SUB": 2500,
                "SUBS.TIER_3_SUB": 2500
            },
            "notification_thresholds": {
                "BITS.CHEER": 1000,
                "BITS.EXTENSION": 1000,
                "BITS.POLL": 1000,
                "SUBS.TIER_1_GIFTED_SUB": 5,
                "SUBS.TIER_1_SUB": 5,
                "SUBS.TIER_2_GIFTED_SUB": 5,
                "SUBS.TIER_2_SUB": 5,
                "SUBS.TIER_3_GIFTED_SUB": 5,
                "SUBS.TIER_3_SUB": 5
            },
            "difficulty_settings": {
                "SUPER HARD": [
                    {
                        "value": 1,
                        "goal": 5000,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739462",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeChimp"
                            }
                        ]
                    },
                    {
                        "value": 2,
                        "goal": 12500,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739479",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeSideeye"
                            }
                        ]
                    },
                    {
                        "value": 3,
                        "goal": 23100,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739481",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeYikes"
                            }
                        ]
                    },
                    {
                        "value": 4,
                        "goal": 37700,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739489",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeBlock"
                            }
                        ]
                    },
                    {
                        "value": 5,
                        "goal": 60000,
                        "rewards": [
                            {
                                "type": "EMOTE",
                                "id": "301739495",
                                "group_id": "",
                                "reward_level": 0,
                                "set_id": "301040478",
                                "token": "HypeLove"
                            }
                        ]
                    }
                ]
            },
            "conductor_rewards": {
                "BITS": {
                    "CURRENT": [
                        {
                            "type": "BADGE",
                            "id": "1",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsxOzIzMzMwMDM3NQ==",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/fae4086c-3190-44d4-83c8-8ef0cbe1a515/2"
                        }
                    ],
                    "FORMER": [
                        {
                            "type": "BADGE",
                            "id": "2",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsyOzIzMzMwMDM3NQ==",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/9c8d038a-3a29-45ea-96d4-5031fb1a7a81/2"
                        }
                    ]
                },
                "SUBS": {
                    "CURRENT": [
                        {
                            "type": "BADGE",
                            "id": "1",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsxOzIzMzMwMDM3NQ==",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/fae4086c-3190-44d4-83c8-8ef0cbe1a515/2"
                        }
                    ],
                    "FORMER": [
                        {
                            "type": "BADGE",
                            "id": "2",
                            "group_id": "hype-train",
                            "reward_level": 0,
                            "badge_id": "aHlwZS10cmFpbjsyOzIzMzMwMDM3NQ==",
                            "image_url": "https://static-cdn.jtvnw.net/badges/v1/9c8d038a-3a29-45ea-96d4-5031fb1a7a81/2"
                        }
                    ]
                }
            },
            "callout_emote_id": "88",
            "callout_emote_token": "PogChamp",
            "use_creator_color": true,
            "primary_hex_color": "",
            "use_personalized_settings": false,
            "has_conductor_badges": true
        },
        "participations": {
            "SUBS.TIER_1_SUB": 6
        },
        "conductors": {},
        "progress": {
            "level": {
                "value": 1,
                "goal": 5000,
                "rewards": [
                    {
                        "type": "EMOTE",
                        "id": "301739462",
                        "group_id": "",
                        "reward_level": 0,
                        "set_id": "301040478",
                        "token": "HypeChimp"
                    }
                ]
            },
            "value": 3000,
            "goal": 5000,
            "total": 3000,
            "remaining_seconds": 299
        }
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "hype-train-events-v1.rewards.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1Rewards { .. },
            }
        ));
    }

    #[test]
    fn hype_train_cooldown() {
        let source = r#"{"type":"MESSAGE","data":{"topic":"hype-train-events-v1.233300375","message":"{\"type\":\"hype-train-cooldown-expiration\"}"}}"#;
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::HypeTrainEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "hype-train-events-v1.1234";
        assert_eq!(
            HypeTrainEventsV1 { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
        let s = "hype-train-events-v1.rewards.1234";
        assert_eq!(
            HypeTrainEventsV1Rewards { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "hype-train-events-v1.1234";
        let right: String = HypeTrainEventsV1 { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
        let s = "hype-train-events-v1.rewards.1234";
        let right: String = HypeTrainEventsV1Rewards { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

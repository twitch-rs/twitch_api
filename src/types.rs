//! Twitch types
//!

use serde::{Deserialize, Serialize};

/// A user ID.
#[aliri_braid::braid(serde)]
pub struct UserId;

/// A reward ID.
#[aliri_braid::braid(serde)]
pub struct RewardId;

/// A reward redemption ID.
#[aliri_braid::braid(serde)]
pub struct RedemptionId;

/// A username, also specified as login. Should not be capitalized.
pub type UserName = Nickname;

/// A reference to a borrowed [`UserName`], also specified as login. Should not be capitalized.
pub type UserNameRef = NicknameRef;

/// A users display name
#[aliri_braid::braid(serde)]
pub struct DisplayName;

/// A nickname, not capitalized.
#[aliri_braid::braid(serde)]
pub struct Nickname;

/// RFC3339 timestamp
#[aliri_braid::braid(serde)]
pub struct Timestamp;

/// A game or category ID
#[aliri_braid::braid(serde)]
pub struct CategoryId;

/// A tag ID
#[aliri_braid::braid(serde)]
pub struct TagId;

/// A video ID
#[aliri_braid::braid(serde)]
pub struct VideoId;

/// An EventSub Subscription ID
#[aliri_braid::braid(serde)]
pub struct EventSubId;

/// A Team ID
#[aliri_braid::braid(serde)]
pub struct TeamId;

/// A Stream ID
#[aliri_braid::braid(serde)]
pub struct StreamId;

/// A message ID
#[aliri_braid::braid(serde)]
pub struct MsgId;

/// A poll ID
#[aliri_braid::braid(serde)]
pub struct PollId;

/// A poll choice ID
#[aliri_braid::braid(serde)]
pub struct PollChoiceId;

/// A prediction ID
#[aliri_braid::braid(serde)]
pub struct PredictionId;

/// A prediction choice ID
#[aliri_braid::braid(serde)]
pub struct PredictionOutcomeId;

/// A Badge set ID
#[aliri_braid::braid(serde)]
pub struct BadgeSetId;

/// A channel chat badge ID
#[aliri_braid::braid(serde)]
pub struct ChatBadgeId;

/// A game or category as defined by Twitch
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TwitchCategory {
    ///Template URL for the game’s box art.
    pub box_art_url: String,
    /// Game or category ID.
    pub id: CategoryId,
    ///Game name.
    pub name: String,
}

/// Subscription tiers
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(field_identifier)]
pub enum SubscriptionTier {
    /// Tier 1. $4.99
    #[serde(rename = "1000")]
    Tier1,
    /// Tier 1. $9.99
    #[serde(rename = "2000")]
    Tier2,
    /// Tier 1. $24.99
    #[serde(rename = "3000")]
    Tier3,
    /// Prime subscription
    Prime,
    /// Other
    Other(String),
}

impl Serialize for SubscriptionTier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            SubscriptionTier::Tier1 => "1000",
            SubscriptionTier::Tier2 => "2000",
            SubscriptionTier::Tier3 => "3000",
            SubscriptionTier::Prime => "Prime",
            SubscriptionTier::Other(o) => o,
        })
    }
}

/// Broadcaster types: "partner", "affiliated", or "".
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum BroadcasterType {
    /// Partner
    #[serde(rename = "partner")]
    Partner,
    /// Affiliated
    #[serde(rename = "affiliated")]
    Affiliated,
    /// None
    #[serde(other)]
    None,
}

impl Serialize for BroadcasterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            BroadcasterType::Partner => "partner",
            BroadcasterType::Affiliated => "affiliated",
            BroadcasterType::None => "",
        })
    }
}

/// User types: "staff", "admin", "global_mod", or "".
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum UserType {
    /// Staff
    #[serde(rename = "staff")]
    Staff,
    /// Admin
    #[serde(rename = "admin")]
    Admin,
    /// Global Moderator
    #[serde(rename = "global_mod")]
    GlobalMod,
    /// None
    #[serde(other)]
    None,
}

impl Serialize for UserType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            UserType::Staff => "staff",
            UserType::Admin => "admin",
            UserType::GlobalMod => "global_mod",
            UserType::None => "",
        })
    }
}

/// Period during which the video was created
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPeriod {
    /// Filter by all. Effectively a no-op
    All,
    /// Filter by from this day only
    Day,
    /// Filter by this week
    Week,
    /// Filter by this month
    Month,
}

/// Type of video
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VideoType {
    /// A live video
    Live,
    // FIXME: What is this?
    /// A playlist video
    Playlist,
    /// A uploaded video
    Upload,
    /// An archived video
    Archive,
    /// A highlight
    Highlight,
    /// A premiere
    Premiere,
    /// A rerun
    Rerun,
    /// A watch party
    WatchParty,
    /// A watchparty premiere,
    WatchPartyPremiere,
    /// A watchparty rerun
    WatchPartyRerun,
}

/// Type of video
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPrivacy {
    /// Video is public
    Public,
    /// Video is private
    Private,
}

/// Length of the commercial in seconds
#[derive(
    displaydoc::Display,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[repr(u64)]
#[non_exhaustive]
pub enum CommercialLength {
    /// 30s
    Length30 = 30,
    /// 60s
    Length60 = 60,
    /// 90s
    Length90 = 90,
    /// 120s
    Length120 = 120,
    /// 150s
    Length150 = 150,
    /// 180s
    Length180 = 180,
}

impl std::convert::TryFrom<u64> for CommercialLength {
    type Error = CommercialLengthParseError;

    fn try_from(l: u64) -> Result<Self, Self::Error> {
        match l {
            30 => Ok(CommercialLength::Length30),
            60 => Ok(CommercialLength::Length60),
            90 => Ok(CommercialLength::Length90),
            120 => Ok(CommercialLength::Length120),
            150 => Ok(CommercialLength::Length150),
            180 => Ok(CommercialLength::Length180),
            other => Err(CommercialLengthParseError::InvalidLength(other)),
        }
    }
}

/// Error for the `TryFrom` on [`CommercialLength`]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum CommercialLengthParseError {
    /// invalid length of {0}
    InvalidLength(u64),
}

/// A user according to many endpoints
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// ID of the user
    #[serde(alias = "user_id")]
    pub id: UserId,
    /// Login name of the user, not capitalized
    #[serde(alias = "user_login")]
    pub login: UserName,
    /// Display name of user
    #[serde(alias = "user_display_name", alias = "user_name")]
    pub display_name: DisplayName,
}

/// Links to the same image of different sizes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GlobalCooldown {
    /// Cooldown enabled
    pub is_enabled: bool,
    /// Cooldown amount
    #[serde(alias = "seconds")]
    pub global_cooldown_seconds: u32,
}

/// Reward redemption max
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(untagged)]
#[non_exhaustive]
pub enum Max {
    /// Max per stream
    MaxPerStream {
        /// Max per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per stream
        #[serde(alias = "value")]
        max_per_stream: u32,
    },
    /// Max per user per stream
    MaxPerUserPerStream {
        /// Max per user per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per user per stream
        #[serde(alias = "value")]
        max_per_user_per_stream: u32,
    },
}

/// Poll choice
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PollChoice {
    /// ID for the choice.
    pub id: String,
    /// Text displayed for the choice.
    pub title: String,
    /// Total number of votes received for the choice across all methods of voting.
    pub votes: Option<i64>,
    /// Number of votes received via Channel Points.
    pub channel_points_votes: Option<i64>,
    /// Number of votes received via Bits.
    pub bits_votes: Option<i64>,
}

// FIXME: Poll status has different name depending on if returned from helix or eventsub. See https://twitch.uservoice.com/forums/310213-developers/suggestions/43402176
/// Status of a poll
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum PollStatus {
    /// Poll is currently in progress.
    #[serde(alias = "active")]
    Active,
    /// Poll has reached its ended_at time.
    #[serde(alias = "completed")]
    Completed,
    /// Poll has been manually terminated before its ended_at time.
    #[serde(alias = "terminated")]
    Terminated,
    /// Poll is no longer visible on the channel.
    #[serde(alias = "archived")]
    Archived,
    /// Poll is no longer visible to any user on Twitch.
    #[serde(alias = "moderated")]
    Moderated,
    /// Something went wrong determining the state.
    #[serde(alias = "invalid")]
    Invalid,
}

// FIXME: Prediction status has different name depending on if returned from helix or eventsub. See https://twitch.uservoice.com/forums/310213-developers/suggestions/43402197
/// Status of the Prediction
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum PredictionStatus {
    /// A winning outcome has been chosen and the Channel Points have been distributed to the users who guessed the correct outcome.
    #[serde(alias = "resolved")]
    Resolved,
    /// The Prediction is active and viewers can make predictions.
    #[serde(alias = "active")]
    Active,
    /// The Prediction has been canceled and the Channel Points have been refunded to participants.
    #[serde(alias = "canceled")]
    Canceled,
    /// The Prediction has been locked and viewers can no longer make predictions.
    #[serde(alias = "locked")]
    Locked,
}

/// Outcome for the Prediction
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PredictionOutcome {
    /// ID for the outcome.
    pub id: String,
    /// Text displayed for outcome.
    pub title: String,
    /// Number of unique users that chose the outcome.
    pub users: Option<i64>,
    /// Number of Channel Points used for the outcome.
    pub channel_points: Option<i64>,
    /// Array of users who were the top predictors. null if none. Top 10
    pub top_predictors: Option<Vec<PredictionTopPredictors>>,
    /// Color for the outcome. Valid values: BLUE, PINK
    pub color: String,
}

// FIXME: eventsub adds prefix `user_*`. See https://discord.com/channels/325552783787032576/326772207844065290/842359030252437514
/// Users who were the top predictors.
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PredictionTopPredictors {
    /// ID of the user.
    #[serde(alias = "user_id")]
    pub id: UserId,
    /// Display name of the user.
    #[serde(alias = "user_name")]
    pub name: DisplayName,
    /// Login of the user.
    #[serde(alias = "user_login")]
    pub login: UserName,
    /// Number of Channel Points used by the user.
    pub channel_points_used: i64,
    /// Number of Channel Points won by the user.
    ///
    /// This value is always null in the event payload for Prediction progress and Prediction lock. This value is 0 if the outcome did not win or if the Prediction was canceled and Channel Points were refunded.
    pub channel_points_won: Option<i64>,
}

/// Status of a message that is or was in AutoMod queue
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum AutomodStatus {
    /// Message has been caught and pending moderation
    Pending,
    /// Message has been allowed
    Allowed,
    /// Message has been denied
    Denied,
    /// Automod message expired in queue
    Expired,
}

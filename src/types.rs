//! Twitch types
//!

/// A user ID.
pub type UserId = String;

/// A username, also specified as login. Should not be capitalized.
pub type UserName = Nickname;

/// A users display name
pub type DisplayName = String;

/// A nickname, not capitalized.
pub type Nickname = String;

/// RFC3339 timestamp
pub type Timestamp = String;

/// A game or category ID
pub type CategoryId = String;

/// A tag ID
pub type TagId = String;

/// A video ID
pub type VideoId = String;

/// A game or category as defined by Twitch
#[derive(PartialEq, serde::Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
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
#[derive(PartialEq, Eq, serde::Deserialize, Clone, Debug)]
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

impl serde::Serialize for SubscriptionTier {
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
#[derive(PartialEq, serde::Deserialize, Clone, Debug)]
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

impl serde::Serialize for BroadcasterType {
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
#[derive(PartialEq, serde::Deserialize, Clone, Debug)]
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

impl serde::Serialize for UserType {
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
#[derive(PartialEq, serde::Deserialize, serde::Serialize, Clone, Debug)]
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
#[derive(PartialEq, serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VideoType {
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
    /// A watchparty premiere,
    WatchPartyPremiere,
    /// A watchparty rerun
    WatchPartyRerun,
}

/// Type of video
#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize, Clone, Debug)]
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
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct User {
    /// ID of the user
    #[serde(alias = "user_id")]
    pub id: UserId,
    /// Login name of the user, not capitalized
    #[serde(alias = "user_login")]
    pub login: UserName,
    /// Display name of user
    #[serde(alias = "user_display_name")]
    pub display_name: DisplayName,
}

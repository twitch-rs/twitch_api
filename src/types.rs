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

/// A Video ID
pub type VideoId = String;

/// A game or category as defined by Twitch
#[derive(PartialEq, serde::Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct TwitchCategory {
    ///Template URL for the game’s box art.
    box_art_url: String,
    /// Game or category ID.
    id: CategoryId,
    ///Game name.
    name: String,
}

/// Subscription tiers
#[derive(PartialEq, serde::Deserialize, Clone, Debug)]
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

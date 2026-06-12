//! Common types for hype trains.

use crate::types;
use serde_derive::{Deserialize, Serialize};

/// A broadcaster participating in a hype train.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SharedTrainParticipant {
    /// The broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
}

/// Type of Hype Train event
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum HypeTrainType {
    /// A treasure train.
    Treasure,
    /// A golden Kappa train.
    GoldenKappa,
    /// A regular train.
    Regular,
    /// An unknown hype train type, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

/// Type of Hype Train event
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum HypeTrainContributionType {
    /// Cheering with bits
    Bits,
    /// Subscription activity like subscribing or gifting subscriptions.
    Subscription,
    /// Covers other contribution methods not listed.
    Other,
    /// An unknown contribution type, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

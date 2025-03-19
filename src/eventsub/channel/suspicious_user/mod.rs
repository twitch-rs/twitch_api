#![doc(alias = "channel.suspicious_user")]
//! A user's moderator privileges on a specified channel are changed.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod message;
pub mod update;

#[doc(inline)]
pub use message::{ChannelSuspiciousUserMessageV1, ChannelSuspiciousUserMessageV1Payload};
#[doc(inline)]
pub use update::{ChannelSuspiciousUserUpdateV1, ChannelSuspiciousUserUpdateV1Payload};

/// A user's low trust status
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum LowTrustStatus {
    /// The user isn't monitored
    None,
    /// The user is actively monitored
    ActiveMonitoring,
    /// The user is restricted
    Restricted,
    /// An unknown low trust status, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

/// Traits of a suspicious user
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum SuspiciousUserType {
    /// A manually added user
    #[serde(rename = "manually_added")]
    Manual,
    /// A detected ban evader
    BanEvader,
    /// A user banned in another channel that shares ban information
    #[serde(rename = "banned_in_shared_channel")]
    SharedChannelBan,
    /// An unknown user type, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

/// Likelyhood that a user is a ban evader
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum BanEvasionEvaluation {
    /// The ban evasion status is not known
    Unknown,
    /// The user is a possible ban evader
    Possible,
    /// The user is a lilkely ban evader
    Likely,
    /// An unknown evaluation, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Other(String),
}

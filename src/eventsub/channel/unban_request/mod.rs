#![doc(alias = "channel.unban_request")]
//! An unban request in a specified channel is changed.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod create;
pub mod resolve;

#[doc(inline)]
pub use create::{ChannelUnbanRequestCreateV1, ChannelUnbanRequestCreateV1Payload};
#[doc(inline)]
pub use resolve::{ChannelUnbanRequestResolveV1, ChannelUnbanRequestResolveV1Payload};

/// A status of an unban request
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum UnbanRequestStatus {
    /// The request was approved.
    Approved,
    /// The request was canceled.
    Canceled,
    /// The request was denied.
    Denied,
    /// An unknown status, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

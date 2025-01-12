#![doc(alias = "user.whisper")]
//! Notifications for whispers (private messages)
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod message;

#[doc(inline)]
pub use message::{UserWhisperMessageV1, UserWhisperMessageV1Payload, Whisper};

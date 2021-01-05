//! Webhook topics for Hype Trains
use crate::helix::webhooks::Topic;

pub mod hypetrain_event;

#[doc(inline)]
pub use hypetrain_event::HypeTrainEventTopic;

//! Topics for subscriptions
use crate::helix::webhooks::Topic;

pub mod subscription_events;

#[doc(inline)]
pub use subscription_events::SubscriptionEventsTopic;

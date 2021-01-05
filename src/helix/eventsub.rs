#![doc(alias = "eventsub")]
//! Helix endpoints regarding EventSub

use crate::{helix, types};
use serde::{Deserialize, Serialize};

pub mod create_eventsub_subscription;
pub mod delete_eventsub_subscription;
pub mod get_eventsub_subscriptions;

#[doc(inline)]
pub use create_eventsub_subscription::{
    CreateEventSubSubscription, CreateEventSubSubscriptionBody, CreateEventSubSubscriptionRequest,
};
#[doc(inline)]
pub use delete_eventsub_subscription::{
    DeleteEventSubSubscription, DeleteEventSubSubscriptionRequest,
};
#[doc(inline)]
pub use get_eventsub_subscriptions::{EventSubSubscription, GetEventSubSubscriptionsRequest};

//! Helix endpoints regarding EventSub

use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod create_conduit;
pub mod create_eventsub_subscription;
pub mod delete_eventsub_subscription;
pub mod get_conduits;
pub mod get_eventsub_subscriptions;

#[doc(inline)]
pub use create_conduit::{CreateConduitBody, CreateConduitRequest};
#[doc(inline)]
pub use create_eventsub_subscription::{
    CreateEventSubSubscription, CreateEventSubSubscriptionBody, CreateEventSubSubscriptionRequest,
};
#[doc(inline)]
pub use delete_eventsub_subscription::{
    DeleteEventSubSubscription, DeleteEventSubSubscriptionRequest,
};
#[doc(inline)]
pub use get_conduits::GetConduitsRequest;
#[doc(inline)]
pub use get_eventsub_subscriptions::{EventSubSubscriptions, GetEventSubSubscriptionsRequest};

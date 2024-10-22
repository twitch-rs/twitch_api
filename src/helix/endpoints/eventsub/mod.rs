//! Helix endpoints regarding EventSub
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Conduits 🟡 4/6</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Conduits](https://dev.twitch.tv/docs/api/reference#get-conduits) | [`HelixClient::get_conduits`](crate::helix::HelixClient::get_conduits) | [`get_conduits`] |
//! | [Create Conduits](https://dev.twitch.tv/docs/api/reference#create-conduits) | [`HelixClient::create_conduit`](crate::helix::HelixClient::create_conduit) | [`create_conduit`] |
//! | [Update Conduits](https://dev.twitch.tv/docs/api/reference#update-conduits) | - | - |
//! | [Delete Conduit](https://dev.twitch.tv/docs/api/reference#delete-conduit) | - | - |
//! | [Get Conduit Shards](https://dev.twitch.tv/docs/api/reference#get-conduit-shards) | [`HelixClient::get_conduit_shards`](crate::helix::HelixClient::get_conduit_shards) | [`get_conduit_shards`] |
//! | [Update Conduit Shards](https://dev.twitch.tv/docs/api/reference#update-conduit-shards) | [`HelixClient::update_conduit_shards`](crate::helix::HelixClient::update_conduit_shards) | [`update_conduit_shards`] |
//!
//! </details>
//!
//! <details open><summary style="cursor: pointer">EventSub 🟢 3/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Create EventSub Subscription](https://dev.twitch.tv/docs/api/reference#create-eventsub-subscription) | [`HelixClient::create_eventsub_subscription`](crate::helix::HelixClient::create_eventsub_subscription) | [`create_eventsub_subscription`] |
//! | [Delete EventSub Subscription](https://dev.twitch.tv/docs/api/reference#delete-eventsub-subscription) | [`HelixClient::delete_eventsub_subscription`](crate::helix::HelixClient::delete_eventsub_subscription) | [`delete_eventsub_subscription`] |
//! | [Get EventSub Subscriptions](https://dev.twitch.tv/docs/api/reference#get-eventsub-subscriptions) | [`HelixClient::get_eventsub_subscriptions`](crate::helix::HelixClient::get_eventsub_subscriptions) | [`get_eventsub_subscriptions`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->

use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod create_conduit;
pub mod create_eventsub_subscription;
pub mod delete_eventsub_subscription;
pub mod get_conduit_shards;
pub mod get_conduits;
pub mod get_eventsub_subscriptions;
pub mod update_conduit_shards;

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
pub use get_conduit_shards::{ConduitShards, GetConduitShardsRequest};
#[doc(inline)]
pub use get_conduits::GetConduitsRequest;
#[doc(inline)]
pub use get_eventsub_subscriptions::{EventSubSubscriptions, GetEventSubSubscriptionsRequest};
#[doc(inline)]
pub use update_conduit_shards::{
    UpdateConduitShardsBody, UpdateConduitShardsRequest, UpdateConduitShardsResponse,
};

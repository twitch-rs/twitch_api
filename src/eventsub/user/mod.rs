//! Subscription types regarding users

use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod authorization;
pub mod update;

#[doc(inline)]
pub use authorization::{UserAuthorizationGrantV1, UserAuthorizationGrantV1Payload};
#[doc(inline)]
pub use authorization::{UserAuthorizationRevokeV1, UserAuthorizationRevokeV1Payload};
#[doc(inline)]
pub use update::{UserUpdateV1, UserUpdateV1Payload};

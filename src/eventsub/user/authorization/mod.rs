//! Authorization from a user has been granted or revoked to a specific client ID
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod grant;
pub mod revoke;

#[doc(inline)]
pub use grant::{UserAuthorizationGrantV1, UserAuthorizationGrantV1Payload};
#[doc(inline)]
pub use revoke::{UserAuthorizationRevokeV1, UserAuthorizationRevokeV1Payload};

//! Helix endpoints regarding channel raids

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod cancel_a_raid;
pub mod start_a_raid;

#[doc(inline)]
pub use cancel_a_raid::{CancelARaidRequest, CancelARaidResponse};
#[doc(inline)]
pub use start_a_raid::{StartARaidRequest, StartARaidResponse};

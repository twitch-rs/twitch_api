#![doc(alias = "mod")]
//! Helix endpoints regarding moderation

use crate::{helix, types};
use serde::{Deserialize, Serialize};

pub mod check_automod_status;
pub mod get_banned_events;
pub mod get_banned_users;
pub mod get_moderator_events;
pub mod get_moderators;

#[doc(inline)]
pub use check_automod_status::{
    CheckAutoModStatus, CheckAutoModStatusBody, CheckAutoModStatusRequest,
};
#[doc(inline)]
pub use get_banned_events::{BannedEvent, GetBannedEventsRequest};
#[doc(inline)]
pub use get_banned_users::{BannedUser, GetBannedUsersRequest};
#[doc(inline)]
pub use get_moderator_events::{GetModeratorEventsRequest, ModeratorEvent};
#[doc(inline)]
pub use get_moderators::{GetModeratorsRequest, Moderator};

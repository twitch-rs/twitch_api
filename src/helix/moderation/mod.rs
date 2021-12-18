#![doc(alias = "mod")]
//! Helix endpoints regarding moderation

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "unsupported")]
pub mod add_blocked_term;
#[cfg(feature = "unsupported")]
pub mod ban_users;
pub mod check_automod_status;
pub mod get_banned_events;
pub mod get_banned_users;
#[cfg(feature = "unsupported")]
pub mod get_blocked_terms;
pub mod get_moderator_events;
pub mod get_moderators;
pub mod manage_held_automod_messages;
#[cfg(feature = "unsupported")]
pub mod unban_user;

#[doc(inline)]
#[cfg(feature = "unsupported")]
pub use add_blocked_term::{AddBlockedTermBody, AddBlockedTermRequest};
#[doc(inline)]
#[cfg(feature = "unsupported")]
pub use ban_users::{BanUser, BanUserError, BanUsersBody, BanUsersRequest, BanUsersResponse};
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
#[doc(inline)]
pub use manage_held_automod_messages::{
    AutoModAction, ManageHeldAutoModMessages, ManageHeldAutoModMessagesBody,
    ManageHeldAutoModMessagesRequest,
};
#[doc(inline)]
#[cfg(feature = "unsupported")]
pub use unban_user::{UnbanUserRequest, UnbanUserResponse};



#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BlockedTerm {
    /// The broadcaster that owns the list of blocked terms.
    pub broadcaster_id: types::UserId,
    /// The UTC date and time (in RFC3339 format) of when the term was blocked.
    pub created_at: types::Timestamp,
    /// The UTC date and time (in RFC3339 format) of when the blocked term is set to expire. After the block expires, user’s will be able to use the term in the broadcaster’s chat room.
    ///
    /// This field is null if the term was added manually or was permanently blocked by AutoMod.
    pub expires_at: Option<types::Timestamp>,
    /// An ID that uniquely identifies this blocked term.
    pub id: types::BlockedTermId,
    /// The moderator that blocked the word or phrase from being used in the broadcaster’s chat room.
    pub moderator_id: types::UserId,
    /// The blocked word or phrase.
    pub text: String,
    /// The UTC date and time (in RFC3339 format) of when the term was updated.
    ///
    /// When the term is added, this timestamp is the same as created_at. The timestamp changes as AutoMod continues to deny the term.
    pub updated_at: types::Timestamp,
}

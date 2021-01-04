//! Topics for users
use crate::helix::webhooks::Topic;

pub mod user_changed;
pub mod user_follows;

#[doc(inline)]
pub use user_changed::UserChangedTopic;
#[doc(inline)]
pub use user_follows::UserFollowsTopic;

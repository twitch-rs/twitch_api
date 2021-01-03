//! Topics for users
use crate::helix::webhooks::Topic;

pub mod user_follows;
pub mod user_changed;

#[doc(inline)]
pub use user_follows::UserFollowsTopic;
#[doc(inline)]
pub use user_changed::UserChangedTopic;



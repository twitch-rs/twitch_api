//! Topics for streams
use crate::helix::webhooks::Topic;

pub mod stream_changed;

#[doc(inline)]
pub use stream_changed::StreamChangedTopic;

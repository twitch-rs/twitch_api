//! Automod related events

use super::{EventSubscription, EventType};
use serde_derive::{Deserialize, Serialize};

pub mod message;
pub mod settings;
pub mod terms;

#[doc(inline)]
#[cfg(feature = "beta")]
pub use message::{AutomodMessageHoldBeta, AutomodMessageHoldBetaPayload};
#[doc(inline)]
pub use message::{AutomodMessageHoldV1, AutomodMessageHoldV1Payload};
#[doc(inline)]
#[cfg(feature = "beta")]
pub use message::{AutomodMessageUpdateBeta, AutomodMessageUpdateBetaPayload};
#[doc(inline)]
pub use message::{AutomodMessageUpdateV1, AutomodMessageUpdateV1Payload};

#[doc(inline)]
pub use terms::{AutomodTermsUpdateV1, AutomodTermsUpdateV1Payload};

#[doc(inline)]
pub use settings::{AutomodSettingsUpdateV1, AutomodSettingsUpdateV1Payload};

/// A category identified by automod for a message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AutomodCategory {
    /// Aggressive behavior or language.
    Aggressive,
    /// Bullying or harassment.
    Bullying,
    /// Discrimination or negative references to disabilities.
    Disability,
    /// Content relating to or expressing sexuality.
    Sexuality,
    /// Inappropriate or suggestive sexual terms
    Sexwords,
    /// Misogynistic language or behavior.
    Misogyny,
    /// Racist language or behavior.
    Racism,
    /// Profane language or swearing.
    Profanity,
    /// An unknown category identified by Automod, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

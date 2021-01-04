//! Various topics

use crate::helix::{self, Request, RequestGet, Response};
use helix::ser;
use serde::{de::DeserializeOwned, Serialize};

pub mod hypetrain;
pub mod moderation;
pub mod streams;
pub mod subscriptions;
pub mod users;

// FIXME: Missing Topic: Extension Transaction Created

/// A webhook topic.
///
pub trait Topic: DeserializeOwned + Serialize + PartialEq {
    /// Helix response
    type Helix: RequestGet + Request;

    /// URL of topic sans `https://api.twitch.tv/helix/`
    const PATH: &'static str;

    /// Scopes needed by this endpoint
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope];
    /// Optional scopes needed by this endpoint
    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];

    /// Defines layout of the url parameters.
    fn query(&self) -> Result<String, ser::Error> { ser::to_string(&self) }

    /// Returns full URI for the request, including query parameters.
    fn get_uri(&self) -> Result<http::Uri, helix::InvalidUri> {
        use std::str::FromStr;
        http::Uri::from_str(&format!(
            "{}{}?{}",
            crate::TWITCH_HELIX_URL,
            <Self as Topic>::PATH,
            self.query()?
        ))
        .map_err(Into::into)
    }
    /// Returns bare URI for the request, NOT including query parameters.
    fn get_bare_uri() -> Result<http::Uri, helix::InvalidUri> {
        use std::str::FromStr;
        http::Uri::from_str(&format!(
            "{}{}?",
            crate::TWITCH_HELIX_URL,
            <Self as Topic>::PATH,
        ))
        .map_err(Into::into)
    }

    /// Parse payload received on webhook.
    ///
    /// Forwards to [`RequestGet::parse_response`]
    fn parse_payload(
        response: http::Response<Vec<u8>>,
    ) -> Result<
        Response<Self::Helix, <Self::Helix as Request>::Response>,
        crate::helix::HelixRequestGetError,
    >
    where Self: Sized {
        <Self::Helix>::parse_response(None, &Self::get_bare_uri()?, response)
    }
}

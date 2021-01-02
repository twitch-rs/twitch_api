#![allow(missing_docs)]
//! Endpoints regarding streams
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, webhooks::??};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = ??::builder()
//!     .user_login(vec!["justinfan1337".to_string()])
//!     .build();
//!
//! // If this doesn't return a result, that would mean the stream is not live.
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```

use crate::{
    helix::{self, Request, RequestGet, Response},
    types,
};
use helix::ser;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod hub;
pub mod topics;

pub trait Topic: DeserializeOwned + Serialize + PartialEq {
    /// Helix response
    type Helix: RequestGet + Request;

    /// URL of topic sans `https://api.twitch.tv/helix/`
    const PATH: &'static str;

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

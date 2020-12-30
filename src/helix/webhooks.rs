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

use crate::{helix, types};
use helix::ser;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod hub;
pub mod topics;

pub trait Topic: DeserializeOwned + Serialize + PartialEq {
    /// Payload for given topic
    type Payload: PartialEq + std::fmt::Debug + DeserializeOwned + Serialize;

    /// URL of topic sans `https://api.twitch.tv/helix/`
    const PATH: &'static str;

    fn query(&self) -> Result<String, ser::Error> { ser::to_string(&self) }

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
}

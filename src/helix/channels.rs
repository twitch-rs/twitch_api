#![doc(alias = "channel")]
//! Helix endpoints regarding channels
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, channels::GetChannelInformationRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetChannelInformationRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//!
//! // Get Channel Information Request only returns one entry.
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod get_channel_editors;
pub mod get_channel_information;
pub mod modify_channel_information;
pub mod start_commercial;

#[doc(inline)]
pub use get_channel_editors::{Editor, GetChannelEditorsRequest};
#[doc(inline)]
pub use get_channel_information::{ChannelInformation, GetChannelInformationRequest};
#[doc(inline)]
pub use modify_channel_information::{
    ModifyChannelInformation, ModifyChannelInformationBody, ModifyChannelInformationRequest,
};
#[doc(inline)]
pub use start_commercial::{StartCommercial, StartCommercialBody, StartCommercialRequest};

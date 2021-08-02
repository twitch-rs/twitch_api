//! Helix endpoints regarding search
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, search::SearchCategoriesRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = SearchCategoriesRequest::builder()
//!     .query("Pokémon")
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod search_categories;
pub mod search_channels;

#[doc(inline)]
pub use search_categories::SearchCategoriesRequest;
#[doc(inline)]
pub use search_channels::{Channel, SearchChannelsRequest};
pub use types::TwitchCategory as Category;

//! Helix endpoints regarding search
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::helix::{HelixClient, search::SearchCategoriesRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = SearchCategoriesRequest::query("Pokémon");
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Search 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Search Categories](https://dev.twitch.tv/docs/api/reference#search-categories) | [`HelixClient::search_categories`](crate::helix::HelixClient::search_categories) | [`search_categories`] |
//! | [Search Channels](https://dev.twitch.tv/docs/api/reference#search-channels) | [`HelixClient::search_channels`](crate::helix::HelixClient::search_channels) | [`search_channels`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->

use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod search_categories;
pub mod search_channels;

#[doc(inline)]
pub use search_categories::SearchCategoriesRequest;
#[doc(inline)]
pub use search_channels::{Channel, SearchChannelsRequest};
pub use types::TwitchCategory as Category;

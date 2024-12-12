//! Helix endpoints regarding content classification cabels
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::helix::{HelixClient, ccls::GetContentClassificationLabelsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::default();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetContentClassificationLabelsRequest::new();
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
//! <details open><summary style="cursor: pointer">CCLs 🟢 1/1</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Content Classification Labels](https://dev.twitch.tv/docs/api/reference#get-content-classification-labels) | [`HelixClient::get_content_classification_labels`](crate::helix::HelixClient::get_content_classification_labels), [`HelixClient::get_content_classification_labels_for_locale`](crate::helix::HelixClient::get_content_classification_labels_for_locale) | [`get_content_classification_labels`] |
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

pub mod get_content_classification_labels;

#[doc(inline)]
pub use get_content_classification_labels::{
    ContentClassificationLabel, GetContentClassificationLabelsRequest,
};

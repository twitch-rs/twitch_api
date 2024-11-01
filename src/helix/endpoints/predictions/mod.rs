//! Helix endpoints regarding channel predictions
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Predictions 🟢 3/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Predictions](https://dev.twitch.tv/docs/api/reference#get-predictions) | - | [`get_predictions`] |
//! | [Create Prediction](https://dev.twitch.tv/docs/api/reference#create-prediction) | - | [`create_prediction`] |
//! | [End Prediction](https://dev.twitch.tv/docs/api/reference#end-prediction) | - | [`end_prediction`] |
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

pub mod create_prediction;
pub mod end_prediction;
pub mod get_predictions;

#[doc(inline)]
pub use get_predictions::{GetPredictionsRequest, Prediction};

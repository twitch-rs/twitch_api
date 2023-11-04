//! Helix endpoints regarding channel predictions

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

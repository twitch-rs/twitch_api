//! Helix endpoints regarding channel predictions

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod create_prediction;
pub mod end_prediction;
pub mod get_predictions;

#[doc(inline)]
pub use get_predictions::{GetPredictionsRequest, Prediction};

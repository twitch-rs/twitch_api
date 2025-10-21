#![doc(alias = "extension.bits_transaction")]
//! A bits transaction for an extension is changed.
use super::ExtensionBitsProduct;
use crate::types;
use serde_derive::{Deserialize, Serialize};
pub mod create;

#[doc(inline)]
pub use create::{ExtensionBitsTransactionCreateV1, ExtensionBitsTransactionCreateV1Payload};

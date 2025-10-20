#![doc(alias = "extensions")]
//! Subscription types regarding extensions

pub mod bits_transaction;
use serde_derive::{Deserialize, Serialize};

#[doc(inline)]
pub use bits_transaction::{
    ExtensionBitsTransactionCreateV1, ExtensionBitsTransactionCreateV1Payload,
};

/// [`extension.bits_transaction.create`](ExtensionBitsTransactionCreateV1) product payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ExtensionBitsProduct {
    /// The Display name of the purchased product
    pub name: String,

    /// The sku of the purchased product
    pub sku: String,

    /// The amount of bits paid for the product
    pub bits: i64,

    /// If the product is in development (bits will always be 0 if this is true)
    pub in_development: bool,
}

#![doc(alias = "whisper")]
#![allow(deprecated)]
//! Helix endpoints regarding whispers
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Whispers 🟢 1/1</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Send Whisper](https://dev.twitch.tv/docs/api/reference#send-whisper) | [`HelixClient::send_whisper`](crate::helix::HelixClient::send_whisper) | [`send_whisper`] |
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

pub mod send_whisper;

#[doc(inline)]
pub use send_whisper::{SendWhisperBody, SendWhisperRequest, SendWhisperResponse};

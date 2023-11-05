#![doc(alias = "whisper")]
#![allow(deprecated)]
//! Helix endpoints regarding whispers
use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod send_whisper;

#[doc(inline)]
pub use send_whisper::{SendWhisperBody, SendWhisperRequest, SendWhisperResponse};

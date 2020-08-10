//! Rust library for talking with the new twitch API aka "Helix".
//!
//! ---

use std::io;
use thiserror::Error;
use twitch_oauth2;

pub mod streams;

static TWITCH_HELIX_STREAMS: &str = "https://api.twitch.tv/helix/";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct HelixClient {
    // TODO: should store oauth...
}
impl HelixClient {
    /// Access GetStreams builder.
    pub fn get_streams() -> () {}
}

pub trait Request {
    const GET: &'static str;
    type Result;
    fn request(&self, oauth: twitch_oauth2::AppAccessToken) -> Result<Self::Result, RequestError>;
}

pub trait Paginated {
    fn cursor_value(&self) -> Cursor;
}

pub type Cursor = String;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("io error")]
    IOError(#[from] io::Error),
    #[error("something happened")]
    Other,
}

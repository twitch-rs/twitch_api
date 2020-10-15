//! Twitch types
//!

/// A user ID.
pub type UserId = String;

/// A username, also specified as login. Should not be capitalized.
pub type UserName = Nickname;

/// A users display name
pub type DisplayName = String;

/// A nickname, not capitalized.
pub type Nickname = String;

/// RFC3339 timestamp
pub type Timestamp = String;

/// A game ID
pub type GameId = String;

/// A tag ID
pub type TagId = String;

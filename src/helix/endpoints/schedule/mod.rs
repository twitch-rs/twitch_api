//! Helix endpoints regarding stream schedules
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Schedule 🟢 6/6</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Channel Stream Schedule](https://dev.twitch.tv/docs/api/reference#get-channel-stream-schedule) | - | [`get_channel_stream_schedule`] |
//! | [Get Channel iCalendar](https://dev.twitch.tv/docs/api/reference#get-channel-icalendar) | [`HelixClient::get_channel_icalendar`](crate::helix::HelixClient::get_channel_icalendar) | [`get_channel_icalendar`] |
//! | [Update Channel Stream Schedule](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule) | - | [`update_channel_stream_schedule`] |
//! | [Create Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment) | - | [`create_channel_stream_schedule_segment`] |
//! | [Update Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment) | - | [`update_channel_stream_schedule_segment`] |
//! | [Delete Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#delete-channel-stream-schedule-segment) | - | [`delete_channel_stream_schedule_segment`] |
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

pub mod create_channel_stream_schedule_segment;
pub mod delete_channel_stream_schedule_segment;
pub mod get_channel_icalendar;
pub mod get_channel_stream_schedule;
pub mod update_channel_stream_schedule;
pub mod update_channel_stream_schedule_segment;

#[doc(inline)]
pub use create_channel_stream_schedule_segment::{
    CreateChannelStreamScheduleSegmentBody, CreateChannelStreamScheduleSegmentRequest,
};
#[doc(inline)]
pub use delete_channel_stream_schedule_segment::{
    DeleteChannelStreamScheduleSegment, DeleteChannelStreamScheduleSegmentRequest,
};
#[doc(inline)]
pub use get_channel_icalendar::GetChannelICalendar;
#[doc(inline)]
pub use get_channel_stream_schedule::GetChannelStreamScheduleRequest;
#[doc(inline)]
pub use update_channel_stream_schedule::{
    UpdateChannelStreamSchedule, UpdateChannelStreamScheduleRequest,
};

/// Scheduled broadcasts for a specific channel given by [Get Channel Stream Schedule](self)
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ScheduledBroadcasts {
    /// Scheduled broadcasts for this stream schedule.
    pub segments: Vec<Segment>,
    /// User ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// Display name of the broadcaster.
    pub broadcaster_name: types::DisplayName,
    /// Login of the broadcaster.
    pub broadcaster_login: types::UserName,
    /// If Vacation Mode is enabled, this includes start and end dates for the vacation. If Vacation Mode is disabled, value is set to null.
    pub vacation: Option<Vacation>,
}

/// Scheduled broadcast for a stream schedule.
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Segment {
    /// The ID for the scheduled broadcast.
    pub id: types::StreamSegmentId,
    /// Scheduled start time for the scheduled broadcast in RFC3339 format.
    pub start_time: types::Timestamp,
    /// Scheduled end time for the scheduled broadcast in RFC3339 format.
    pub end_time: types::Timestamp,
    /// Title for the scheduled broadcast.
    pub title: String,
    /// Used with recurring scheduled broadcasts. Specifies the date of the next recurring broadcast in RFC3339 format if one or more specific broadcasts have been deleted in the series. Set to null otherwise.
    pub canceled_until: Option<types::Timestamp>,
    /// The category for the scheduled broadcast. Set to null if no category has been specified.
    pub category: Option<Category>,
    /// Indicates if the scheduled broadcast is recurring weekly.
    pub is_recurring: bool,
}

// FIXME: Similar to types::TwitchCategory, missing box_art
/// The category for a scheduled broadcast.
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Category {
    /// Game/category ID.
    pub id: types::CategoryId,
    /// Game/category name.
    pub name: String,
}

/// Information on Vacation Mode
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Vacation {
    /// Start time for vacation specified in RFC3339 format.
    pub start_time: types::Timestamp,
    /// End time for vacation specified in RFC3339 format.
    pub end_time: types::Timestamp,
}

//! Helix endpoints regarding subscriptions

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod get_channel_teams;
pub mod get_teams;

/// Information about a team. Get team members with [`GetChannelTeamsRequest`](get_channel_teams::GetChannelTeamsRequest)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TeamInformation {
    /// URL of the Team background image.
    background_image_url: Option<String>,
    /// URL for the Team banner.
    banner: Option<String>,
    /// Date and time the Team was created.
    created_at: String,
    /// Date and time the Team was last updated.
    updated_at: String,
    /// Team description.
    info: String,
    /// Image URL for the Team logo.
    thumbnail_url: String,
    /// Team name.
    team_name: String,
    /// Team display name.
    team_display_name: String,
    /// Team ID.
    id: types::TeamId,
}

//! PubSub messages for raids
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A user follows the channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "String", try_from = "String")]
pub struct Raid {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(Raid, "raid", channel_id);

impl pubsub::Topic for Raid {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Raid go
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct RaidGoV2 {
    /// ID of user would be raided
    pub creator_id: types::UserId,
    /// Raid will be force started in this many seconds
    pub force_raid_now_seconds: i64,
    /// ID of raid
    pub id: String,
    /// ID of broadcaster doing raid
    pub source_id: types::UserId,
    /// Display name of targeted broadcaster/user
    pub target_display_name: types::DisplayName,
    /// ID of targeted broadcaster/user
    pub target_id: types::UserId,
    /// Username of targeted broadcaster/user
    pub target_login: types::UserName,
    /// Profile picture of targeted broadcaster/user
    pub target_profile_image: String,
    /// Jitter amount
    pub transition_jitter_seconds: i64,
    /// Amount of viewers that will join raid
    pub viewer_count: i64,
}

/// Raid update
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct RaidUpdateV2 {
    /// ID of user would be raided
    pub creator_id: types::UserId,
    /// Raid will be force started in this many seconds
    pub force_raid_now_seconds: i64,
    /// ID of raid
    pub id: String,
    /// ID of broadcaster doing raid
    pub source_id: types::UserId,
    /// Display name of targeted broadcaster/user
    pub target_display_name: types::DisplayName,
    /// ID of targeted broadcaster/user
    pub target_id: types::UserId,
    /// Username of targeted broadcaster/user
    pub target_login: types::UserName,
    /// Profile picture of targeted broadcaster/user
    pub target_profile_image: String,
    /// Jitter amount
    pub transition_jitter_seconds: i64,
    /// Amount of viewers that will join raid
    pub viewer_count: i64,
}

/// Raid canceled
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct RaidCancelV2 {
    /// ID of user would be raided
    pub creator_id: types::UserId,
    /// Raid would have be force started in this many seconds
    pub force_raid_now_seconds: i64,
    /// ID of raid
    pub id: String,
    /// ID of broadcaster doing raid
    pub source_id: types::UserId,
    /// Display name of targeted broadcaster/user
    pub target_display_name: types::DisplayName,
    /// ID of targeted broadcaster/user
    pub target_id: types::UserId,
    /// Username of targeted broadcaster/user
    pub target_login: types::UserName,
    /// Profile picture of targeted broadcaster/user
    pub target_profile_image: String,
    /// Jitter amount
    pub transition_jitter_seconds: i64,
    /// Amount of viewers that would join raid
    pub viewer_count: i64,
}

/// Reply from [Raid]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "raid")]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum RaidReply {
    /// Raid go
    #[serde(rename = "raid_go_v2")]
    RaidGoV2(RaidGoV2),
    /// Raid update
    #[serde(rename = "raid_update_v2")]
    RaidUpdateV2(RaidUpdateV2),
    /// Raid canceled
    #[serde(rename = "raid_cancel_v2")]
    RaidCancelV2(RaidCancelV2),
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;

    #[test]
    fn raid() {
        let message = r##"
{
    "type": "raid_go_v2",
    "raid": {
        "id": "7fbea6d0-1337-4c61-8c92-b7510e639010",
        "creator_id": "27620241",
        "source_id": "27620241",
        "target_id": "1234",
        "target_login": "tmi",
        "target_display_name": "TMI",
        "target_profile_image": "https://static-cdn.jtvnw.net/jtv_user_pictures/tmi-profile_image-deadbeef1234-70x70.jpeg",
        "transition_jitter_seconds": 5,
        "force_raid_now_seconds": 90,
        "viewer_count": 5238
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "raid.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::Raid { .. },
            }
        ));
    }

    #[test]
    fn raid_update() {
        let message = r##"
{
    "type": "raid_update_v2",
    "raid": {
        "id": "7fbea6d0-4087-48e7-a395-c0ecd15ef551",
        "creator_id": "123455",
        "source_id": "123455",
        "target_id": "123",
        "target_login": "tmi",
        "target_display_name": "tmi",
        "target_profile_image": "https://static-cdn.jtvnw.net/jtv_user_pictures/deadbeef-profile_image-70x70.png",
        "transition_jitter_seconds": 0,
        "force_raid_now_seconds": 90,
        "viewer_count": 143
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "raid.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::Raid { .. },
            }
        ));
    }

    #[test]
    fn raid_cancel() {
        let message = r##"
{
    "type": "raid_cancel_v2",
    "raid": {
        "id": "7fbea6d0-5b4b-4b8f-a79d-c9e94bf88bbf",
        "creator_id": "123455",
        "source_id": "123455",
        "target_id": "123",
        "target_login": "tmi",
        "target_display_name": "TMI",
        "target_profile_image": "https://static-cdn.jtvnw.net/jtv_user_pictures/deadbeef-profile_image-70x70.png",
        "transition_jitter_seconds": 4,
        "force_raid_now_seconds": 90,
        "viewer_count": 4463
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "raid.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::Raid { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "raid.1234";
        assert_eq!(Raid { channel_id: 1234 }, s.to_string().try_into().unwrap());
    }

    #[test]
    fn check_ser() {
        let s = "raid.1234";
        let right: String = Raid { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

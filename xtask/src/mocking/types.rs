#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub box_art_url: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Client {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Secret")]
    pub secret: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "IsExtension")]
    pub is_extension: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stream {
    pub game_id: String,
    pub game_name: String,
    pub id: String,
    pub is_mature: bool,
    pub language: String,
    pub started_at: String,
    pub tag_ids: Vec<String>,
    pub thumbnail_url: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub viewer_count: i64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subscription {
    pub broadcaster_id: String,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub is_gift: bool,
    pub plan_name: String,
    pub tier: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    pub id: String,
    pub is_auto: bool,
    pub tag_name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Team {
    pub background_image_url: Option<String>,
    pub banner: Option<String>,
    pub created_at: String,
    pub id: String,
    pub info: String,
    pub team_display_name: String,
    pub team_name: String,
    pub thumbnail_url: String,
    pub updated_at: String,
    pub users: Vec<TeamUser>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TeamUser {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub broadcaster_type: String,
    pub created_at: String,
    pub delay: i64,
    pub description: String,
    pub display_name: String,
    pub email: String,
    pub game_id: GameId,
    pub game_name: GameName,
    pub id: String,
    pub login: String,
    pub offline_image_url: String,
    pub profile_image_url: String,
    pub stream_language: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub view_count: i64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameId {
    #[serde(rename = "String")]
    pub string: String,
    #[serde(rename = "Valid")]
    pub valid: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameName {
    #[serde(rename = "String")]
    pub string: String,
    #[serde(rename = "Valid")]
    pub valid: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Video {
    pub created_at: String,
    pub duration: String,
    pub id: String,
    pub muted_segments: Vec<MutedSegment>,
    pub published_at: String,
    pub stream_id: String,
    pub thumbnail_url: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub video_description: String,
    pub video_language: String,
    pub view_count: i64,
    pub viewable: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutedSegment {
    pub duration: i64,
    pub video_offset: i64,
}

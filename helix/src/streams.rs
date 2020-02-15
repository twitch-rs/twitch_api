
use typed_builder::TypedBuilder;
use serde::Deserialize;

#[derive(PartialEq, TypedBuilder, Deserialize)]
pub struct GetStreamsReq {
    #[builder(default)]
    after: String,
    #[builder(default)]
    before: String,
    #[builder(default=20)]
    first: usize, //max 100, default 20
    #[builder(default)]
    game_id: Vec<String>,
    #[builder(default)]
    language: String,
    #[builder(default)]
    user_id: String,
    #[builder(default)]
    user_login: String,
    
}

pub struct GetStreams {
    id: String,
    user_id: String,
    user_name: String,
    game_id: String,
    type_: String,
    title: String,
    viewer_count: usize,
    language: String, 
    thumbnail_url: String, // variable height/width returned
    tag_ids: Vec<String>, // TODO: Make better
}

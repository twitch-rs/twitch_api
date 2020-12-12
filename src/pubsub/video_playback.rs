//! PubSub messages for (live) stream playback information
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// Statistics about stream
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct VideoPlayback {
    /// The channel_login to watch.
    pub channel_login: types::DisplayName,
}

impl_de_ser!(VideoPlayback, "video-playback", channel_login);

impl pubsub::Topic for VideoPlayback {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[]; // FIXME: dunno
}

/// Statistics about stream
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct VideoPlaybackById {
    /// The channel_login to watch.
    pub channel_id: u32,
}

impl_de_ser!(VideoPlaybackById, "video-playback-by-id", channel_id);

impl pubsub::Topic for VideoPlaybackById {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[]; // FIXME: dunno
}

/// Reply from [VideoPlayback] and [VideoPlaybackById]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum VideoPlaybackReply {
    /// Commercial started
    ///
    /// Commercial started in full-screen or PiP
    #[serde(rename = "commercial")]
    Commercial {
        /// Length of commercial
        length: i64,
        /// Epoch Server time when commercial started
        server_time: f64,
        /// Commercial is scheduled or not.
        #[doc(hidden)]
        scheduled: Option<bool>,
    },
    /// Current viewcount on playback
    #[serde(rename = "viewcount")]
    ViewCount {
        /// Epoch Server time
        server_time: f64,
        /// Current viewers
        viewers: i64,
    },
    /// VOD Watchparty.
    #[serde(rename = "watchparty-vod")]
    WatchPartyVod {
        /// information about VOD.
        vod: Vod,
    },
    /// Stream started
    #[serde(rename = "stream-up")]
    StreamUp {
        /// Epoch Server time
        server_time: f64,
        /// Delay as set in broadcaster settings.
        play_delay: i64,
    },
    /// Stream ended
    #[serde(rename = "stream-down")]
    StreamDown {
        /// Epoch Server time
        server_time: f64,
    },
    /// Channel hit by TOS strike, meaning it will end
    #[serde(rename = "tos-strike")]
    TosStrike {
        /// Epoch Server time
        server_time: f64,
    },
}

/// Video on Demand
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Vod {
    /// Type of broadcast
    pub broadcast_type: BroadcastType,
    /// Url increment picture. Unknown usage
    pub increment_url: String,
    /// Title of VOD
    pub title: String,
    /// Availability of VOD
    pub viewable: types::VideoPrivacy,
    /// ID of current VOD
    pub vod_id: String,
    /// ID of current watch party
    pub wp_id: String,
    /// Type of current watch party
    pub wp_type: WatchpartyType,
}

/// Watch party type
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum WatchpartyType {
    /// A rerun, i.e a highlight or saved broadcast
    Rerun,
    /// A premiere, i.e a uploaded video
    Premiere,
}

/// Type of broadcast
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum BroadcastType {
    /// Archive
    Archive,
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;
    #[test]
    fn video_playback() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "video-playback.tmi",
        "message": "{\"type\":\"viewcount\",\"server_time\":1603127341.505835,\"viewers\":2}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::VideoPlayback { .. },
            }
        ));
    }

    #[test]
    fn video_playback_by_id() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "video-playback-by-id.1234",
        "message": "{\"type\":\"viewcount\",\"server_time\":1603127341.505835,\"viewers\":2}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::VideoPlaybackById { .. },
            }
        ));
    }

    #[test]
    fn stream_up() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "video-playback.tmi",
        "message": "{\"server_time\":1603291436,\"play_delay\":0,\"type\":\"stream-up\"}"
    }
}"#;
        if let Response::Message { data } = dbg!(Response::parse(source).unwrap()) {
            if let TopicData::VideoPlayback { reply, .. } = data {
                assert!(matches!(*reply, VideoPlaybackReply::StreamUp { .. }))
            } else {
                panic!("not a videoplayback")
            }
        } else {
            panic!("not a message")
        }
    }

    #[test]
    fn stream_down() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "video-playback.tmi",
        "message": "{\"server_time\":1603141689,\"type\":\"stream-down\"}"
    }
}"#;
        if let Response::Message { data } = dbg!(Response::parse(source).unwrap()) {
            if let TopicData::VideoPlayback { reply, .. } = data {
                assert!(matches!(*reply, VideoPlaybackReply::StreamDown { .. }))
            } else {
                panic!("not a videoplayback")
            }
        } else {
            panic!("not a message")
        }
    }

    #[test]
    fn watch_party_vod_rerun() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "video-playback-by-id.1234",
        "message": "{\"type\":\"watchparty-vod\",\"vod\":{\"wp_id\":\"\",\"wp_type\":\"rerun\",\"increment_url\":\"https://countess.twitch.tv/ping.gif?u=%7B%22id%22%3A%22711110781%22%2C%22type%22%3A%22vod%22%7D\",\"vod_id\":\"1337\",\"title\":\"hi\",\"broadcast_type\":\"archive\",\"viewable\":\"public\"}}"
    }
}"#;
        if let Response::Message { data } = dbg!(Response::parse(source).unwrap()) {
            if let TopicData::VideoPlaybackById { reply, .. } = data {
                assert!(matches!(
                    *reply,
                    VideoPlaybackReply::WatchPartyVod { vod: Vod { .. } }
                ))
            } else {
                panic!("not a videoplayback")
            }
        } else {
            panic!("not a message")
        }
    }

    #[test]
    fn commercial1() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "video-playback-by-id.1234",
        "message": "{\"type\":\"commercial\",\"server_time\":1603209658.186545,\"length\":60}"
    }
}"#;
        if let Response::Message { data } = dbg!(Response::parse(source).unwrap()) {
            if let TopicData::VideoPlaybackById { reply, .. } = data {
                assert!(matches!(*reply, VideoPlaybackReply::Commercial { .. }))
            } else {
                panic!("not a videoplayback")
            }
        } else {
            panic!("not a message")
        }
    }

    #[test]
    fn commercial2() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "video-playback-by-id.1234",
        "message": "{\"type\":\"commercial\",\"server_time\":1604022504.517951,\"length\":175}"
    }
}"#;
        if let Response::Message { data } = dbg!(Response::parse(source).unwrap()) {
            if let TopicData::VideoPlaybackById { reply, .. } = data {
                assert!(matches!(*reply, VideoPlaybackReply::Commercial { .. }))
            } else {
                panic!("not a videoplayback")
            }
        } else {
            panic!("not a message")
        }
    }

    #[test]
    fn commercial_scheduled() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "video-playback-by-id.1234",
        "message": "{\"type\":\"commercial\",\"server_time\":1604087214.932556,\"length\":180,\"scheduled\":false}"
    }
}"#;
        if let Response::Message { data } = dbg!(Response::parse(source).unwrap()) {
            if let TopicData::VideoPlaybackById { reply, .. } = data {
                assert!(matches!(*reply, VideoPlaybackReply::Commercial { .. }))
            } else {
                panic!("not a videoplayback")
            }
        } else {
            panic!("not a message")
        }
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "video-playback.tmi";
        assert_eq!(
            VideoPlayback {
                channel_login: "tmi".to_string()
            },
            s.to_string().try_into().unwrap()
        );
        let s = "video-playback-by-id.1234";
        assert_eq!(
            VideoPlaybackById { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "video-playback.tmi";
        let right: String = VideoPlayback {
            channel_login: "tmi".to_string(),
        }
        .into();
        assert_eq!(s.to_string(), right);
        let s = "video-playback-by-id.1234";
        let right: String = VideoPlaybackById { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

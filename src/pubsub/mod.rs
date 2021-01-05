#![doc(alias = "pubsub")]
//! Holds serializable pubsub stuff
//!
//! Use [`TopicSubscribe::to_message`] to send subscription listen and parse the responses with [`Response::parse`]
//! # Notes
//!
//! If you find that a pubsub topic reply has a field that has not yet been added to this crate, and you don't need that field, you can enable the
//! <span
//!   class="module-item stab portability"
//!   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
//! ><code>allow_unknown_fields</code></span>
//! feature for this crate in your Cargo manifest to ignore it (and other) fields.
//!
//! # Undocumented features
//!
//! This crate has some pubsub topics that are not documented by twitch. These may stop working at any time. To enable these, use feature
//! <span
//!   class="module-item stab portability"
//!   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
//! ><code>unsupported</code></span>
//! to use them. Note that this crate doesn't try to keep changes to these pubsub topics semver compatible.

static ERROR_TRYFROM: &str = "no match";

/// Implement `From<$type> for String` for serializing and `TryFrom<String> for $type` for deserializing.
macro_rules! impl_de_ser {
    (@field $e:expr) => {".{}"};
    ($type:ident, $fmt:literal, $($field:ident),* $(,)? $(?$opt_field:ident),* $(,)?) => {
        impl From<$type> for String {
            fn from(t: $type) -> Self { format!(concat!($fmt, $(impl_de_ser!(@field $field),)+ $(impl_de_ser!(@field $opt_field),)*), $(t.$field,)*$(t.$opt_field.map(|f| f.to_string()).unwrap_or_default(),)*).trim_end_matches(".").to_owned() }
        }

        impl ::std::convert::TryFrom<String> for $type {
            type Error = &'static str;

            fn try_from(s: String) -> ::std::result::Result<Self, Self::Error> {
                if s.starts_with($fmt) {
                    let sub_s = s.strip_prefix($fmt).ok_or("could not strip str, this should never be hit")?;
                    match sub_s.split('.').collect::<Vec<_>>().as_slice() {
                        ["", $($field,)* $($opt_field,)*] => {
                            Ok($type {
                                $(
                                    $field: $field.parse()
                                            .map_err(|_| concat!("could not parse field <", stringify!($field), ">"))?,
                                )*
                                $(
                                    $opt_field: Some($opt_field.parse()
                                            .map_err(|_| concat!("could not parse field <", stringify!($opt_field), ">"))?),
                                )*
                            } )
                        }
                        #[allow(unreachable_patterns)]
                        ["", $($field,)*] => {
                            Ok($type {
                                $(
                                    $field: $field.parse()
                                            .map_err(|_| concat!("could not parse field <", stringify!($field), ">"))?,
                                )*
                                $(
                                    $opt_field: None,
                                )*
                            } )
                        }
                        _ => Err(crate::pubsub::ERROR_TRYFROM)
                    }
                } else {
                    Err(crate::pubsub::ERROR_TRYFROM)
                }
            }
        }
    };
}

use serde::{Deserialize, Deserializer, Serialize};
pub mod channel_bits;
pub mod channel_bits_badge;
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
pub mod channel_cheer;
pub mod channel_points;
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
pub mod channel_sub_gifts;
pub mod channel_subscriptions;
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
pub mod community_points;
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
pub mod following;
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
pub mod hypetrain;
pub mod moderation;
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
pub mod raid;
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
pub mod video_playback;

/// A logical partition of messages that clients may subscribe to, to get messages.
///
/// also known as event
#[cfg_attr(nightly, doc(spotlight))]
pub trait Topic: Serialize {
    /// Scopes needed by this topic
    ///
    /// This constant
    /// <span
    ///   class="module-item stab portability"
    ///   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
    /// ><code>unsupported</code></span>
    #[cfg(feature = "twitch_oauth2")]
    #[cfg_attr(nightly, doc(feature = "twitch_oauth2"))]
    const SCOPE: &'static [twitch_oauth2::Scope];
}

/// Message that can be serialized to be sent to twitchs PubSub server to subscribe or unsubscribe to a [Topic]
pub enum TopicSubscribe {
    /// Subscribe/Listen
    Listen {
        /// Random string to identify the response associated with this request.
        nonce: Option<String>,
        /// List of topics to listen on.
        topics: Vec<String>,
        /// OAuth token required to listen on some topics.
        auth_token: String,
    },
    /// Unsubscribe/Unlisten
    Unlisten {
        /// Random string to identify the response associated with this request.
        nonce: Option<String>,
        /// List of topics to listen on.
        topics: Vec<String>,
        /// OAuth token required to listen on some topics.
        auth_token: String,
    },
}

impl Serialize for TopicSubscribe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        #[derive(Serialize)]
        struct ITopicSubscribeData<'a> {
            topics: &'a [String],
            auth_token: &'a str,
        }
        #[derive(Serialize)]
        struct ITopicSubscribe<'a> {
            #[serde(rename = "type")]
            _type: &'static str,
            nonce: Option<&'a str>,
            data: ITopicSubscribeData<'a>,
        }

        match self {
            TopicSubscribe::Listen {
                nonce,
                topics,
                auth_token,
            } => ITopicSubscribe {
                _type: "LISTEN",
                nonce: nonce.as_deref(),
                data: ITopicSubscribeData {
                    topics: topics.as_slice(),
                    auth_token,
                },
            }
            .serialize(serializer),
            TopicSubscribe::Unlisten {
                nonce,
                topics,
                auth_token,
            } => ITopicSubscribe {
                _type: "UNLISTEN",
                nonce: nonce.as_deref(),
                data: ITopicSubscribeData {
                    topics: topics.as_slice(),
                    auth_token,
                },
            }
            .serialize(serializer),
        }
    }
}

impl TopicSubscribe {
    /// Convert this [`TopicSubscribe`] to a string which you can send with your client
    pub fn to_message(&self) -> Result<String, serde_json::Error> { serde_json::to_string(&self) }
}
/// Response from twitch PubSub
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TwitchResponse {
    /// The nonce that was passed in the request, if one was provided there
    pub nonce: Option<String>,
    /// The error message associated with the request, or an empty string if there is no error
    pub error: Option<String>,
}

impl TwitchResponse {
    /// Whether response indicates success or not
    pub fn is_successful(&self) -> bool { self.error.as_ref().map_or(true, |s| s.is_empty()) }
}

// FIXME: Add example
/// Message response from twitch PubSub.
///
/// See [TwitchResponse]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub enum TopicData {
    /// Response from the [channel_bits::ChannelBitsEventsV2] topic.
    ChannelBitsEventsV2 {
        /// Topic message
        topic: channel_bits::ChannelBitsEventsV2,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<channel_bits::ChannelBitsEventsV2Reply>,
    },
    /// Response from the [channel_bits_badge::ChannelBitsBadgeUnlocks] topic.
    ChannelBitsBadgeUnlocks {
        /// Topic message
        topic: channel_bits_badge::ChannelBitsBadgeUnlocks,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<channel_bits_badge::ChannelBitsBadgeUnlocksReply>,
    },
    /// Response from the [moderation::ChatModeratorActions] topic.
    ChatModeratorActions {
        /// Topic message
        topic: moderation::ChatModeratorActions,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<moderation::ChatModeratorActionsReply>,
    },
    /// Response from the [channel_points::ChannelPointsChannelV1] topic.
    ChannelPointsChannelV1 {
        /// Topic message
        topic: channel_points::ChannelPointsChannelV1,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<channel_points::ChannelPointsChannelV1Reply>,
    },
    /// Response from the [channel_subscriptions::ChannelSubscribeEventsV1] topic.
    ChannelSubscribeEventsV1 {
        /// Topic message
        topic: channel_subscriptions::ChannelSubscribeEventsV1,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<channel_subscriptions::ChannelSubscribeEventsV1Reply>, // FIXME: :)
    },
    /// Response from the [community_points::CommunityPointsChannelV1] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    CommunityPointsChannelV1 {
        /// Topic message
        topic: community_points::CommunityPointsChannelV1,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<channel_points::ChannelPointsChannelV1Reply>,
    },
    /// Response from the [channel_cheer::ChannelCheerEventsPublicV1] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    ChannelCheerEventsPublicV1 {
        /// Topic message
        topic: channel_cheer::ChannelCheerEventsPublicV1,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<channel_cheer::ChannelCheerEventsPublicV1Reply>,
    },
    /// Response from the [channel_sub_gifts::ChannelSubGiftsV1] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    ChannelSubGiftsV1 {
        /// Topic message
        topic: channel_sub_gifts::ChannelSubGiftsV1,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<channel_sub_gifts::ChannelSubGiftsV1Reply>,
    },

    /// Response from the [video_playback::VideoPlayback] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    VideoPlayback {
        /// Topic message
        topic: video_playback::VideoPlayback,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<video_playback::VideoPlaybackReply>,
    },
    /// Response from the [video_playback::VideoPlaybackById] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    VideoPlaybackById {
        /// Topic message
        topic: video_playback::VideoPlaybackById,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<video_playback::VideoPlaybackReply>,
    },
    /// Response from the [hypetrain::HypeTrainEventsV1] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    HypeTrainEventsV1 {
        /// Topic message
        topic: hypetrain::HypeTrainEventsV1,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<hypetrain::HypeTrainEventsV1Reply>, // FIXME: May not be correct
    },
    /// Response from the [hypetrain::HypeTrainEventsV1Rewards] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    HypeTrainEventsV1Rewards {
        /// Topic message
        topic: hypetrain::HypeTrainEventsV1Rewards,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<hypetrain::HypeTrainEventsV1Reply>,
    },
    /// Response from the [following::Following] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    Following {
        /// Topic message
        topic: following::Following,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<following::FollowingReply>,
    },
    /// Response from the [raid::Raid] topic.
    #[cfg(feature = "unsupported")]
    #[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
    Raid {
        /// Topic message
        topic: raid::Raid,
        /// Message reply from topic subscription
        #[serde(rename = "message")]
        reply: Box<raid::RaidReply>,
    },
}

// This impl is here because otherwise we hide the errors from deser
impl<'de> Deserialize<'de> for TopicData {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // FIXME: make into macro or actually upstream into serde..., untagged_force = "field"

        #[derive(Deserialize, PartialEq, Eq, Debug)]
        #[serde(untagged)]
        enum ITopicMessage {
            #[cfg(feature = "unsupported")]
            CommunityPointsChannelV1(community_points::CommunityPointsChannelV1),
            ChannelBitsEventsV2(channel_bits::ChannelBitsEventsV2),
            ChannelBitsBadgeUnlocks(channel_bits_badge::ChannelBitsBadgeUnlocks),
            #[cfg(feature = "unsupported")]
            ChannelCheerEventsPublicV1(channel_cheer::ChannelCheerEventsPublicV1),
            #[cfg(feature = "unsupported")]
            ChannelSubGiftsV1(channel_sub_gifts::ChannelSubGiftsV1),
            ChatModeratorActions(moderation::ChatModeratorActions),
            ChannelPointsChannelV1(channel_points::ChannelPointsChannelV1),
            ChannelSubscribeEventsV1(channel_subscriptions::ChannelSubscribeEventsV1),
            #[cfg(feature = "unsupported")]
            VideoPlayback(video_playback::VideoPlayback),
            #[cfg(feature = "unsupported")]
            VideoPlaybackById(video_playback::VideoPlaybackById),
            #[cfg(feature = "unsupported")]
            HypeTrainEventsV1(hypetrain::HypeTrainEventsV1),
            #[cfg(feature = "unsupported")]
            HypeTrainEventsV1Rewards(hypetrain::HypeTrainEventsV1Rewards),
            #[cfg(feature = "unsupported")]
            Following(following::Following),
            #[cfg(feature = "unsupported")]
            Raid(raid::Raid),
        }

        #[derive(Deserialize, Debug)]
        struct ITopicData {
            topic: ITopicMessage,
            message: String,
        }
        let reply = ITopicData::deserialize(deserializer).map_err(|e| {
            serde::de::Error::custom(format!("could not deserialize topic reply: {}", e))
        })?;
        Ok(match reply.topic {
            #[cfg(feature = "unsupported")]
            ITopicMessage::CommunityPointsChannelV1(topic) => TopicData::CommunityPointsChannelV1 {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            ITopicMessage::ChannelBitsEventsV2(topic) => TopicData::ChannelBitsEventsV2 {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            ITopicMessage::ChannelBitsBadgeUnlocks(topic) => TopicData::ChannelBitsBadgeUnlocks {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            ITopicMessage::ChannelSubGiftsV1(topic) => TopicData::ChannelSubGiftsV1 {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            ITopicMessage::ChannelCheerEventsPublicV1(topic) => {
                TopicData::ChannelCheerEventsPublicV1 {
                    topic,
                    reply: serde_json::from_str(&reply.message)
                        .map_err(serde::de::Error::custom)?,
                }
            }
            ITopicMessage::ChatModeratorActions(topic) => TopicData::ChatModeratorActions {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            ITopicMessage::ChannelPointsChannelV1(topic) => TopicData::ChannelPointsChannelV1 {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            ITopicMessage::ChannelSubscribeEventsV1(topic) => TopicData::ChannelSubscribeEventsV1 {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            ITopicMessage::VideoPlayback(topic) => TopicData::VideoPlayback {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            ITopicMessage::VideoPlaybackById(topic) => TopicData::VideoPlaybackById {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            ITopicMessage::HypeTrainEventsV1(topic) => TopicData::HypeTrainEventsV1 {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            ITopicMessage::HypeTrainEventsV1Rewards(topic) => TopicData::HypeTrainEventsV1Rewards {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            ITopicMessage::Following(topic) => TopicData::Following {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            ITopicMessage::Raid(topic) => TopicData::Raid {
                topic,
                reply: serde_json::from_str(&reply.message).map_err(serde::de::Error::custom)?,
            },
        })
    }
}

/// Response from twitchs PubSub server.
/// Either a response indicating status of something or a message from a topic
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    /// Response from a subscription/unsubscription
    #[serde(rename = "RESPONSE")]
    Response(TwitchResponse),
    /// Message received containing all applicable data
    #[serde(rename = "MESSAGE")]
    Message {
        /// Data corresponding to [topic](Topic) message
        data: TopicData,
    },
}

impl Response {
    // FIXME: Add example
    /// Parse string slice as a response.
    pub fn parse(source: &str) -> Result<Response, serde_json::Error> {
        serde_json::from_str(source)
    }
}

/// Deserialize 'null' as <T as Default>::Default
fn deserialize_default_from_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default, {
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error() {
        let source = r#"
{
    "type": "RESPONSE",
    "nonce": "44h1k13746815ab1r2",
    "error": ""
}
"#;
        let expected = Response::Response(TwitchResponse {
            nonce: Some(String::from("44h1k13746815ab1r2")),
            error: Some(String::new()),
        });
        let actual = Response::parse(source).unwrap();
        assert_eq!(expected, actual);
    }
}

//! PubSub messages for when AutoMod flags a message as potentially inappropriate, and when a moderator takes action on a message.
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A user follows the channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "String", try_from = "String")]
pub struct AutoModQueue {
    /// The currently authenticated moderator
    pub moderator_id: u32,
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    AutoModQueue,
    "automod-queue",
    moderator_id,
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for AutoModQueue {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelModerate];

    fn into_topic(self) -> pubsub::Topics { super::Topics::AutoModQueue(self) }
}

/// Reply from [AutoModQueue]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(tag = "type", content = "data")]
#[non_exhaustive]
pub enum AutoModQueueReply {
    /// Message held by automod
    #[serde(rename = "automod_caught_message")]
    AutoModCaughtMessage(AutoModCaughtMessage),
}

/// Message held by automod
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutoModCaughtMessage {
    /// Classification of caught message
    pub content_classification: ContentClassification,
    /// The message that was sent
    pub message: Message,
    // TODO: What is this?
    /// Code for reason
    #[serde(
        default,
        deserialize_with = "pubsub::deserialize_none_from_empty_string"
    )]
    pub reason_code: Option<String>,
    /// User ID of who resolved the message in the queue
    #[serde(
        default,
        deserialize_with = "pubsub::deserialize_none_from_empty_string"
    )]
    pub resolver_id: Option<types::UserId>,
    /// Username of who resolved the message in the queue
    #[serde(
        default,
        deserialize_with = "pubsub::deserialize_none_from_empty_string"
    )]
    pub resolver_login: Option<types::UserName>,
    /// Status of the message in the queue
    pub status: types::AutomodStatus,
}

/// Classification for content according to AutoMod
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ContentClassification {
    // FIXME: Enum?
    /// Category for classification
    ///
    /// On twitch, these are the different categories available for AutoMod
    ///
    /// * Aggression
    ///   Threatening, inciting, or promoting violence or other harm
    /// * Bullying
    ///   Name-calling, insults, or antagonization
    /// * Disability
    ///   Demonstrating hatred or prejudice based on perceived or actual mental or physical abilities
    /// * Sexuality, sex, or gender
    ///   Demonstrating hatred or prejudice based on sexual identity, sexual orientation, gender identity, or gender expression
    /// * Misogyny
    ///   Demonstrating hatred or prejudice against women, including sexual objectification
    /// * Race, ethnicity, or religion
    ///   Demonstrating hatred or prejudice based on race, ethnicity, or religion
    /// * Sex-based terms
    ///   Sexual acts, anatomy
    /// * Swearing
    ///   Swear words, &*^!#@%
    pub category: String,
    /// Level of classification, eg. how strongly related the classification is related according to AutoMod
    pub level: i64,
}

/// Message that was caught by AutoMod
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Message {
    /// The content of the message
    pub content: Content,
    /// Chat ID of the message
    pub id: types::MsgId,
    /// User that sent the message
    pub sender: types::User,
    /// Time at which the message was sent
    pub sent_at: types::Timestamp,
}

/// The contents of a AutoMod message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Content {
    /// The message split up in fragments.
    ///
    /// The message can be retrieved in full with [`text`](Self::text)
    pub fragments: Vec<Fragment>,
    /// The full message that was sent
    pub text: String,
}

/// A fragment of a AutoModded message
///
/// Can either be regular text, or classified as part of the reason for AutoMod
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(untagged)]
#[non_exhaustive]
pub enum Fragment {
    /// Fragment that is classified under a AutoMod category which is being filtered out
    AutomodFragment {
        /// Text associated with this fragment
        text: String,
        /// AutoMod classification of the fragment
        automod: Automod,
    },
    /// Fragment that is not classified under a AutoMod category
    TextFragment {
        /// Text associated with this fragment
        text: String,
    },
}

/// Specific AutoMod classification
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Automod {
    // FIXME: This should be a hash map of enum, i64
    /// The different topics and their level for the automod reason.
    ///
    /// # Example
    /// ```text
    /// "topics": {
    ///     "vulgar": 6
    /// }
    /// ```
    pub topics: std::collections::HashMap<String, i64>,
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;
    #[test]
    fn automodcaught() {
        let source = r#"
        {"type":"MESSAGE","data":{"topic":"automod-queue.27620241.27620241","message":"{\"type\":\"automod_caught_message\",\"data\":{\"content_classification\":{\"category\":\"swearing\",\"level\":2},\"message\":{\"content\":{\"text\":\"fuck you xd\",\"fragments\":[{\"text\":\"fuck you\",\"automod\":{\"topics\":{\"vulgar\":6}}},{\"text\":\" xd\"}]},\"id\":\"a7e3f713-b220-444a-b54a-348b981b6bf0\",\"sender\":{\"user_id\":\"268131879\",\"login\":\"prettyb0i_swe\",\"display_name\":\"prettyb0i_swe\"},\"sent_at\":\"2021-05-17T19:28:31.062898778Z\"},\"reason_code\":\"\",\"resolver_id\":\"27620241\",\"resolver_login\":\"emilgardis\",\"status\":\"DENIED\"}}"}}
        "#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::AutoModQueue { .. },
            }
        ));
    }

    #[test]
    fn automodcaught2() {
        let source = r#"
        {"type":"MESSAGE","data":{"topic":"automod-queue.27620241.27620241","message":"{\"type\":\"automod_caught_message\",\"data\":{\"content_classification\":{\"category\":\"aggression\",\"level\":4},\"message\":{\"content\":{\"text\":\"you suck balls\",\"fragments\":[{\"text\":\"you suck balls\",\"automod\":{\"topics\":{\"bullying\":3,\"dating_and_sexting\":7,\"vulgar\":5}}}]},\"id\":\"23b15313-ff6c-4e1c-8d0d-ea9c382a3806\",\"sender\":{\"user_id\":\"268131879\",\"login\":\"prettyb0i_swe\",\"display_name\":\"prettyb0i_swe\"},\"sent_at\":\"2021-05-29T13:12:41.237693525Z\"},\"reason_code\":\"\",\"resolver_id\":\"\",\"resolver_login\":\"\",\"status\":\"PENDING\"}}"}}
        "#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::AutoModQueue { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "automod-queue.27620241.27620241";
        assert_eq!(
            AutoModQueue {
                channel_id: 27620241,
                moderator_id: 27620241
            },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "automod-queue.27620241.27620241";
        let right: String = AutoModQueue {
            channel_id: 27620241,
            moderator_id: 27620241,
        }
        .into();
        assert_eq!(s.to_string(), right);
    }
}

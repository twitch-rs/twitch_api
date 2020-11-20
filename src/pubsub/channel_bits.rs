//! PubSub messages for bits
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// Anyone cheers in a specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct ChannelBitsEventsV2 {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(ChannelBitsEventsV2, "channel-bits-events-v2", channel_id);

impl pubsub::Topic for ChannelBitsEventsV2 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::BitsRead];
}

/// Reply from [ChannelBitsEventsV2]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(tag = "message_type")]
#[non_exhaustive]
pub enum ChannelBitsEventsV2Reply {
    /// Bits event
    #[serde(rename = "bits_event")]
    BitsEvent {
        /// Data associated with reply
        data: BitsEventData,
        /// Message ID of message associated with this `bits_event`
        message_id: String,
        /// Version of `channel-bits-events-v2` reply
        version: String,
        #[doc(hidden)]
        #[serde(default)] // FIXME: docs seems to be wrong here.
        is_anonymous: bool,
    },
}

/// Data for bits event
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BitsEventData {
    /// If set, describes new unlocked badge for user
    pub badge_entitlement: Option<BadgeEntitlement>,
    /// The number of bits that were sent.
    pub bits_used: i64,
    /// ID of channel where message was sent
    pub channel_id: types::UserId,
    /// Username of channel where message was sent
    pub channel_name: types::UserName,
    /// The full message that was sent with the bits.
    pub chat_message: String,
    /// Context of `bits_event`, seems to only be [`cheer`](BitsContext::Cheer)
    pub context: BitsContext,
    #[serde(default)] // FIXME: docs don't have this field here, but actual responses do
    /// Whether the cheer was anonymous.
    pub is_anonymous: bool,
    /// Time when pubsub message was sent
    pub time: types::Timestamp,
    /// The total number of bits that were ever sent by the user in the channel.
    pub total_bits_used: i64,
    /// ID of user that sent message
    pub user_id: types::UserId,
    /// Name of user that sent message
    pub user_name: types::UserName,
}

/// [`ChannelBitsEventsV2Reply::BitsEvent`] event unlocked new badge for user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BadgeEntitlement {
    /// New version of badge
    new_version: u64,
    /// Previous version of badge
    previous_version: u64,
}

/// Context that triggered pubsub message
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BitsContext {
    /// Cheer
    #[serde(rename = "cheer")]
    Cheer,
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;
    #[test]
    fn bits_event() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "channel-bits-events-v2.1234",
        "message": "{\"data\":{\"user_name\":\"justintv\",\"channel_name\":\"tmi\",\"user_id\":\"12345\",\"channel_id\":\"1234\",\"time\":\"2020-10-19T17:50:24.807841596Z\",\"chat_message\":\"Corgo1 Corgo1 Corgo1 Corgo1 Corgo1\",\"bits_used\":5,\"total_bits_used\":29,\"is_anonymous\":false,\"context\":\"cheer\",\"badge_entitlement\":null},\"version\":\"1.0\",\"message_type\":\"bits_event\",\"message_id\":\"d1831817-95f2-5dfa-8864-f36f16eeb5d8\"}"
    }
}"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message{
                data: TopicData::ChannelBitsEventsV2 { .. },
            }
        ));
    }

    #[test]
    fn bits_event_documented() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
       "topic": "channel-bits-events-v2.46024993",
       "message": "{\"data\":{\"user_name\":\"jwp\",\"channel_name\":\"bontakun\",\"user_id\":\"95546976\",\"channel_id\":\"46024993\",\"time\":\"2017-02-09T13:23:58.168Z\",\"chat_message\":\"cheer10000 New badge hype!\",\"bits_used\":10000,\"total_bits_used\":25000,\"context\":\"cheer\",\"badge_entitlement\":{\"new_version\":25000,\"previous_version\":10000}},\"version\":\"1.0\",\"message_type\":\"bits_event\",\"message_id\":\"8145728a4-35f0-4cf7-9dc0-f2ef24de1eb6\",\"is_anonymous\":true}"
    }
}
"#;

        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message{
                data: TopicData::ChannelBitsEventsV2 { .. },
            }
        ));
    }
    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "channel-bits-events-v2.1234";
        assert_eq!(
            ChannelBitsEventsV2 { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "channel-bits-events-v2.1234";
        let right: String = ChannelBitsEventsV2 { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

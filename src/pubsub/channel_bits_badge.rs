#![doc(alias = "channel-bits-badge-unlocks")]
//! PubSub messages for bits
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// Anyone shares a bit badge in a specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(into = "String", try_from = "String")]
pub struct ChannelBitsBadgeUnlocks {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    ChannelBitsBadgeUnlocks,
    "channel-bits-badge-unlocks",
    channel_id
);

impl pubsub::Topic for ChannelBitsBadgeUnlocks {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::BitsRead];

    fn into_topic(self) -> pubsub::Topics { super::Topics::ChannelBitsBadgeUnlocks(self) }
}

/// Reply from [ChannelBitsBadgeUnlocks]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelBitsBadgeUnlocksReply {
    /// Value of Bits badge tier that was earned (1000, 10000, etc.)
    pub badge_tier: i64,
    /// ID of channel where user earned the new Bits badge
    pub channel_id: types::UserId,
    /// Login of channel where user earned the new Bits badge
    pub channel_name: types::UserName,
    /// Custom message included with share
    pub chat_message: Option<String>,
    /// Time when the new Bits badge was earned.
    pub time: types::Timestamp,
    /// ID of user who earned the new Bits badge
    pub user_id: types::UserId,
    /// Login of user who earned the new Bits badge
    pub user_name: types::UserName,
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;
    #[test]
    fn bits_badge_unlock() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "channel-bits-badge-unlocks.80525799",
        "message": "{\"user_id\":\"1234\",\"user_name\":\"tmi\",\"channel_id\":\"12345\",\"channel_name\":\"justintv\",\"badge_tier\":25000,\"chat_message\":\"All the bits was for this. Worth it.\",\"time\":\"2020-11-01T20:14:14.075154315Z\"}"
    }
}
"#;
        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelBitsBadgeUnlocks { .. },
            }
        ));
    }

    #[test]
    fn bits_badge_unlock_documented() {
        let source = r#"
{
    "type": "MESSAGE",
    "data": {
        "topic": "channel-bits-badge-unlocks.401394874",
        "message": "{\"user_id\":\"232889822\",\"user_name\":\"willowolf\",\"channel_id\":\"401394874\",\"channel_name\":\"fun_test12345\",\"badge_tier\":1000,\"chat_message\":\"this should be received by the public pubsub listener\",\"time\":\"2020-12-06T00:01:43.71253159Z\"}"
    }
}
"#;

        let actual = dbg!(Response::parse(source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelBitsBadgeUnlocks { .. },
            }
        ));
    }
    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "channel-bits-badge-unlocks.1234";
        assert_eq!(
            ChannelBitsBadgeUnlocks { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "channel-bits-badge-unlocks.1234";
        let right: String = ChannelBitsBadgeUnlocks { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

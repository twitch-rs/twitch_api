#![doc(alias = "cheer")]
#![doc(alias = "channel-cheer-events-public-v1")]
//! PubSub messages for cheer events
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A user redeems a cheer with shared rewards.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(into = "String", try_from = "String")]
pub struct ChannelCheerEventsPublicV1 {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    ChannelCheerEventsPublicV1,
    "channel-cheer-events-public-v1",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for ChannelCheerEventsPublicV1 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];

    fn into_topic(self) -> pubsub::Topics { super::Topics::ChannelCheerEventsPublicV1(self) }
}

/// Reply from [ChannelCheerEventsPublicV1]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
#[non_exhaustive]
pub enum ChannelCheerEventsPublicV1Reply {
    /// A cheer bomb happened
    #[serde(rename = "cheerbomb")]
    CheerBomb {
        /// Display name of user
        #[serde(rename = "displayName")]
        display_name: types::DisplayName,
        /// Domain of cheer reward. Name of active twitch event
        domain: String,
        /// Selected count for cheer. e.g How many that will receive rewards
        #[serde(rename = "selectedCount")]
        selected_count: i64,
        /// Unknown
        #[serde(rename = "totalRewardCount")]
        total_reward_count: i64,
        /// Unknown
        #[serde(rename = "triggerAmount")]
        trigger_amount: i64,
        /// Type of cheerbomb.
        #[serde(rename = "triggerType")]
        trigger_type: TriggerType,
        /// Id of the user
        #[serde(rename = "userID")]
        user_id: types::UserId,
        /// Login name of the user, not capitalized
        #[serde(rename = "userLogin")]
        user_login: types::UserName,
    },
}

/// Trigger for cheer event/cheer bomb
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum TriggerType {
    /// Subscription
    Subscription,
    /// Subscription gift
    SubGift,
    /// Cheer
    Cheer,
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;
    #[test]
    fn channel_cheer_event_cheerbomb_sub() {
        let message = r##"
{
    "type": "cheerbomb",
    "data": {
        "userID": "1234",
        "displayName": "TMI",
        "userLogin": "tmi",
        "selectedCount": 5,
        "triggerType": "SUBSCRIPTION",
        "triggerAmount": 1,
        "totalRewardCount": 5,
        "domain": "kpop_megacommerce"
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE","data": {{ "topic": "channel-cheer-events-public-v1.27620241", "message": {message:?} }}}}"#
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelCheerEventsPublicV1 { .. },
            }
        ));
    }

    #[test]
    fn channel_cheer_event_cheerbomb_subgift() {
        let message = r##"
{
    "type": "cheerbomb",
    "data": {
        "userID": "1234",
        "displayName": "tmi",
        "userLogin": "TMI",
        "selectedCount": 25,
        "triggerType": "SUBGIFT",
        "triggerAmount": 5,
        "totalRewardCount": 25,
        "domain": "kpop_megacommerce"
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE","data": {{ "topic": "channel-cheer-events-public-v1.27620241", "message": {message:?} }}}}"#
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelCheerEventsPublicV1 { .. },
            }
        ));
    }

    #[test]
    fn channel_cheer_event_cheerbomb_cheer() {
        let message = r##"
{
    "type": "cheerbomb",
    "data": {
        "userID": "1234",
        "displayName": "tmi",
        "userLogin": "TMI",
        "selectedCount": 10,
        "triggerType": "CHEER",
        "triggerAmount": 600,
        "totalRewardCount": 10,
        "domain": "kpop_megacommerce"
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE","data": {{ "topic": "channel-cheer-events-public-v1.27620241", "message": {message:?} }}}}"#
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::ChannelCheerEventsPublicV1 { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "channel-cheer-events-public-v1.1234";
        assert_eq!(
            ChannelCheerEventsPublicV1 { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "channel-cheer-events-public-v1.1234";
        let right: String = ChannelCheerEventsPublicV1 { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

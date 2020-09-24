//! PubSub messages for subscriptions
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A subscription event happens in channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "String", try_from = "String")]
pub struct ChannelSubscribeEventsV1 {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    ChannelSubscribeEventsV1,
    "channel-subscribe-events-v1",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for ChannelSubscribeEventsV1 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelSubscriptions];
}

// FIXME: Missing anonsubgift and anonresubgift
// Should probably share fields.
/// Reply from [ChannelSubscribeEventsV1]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "context")]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum ChannelSubscribeEventsV1Reply {
    /// A subscription
    #[serde(rename = "sub")]
    Sub {
        // FIXME: Could be for month that subscription ends
        /// Unknown
        benefit_end_month: i64,
        /// ID of the channel that has been subscribed or subgifted
        channel_id: types::UserId,
        /// Name of the channel that has been subscribed or subgifted
        channel_name: types::UserName,
        /// Cumulative months that user has been subscribed
        cumulative_months: i64,
        /// Resubscription is a gift
        is_gift: bool,
        #[doc(hidden)]
        months: i64,
        // FIXME: should be a enum
        /// Duration of subscription, e.g 1, 3 or 6
        multi_month_duration: i64,
        /// Message sent with this subscription
        sub_message: SubMessage,
        /// Subscription plan
        sub_plan: types::SubscriptionTier,
        /// Name of subscription plan
        sub_plan_name: String,
        /// Time when pubsub message was sent
        time: types::Timestamp,
        /// ID of user that subscribed
        user_id: types::UserId,
        /// Username of user that subscribed
        user_name: types::UserName,
        /// Display name of user that subscribed
        display_name: types::DisplayName,
    },
    /// A resubscription
    #[serde(rename = "resub")]
    ReSub {
        // missing in documented example
        // FIXME: Could be for month that subscription ends
        /// Unknown
        benefit_end_month: Option<i64>,
        /// ID of the channel that has been subscribed or subgifted
        channel_id: types::UserId,
        /// Name of the channel that has been subscribed or subgifted
        channel_name: types::UserName,
        /// Cumulative months that user has been subscribed
        cumulative_months: i64,
        /// Resubscription is a gift
        is_gift: bool,
        /// Months the user has been subscribed for in a row.
        streak_months: Option<i64>,
        #[doc(hidden)]
        months: i64,
        /// Duration of subscription, e.g 1, 3 or 6
        multi_month_duration: Option<i64>,
        /// Message sent with this subscription
        sub_message: SubMessage,
        /// Subscription plan
        sub_plan: types::SubscriptionTier,
        /// Name of subscription plan
        sub_plan_name: String,
        /// Time when pubsub message was sent
        time: types::Timestamp,
        /// ID of user that subscribed
        user_id: types::UserId,
        /// Username of user that subscribed
        user_name: types::UserName,
        /// Display name of user that subscribed
        display_name: types::DisplayName,
    },
    /// A gifted subscription happened
    #[serde(rename = "subgift")]
    SubGift {
        // missing in documented example
        // FIXME: Could be for month that subscription ends
        /// Unknown
        benefit_end_month: Option<i64>,
        /// ID of the channel that has been subscribed or subgifted
        channel_id: types::UserId,
        /// Name of the channel that has been subscribed or subgifted
        channel_name: types::UserName,
        #[doc(hidden)]
        is_gift: bool,
        /// Months
        months: i64,
        // FIXME: should be a enum
        /// Duration of subscription, e.g 1, 3 or 6
        multi_month_duration: Option<i64>,
        /// Display name of user that received gifted subscription
        recipient_display_name: types::DisplayName,
        /// Username of user that received gifted subscription
        recipient_id: types::UserId,
        /// Username of user that received gifted subscription
        recipient_user_name: types::UserName,
        /// Message sent with this subscription
        #[doc(hidden)]
        sub_message: SubMessage,
        /// Subscription plan
        sub_plan: types::SubscriptionTier,
        /// Name of subscription plan
        sub_plan_name: String,
        /// Time when pubsub message was sent
        time: types::Timestamp,
        /// ID of user that purchased gifted subscription
        user_id: types::UserId,
        /// Username of user that purchased gifted subscription
        user_name: types::UserName,
        /// Display name of user that purchased gifted subscription
        display_name: types::DisplayName,
    },
}

/// Described where in a message a emote is
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Emote {
    // FIXME: Mention how to get the emote picture
    /// ID of emote
    pub id: String,
    /// Start index of emote in message
    pub start: i64,
    /// End index of emote in message
    pub end: i64,
}

/// Message sent with subscription
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SubMessage {
    /// Emotes in subscription message
    #[serde(deserialize_with = "pubsub::deserialize_default_from_null")]
    pub emotes: Vec<Emote>,
    /// Message in subscription
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;

    #[test]
    fn subscription_doc_example_resub() {
        // twitch docs broken as usual. /emotes/id  is a string and /months is missing
        let message = r##"
{
    "user_name": "tww2",
    "display_name": "TWW2",
    "channel_name": "mr_woodchuck",
    "user_id": "13405587",
    "channel_id": "89614178",
    "time": "2015-12-19T16:39:57-08:00",
    "sub_plan": "1000",
    "sub_plan_name": "Channel Subscription (mr_woodchuck)",
    "cumulative_months": 9,
    "streak_months": 3,
    "months": 0,
    "context": "resub",
    "is_gift": false,
    "sub_message": {
        "message": "A Twitch baby is born! KappaHD",
        "emotes": [
            {
                "start": 23,
                "end": 7,
                "id": "2867"
            }
        ]
    }
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "channel-subscribe-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message{
                data: TopicData::ChannelSubscribeEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn subscription_doc_example_subgift() {
        let message = r##"
{
    "user_name": "tww2",
    "display_name": "TWW2",
    "channel_name": "mr_woodchuck",
    "user_id": "13405587",
    "channel_id": "89614178",
    "time": "2015-12-19T16:39:57-08:00",
    "sub_plan": "1000",
    "sub_plan_name": "Channel Subscription (mr_woodchuck)",
    "months": 9,
    "context": "subgift",
    "is_gift": true,
    "sub_message": {
        "message": "",
        "emotes": null
    },
    "recipient_id": "19571752",
    "recipient_user_name": "forstycup",
    "recipient_display_name": "forstycup"
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "channel-subscribe-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message{
                data: TopicData::ChannelSubscribeEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn new_sub() {
        let message = r##"
{
    "benefit_end_month": 11,
    "user_name": "tmi",
    "display_name": "tmi",
    "channel_name": "emilgardis",
    "user_id": "1234",
    "channel_id": "27620241",
    "time": "2020-10-20T22:17:43.242793831Z",
    "sub_message": {
        "message": "",
        "emotes": null
    },
    "sub_plan": "1000",
    "sub_plan_name": "Channel Subscription (emilgardis)",
    "months": 0,
    "cumulative_months": 1,
    "context": "sub",
    "is_gift": false,
    "multi_month_duration": 0
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "channel-subscribe-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message{
                data: TopicData::ChannelSubscribeEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn gifted_sub() {
        let message = r##"
{
    "benefit_end_month": 0,
    "user_name": "tmi",
    "display_name": "tmi",
    "channel_name": "emilgardis",
    "user_id": "1234",
    "channel_id": "27620241",
    "recipient_id": "1337",
    "recipient_user_name": "justintv",
    "recipient_display_name": "justintv",
    "time": "2020-10-20T22:18:17.542444893Z",
    "sub_message": {
        "message": "",
        "emotes": null
    },
    "sub_plan": "1000",
    "sub_plan_name": "Channel Subscription (emilgardis)",
    "months": 1,
    "context": "subgift",
    "is_gift": true,
    "multi_month_duration": 1
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "channel-subscribe-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message{
                data: TopicData::ChannelSubscribeEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn resub() {
        let message = r##"
{
    "benefit_end_month": 0,
    "user_name": "tmi",
    "display_name": "tmi",
    "channel_name": "emilgardis",
    "user_id": "1234",
    "channel_id": "80525799",
    "time": "2020-10-25T17:15:36.541972298Z",
    "sub_message": {
        "message": "message here",
        "emotes": [
            {
                "start": 191,
                "end": 197,
                "id": "12342378"
            }
        ]
    },
    "sub_plan": "2000",
    "sub_plan_name": "Channel Subscription (emilgardis): $9.99 Sub",
    "months": 0,
    "cumulative_months": 12,
    "streak_months": 12,
    "context": "resub",
    "is_gift": false,
    "multi_month_duration": 0
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "channel-subscribe-events-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message{
                data: TopicData::ChannelSubscribeEventsV1 { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "channel-subscribe-events-v1.1234";
        assert_eq!(
            ChannelSubscribeEventsV1 { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "channel-subscribe-events-v1.1234";
        let right: String = ChannelSubscribeEventsV1 { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

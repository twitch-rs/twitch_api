//! PubSub messages for sub gifts
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A user gifts subs.
///
/// This allows one to know how many subs were gifted in a single event. See also [`pubsub::channel_subscriptions::ChannelSubscribeEventsV1`] which needs token from broadcaster
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "String", try_from = "String")]
pub struct ChannelSubGiftsV1 {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    ChannelSubGiftsV1,
    "channel-sub-gifts-v1",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for ChannelSubGiftsV1 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

/// Reply from [ChannelSubGiftsV1]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChannelSubGiftsV1Reply {
    /// Mystery gift
    #[serde(rename = "mystery-gift-purchase")]
    MysteryGiftPurchase {
        /// Channel where mystery gifts were distributed
        channel_id: types::UserId,
        /// Amount of mystery gifts
        count: i64,
        /// Tier of mystery gifts
        tier: types::SubscriptionTier,
        /// User ID of user that purchased mystery gifts
        user_id: types::UserId,
        /// Unknown
        uuid: String,
    },
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;

    #[test]
    fn channel_sub_gifts() {
        let message = r##"
{
    "count": 5,
    "tier": "1000",
    "user_id": "1234",
    "channel_id": "27620241",
    "uuid": "d749201e-675d-46fb-8ac7-ff2418a0bb99",
    "type": "mystery-gift-purchase"
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "channel-sub-gifts-v1.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message{
                data: TopicData::ChannelSubGiftsV1 { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "channel-sub-gifts-v1.1234";
        assert_eq!(
            ChannelSubGiftsV1 { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "channel-sub-gifts-v1.1234";
        let right: String = ChannelSubGiftsV1 { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

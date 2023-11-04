#![doc(alias = "community-points-channel-v1")]
//! PubSub messages for community points.
//!
//! See also [`pubsub::channel_points`]
use crate::pubsub;
use serde_derive::{Deserialize, Serialize};

/// A user redeems an reward using channel points.
///
/// Reply is [`pubsub::channel_points::ChannelPointsChannelV1Reply`]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(into = "String", try_from = "String")]
pub struct CommunityPointsChannelV1 {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    CommunityPointsChannelV1,
    "community-points-channel-v1",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for CommunityPointsChannelV1 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];

    fn into_topic(self) -> pubsub::Topics { super::Topics::CommunityPointsChannelV1(self) }
}
#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;

    #[test]
    fn channel_point_redeem() {
        let message = r##"
{
    "type": "reward-redeemed",
    "data": {
        "timestamp": "2020-10-10T19:13:30.536153182Z",
        "redemption": {
            "id": "b021f290-bedb-49c2-b90f-e6ceb1c0d4ab",
            "user": {
                "id": "27620241",
                "login": "emilgardis",
                "display_name": "emilgardis"
            },
            "channel_id": "27620241",
            "redeemed_at": "2020-10-10T19:13:30.536153182Z",
            "reward": {
                "id": "252e209d-4f16-4886-a0d1-97f458ad5698",
                "channel_id": "27620241",
                "title": "Hydration",
                "prompt": "Make Emilgardis drink water",
                "cost": 2000,
                "is_user_input_required": true,
                "is_sub_only": false,
                "image": null,
                "default_image": {
                    "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                    "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                    "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
                },
                "background_color": "#81AEFF",
                "is_enabled": true,
                "is_paused": false,
                "is_in_stock": true,
                "max_per_stream": {
                    "is_enabled": false,
                    "max_per_stream": 10
                },
                "should_redemptions_skip_request_queue": false,
                "template_id": null,
                "updated_for_indicator_at": "2020-02-06T17:29:19.737311439Z",
                "max_per_user_per_stream": {
                    "is_enabled": false,
                    "max_per_user_per_stream": 0
                },
                "global_cooldown": {
                    "is_enabled": false,
                    "global_cooldown_seconds": 0
                },
                "redemptions_redeemed_current_stream": 0,
                "cooldown_expires_at": null
            },
            "user_input": "bap",
            "status": "UNFULFILLED"
        }
    }
}
"##;
        let source = format!(
            r#"{{"type": "MESSAGE","data": {{ "topic": "community-points-channel-v1.27620241", "message": {message:?} }}}}"#
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::CommunityPointsChannelV1 { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "community-points-channel-v1.1234";
        assert_eq!(
            CommunityPointsChannelV1 { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "community-points-channel-v1.1234";
        let right: String = CommunityPointsChannelV1 { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

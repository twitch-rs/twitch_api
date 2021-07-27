#![doc(alias = "follow")]
#![doc(alias = "follows")]
//! PubSub messages for follows
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A user follows the channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(into = "String", try_from = "String")]
pub struct Following {
    /// The channel_id to watch. Can be fetched with the [Get Users](crate::helix::users::get_users) endpoint
    pub channel_id: u32,
}

impl_de_ser!(
    Following,
    "following",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for Following {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];

    fn into_topic(self) -> pubsub::Topics { super::Topics::Following(self) }
}

/// Reply from [Following]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(tag = "type")]
#[non_exhaustive]
pub struct FollowingReply {
    /// Display name of user that followed
    pub display_name: types::DisplayName,
    /// ID of the user that followed
    pub user_id: types::UserId,
    /// Name of the user that followed
    pub username: types::UserName,
}

#[cfg(test)]
mod tests {
    use super::super::{Response, TopicData};
    use super::*;

    #[test]
    fn follow() {
        let message = r##"
{
    "display_name": "tmi",
    "username": "tmi",
    "user_id": "1234"
}
"##;

        let source = format!(
            r#"{{"type": "MESSAGE", "data": {{ "topic": "following.27620241", "message": {:?} }}}}"#,
            message
        );
        let actual = dbg!(Response::parse(&source).unwrap());
        assert!(matches!(
            actual,
            Response::Message {
                data: TopicData::Following { .. },
            }
        ));
    }

    #[test]
    fn check_deser() {
        use std::convert::TryInto as _;
        let s = "following.1234";
        assert_eq!(
            Following { channel_id: 1234 },
            s.to_string().try_into().unwrap()
        );
    }

    #[test]
    fn check_ser() {
        let s = "following.1234";
        let right: String = Following { channel_id: 1234 }.into();
        assert_eq!(s.to_string(), right);
    }
}

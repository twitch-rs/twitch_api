#![doc(alias = "channel.shared_chat.update")]
//! The active shared chat session the channel is in changed.

use super::*;

/// [`channel.shared_chat.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshared_chatupdate): the active shared chat session the channel is in changed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSharedChatUpdateV1 {
    /// The User ID of the channel to receive shared chat session update events for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelSharedChatUpdateV1 {
    /// The User ID of the channel to receive shared chat session update events for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelSharedChatUpdateV1 {
    type Payload = ChannelSharedChatUpdateV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelSharedChatUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
    const VERSION: &'static str = "1";
}

/// [`channel.shared_chat.update`](ChannelSharedChatUpdateV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSharedChatUpdateV1Payload {
    /// The unique identifier for the shared chat session.
    pub session_id: types::SharedChatSessionId,
    /// The User ID of the channel in the subscription condition.
    pub broadcaster_user_id: types::UserId,
    /// The display name of the channel in the subscription condition.
    pub broadcaster_user_name: types::DisplayName,
    /// The user login of the channel in the subscription condition.
    pub broadcaster_user_login: types::UserName,
    /// The User ID of the host channel.
    pub host_broadcaster_user_id: types::UserId,
    /// The display name of the host channel.
    pub host_broadcaster_user_name: types::DisplayName,
    /// The user login of the host channel.
    pub host_broadcaster_user_login: types::UserName,
    /// The list of participants in the session.
    pub participants: Vec<Participant>,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "5f65d0a7-0069-4f2b-944b-bc81b160ae49",
            "status": "enabled",
            "type": "channel.shared_chat.update",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "1971641"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQOtgGkFvXRlSkij343CndhIGY2VsbC1h"
            },
            "created_at": "2023-10-06T18:04:38.807682738Z",
            "cost": 0
        },
        "event": {
            "session_id": "2b64a92a-dbb8-424e-b1c3-304423ba1b6f",
            "broadcaster_user_id": "1971641",
            "broadcaster_user_login": "streamer",
            "broadcaster_user_name": "streamer",
            "host_broadcaster_user_id": "1971641",
            "host_broadcaster_user_login": "streamer",
            "host_broadcaster_user_name": "streamer",
            "participants": [
                {
                    "broadcaster_user_id": "1971641",
                    "broadcaster_user_name": "streamer",
                    "broadcaster_user_login": "streamer"
                },
                {
                    "broadcaster_user_id": "112233",
                    "broadcaster_user_name": "streamer33",
                    "broadcaster_user_login": "streamer33"
                },
                {
                    "broadcaster_user_id": "332211",
                    "broadcaster_user_name": "streamer11",
                    "broadcaster_user_login": "streamer11"
                }
            ]
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelSharedChatUpdateV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid message type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1971641");
    assert_eq!(notif.broadcaster_user_id, notif.host_broadcaster_user_id);
    assert_eq!(
        notif.broadcaster_user_login,
        notif.host_broadcaster_user_login
    );
    assert_eq!(
        notif.broadcaster_user_name,
        notif.host_broadcaster_user_name
    );
    assert_eq!(
        notif.session_id.as_str(),
        "2b64a92a-dbb8-424e-b1c3-304423ba1b6f"
    );
    assert_eq!(notif.participants.len(), 3);
    assert_eq!(
        notif.participants[0].broadcaster_user_id.as_str(),
        "1971641"
    );
    assert_eq!(notif.participants[1].broadcaster_user_id.as_str(), "112233");
    assert_eq!(notif.participants[2].broadcaster_user_id.as_str(), "332211");
}

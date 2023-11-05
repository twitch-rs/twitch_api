//! Types for Eventsub websocket
use std::borrow::Cow;

use crate::types;
use serde_derive::{Deserialize, Serialize};

use super::{Event, EventType};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
/// Defines a message that the EventSub WebSocket server sends your client when an event that you subscribe to occurs.
pub struct NotificationMetadata<'a> {
    /// An ID that uniquely identifies the message. Twitch sends messages at least once, but if Twitch is unsure of whether you received a notification, it’ll resend the message. This means you may receive a notification twice. If Twitch resends the message, the message ID will be the same.
    #[serde(borrow = "'a")]
    pub message_id: Cow<'a, str>,
    /// The UTC date and time that the message was sent.
    #[serde(borrow = "'a")]
    pub message_timestamp: Cow<'a, types::TimestampRef>,
    /// The type of event sent in the message.
    pub subscription_type: EventType,
    /// The version number of the subscription type’s definition. This is the same value specified in the subscription request.
    #[serde(borrow = "'a")]
    pub subscription_version: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
/// Defines the first message that the EventSub WebSocket server sends after your client connects to it.
pub struct WelcomeMetadata<'a> {
    /// An ID that uniquely identifies the message. Twitch sends messages at least once, but if Twitch is unsure of whether you received a notification, it’ll resend the message. This means you may receive a notification twice. If Twitch resends the message, the message ID will be the same.
    #[serde(borrow = "'a")]
    pub message_id: Cow<'a, str>,
    /// The UTC date and time that the message was sent.
    #[serde(borrow = "'a")]
    pub message_timestamp: Cow<'a, types::TimestampRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
/// Defines the first message that the EventSub WebSocket server sends after your client connects to it.
pub struct WelcomePayload<'a> {
    /// Session information
    #[serde(borrow = "'a")]
    pub session: SessionData<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
/// Session information sent with the welcome
pub struct SessionData<'a> {
    /// An ID that uniquely identifies this WebSocket connection. Use this ID to set the session_id field in all subscription requests.
    #[serde(borrow = "'a")]
    pub id: Cow<'a, str>,
    /// The connection’s status, which is set to connected.
    #[serde(borrow = "'a")]
    pub status: Cow<'a, str>,
    /// The maximum number of seconds that you should expect silence before receiving a keepalive message. For a welcome message, this is the number of seconds that you have to subscribe to an event after receiving the welcome message. If you don’t subscribe to an event within this window, the socket is disconnected.
    pub keepalive_timeout_seconds: Option<i64>,
    /// The URL to reconnect to if you get a Reconnect message. Is set to null.
    #[serde(borrow = "'a")]
    pub reconnect_url: Option<Cow<'a, str>>,
    /// The UTC date and time that the connection was created.
    #[serde(borrow = "'a")]
    pub connected_at: Cow<'a, types::TimestampRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
/// Defines the message that the EventSub WebSocket server sends your client to indicate that the WebSocket connection is healthy.
pub struct KeepaliveMetadata<'a> {
    /// An ID that uniquely identifies the message. Twitch sends messages at least once, but if Twitch is unsure of whether you received a notification, it’ll resend the message. This means you may receive a notification twice. If Twitch resends the message, the message ID is the same.
    #[serde(borrow = "'a")]
    pub message_id: Cow<'a, str>,
    /// The UTC date and time that the message was sent.
    #[serde(borrow = "'a")]
    pub message_timestamp: Cow<'a, types::TimestampRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
/// Defines the message that the EventSub WebSocket server sends if the server must drop the connection.
pub struct ReconnectMetadata<'a> {
    /// An ID that uniquely identifies the message. Twitch sends messages at least once, but if Twitch is unsure of whether you received a notification, it’ll resend the message. This means you may receive a notification twice. If Twitch resends the message, the message ID is the same.
    #[serde(borrow = "'a")]
    pub message_id: Cow<'a, str>,
    /// The UTC date and time that the message was sent.
    #[serde(borrow = "'a")]
    pub message_timestamp: Cow<'a, types::TimestampRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
/// Defines the message that the EventSub WebSocket server sends if the server must drop the connection.
pub struct ReconnectPayload<'a> {
    #[serde(borrow = "'a")]
    /// Session data
    pub session: SessionData<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
/// Defines the message that the EventSub WebSocket server sends if the user no longer exists or they revoked the authorization token that the subscription relied on.
pub struct RevocationMetadata<'a> {
    /// An ID that uniquely identifies the message. Twitch sends messages at least once, but if Twitch is unsure of whether you received a notification, it’ll resend the message. This means you may receive a notification twice. If Twitch resends the message, the message ID will be the same.
    #[serde(borrow = "'a")]
    pub message_id: Cow<'a, str>,
    /// The UTC date and time that the message was sent.
    #[serde(borrow = "'a")]
    pub message_timestamp: Cow<'a, types::TimestampRef>,
    /// The type of event sent in the message.
    pub subscription_type: EventType,
    /// The version number of the subscription type’s definition. This is the same value specified in the subscription request.
    #[serde(borrow = "'a")]
    pub subscription_version: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(tag = "message_type")]
#[non_exhaustive]
/// Metadata for a websocket event
pub enum EventsubWebsocketMetadata<'a> {
    /// Defines a message that the EventSub WebSocket server sends your client when an event that you subscribe to occurs.
    #[serde(rename = "notification")]
    Notification(#[serde(borrow = "'a")] NotificationMetadata<'a>),
    /// Defines the first message that the EventSub WebSocket server sends after your client connects to it.
    #[serde(rename = "session_welcome")]
    Welcome(#[serde(borrow = "'a")] WelcomeMetadata<'a>),
    /// Defines the message that the EventSub WebSocket server sends your client to indicate that the WebSocket connection is healthy.
    #[serde(rename = "session_keepalive")]
    Keepalive(#[serde(borrow = "'a")] KeepaliveMetadata<'a>),
    /// Defines the message that the EventSub WebSocket server sends if the server must drop the connection.s
    #[serde(rename = "session_reconnect")]
    Reconnect(#[serde(borrow = "'a")] ReconnectMetadata<'a>),
    /// Defines the message that the EventSub WebSocket server sends if the user no longer exists or they revoked the authorization token that the subscription relied on.
    #[serde(rename = "revocation")]
    Revocation(#[serde(borrow = "'a")] RevocationMetadata<'a>),
}

impl<'d> EventsubWebsocketMetadata<'d> {
    /// Get message id
    pub fn message_id(&self) -> Cow<'d, str> {
        match self {
            EventsubWebsocketMetadata::Welcome(WelcomeMetadata { message_id, .. })
            | EventsubWebsocketMetadata::Keepalive(KeepaliveMetadata { message_id, .. })
            | EventsubWebsocketMetadata::Notification(NotificationMetadata {
                message_id, ..
            })
            | EventsubWebsocketMetadata::Revocation(RevocationMetadata { message_id, .. })
            | EventsubWebsocketMetadata::Reconnect(ReconnectMetadata { message_id, .. }) => {
                message_id.clone()
            }
        }
    }

    /// Get message timestamp
    pub fn message_timestamp(&self) -> Cow<'d, types::TimestampRef> {
        match self {
            EventsubWebsocketMetadata::Welcome(WelcomeMetadata {
                message_timestamp, ..
            })
            | EventsubWebsocketMetadata::Keepalive(KeepaliveMetadata {
                message_timestamp, ..
            })
            | EventsubWebsocketMetadata::Notification(NotificationMetadata {
                message_timestamp,
                ..
            })
            | EventsubWebsocketMetadata::Revocation(RevocationMetadata {
                message_timestamp, ..
            })
            | EventsubWebsocketMetadata::Reconnect(ReconnectMetadata {
                message_timestamp, ..
            }) => message_timestamp.clone(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum EventsubWebsocketData<'a> {
    Welcome {
        #[serde(borrow = "'a")]
        metadata: WelcomeMetadata<'a>,
        #[serde(borrow = "'a")]
        payload: WelcomePayload<'a>,
    },
    Keepalive {
        #[serde(borrow = "'a")]
        metadata: KeepaliveMetadata<'a>,
        payload: (),
    },
    Notification {
        #[serde(borrow = "'a")]
        metadata: NotificationMetadata<'a>,
        payload: Event,
    },
    Revocation {
        #[serde(borrow = "'a")]
        metadata: RevocationMetadata<'a>,
        payload: Event,
    },
    Reconnect {
        #[serde(borrow = "'a")]
        metadata: ReconnectMetadata<'a>,
        #[serde(borrow = "'a")]
        payload: ReconnectPayload<'a>,
    },
}

impl<'d> EventsubWebsocketData<'d> {
    /// Get message id
    pub fn message_id(&self) -> Cow<'d, str> { self.metadata().message_id() }

    /// Get message timestamp
    pub fn message_timestamp(&self) -> Cow<'d, types::TimestampRef> {
        self.metadata().message_timestamp()
    }

    /// Get metadata for the event
    pub fn metadata(&self) -> EventsubWebsocketMetadata<'d> {
        match self {
            EventsubWebsocketData::Welcome { metadata, .. } => {
                EventsubWebsocketMetadata::Welcome(metadata.clone())
            }
            EventsubWebsocketData::Keepalive { metadata, .. } => {
                EventsubWebsocketMetadata::Keepalive(metadata.clone())
            }
            EventsubWebsocketData::Notification { metadata, .. } => {
                EventsubWebsocketMetadata::Notification(metadata.clone())
            }
            EventsubWebsocketData::Revocation { metadata, .. } => {
                EventsubWebsocketMetadata::Revocation(metadata.clone())
            }
            EventsubWebsocketData::Reconnect { metadata, .. } => {
                EventsubWebsocketMetadata::Reconnect(metadata.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome() {
        let msg = r#"
        {
            "metadata": {
              "message_id": "96a3f3b5-5dec-4eed-908e-e11ee657416c",
              "message_type": "session_welcome",
              "message_timestamp": "2022-10-19T14:56:51.634234626Z"
            },
            "payload": {
              "session": {
                "id": "AQoQILE98gtqShGmLD7AM6yJThAB",
                "status": "connected",
                "connected_at": "2022-10-19T14:56:51.616329898Z",
                "keepalive_timeout_seconds": 10,
                "reconnect_url": null
              }
            }
          }
        "#;
        let event = Event::parse_websocket(msg).unwrap();
        assert_eq!(event.message_id(), "96a3f3b5-5dec-4eed-908e-e11ee657416c");
        assert!(matches!(
            event,
            EventsubWebsocketData::Welcome {
                metadata: _,
                payload: _,
            }
        ));
    }

    #[test]
    fn keepalive() {
        let msg = r#"
        {
            "metadata": {
              "message_id": "8d8e0935-0e0c-479a-8fa2-ad4c02cef742",
              "message_type": "session_keepalive",
              "message_timestamp": "2022-11-04T13:11:49.770459792Z"
            },
            "payload": {}
        }
        "#;
        let event = Event::parse_websocket(msg).unwrap();
        assert_eq!(event.message_id(), "8d8e0935-0e0c-479a-8fa2-ad4c02cef742");
        assert!(matches!(
            event,
            EventsubWebsocketData::Keepalive {
                metadata: _,
                payload: ()
            }
        ))
    }

    #[test]
    fn notification() {
        let msg = r#"
        {
            "metadata": {
              "message_id": "befa7b53-d79d-478f-86b9-120f112b044e",
              "message_type": "notification",
              "message_timestamp": "2019-11-16T10:11:12.123Z",
              "subscription_type": "channel.follow",
              "subscription_version": "1"
            },
            "payload": {
              "subscription": {
                "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
                "status": "enabled",
                "type": "channel.follow",
                "version": "1",
                "cost": 1,
                "condition": {
                  "broadcaster_user_id": "12826"
                },
                "transport": {
                  "method": "websocket",
                  "session_id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB"
                },
                "created_at": "2019-11-16T10:11:12.123Z"
              },
              "event": {
                "user_id": "1337",
                "user_login": "awesome_user",
                "user_name": "Awesome_User",
                "broadcaster_user_id": "12826",
                "broadcaster_user_login": "twitch",
                "broadcaster_user_name": "Twitch",
                "followed_at": "2020-07-15T18:16:11.17106713Z"
              }
            }
          }
        "#;
        let event = Event::parse_websocket(msg).unwrap();
        assert_eq!(event.message_id(), "befa7b53-d79d-478f-86b9-120f112b044e");
        assert!(matches!(
            event,
            EventsubWebsocketData::Notification {
                metadata: _,
                payload: Event::ChannelFollowV1(_),
            }
        ))
    }

    #[test]
    fn reconnect() {
        let msg = r#"
        {
            "metadata": {
              "message_id": "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
              "message_type": "session_reconnect",
              "message_timestamp": "2019-11-18T09:10:11.234Z"
            },
            "payload": {
              "session": {
                "id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB",
                "status": "reconnecting",
                "keepalive_timeout_seconds": null,
                "reconnect_url": "wss://eventsub.wss.twitch.tv?...",
                "connected_at": "2019-11-16T10:11:12.123Z"
              }
            }
          }
        "#;
        let event = Event::parse_websocket(msg).unwrap();
        assert_eq!(event.message_id(), "84c1e79a-2a4b-4c13-ba0b-4312293e9308");
        assert!(matches!(
            event,
            EventsubWebsocketData::Reconnect {
                metadata: _,
                payload: _,
            }
        ))
    }

    #[test]
    fn revocation() {
        let msg = r#"
        {
            "metadata": {
              "message_id": "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
              "message_type": "revocation",
              "message_timestamp": "2019-11-16T10:11:12.123Z",
              "subscription_type": "channel.follow",
              "subscription_version": "1"
            },
            "payload": {
              "subscription": {
                "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
                "status": "authorization_revoked",
                "type": "channel.follow",
                "version": "1",
                "cost": 1,
                "condition": {
                  "broadcaster_user_id": "12826"
                },
                "transport": {
                  "method": "websocket",
                  "session_id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB"
                },
                "created_at": "2019-11-16T10:11:12.123Z"
              }
            }
          }
        "#;
        let event = Event::parse_websocket(msg).unwrap();
        assert_eq!(event.message_id(), "84c1e79a-2a4b-4c13-ba0b-4312293e9308");
        assert!(matches!(
            event,
            EventsubWebsocketData::Revocation {
                metadata: _,
                payload: _,
            }
        ))
    }
}

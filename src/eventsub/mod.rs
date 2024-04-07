//! Holds serializable EventSub stuff
//!
//! Use [`CreateEventSubSubscription`](crate::helix::eventsub::CreateEventSubSubscriptionRequest) to subscribe to an event according to the [EventSub guide](https://dev.twitch.tv/docs/eventsub).
//! Parse the response payload text with [`Event::parse_http`] or [`Event::parse_websocket`]
//!
//! # Examples
//!
//! See [`examples/`](https://github.com/twitch-rs/twitch_api/tree/main/examples/eventsub) for a more complete example of using eventsub.
// FIXME: Use the actual link to the source files, currently can't do that on docs.rs since the workspace member is removed in the tarball
//!
//! Subscribe to a channel's follow events:
//!
//! ```rust, no_run
//! use twitch_api::{
//!     eventsub::{channel::ChannelFollowV2, Transport, TransportMethod},
//!     helix,
//! };
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//!
//! let event = ChannelFollowV2::new("1234", "5678");
//! let transport = Transport::webhook(
//!     "https://example.org/eventsub/channelfollow",
//!     String::from("secretabcd"),
//! );
//!
//! let event_information = client
//!     .create_eventsub_subscription(event, transport, &token)
//!     .await?;
//!
//! println!("event id: {:?}", event_information.id);
//! # Ok(())
//! # }
//! ```
//!
//! You'll now get a http POST request to the url you specified as the `callback`.
//! You need to respond to this request from your webserver with a 200 OK response with the [`challenge`](VerificationRequest::challenge) as the body.
//! After this, you'll get notifications
//! ```rust
//! use twitch_api::eventsub::{Event, Payload, Message};
//! pub fn parse_request(
//!     request: &http::Request<Vec<u8>>,
//! ) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error + Send + Sync + 'static>> {
//!     // First, we verify the response, assuring it's legit.
//!     if !Event::verify_payload(request, b"secretabcd") {
//!         return Err(todo!());
//!     }
//!     match Event::parse_http(request)? {
//!         Event::ChannelFollowV2(Payload {
//!             message: Message::VerificationRequest(ver),
//!             ..
//!         }) => {
//!             // We've verified the request, so we can respond to it with the challenge
//!             Ok(http::Response::builder()
//!                 .status(200)
//!                 .body(ver.challenge.into_bytes())?)
//!         },
//!         Event::ChannelFollowV2(Payload {
//!             message: Message::Notification(notif),
//!             ..
//!         }) => {
//!             // make sure you save the `Twitch-Eventsub-Message-Id` headers value,
//!             // twitch may resend notifications, and in those cases you should just return 200 OK.
//!
//!             // Do whatever you need to do with the event. Preferably send the event to a channel.
//!             println!("user {:?} followed {:?}", notif.user_name, notif.broadcaster_user_name);
//!             Ok(http::Response::builder().status(200).body(vec![])?)
//!         }
//!         _ => Ok(http::Response::builder().status(200).body(vec![])?),
//!     }
//! }
//! ```

use std::borrow::Cow;

use crate::types;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

use crate::parse_json;

pub mod channel;
pub mod event;
pub mod stream;
pub mod user;

#[doc(inline)]
pub use event::{Event, EventType};

pub use event::websocket::*;

/// An EventSub subscription.
pub trait EventSubscription: DeserializeOwned + serde::Serialize + PartialEq + Clone {
    /// Payload for given subscription
    type Payload: PartialEq + std::fmt::Debug + DeserializeOwned + serde::Serialize + Clone;

    /// Scopes needed by this subscription
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator;
    /// Optional scopes needed by this subscription
    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    /// Subscription type version
    const VERSION: &'static str;
    /// Subscription type name.
    const EVENT_TYPE: EventType;

    /// Creates the [`condition`](https://dev.twitch.tv/docs/eventsub/eventsub-reference#conditions) for this EventSub subscription
    fn condition(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

/// Verification Request
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct VerificationRequest {
    /// Challenge string.
    ///
    /// After verifying that the response is legit, send back this challenge.
    /// You can do so with [`Event::verify_payload`]
    pub challenge: String,
}

/// Subscription message/payload. Received on events and other messages.
///
/// Use [`Event::parse_http`] to construct
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum Message<E: EventSubscription + Clone> {
    /// Webhook Callback Verification
    VerificationRequest(VerificationRequest),
    /// A [subscription revocation](https://dev.twitch.tv/docs/eventsub#subscription-revocation)
    Revocation(),
    /// A notification holding some event data.
    #[serde(bound = "E: EventSubscription")]
    Notification(<E as EventSubscription>::Payload),
}

impl<E: EventSubscription + Clone> Message<E> {
    /// Returns `true` if the message is [`VerificationRequest`].
    ///
    /// [`VerificationRequest`]: Message::VerificationRequest
    pub fn is_verification_request(&self) -> bool { matches!(self, Self::VerificationRequest(..)) }

    /// Returns `true` if the message is [`Revocation`].
    ///
    /// [`Revocation`]: Message::Revocation
    pub fn is_revocation(&self) -> bool { matches!(self, Self::Revocation(..)) }

    /// Returns `true` if the message is [`Notification`].
    ///
    /// [`Notification`]: Message::Notification
    pub fn is_notification(&self) -> bool { matches!(self, Self::Notification(..)) }
}

impl<E: EventSubscription> Payload<E> {
    /// Parse string slice as a [`Payload`], this will assume your string is from an eventsub message with type `notification`
    ///
    /// The string should be a
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::eventsub::{channel::ChannelFollowV2, Payload};
    /// let notification = r#"
    /// {
    ///     "subscription": {
    ///         "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
    ///         "type": "channel.follow",
    ///         "version": "2",
    ///         "status": "enabled",
    ///         "cost": 0,
    ///         "condition": {
    ///            "broadcaster_user_id": "1337",
    ///            "moderator_user_id": "1337"
    ///         },
    ///          "transport": {
    ///             "method": "webhook",
    ///             "callback": "https://example.com/webhooks/callback"
    ///         },
    ///         "created_at": "2019-11-16T10:11:12.634234626Z"
    ///     },
    ///     "event": {
    ///         "user_id": "1234",
    ///         "user_login": "cool_user",
    ///         "user_name": "Cool_User",
    ///         "broadcaster_user_id": "1337",
    ///         "broadcaster_user_login": "cooler_user",
    ///         "broadcaster_user_name": "Cooler_User",
    ///         "followed_at": "2020-07-15T18:16:11.17106713Z"
    ///     }
    /// }
    /// "#;
    /// let payload: Payload<ChannelFollowV2> =
    ///     Payload::parse_notification(notification)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse_notification(source: &str) -> Result<Payload<E>, PayloadParseError> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct Notification<E: EventSubscription> {
            #[serde(bound = "E: EventSubscription")]
            pub subscription: EventSubscriptionInformation<E>,
            #[serde(bound = "E: EventSubscription")]
            pub event: <E as EventSubscription>::Payload,
        }

        let Notification {
            subscription,
            event,
        } = parse_json::<Notification<E>>(source, true)?;

        Ok(Payload {
            subscription,
            message: Message::Notification(event),
        })
    }

    /// Parse string slice as a [`Payload`] with a message of [`Message::Revocation`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::eventsub::{channel::ChannelFollowV2, Payload};
    /// let notification = r#"
    /// {
    ///     "subscription": {
    ///         "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
    ///         "status": "authorization_revoked",
    ///         "type": "channel.follow",
    ///         "cost": 0,
    ///         "version": "2",
    ///         "condition": {
    ///             "broadcaster_user_id": "1337",
    ///             "moderator_user_id": "1337"
    ///         },
    ///         "transport": {
    ///             "method": "webhook",
    ///             "callback": "https://example.com/webhooks/callback"
    ///         },
    ///         "created_at": "2019-11-16T10:11:12.123Z"
    ///     }
    /// }
    /// "#;
    /// let payload: Payload<ChannelFollowV2> =
    ///     Payload::parse_revocation(notification)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse_revocation(source: &str) -> Result<Payload<E>, PayloadParseError> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct Notification<E: EventSubscription> {
            #[serde(bound = "E: EventSubscription")]
            pub subscription: EventSubscriptionInformation<E>,
        }

        let Notification { subscription } = parse_json::<Notification<E>>(source, true)?;

        Ok(Payload {
            subscription,
            message: Message::Revocation(),
        })
    }

    /// Parse string slice as a [`Payload`] with a message of [`Message::VerificationRequest`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::eventsub::{channel::ChannelFollowV2, Payload};
    /// let notification = r#"
    /// {
    ///     "challenge": "pogchamp-kappa-360noscope-vohiyo",
    ///     "subscription": {
    ///         "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
    ///         "status": "webhook_callback_verification_pending",
    ///         "type": "channel.follow",
    ///         "version": "2",
    ///         "cost": 1,
    ///         "condition": {
    ///             "broadcaster_user_id": "12826",
    ///             "moderator_user_id": "12826"
    ///         },
    ///         "transport": {
    ///             "method": "webhook",
    ///             "callback": "https://example.com/webhooks/callback"
    ///         },
    ///         "created_at": "2019-11-16T10:11:12.123Z"
    ///     }
    /// }
    /// "#;
    /// let payload: Payload<ChannelFollowV2> =
    ///     Payload::parse_verification_request(notification)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse_verification_request(source: &str) -> Result<Payload<E>, PayloadParseError> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct Notification<E: EventSubscription> {
            #[serde(bound = "E: EventSubscription")]
            pub subscription: EventSubscriptionInformation<E>,
            #[serde(bound = "E: EventSubscription")]
            pub challenge: String,
        }

        let Notification {
            subscription,
            challenge,
        } = parse_json::<Notification<E>>(source, true)?;

        Ok(Payload {
            subscription,
            message: Message::VerificationRequest(VerificationRequest { challenge }),
        })
    }

    /// Parse http post request as a [Payload] with a specific [event](EventSubscription).
    ///
    /// If you don't know what event this payload is, use [`Event::parse_http`] instead.
    ///
    /// If your [`Request<B>`](http::Request) is of another type that doesn't implement `AsRef<[u8]>`, try converting it with [`Request::map`](http::Request::map)
    ///
    /// ```rust
    /// use http::Request;
    /// use twitch_api::eventsub::{Payload, channel::ChannelFollowV2};
    /// # struct Body {} impl Body { fn new() -> Self {Body {}} fn to_bytes(&self) -> &[u8] { &[] } }
    /// # fn a() -> Result<(), twitch_api::eventsub::PayloadParseError> {
    /// // Example of a request with a body that doesn't implement `AsRef<[u8]>`
    /// let original_request: Request<Body> = http::Request::new(Body::new());
    /// // Convert to a request with a body of `Vec<u8>`, which does implement `AsRef<[u8]>`
    /// let converted_request: Request<Vec<u8>> = original_request.map(|r| r.to_bytes().to_owned());
    /// Payload::<ChannelFollowV2>::parse_http(&converted_request)?
    /// # ; Ok(())}
    /// ```
    pub fn parse_http<B>(request: &http::Request<B>) -> Result<Payload<E>, PayloadParseError>
    where B: AsRef<[u8]> {
        // FIXME: Add some debug assertions for version and type

        let source = request.body().as_ref().into();
        let ty = request
            .headers()
            .get("Twitch-Eventsub-Message-Type")
            .map(|v| v.as_bytes())
            .unwrap_or_else(|| b"notification")
            .into();
        Self::parse_request(ty, source)
    }

    /// Parse a slice as a [`Payload`] with a specific message type. You should not use this, instead, use [`Payload::parse_http`] or the specific `parse_*` functions
    #[doc(hidden)]
    pub fn parse_request<'a>(
        ty: Cow<'a, [u8]>,
        source: Cow<'a, [u8]>,
    ) -> Result<Payload<E>, PayloadParseError> {
        let source = std::str::from_utf8(&source)?;
        Self::parse_request_str(ty.as_ref(), source)
    }

    /// Parse a string slice as a [`Payload`] with a specific message type. You should not use this, instead, use [`Payload::parse_http`] or the specific `parse_*` functions
    #[doc(hidden)]
    pub fn parse_request_str<'a>(
        ty: &'a [u8],
        source: &'a str,
    ) -> Result<Payload<E>, PayloadParseError> {
        match ty {
            b"notification" => Payload::parse_notification(source),
            b"webhook_callback_verification" => Payload::parse_verification_request(source),
            b"revocation" => Payload::parse_revocation(source),
            typ => Err(PayloadParseError::UnknownMessageType(
                String::from_utf8_lossy(typ).into_owned(),
            )),
        }
    }
}

/// Errors that can happen when parsing payload
#[derive(thiserror::Error, displaydoc::Display, Debug)]
#[non_exhaustive]
pub enum PayloadParseError {
    /// could not parse [`http::Request::body()`] as UTF8
    Utf8Error(#[from] std::str::Utf8Error),
    /// could not parse [`http::Request::body()`] as a [`Payload`]
    DeserializeError(#[from] crate::DeserError),
    /// unknown message type encountered: {0}
    UnknownMessageType(String),
    /// unknown event type encountered: {0}
    UnknownEventType(String),
    /// event could not be parsed, some context missing
    MalformedEvent,
    /// could not find an implementation for version `{version}` on event type `{event_type}` in this library
    UnimplementedEvent {
        /// Version
        version: String,
        /// Event type
        event_type: EventType,
    },
}

/// Notification received
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Payload<E: EventSubscription + Clone> {
    /// Subscription information.
    #[serde(bound = "E: EventSubscription")]
    pub subscription: EventSubscriptionInformation<E>,
    /// Event information.
    #[serde(bound = "E: EventSubscription")]
    pub message: Message<E>,
}

impl<E: EventSubscription + Clone> Payload<E> {
    /// Convenience method for getting the event type from the payload.
    pub fn get_event_type(&self) -> EventType { E::EVENT_TYPE }

    /// Convenience method for getting the event version from the payload.
    pub fn get_event_version(&self) -> &'static str { E::VERSION }
}

/// Metadata about the subscription.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct EventSubscriptionInformation<E: EventSubscription> {
    /// ID of the subscription.
    pub id: types::EventSubId,
    /// Status of EventSub subscription
    pub status: Status,
    /// How much the subscription counts against your limit.
    pub cost: usize,
    /// Subscription-specific parameters.
    #[serde(bound = "E: EventSubscription")]
    pub condition: E,
    /// The time the notification was created.
    pub created_at: types::Timestamp,
    /// Transport method
    pub transport: TransportResponse,
    /// Event type. Consider using [`E::EVENT_TYPE`](EventSubscription::EVENT_TYPE) instead.
    #[serde(rename = "type")]
    pub type_: EventType,
    /// Event version. Consider using [`E::VERSION`](EventSubscription::VERSION) instead.
    pub version: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
/// Webhook transport
pub struct WebhookTransport {
    /// Callback
    pub callback: String,
    /// Secret attached to the subscription.
    ///
    /// # Notes
    ///
    /// Secret must be between 10 and 100 characters
    pub secret: String,
}

impl std::fmt::Debug for WebhookTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookTransport")
            .field("callback", &self.callback)
            .field("secret", &"[redacted]")
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
/// Websocket transport
pub struct WebsocketTransport {
    /// An ID that identifies the WebSocket to send notifications to.
    ///
    /// When you connect to EventSub using WebSockets, the server returns the ID in the Welcome message.
    pub session_id: String,
}

/// Transport setting for event notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "lowercase")]
#[non_exhaustive]
pub enum Transport {
    /// Webhook transport
    Webhook(WebhookTransport),
    /// Websocket transport
    Websocket(WebsocketTransport),
}

impl Transport {
    /// Convenience method for making a webhook transport
    pub fn webhook(callback: impl std::string::ToString, secret: String) -> Transport {
        Transport::Webhook(WebhookTransport {
            callback: callback.to_string(),
            secret,
        })
    }

    /// Convenience method for making a websocket transport
    pub fn websocket(session_id: impl std::string::ToString) -> Transport {
        Transport::Websocket(WebsocketTransport {
            session_id: session_id.to_string(),
        })
    }

    /// Returns `true` if the transport is [`Webhook`].
    ///
    /// [`Webhook`]: Transport::Webhook
    #[must_use]
    pub fn is_webhook(&self) -> bool { matches!(self, Self::Webhook(..)) }

    /// Returns `true` if the transport is [`Websocket`].
    ///
    /// [`Websocket`]: Transport::Websocket
    #[must_use]
    pub fn is_websocket(&self) -> bool { matches!(self, Self::Websocket(..)) }

    /// Returns `Some(&WebhookTransport)` if this transport is a [webhook](WebhookTransport)
    pub fn as_webhook(&self) -> Option<&WebhookTransport> {
        if let Self::Webhook(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Some(&WebsocketTransport)` if this transport is a [websocket](WebsocketTransport)
    pub fn as_websocket(&self) -> Option<&WebsocketTransport> {
        if let Self::Websocket(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Some(WebhookTransport)` if this transport is a [webhook](WebhookTransport), `None` if not
    pub fn try_into_webhook(self) -> Option<WebhookTransport> {
        if let Self::Webhook(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Some(WebsocketTransport)` if this transport is a [websocket](WebsocketTransport), `Err(())` if not
    pub fn try_into_websocket(self) -> Option<WebsocketTransport> {
        if let Self::Websocket(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
/// Websocket transport
pub struct WebsocketTransportResponse {
    /// An ID that identifies the WebSocket that notifications are sent to.
    pub session_id: String,
    /// The UTC date and time that the WebSocket connection was established.
    ///
    /// # Notes
    ///
    /// Only returned on helix response
    pub connected_at: Option<types::Timestamp>,
    /// The UTC date and time that the WebSocket connection was lost.
    ///
    /// # Notes
    ///
    /// Only returned on helix response
    pub disconnected_at: Option<types::Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
/// Webhook transport
pub struct WebhookTransportResponse {
    /// Callback
    pub callback: String,
}

/// Transport response on event notification
///
/// Does not include secret.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "lowercase")]
#[non_exhaustive]
pub enum TransportResponse {
    /// Webhook transport response
    Webhook(WebhookTransportResponse),
    /// Websocket transport response
    Websocket(WebsocketTransportResponse),
}

impl TransportResponse {
    /// Returns `true` if the transport response is [`Webhook`].
    ///
    /// [`Webhook`]: TransportResponse::Webhook
    #[must_use]
    pub fn is_webhook(&self) -> bool { matches!(self, Self::Webhook(..)) }

    /// Returns `true` if the transport response is [`Websocket`].
    ///
    /// [`Websocket`]: TransportResponse::Websocket
    #[must_use]
    pub fn is_websocket(&self) -> bool { matches!(self, Self::Websocket(..)) }

    /// Returns `Some(&WebhookTransport)` if this transport response is a [webhook](WebhookTransportResponse)
    pub fn as_webhook(&self) -> Option<&WebhookTransportResponse> {
        if let Self::Webhook(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Some(&WebsocketTransport)` if this transport response is a [websocket](WebsocketTransportResponse)
    pub fn as_websocket(&self) -> Option<&WebsocketTransportResponse> {
        if let Self::Websocket(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Ok(WebhookTransport)` if this transport response is a [webhook](WebhookTransportResponse)
    pub fn try_into_webhook(self) -> Result<WebhookTransportResponse, Self> {
        if let Self::Webhook(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `Ok(WebsocketTransport)` if this transport response is a [websocket](WebsocketTransportResponse)
    pub fn try_into_websocket(self) -> Result<WebsocketTransportResponse, Self> {
        if let Self::Websocket(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

/// Transport method
///
/// Currently, only webhooks are supported
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    /// Webhook
    Webhook,
    /// Eventsub
    Websocket,
}

/// Subscription request status
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Status {
    /// Twitch has verified your callback and is able to send you notifications.
    #[serde(rename = "enabled")]
    Enabled,
    /// Twitch is verifying that you own the callback specified in the create subscription request. For information about how it does this, see [Verifying your callback](https://dev.twitch.tv/docs/eventsub/handling-webhook-events/#responding-to-a-challenge-request). Used only for webhook subscriptions.
    #[serde(rename = "webhook_callback_verification_pending")]
    WebhookCallbackVerificationPending,
    /// Twitch failed to verify that you own the callback specified in the create subscription request. Fix your event handler to correctly respond to the challenge, and then try subscribing again. Used only for webhook subscriptions.
    #[serde(rename = "webhook_callback_verification_failed")]
    WebhookCallbackVerificationFailed,
    /// Twitch revoked your subscription because the notification delivery failure rate was too high. Used only for webhook subscriptions.
    #[serde(rename = "notification_failures_exceeded")]
    NotificationFailuresExceeded,
    /// Twitch revoked your subscription because the users in the [`condition`](EventSubSubscription::condition) object revoked their authorization letting you get events on their behalf, or changed their password.
    #[serde(rename = "authorization_revoked")]
    AuthorizationRevoked,
    /// The moderator that authorized the subscription is no longer one of the broadcaster's moderators.
    #[serde(rename = "moderator_removed")]
    ModeratorRemoved,
    /// Twitch revoked your subscription because the users in the [`condition`](EventSubSubscription::condition) object are no longer Twitch users.
    #[serde(rename = "user_removed")]
    UserRemoved,
    /// Twitch revoked your subscription because the subscribed to subscription type and version is no longer supported.
    #[serde(rename = "version_removed")]
    VersionRemoved,
    /// The subscription to the beta subscription type was removed due to maintenance.
    #[serde(rename = "beta_maintenance")]
    BetaMaintenance,
    /// The client closed the connection.
    #[serde(rename = "websocket_disconnected")]
    WebsocketDisconnected,
    /// The client failed to respond to a ping message.
    #[serde(rename = "websocket_failed_ping_pong")]
    WebsocketFailedPingPong,
    /// The client sent a non-pong message. Clients may only send pong messages (and only in response to a ping message).
    #[serde(rename = "websocket_received_inbound_traffic")]
    WebsocketReceivedInboundTraffic,
    /// The client failed to subscribe to events within the required time.
    #[serde(rename = "websocket_connection_unused")]
    WebsocketConnectionUnused,
    /// The Twitch WebSocket server experienced an unexpected error.
    #[serde(rename = "websocket_internal_error")]
    WebsocketInternalError,
    /// The Twitch WebSocket server timed out writing the message to the client.
    #[serde(rename = "websocket_network_timeout")]
    WebsocketNetworkTimeout,
    /// The Twitch WebSocket server experienced a network error writing the message to the client.
    #[serde(rename = "websocket_network_error")]
    WebsocketNetworkError,
    /// The client failed to reconnect to the Twitch WebSocket server within the required time after a Reconnect Message.
    #[serde(rename = "websocket_failed_to_reconnect")]
    WebsocketFailedToReconnect,
}

/// General information about an EventSub subscription.
///
/// Returned by [`Event::subscription`]
///
/// See also [`EventSubscriptionInformation`]
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
#[cfg(feature = "eventsub")]
#[cfg_attr(nightly, doc(cfg(feature = "eventsub")))]
pub struct EventSubSubscription {
    /// How much the subscription counts against your limit.
    pub cost: usize,
    /// JSON object specifying custom parameters for the subscription.
    // FIXME: Should be [eventsub::Condition]
    pub condition: serde_json::Value,
    /// RFC3339 timestamp indicating when the subscription was created.
    pub created_at: types::Timestamp,
    /// ID of the subscription.
    pub id: types::EventSubId,
    /// Status of the subscription.
    pub status: Status,
    /// Notification delivery specific information. Includes the transport method and callback URL.
    pub transport: TransportResponse,
    /// The category of the subscription.
    #[serde(rename = "type")]
    pub type_: EventType,
    /// The version of the subscription.
    pub version: String,
}

/// General information about a [Conduit](https://dev.twitch.tv/docs/eventsub/handling-conduit-events/)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
#[cfg(feature = "eventsub")]
#[cfg_attr(nightly, doc(cfg(feature = "eventsub")))]
pub struct Conduit {
    /// Conduit ID
    pub id: String,
    /// Number of shards associated with this conduit
    pub shard_count: usize,
}

pub(crate) trait NamedField {
    const NAME: &'static str;
}

/// Deserialize {"field": field} as { field ...} and serialize in reverse
mod enum_field_as_inner {
    use serde::ser::SerializeMap;

    use super::*;
    pub(crate) fn deserialize<'de, D, S>(deserializer: D) -> Result<S, D::Error>
    where
        D: serde::Deserializer<'de>,
        S: serde::Deserialize<'de> + NamedField, {
        struct Inner<S>(std::marker::PhantomData<S>);
        impl<'de, S> serde::de::Visitor<'de> for Inner<S>
        where S: serde::Deserialize<'de> + NamedField
        {
            type Value = S;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("any object")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where A: serde::de::MapAccess<'de> {
                let mut map = map;
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == S::NAME {
                        value = Some(map.next_value()?);
                    } else {
                        map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| serde::de::Error::missing_field(S::NAME))
            }
        }

        deserializer.deserialize_any(Inner(std::marker::PhantomData))
    }

    pub(crate) fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: serde::Serialize + NamedField,
        S: serde::Serializer, {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(T::NAME, value)?;
        map.end()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_websocket_notification() {
        let frame = r#"
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
        }"#;

        crate::eventsub::Event::parse_websocket(frame).unwrap();
    }

    #[test]
    fn test_verification_response() {
        use http::header::{HeaderMap, HeaderName, HeaderValue};

        #[rustfmt::skip]
        let headers: HeaderMap = vec![
            ("Twitch-Eventsub-Message-Id", "e76c6bd4-55c9-4987-8304-da1588d8988b"),
            ("Twitch-Eventsub-Message-Retry", "0"),
            ("Twitch-Eventsub-Message-Type", "webhook_callback_verification"),
            ("Twitch-Eventsub-Message-Signature", "sha256=f56bf6ce06a1adf46fa27831d7d15d"),
            ("Twitch-Eventsub-Message-Timestamp", "2019-11-16T10:11:12.123Z"),
            ("Twitch-Eventsub-Subscription-Type", "channel.follow"),
            ("Twitch-Eventsub-Subscription-Version", "1"),
            ].into_iter()
        .map(|(h, v)| {
            (
                h.parse::<HeaderName>().unwrap(),
                v.parse::<HeaderValue>().unwrap(),
            )
        })
        .collect();

        let body = r#"{
            "challenge": "pogchamp-kappa-360noscope-vohiyo",
            "subscription": {
                "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
                "status": "webhook_callback_verification_pending",
                "type": "channel.follow",
                "version": "1",
                "cost": 1,
                "condition": {
                        "broadcaster_user_id": "12826"
                },
                "transport": {
                    "method": "webhook",
                    "callback": "https://example.com/webhooks/callback"
                },
                "created_at": "2019-11-16T10:11:12.123Z"
            }
        }"#;

        let mut request = http::Request::builder();
        let _ = std::mem::replace(request.headers_mut().unwrap(), headers);
        let request = request.body(body.as_bytes().to_vec()).unwrap();
        let _payload = dbg!(crate::eventsub::Event::parse_http(&request).unwrap());
        let payload = dbg!(crate::eventsub::Event::parse(
            std::str::from_utf8(request.body()).unwrap()
        )
        .unwrap());
        crate::tests::roundtrip(&payload)
    }

    #[test]
    fn test_revoke() {
        use http::header::{HeaderMap, HeaderName, HeaderValue};

        #[rustfmt::skip]
        let headers: HeaderMap = vec![
            ("Content-Length", "458"),
            ("Twitch-Eventsub-Message-Id", "84c1e79a-2a4b-4c13-ba0b-4312293e9308"),
            ("Twitch-Eventsub-Message-Retry", "0"),
            ("Twitch-Eventsub-Message-Type", "revocation"),
            ("Twitch-Eventsub-Message-Signature", "sha256=c1f92c51dab9888b0d6fb5f7e8e758"),
            ("Twitch-Eventsub-Message-Timestamp", "2019-11-16T10:11:12.123Z"),
            ("Twitch-Eventsub-Subscription-Type", "channel.follow"),
            ("Twitch-Eventsub-Subscription-Version", "1"),
            ].into_iter()
        .map(|(h, v)| {
            (
                h.parse::<HeaderName>().unwrap(),
                v.parse::<HeaderValue>().unwrap(),
            )
        })
        .collect();

        let body = r#"{"subscription":{"id":"f1c2a387-161a-49f9-a165-0f21d7a4e1c4","status":"authorization_revoked","type":"channel.follow","cost":1,"version":"1","condition":{"broadcaster_user_id":"12826"},"transport":{"method":"webhook","callback":"https://example.com/webhooks/callback"},"created_at":"2019-11-16T10:11:12.123Z"}}"#;
        let mut request = http::Request::builder();
        let _ = std::mem::replace(request.headers_mut().unwrap(), headers);
        let request = request.body(body.as_bytes().to_vec()).unwrap();
        let _payload = dbg!(crate::eventsub::Event::parse_http(&request).unwrap());
        let payload = dbg!(crate::eventsub::Event::parse(
            std::str::from_utf8(request.body()).unwrap()
        )
        .unwrap());
        crate::tests::roundtrip(&payload)
    }
    #[test]
    #[cfg(feature = "hmac")]
    fn verify_request() {
        use http::header::{HeaderMap, HeaderName, HeaderValue};

        let secret = b"secretabcd";
        #[rustfmt::skip]
    let headers: HeaderMap = vec![
        ("Content-Length", "458"),
        ("Content-Type", "application/json"),
        ("Twitch-Eventsub-Message-Id", "ae2ff348-e102-16be-a3eb-6830c1bf38d2"),
        ("Twitch-Eventsub-Message-Retry", "0"),
        ("Twitch-Eventsub-Message-Signature", "sha256=d10f5bd9474b7ac7bd7105eb79c2d52768b4d0cd2a135982c3bf5a1d59a78823"),
        ("Twitch-Eventsub-Message-Timestamp", "2021-02-19T23:47:00.8091512Z"),
        ("Twitch-Eventsub-Message-Type", "notification"),
        ("Twitch-Eventsub-Subscription-Type", "channel.follow"),
        ("Twitch-Eventsub-Subscription-Version", "1"),
    ].into_iter()
        .map(|(h, v)| {
            (
                h.parse::<HeaderName>().unwrap(),
                v.parse::<HeaderValue>().unwrap(),
            )
        })
        .collect();

        let body = r#"{"subscription":{"id":"ae2ff348-e102-16be-a3eb-6830c1bf38d2","status":"enabled","type":"channel.follow","version":"1","condition":{"broadcaster_user_id":"44429626"},"transport":{"method":"webhook","callback":"null"},"created_at":"2021-02-19T23:47:00.7621315Z"},"event":{"user_id":"28408015","user_login":"testFromUser","user_name":"testFromUser","broadcaster_user_id":"44429626","broadcaster_user_login":"44429626","broadcaster_user_name":"testBroadcaster"}}"#;
        let mut request = http::Request::builder();
        let _ = std::mem::replace(request.headers_mut().unwrap(), headers);
        let request = request.body(body.as_bytes().to_vec()).unwrap();
        dbg!(&body);
        assert!(crate::eventsub::Event::verify_payload(&request, secret));
    }

    #[test]
    #[cfg(feature = "hmac")]
    fn verify_request_challenge() {
        use http::header::{HeaderMap, HeaderName, HeaderValue};

        let secret = b"HELLOabc2321";
        #[rustfmt::skip]
        let headers: HeaderMap = vec![
            ("Twitch-Eventsub-Message-Id", "8d8fa82b-9792-79da-4e11-a6fa58a7a582"),
            ("Twitch-Eventsub-Message-Retry", "0"),
            ("Twitch-Eventsub-Message-Signature", "sha256=091f6a5c74fba820f2d50e9d0c5e7650556ee009375af2cc662e610e670bc412"),
            ("Twitch-Eventsub-Message-Timestamp", "2022-02-06T04:03:24.2726598Z"),
            ("Twitch-Eventsub-Message-Type", "webhook_callback_verification"),
            ("Twitch-Eventsub-Subscription-Type", "channel.subscribe"),
            ("Twitch-Eventsub-Subscription-Version", "1"),
            ].into_iter()
        .map(|(h, v)| {
            (
                h.parse::<HeaderName>().unwrap(),
                v.parse::<HeaderValue>().unwrap(),
            )
        })
        .collect();

        let body = r#"{"challenge":"11535768-497e-14ec-8197-ba2cb5341a01","subscription":{"id":"8d8fa82b-9792-79da-4e11-a6fa58a7a582","status":"webhook_callback_verification_pending","type":"channel.subscribe","version":"1","condition":{"broadcaster_user_id":"88525095"},"transport":{"method":"webhook","callback":"http://localhost:80/twitch/eventsub"},"created_at":"2022-02-06T04:03:24.2706497Z","cost":0}}"#;

        let mut request = http::Request::builder();
        let _ = std::mem::replace(request.headers_mut().unwrap(), headers);
        let request = request.body(body.as_bytes().to_vec()).unwrap();
        let _payload = dbg!(crate::eventsub::Event::parse_http(&request).unwrap());
        assert!(crate::eventsub::Event::verify_payload(&request, secret));
    }
}

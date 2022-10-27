//! Holds serializable EventSub stuff
//!
//! Use [`CreateEventSubSubscription`](crate::helix::eventsub::CreateEventSubSubscriptionRequest) to subscribe to an event according to the [EventSub guide](https://dev.twitch.tv/docs/eventsub).
//! Parse the response payload text with [`Payload::parse`] or [`Payload::parse_http`].
//!
//! # Examples
//!
//! See [`examples/`](https://github.com/twitch-rs/twitch_api/tree/main/examples/eventsub) for a more complete example of using eventsub.
// FIXME: Use the actual link to the source files, currently can't do that on docs.rs since the workspace member is removed in the tarball
//!
//! Subscribe to a channel's follow events:
//!
//! ```rust, no_run
//! use twitch_api::eventsub::{channel::ChannelFollowV1, Transport, TransportMethod};
//! use twitch_api::helix::{self, eventsub::{
//!     CreateEventSubSubscriptionBody, CreateEventSubSubscriptionRequest,
//! }};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//!
//! let event = ChannelFollowV1::builder()
//!     .broadcaster_user_id("1234")
//!     .build();
//! let transport = Transport::webhook(
//!     "https://example.org/eventsub/channelfollow",
//!     String::from("secretabcd"),
//! );
//!
//! let request = CreateEventSubSubscriptionRequest::default();
//! let body = CreateEventSubSubscriptionBody::builder()
//!     .subscription(event)
//!     .transport(transport)
//!     .build();
//!
//! let event_information = client.req_post(request, body, &token).await?.data;
//!
//! println!("event id: {:?}", event_information.id);
//! # Ok(())
//! # }
//! ```
//!
//! You'll now get a http POST request to the url you specified as the `callback`.
//! You need to respond to this request from your webserver with a 200 OK response with the [`challenge`](VerificationRequest::challenge) as the body.
//! After this, you'll get notifications
//!
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
//!         Event::ChannelFollowV1(Payload {
//!             message: Message::VerificationRequest(ver),
//!             ..
//!         }) => {
//!             // We've verified the request, so we can respond to it with the challenge
//!             Ok(http::Response::builder()
//!                 .status(200)
//!                 .body(ver.challenge.into_bytes())?)
//!         },
//!         Event::ChannelFollowV1(Payload {
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
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::parse_json;

pub mod channel;
pub mod event;
pub mod stream;
pub mod user;

#[doc(inline)]
pub use event::{Event, EventType};

/// An EventSub subscription.
pub trait EventSubscription: DeserializeOwned + Serialize + PartialEq + Clone {
    /// Payload for given subscription
    type Payload: PartialEq + std::fmt::Debug + DeserializeOwned + Serialize + Clone;

    /// Scopes needed by this subscription
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope];
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
    pub fn parse(source: &str) -> Result<Payload<E>, PayloadParseError> {
        Self::parse_notification(source)
    }

    /// Parse string slice as a [`Payload`] with a message of [`Message::Notification`].
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
    /// use twitch_api::eventsub::{Payload, channel::ChannelFollowV1};
    /// # struct Body {} impl Body { fn new() -> Self {Body {}} fn to_bytes(&self) -> &[u8] { &[] } }
    /// # fn a() -> Result<(), twitch_api::eventsub::PayloadParseError> {
    /// // Example of a request with a body that doesn't implement `AsRef<[u8]>`
    /// let original_request: Request<Body> = http::Request::new(Body::new());
    /// // Convert to a request with a body of `Vec<u8>`, which does implement `AsRef<[u8]>`
    /// let converted_request: Request<Vec<u8>> = original_request.map(|r| r.to_bytes().to_owned());
    /// Payload::<ChannelFollowV1>::parse_http(&converted_request)?
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

    /// Parse a string slice as a [`Payload`] with a specific message type. You should not use this, instead, use [`Payload::parse_http`] or [`Payload::parse`].
    #[doc(hidden)]
    pub fn parse_request<'a>(
        ty: Cow<'a, [u8]>,
        source: Cow<'a, [u8]>,
    ) -> Result<Payload<E>, PayloadParseError> {
        let source = std::str::from_utf8(&source)?;
        match ty.as_ref() {
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

/// Transport setting for event notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Transport {
    /// Method for transport
    pub method: TransportMethod,
    /// Callback
    pub callback: String,
    /// Secret attached to the subscription.
    ///
    /// # Notes
    ///
    /// Secret must be between 10 and 100 characters
    pub secret: String,
}

impl Transport {
    /// Convenience method for making a webhook transport
    pub fn webhook(callback: impl std::string::ToString, secret: String) -> Transport {
        Transport {
            method: TransportMethod::Webhook,
            callback: callback.to_string(),
            secret,
        }
    }
}

/// Transport response on event notification
///
/// Does not include secret.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TransportResponse {
    /// Method for transport
    pub method: TransportMethod,
    /// Callback
    pub callback: String,
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
}

///  Subscription request status
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")] // FIXME: Most examples use kebab-case... but reality seems to be snake_case
pub enum Status {
    /// Designates that the subscription is in an operable state and is valid.
    Enabled,
    /// Webhook is pending verification of the callback specified in the subscription creation request.
    WebhookCallbackVerificationPending,
    /// Webhook failed verification of the callback specified in the subscription creation request.
    WebhookCallbackVerificationFailed,
    /// Notification delivery failure rate was too high.
    NotificationFailuresExceeded,
    /// Authorization for user(s) in the condition was revoked.
    AuthorizationRevoked,
    /// A user in the condition of the subscription was removed.
    UserRemoved,
}

/// General information about an EventSub subscription.
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

#[cfg(test)]
mod test {

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

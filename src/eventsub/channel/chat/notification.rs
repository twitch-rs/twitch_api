#![doc(alias = "channel.chat.notification")]
//! An event that appears in chat occurs, such as someone subscribing to the channel or a subscription is gifted.

use super::*;
/// [`channel.chat.notification`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatnotification): an event that appears in chat occurs, such as someone subscribing to the channel or a subscription is gifted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatNotificationV1 {
    /// User ID of the channel to receive chat notification events for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID to read chat as.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub user_id: types::UserId,
}

impl ChannelChatNotificationV1 {
    /// Get chat notifications on broadcasters channel reading chat as a specific user.
    pub fn new(
        broadcaster_user_id: impl Into<types::UserId>,
        user_id: impl Into<types::UserId>,
    ) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
            user_id: user_id.into(),
        }
    }
}

impl EventSubscription for ChannelChatNotificationV1 {
    type Payload = ChannelChatNotificationV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelChatNotification;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadChat];
    const VERSION: &'static str = "1";
}

// XXX: this struct can never be deny_unknown_fields
/// [`channel.chat.notification`](ChannelChatNotificationV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChannelChatNotificationV1Payload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The user ID of the user that sent the message.
    #[serde(flatten)]
    pub chatter: Chatter,
    /// List of chat badges.
    pub badges: Vec<Badge>,
    /// The message Twitch shows in the chat room for this notice.
    pub system_message: String,
    /// A UUID that identifies the message.
    pub message_id: types::MsgId,
    /// The structured chat message
    pub message: Message,
    /// The notification
    #[serde(flatten)]
    pub notification: Notification,
}

/// Information about the user that triggered this notification
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Chatter {
    /// Chatter
    Chatter {
        /// The user ID of the user that sent the message.
        chatter_user_id: types::UserId,
        /// The user name of the user that sent the message.
        chatter_user_name: types::DisplayName,
        /// The user login of the user that sent the message.
        chatter_user_login: types::UserName,
        /// The color of the user's name in the chat room.
        /// This is a hexadecimal RGB color code in the form, `#<RGB>`.
        /// This may be empty if it is never set.
        color: types::HexColor,
    },
    /// Chatter is anonymous
    Anonymous,
}

impl Chatter {
    /// Returns `true` if the chatter is [`Anonymous`].
    ///
    /// [`Anonymous`]: Chatter::Anonymous
    #[must_use]
    pub fn is_anonymous(&self) -> bool { matches!(self, Self::Anonymous) }

    /// Returns `true` if the chatter is [`Chatter`].
    ///
    /// [`Chatter`]: Chatter::Chatter
    #[must_use]
    pub fn is_chatter(&self) -> bool { matches!(self, Self::Chatter { .. }) }
}

impl serde::Serialize for Chatter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        #[derive(Default, Serialize)]
        struct InnerChatter<'a> {
            chatter_user_id: Option<&'a types::UserIdRef>,
            chatter_user_name: Option<&'a types::DisplayNameRef>,
            chatter_user_login: Option<&'a types::UserNameRef>,
            color: Option<&'a types::HexColor>,
            chatter_is_anonymous: bool,
        }

        match self {
            Chatter::Chatter {
                chatter_user_id,
                chatter_user_name,
                chatter_user_login,
                color,
            } => InnerChatter {
                chatter_user_id: Some(chatter_user_id),
                chatter_user_name: Some(chatter_user_name),
                chatter_user_login: Some(chatter_user_login),
                color: Some(color),
                chatter_is_anonymous: false,
            },
            Chatter::Anonymous => InnerChatter {
                chatter_is_anonymous: true,
                ..Default::default()
            },
        }
        .serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Chatter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        #[derive(Deserialize)]
        struct InnerChatter {
            chatter_user_id: Option<types::UserId>,
            chatter_user_name: Option<types::DisplayName>,
            chatter_user_login: Option<types::UserName>,
            color: Option<types::HexColor>,
            chatter_is_anonymous: bool,
        }

        let chatter = InnerChatter::deserialize(deserializer)?;
        if chatter.chatter_is_anonymous {
            #[cfg(feature = "tracing")]
            if let Some(c) = chatter.color {
                if c.as_str() != "" {
                    tracing::error!("got an anonymous user with color set to {c}");
                }
            }
            Ok(Chatter::Anonymous)
        } else {
            Ok(Chatter::Chatter {
                chatter_user_id: chatter
                    .chatter_user_id
                    .ok_or_else(|| serde::de::Error::missing_field("chatter_user_id"))?,
                chatter_user_name: chatter
                    .chatter_user_name
                    .ok_or_else(|| serde::de::Error::missing_field("chatter_user_name"))?,
                chatter_user_login: chatter
                    .chatter_user_login
                    .ok_or_else(|| serde::de::Error::missing_field("chatter_user_login"))?,
                color: chatter
                    .color
                    .ok_or_else(|| serde::de::Error::missing_field("color"))?,
            })
        }
    }
}

/// All possible notifications in [`ChannelChatNotificationV1Payload`]
// XXX: this struct can never be deny_unknown_fields
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "notice_type", rename_all = "snake_case")]
pub enum Notification {
    /// Information about the sub event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    #[serde(rename = "sub")]
    Subscription(Subscription),
    /// Information about the resub event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    #[serde(rename = "resub")]
    Resubscription(Resubscription),
    /// Information about the gift sub event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    SubGift(SubGift),
    /// Information about the community gift sub event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    CommunitySubGift(CommunitySubGift),
    /// Information about the community gift paid upgrade event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    GiftPaidUpgrade(GiftPaidUpgrade),
    /// Information about the Prime gift paid upgrade event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    PrimePaidUpgrade(PrimePaidUpgrade),
    /// Information about the raid event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Raid(Raid),
    /// a unraid event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Unraid(Unraid),
    /// Information about the pay it forward event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    PayItForward(PayItForward),
    /// Information about the announcement event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    Announcement(Announcement),
    /// Information about the charity donation event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    CharityDonation(CharityDonation),
    /// Information about the bits badge tier event.
    #[serde(with = "crate::eventsub::enum_field_as_inner")]
    BitsBadgeTier(BitsBadgeTier),
}

impl crate::eventsub::NamedField for Subscription {
    const NAME: &'static str = "sub";
}
impl crate::eventsub::NamedField for Resubscription {
    const NAME: &'static str = "resub";
}
impl crate::eventsub::NamedField for SubGift {
    const NAME: &'static str = "sub_gift";
}
impl crate::eventsub::NamedField for CommunitySubGift {
    const NAME: &'static str = "community_sub_gift";
}
impl crate::eventsub::NamedField for GiftPaidUpgrade {
    const NAME: &'static str = "gift_paid_upgrade";
}
impl crate::eventsub::NamedField for PrimePaidUpgrade {
    const NAME: &'static str = "prime_paid_upgrade";
}
impl crate::eventsub::NamedField for Raid {
    const NAME: &'static str = "raid";
}
impl crate::eventsub::NamedField for Unraid {
    const NAME: &'static str = "unraid";
}
impl crate::eventsub::NamedField for PayItForward {
    const NAME: &'static str = "pay_it_forward";
}
impl crate::eventsub::NamedField for Announcement {
    const NAME: &'static str = "announcement";
}
impl crate::eventsub::NamedField for CharityDonation {
    const NAME: &'static str = "charity_donation";
}
impl crate::eventsub::NamedField for BitsBadgeTier {
    const NAME: &'static str = "bits_badge_tier";
}

/// A subscription notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Subscription {
    /// The type of subscription plan being used. Possible values are:
    ///
    /// * `1000` — First level of paid or Prime subscription
    /// * `2000` — Second level of paid subscription
    /// * `3000` — Third level of paid subscription
    pub sub_tier: types::SubscriptionTier,
    /// Indicates if the subscription was obtained through Amazon Prime.
    pub is_prime: bool,
    /// The number of months the subscription is for.
    pub duration_months: i32,
}

/// A gifter
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Gifter {
    /// No gifter
    None,
    /// An anonymous gifter
    Anonymous,
    /// A gifter
    Gifter {
        /// The user ID of the subscription gifter. Null if anonymous.
        gifter_user_id: types::UserId,
        /// The user name of the subscription gifter. Null if anonymous.
        gifter_user_name: types::DisplayName,
        /// The user login of the subscription gifter. Null if anonymous.
        gifter_user_login: types::UserName,
    },
}

impl Gifter {
    /// Returns `true` if the gifter is [`Anonymous`].
    ///
    /// [`Anonymous`]: Gifter::Anonymous
    #[must_use]
    pub fn is_anonymous(&self) -> bool { matches!(self, Self::Anonymous) }

    /// Returns `true` if the gifter is [`Gifter`].
    ///
    /// [`Gifter`]: Gifter::Gifter
    #[must_use]
    pub fn is_gifter(&self) -> bool { matches!(self, Self::Gifter { .. }) }
}

impl serde::Serialize for Gifter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        #[derive(Default, Serialize)]
        struct InnerGifter<'a> {
            gifter_user_id: Option<&'a types::UserIdRef>,
            gifter_user_name: Option<&'a types::DisplayNameRef>,
            gifter_user_login: Option<&'a types::UserNameRef>,
            gifter_is_anonymous: Option<bool>,
        }

        match self {
            Gifter::Gifter {
                gifter_user_id,
                gifter_user_name,
                gifter_user_login,
            } => InnerGifter {
                gifter_user_id: Some(gifter_user_id),
                gifter_user_name: Some(gifter_user_name),
                gifter_user_login: Some(gifter_user_login),
                gifter_is_anonymous: Some(false),
            },
            Gifter::Anonymous => InnerGifter {
                gifter_is_anonymous: Some(true),
                ..Default::default()
            },
            Gifter::None => Default::default(),
        }
        .serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Gifter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        #[derive(Deserialize)]
        struct InnerGifter {
            gifter_user_id: Option<types::UserId>,
            gifter_user_name: Option<types::DisplayName>,
            gifter_user_login: Option<types::UserName>,
            gifter_is_anonymous: Option<bool>,
        }

        let gifter = InnerGifter::deserialize(deserializer)?;
        if let Some(true) = gifter.gifter_is_anonymous {
            Ok(Gifter::Anonymous)
        } else if let None = gifter.gifter_is_anonymous {
            Ok(Gifter::None)
        } else {
            Ok(Gifter::Gifter {
                gifter_user_id: gifter
                    .gifter_user_id
                    .ok_or_else(|| serde::de::Error::missing_field("gifter_user_id"))?,
                gifter_user_name: gifter
                    .gifter_user_name
                    .ok_or_else(|| serde::de::Error::missing_field("gifter_user_name"))?,
                gifter_user_login: gifter
                    .gifter_user_login
                    .ok_or_else(|| serde::de::Error::missing_field("gifter_user_login"))?,
            })
        }
    }
}

/// A resubcription notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Resubscription {
    /// The total number of months the user has subscribed.
    pub cumulative_months: i32,
    /// The number of months the subscription is for.
    pub duration_months: i32,
    /// The number of consecutive months the user has subscribed.
    pub streak_months: Option<i32>,
    /// The type of subscription plan being used. Possible values are:
    ///
    /// * `1000` — First level of paid or Prime subscription
    /// * `2000` — Second level of paid subscription
    /// * `3000` — Third level of paid subscription
    pub sub_tier: types::SubscriptionTier,
    /// Indicates if the resub was obtained through Amazon Prime.
    pub is_prime: bool,
    /// Whether or not the resub was a result of a gift.
    pub is_gift: bool,
    // FIXME: This might eat errors
    /// The gifter
    #[serde(flatten)]
    pub gifter: Gifter,
}

/// A subscription gift notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SubGift {
    /// The number of months the subscription is for.
    pub duration_months: i32,
    /// The amount of gifts the gifter has given in this channel. Null if anonymous.
    pub cumulative_total: Option<i32>,
    /// The user ID of the subscription gift recipient.
    pub recipient_user_id: types::UserId,
    /// The user name of the subscription gift recipient.
    pub recipient_user_name: types::DisplayName,
    /// The user login of the subscription gift recipient.
    pub recipient_user_login: types::UserName,
    /// The type of subscription plan being used. Possible values are:
    ///
    /// * `1000` — First level of paid subscription
    /// * `2000` — Second level of paid subscription
    /// * `3000` — Third level of paid subscription
    pub sub_tier: types::SubscriptionTier,
    /// The ID of the associated community gift. Null if not associated with a community gift.
    pub community_gift_id: Option<types::CommunityGiftId>,
}

/// A gift notification for multiple gifted subscriptions. Followed by [`CommunitySubGift::total`] amount of [`SubGift`]s.
///
/// Contains the id for [`SubGift::community_gift_id`]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CommunitySubGift {
    /// The ID of the associated community gift.
    pub id: types::CommunityGiftId,
    /// Number of subscriptions being gifted.
    pub total: i32,
    /// The type of subscription plan being used. Possible values are:
    ///
    /// * `1000` — First level of paid subscription
    /// * `2000` — Second level of paid subscription
    /// * `3000` — Third level of paid subscription
    pub sub_tier: types::SubscriptionTier,
    /// The amount of gifts the gifter has given in this channel. Null if anonymous.
    pub cumulative_total: Option<i32>,
}

/// A gift notification for a paid upgrade of a previously gifted subscription.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GiftPaidUpgrade {
    /// The gifter
    #[serde(flatten)]
    pub gifter: Gifter,
}

/// A notification for a paid upgrade of a previous Twitch Prime channel subscription.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PrimePaidUpgrade {
    /// The type of subscription plan being used. Possible values are:
    ///
    /// * `1000` — First level of paid subscription
    /// * `2000` — Second level of paid subscription
    /// * `3000` — Third level of paid subscription
    pub sub_tier: types::SubscriptionTier,
}

/// A raid notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Raid {
    /// The user ID of the broadcaster raiding this channel.
    pub user_id: types::UserId,
    /// The user name of the broadcaster raiding this channel.
    pub user_name: types::DisplayName,
    /// The login name of the broadcaster raiding this channel.
    pub user_login: types::UserName,
    /// The number of viewers raiding this channel from the broadcaster’s channel.
    pub viewer_count: i32,
    /// Profile image URL of the broadcaster raiding this channel.
    pub profile_image_url: String,
}

/// A unraid notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Unraid {}

/// A pay it forward notification
///
/// This event is triggered when a user gifts a subscription to another user in the channel when they were themselves gifted a subscription by another user in the channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PayItForward {
    /// The gifter
    #[serde(flatten)]
    pub gifter: Gifter,
    /// The user ID of the subscription gift recipient.
    pub recipient_user_id: Option<types::UserId>,
    /// The user name of the subscription gift recipient.
    pub recipient_user_name: Option<types::DisplayName>,
    /// The user login of the subscription gift recipient.
    pub recipient_user_login: Option<types::UserName>,
}

/// A announcement notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Announcement {
    /// Color of the announcement.
    pub color: crate::extra::AnnouncementColor,
}

/// A charity donation notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CharityDonation {
    /// Name of the charity.
    pub charity_name: String,
    /// An object that contains the amount of money that the user paid.
    pub amount: crate::extra::DonationAmount,
}

/// A bits badge tier upgrade notification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BitsBadgeTier {
    /// The tier of the Bits badge the user just earned. For example, 100, 1000, or 10000.
    pub tier: i32,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.chat.notification",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337",
                "user_id": "9001"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2023-04-11T10:11:12.123Z"
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "Cool_User",
            "broadcaster_user_login": "cool_user",
            "chatter_user_id": "444",
            "chatter_user_login": "cool_chatter",
            "chatter_user_name": "Cool_Chatter",
            "chatter_is_anonymous": false,
            "color": "red",
            "badges": [
              {
                "set_id": "moderator",
                "id": "1",
                "info": ""
              },
              {
                "set_id": "subscriber",
                "id": "12",
                "info": "16"
              },
              {
                "set_id": "sub-gifter",
                "id": "1",
                "info": ""
              }
            ],
            "system_message": "chat message",
            "message_id": "message-id",
            "message": {
                "text": "chat-msg",
                "fragments": [
                {
                    "type": "emote",
                    "text": "chat-msg",
                    "cheermote": null,
                    "emote": {
                    "id": "emote-id",
                    "emote_set_id": "emote-set",
                    "owner_id": "emote-owner",
                    "format": [
                        "static"
                    ]
                    },
                    "mention": null
                }
                ]
            },
            "notice_type": "announcement",
            "sub": null,
            "resub": null,
            "sub_gift": null,
            "community_sub_gift": null,
            "gift_paid_upgrade": null,
            "prime_paid_upgrade": null,
            "pay_it_forward": null,
            "raid": null,
            "unraid": null,
            "announcement": {
                "color": "blue"
            },
            "bits_badge_tier": null,
            "charity_donation": null
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#[cfg(test)]
#[test]
fn parse_payload_resub_without_message() {
    let payload = r##"
    {
        "subscription": {
            "id": "eebd50e7-2e58-4034-849b-d47e935632da5",
            "status": "enabled",
            "type": "channel.chat.notification",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "1337",
                "user_id": "27620241"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQTaaaaaaaab2QtdG5r8vMSBIaaaaaaaaa"
            },
            "created_at": "2023-11-19T21:31:08.935820817Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "chatter_user_id": "1234",
            "chatter_user_login": "justinfan",
            "chatter_user_name": "justinfan",
            "chatter_is_anonymous": false,
            "color": "#E20072",
            "badges": [
                {
                    "set_id": "subscriber",
                    "id": "60",
                    "info": "65"
                },
                {
                    "set_id": "twitchconAmsterdam2020",
                    "id": "1",
                    "info": ""
                }
            ],
            "system_message": "justinfan subscribed at Tier 1. They've subscribed for 65 months!",
            "message_id": "5dfe4963-9db8-44a9-9f69-27452aaaaa30",
            "message": {
                "text": "",
                "fragments": []
            },
            "notice_type": "resub",
            "sub": null,
            "resub": {
                "cumulative_months": 65,
                "duration_months": 0,
                "streak_months": null,
                "sub_tier": "1000",
                "is_prime": false,
                "is_gift": false,
                "gifter_is_anonymous": null,
                "gifter_user_id": null,
                "gifter_user_name": null,
                "gifter_user_login": null
            },
            "sub_gift": null,
            "community_sub_gift": null,
            "gift_paid_upgrade": null,
            "prime_paid_upgrade": null,
            "pay_it_forward": null,
            "raid": null,
            "unraid": null,
            "announcement": null,
            "bits_badge_tier": null,
            "charity_donation": null
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#[cfg(test)]
#[test]
fn parse_payload_cheer_just_text() {
    let payload = r##"
    {
        "subscription": {
            "id": "2237f256-1b83-4ec9-956e-c3578925e8e6",
            "status": "enabled",
            "type": "channel.chat.notification",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "67931625",
                "user_id": "27620241"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQuf4yiyFuTd64mlIG4zbSOxIGY2VsbC1j"
            },
            "created_at": "2023-11-19T22:08:49.127052362Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "67931625",
            "broadcaster_user_login": "amar",
            "broadcaster_user_name": "Amar",
            "chatter_user_id": "101572475",
            "chatter_user_login": "justinfan",
            "chatter_user_name": "justinfan",
            "chatter_is_anonymous": false,
            "color": "#FA6E02",
            "badges": [
                {
                    "set_id": "moderator",
                    "id": "1",
                    "info": ""
                },
                {
                    "set_id": "subscriber",
                    "id": "48",
                    "info": "58"
                },
                {
                    "set_id": "partner",
                    "id": "1",
                    "info": ""
                }
            ],
            "system_message": "justinfan subscribed at Tier 1. They've subscribed for 58 months!",
            "message_id": "aaa15585-0f43-4c32-8c48-56d58e9567a7",
            "message": {
                "text": "GIB MIR DEN POKAL",
                "fragments": [
                    {
                        "type": "text",
                        "text": "GIB MIR DEN POKAL",
                        "cheermote": null,
                        "emote": null,
                        "mention": null
                    }
                ]
            },
            "notice_type": "resub",
            "sub": null,
            "resub": {
                "cumulative_months": 58,
                "duration_months": 0,
                "streak_months": null,
                "sub_tier": "1000",
                "is_prime": false,
                "is_gift": false,
                "gifter_is_anonymous": null,
                "gifter_user_id": null,
                "gifter_user_name": null,
                "gifter_user_login": null
            },
            "sub_gift": null,
            "community_sub_gift": null,
            "gift_paid_upgrade": null,
            "prime_paid_upgrade": null,
            "pay_it_forward": null,
            "raid": null,
            "unraid": null,
            "announcement": null,
            "bits_badge_tier": null,
            "charity_donation": null
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#[cfg(test)]
#[test]
fn parse_payload_sub_gift_anon() {
    let payload = r##"
    {
        "subscription": {
            "id": "f6b57ae8-add7-4faa-a396-4d6c87cb1337",
            "status": "enabled",
            "type": "channel.chat.notification",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "1337",
                "user_id": "27620241"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQ_ZPE5-zrR2W3HJjkQxrFfxIGY2Vs1337"
            },
            "created_at": "2023-11-20T09:52:07.940291459Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "chatter_user_id": null,
            "chatter_user_login": null,
            "chatter_user_name": null,
            "chatter_is_anonymous": true,
            "color": "",
            "badges": [],
            "system_message": "An anonymous user is gifting 1 Tier 1 Subs to Cool_User's community!",
            "message_id": "fa2135ca-34da-413f-1337-4efa3c9a6bac",
            "message": {
                "text": "",
                "fragments": []
            },
            "notice_type": "community_sub_gift",
            "sub": null,
            "resub": null,
            "sub_gift": null,
            "community_sub_gift": {
                "id": "12111455614691086753",
                "total": 1,
                "cumulative_total": null,
                "sub_tier": "1000"
            },
            "gift_paid_upgrade": null,
            "prime_paid_upgrade": null,
            "pay_it_forward": null,
            "raid": null,
            "unraid": null,
            "announcement": null,
            "bits_badge_tier": null,
            "charity_donation": null
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#[cfg(test)]
#[test]
pub fn resub_doesnt_eat_gifter_error() {
    let payload = r#"
    {
        "cumulative_months": 2,
        "duration_months": 0,
        "streak_months": null,
        "sub_tier": "1000",
        "is_prime": false,
        "is_gift": true,
        "gifter_is_anonymous": false,
        "gifter_user_id": "1234",
        "gifter_user_name": 1,
        "gifter_user_login": "justinfan1"
      }
    "#;

    dbg!(serde_json::from_str::<Resubscription>(payload).unwrap_err());
}
#[cfg(test)]
#[test]
fn parse_payload_resub_gifted() {
    let payload = r##"
    {
        "subscription": {
          "id": "96f9a91e-f1e0-43af-82ac-a6e934771337",
          "status": "enabled",
          "type": "channel.chat.notification",
          "version": "1",
          "condition": {
            "broadcaster_user_id": "165081337",
            "user_id": "27620241"
          },
          "transport": {
            "method": "websocket",
            "session_id": "AgoQUlB8aB2SSsavWVfcs5ljnBIGY2Vs1337"
          },
          "created_at": "2023-11-20T16:41:22.999246448Z",
          "cost": 0
        },
        "event": {
          "broadcaster_user_id": "1337",
          "broadcaster_user_login": "Cool_User",
          "broadcaster_user_name": "cool_user",
          "chatter_user_id": "5678",
          "chatter_user_login": "someone1",
          "chatter_user_name": "someone1",
          "chatter_is_anonymous": false,
          "color": "",
          "badges": [
            {
              "set_id": "subscriber",
              "id": "2",
              "info": "2"
            }
          ],
          "system_message": "someone1 subscribed at Tier 1. They've subscribed for 2 months!",
          "message_id": "101ab672-fcde-4d71-8011-ac2859786cea",
          "message": {
            "text": "",
            "fragments": []
          },
          "notice_type": "resub",
          "sub": null,
          "resub": {
            "cumulative_months": 2,
            "duration_months": 0,
            "streak_months": null,
            "sub_tier": "1000",
            "is_prime": false,
            "is_gift": true,
            "gifter_is_anonymous": false,
            "gifter_user_id": "1234",
            "gifter_user_name": "justinfan1",
            "gifter_user_login": "justinfan1"
          },
          "sub_gift": null,
          "community_sub_gift": null,
          "gift_paid_upgrade": null,
          "prime_paid_upgrade": null,
          "pay_it_forward": null,
          "raid": null,
          "unraid": null,
          "announcement": null,
          "bits_badge_tier": null,
          "charity_donation": null
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#[cfg(test)]
#[test]
fn parse_payload_pay_it_forward() {
    let payload = r##"
    {
        "subscription": {
            "id": "96f9a91e-f1e0-43af-82ac-a6e934778805",
            "status": "enabled",
            "type": "channel.chat.notification",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "165080419",
                "user_id": "27620241"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AgoQUlB8aB2SSsavWVfcs5ljnBIGY2VsbC1j"
            },
            "created_at": "2023-11-20T16:41:22.999246448Z",
            "cost": 0
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "Cool_User",
            "broadcaster_user_name": "cool_user",
            "chatter_user_id": "1234",
            "chatter_user_login": "justinfan1",
            "chatter_user_name": "justinfan1",
            "chatter_is_anonymous": false,
            "color": "#03FCFC",
            "badges": [
                {
                    "set_id": "subscriber",
                    "id": "3",
                    "info": "5"
                },
                {
                    "set_id": "rplace-2023",
                    "id": "1",
                    "info": ""
                }
            ],
            "system_message": "justinfan1 is paying forward the Gift they got from SomeoneElse to the community!",
            "message_id": "3bc0badf-1d2c-45cd-8743-62ba2f411337",
            "message": {
                "text": "",
                "fragments": []
            },
            "notice_type": "pay_it_forward",
            "sub": null,
            "resub": null,
            "sub_gift": null,
            "community_sub_gift": null,
            "gift_paid_upgrade": null,
            "prime_paid_upgrade": null,
            "pay_it_forward": {
                "recipient_user_id": null,
                "recipient_user_name": null,
                "recipient_user_login": null,
                "gifter_is_anonymous": false,
                "gifter_user_id": "5678",
                "gifter_user_name": "SomeoneElse",
                "gifter_user_login": "someoneelse"
            },
            "raid": null,
            "unraid": null,
            "announcement": null,
            "bits_badge_tier": null,
            "charity_donation": null
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}

#[cfg(test)]
#[test]
fn parse_payload_examples() {
    let payloads = vec![
        r##"
{"subscription":{"id":"d46bb9d5-7b78-4495-a8f6-31e0bfe74422","status":"enabled","type":"channel.chat.notification","version":"1","condition":{"broadcaster_user_id":"67931625","user_id":"27620241"},"transport":{"method":"websocket","session_id":"AgoQUlB8aB2SSsavWVfcs5ljnBIGY2VsbC1j"},"created_at":"2023-11-20T16:41:21.3673997Z","cost":0},"event":{"broadcaster_user_id":"67931625","broadcaster_user_login":"amar","broadcaster_user_name":"Amar","chatter_user_id":"276319092","chatter_user_login":"baba_avil","chatter_user_name":"BaBa_Avil","chatter_is_anonymous":false,"color":"#9ACD32","badges":[{"set_id":"subscriber","id":"3","info":"4"},{"set_id":"premium","id":"1","info":""}],"system_message":"BaBa_Avil is paying forward the Gift they got from sarius05 to melizhr!","message_id":"1b1cf50d-d01b-4019-a564-cad2937bfeae","message":{"text":"","fragments":[]},"notice_type":"pay_it_forward","sub":null,"resub":null,"sub_gift":null,"community_sub_gift":null,"gift_paid_upgrade":null,"prime_paid_upgrade":null,"pay_it_forward":{"recipient_user_id":"924524794","recipient_user_name":"melizhr","recipient_user_login":"melizhr","gifter_is_anonymous":false,"gifter_user_id":"434322296","gifter_user_name":"sarius05","gifter_user_login":"sarius05"},"raid":null,"unraid":null,"announcement":null,"bits_badge_tier":null,"charity_donation":null}}
"##,
        r##"
{"subscription":{"id":"96f9a91e-f1e0-43af-82ac-a6e934778805","status":"enabled","type":"channel.chat.notification","version":"1","condition":{"broadcaster_user_id":"165080419","user_id":"27620241"},"transport":{"method":"websocket","session_id":"AgoQUlB8aB2SSsavWVfcs5ljnBIGY2VsbC1j"},"created_at":"2023-11-20T16:41:22.999246448Z","cost":0},"event":{"broadcaster_user_id":"165080419","broadcaster_user_login":"elraenn","broadcaster_user_name":"Elraenn","chatter_user_id":"735414451","chatter_user_login":"deniztony","chatter_user_name":"DenizTony","chatter_is_anonymous":false,"color":"#03FCFC","badges":[{"set_id":"subscriber","id":"3","info":"5"},{"set_id":"rplace-2023","id":"1","info":""}],"system_message":"DenizTony is paying forward the Gift they got from VikingHido to the community!","message_id":"3bc0badf-1d2c-45cd-8743-62ba2f41a2f6","message":{"text":"","fragments":[]},"notice_type":"pay_it_forward","sub":null,"resub":null,"sub_gift":null,"community_sub_gift":null,"gift_paid_upgrade":null,"prime_paid_upgrade":null,"pay_it_forward":{"recipient_user_id":null,"recipient_user_name":null,"recipient_user_login":null,"gifter_is_anonymous":false,"gifter_user_id":"446352253","gifter_user_name":"VikingHido","gifter_user_login":"vikinghido"},"raid":null,"unraid":null,"announcement":null,"bits_badge_tier":null,"charity_donation":null}}
"##,
        r##"
{"subscription":{"id":"96f9a91e-f1e0-43af-82ac-a6e934778805","status":"enabled","type":"channel.chat.notification","version":"1","condition":{"broadcaster_user_id":"165080419","user_id":"27620241"},"transport":{"method":"websocket","session_id":"AgoQUlB8aB2SSsavWVfcs5ljnBIGY2VsbC1j"},"created_at":"2023-11-20T16:41:22.999246448Z","cost":0},"event":{"broadcaster_user_id":"165080419","broadcaster_user_login":"elraenn","broadcaster_user_name":"Elraenn","chatter_user_id":"425829220","chatter_user_login":"kecogluali5","chatter_user_name":"Kecogluali5","chatter_is_anonymous":false,"color":"#0000FF","badges":[{"set_id":"subscriber","id":"12","info":"12"}],"system_message":"Kecogluali5 is paying forward the Gift they got from CasinoZEBERUS to the community!","message_id":"573a20ff-8f88-499e-be2b-c7079eb76f6c","message":{"text":"","fragments":[]},"notice_type":"pay_it_forward","sub":null,"resub":null,"sub_gift":null,"community_sub_gift":null,"gift_paid_upgrade":null,"prime_paid_upgrade":null,"pay_it_forward":{"recipient_user_id":null,"recipient_user_name":null,"recipient_user_login":null,"gifter_is_anonymous":false,"gifter_user_id":"967793245","gifter_user_name":"CasinoZEBERUS","gifter_user_login":"casinozeberus"},"raid":null,"unraid":null,"announcement":null,"bits_badge_tier":null,"charity_donation":null}}
"##,
        //r#""#,
    ];
    for payload in payloads {
        let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
        crate::tests::roundtrip(&val)
    }
}

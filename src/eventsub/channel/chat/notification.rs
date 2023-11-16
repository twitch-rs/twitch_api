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

/// [`channel.chat.notification`](ChannelChatNotificationV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelChatNotificationV1Payload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The user ID of the user that sent the message.
    pub chatter_user_id: types::UserId,
    /// The user name of the user that sent the message.
    pub chatter_user_name: types::DisplayName,
    /// The user login of the user that sent the message.
    pub chatter_user_login: types::UserName,
    /// Whether or not the chatter is anonymous.
    pub chatter_is_anonymous: bool,
    /// The color of the user’s name in the chat room.
    pub color: types::HexColor,
    /// List of chat badges.
    pub badges: Vec<Badge>,
    /// The message Twitch shows in the chat room for this notice.
    pub system_message: String,
    /// A UUID that identifies the message.
    pub message_id: types::MsgId,
    /// The structured chat message
    pub message: Message,
    /// The type of notice.
    pub notice_type: NoticeType,
    /// Information about the sub event. Null if notice_type is not sub.
    pub sub: Option<Subscription>,
    /// Information about the resub event. Null if notice_type is not resub.
    pub resub: Option<Resubscription>,
    /// Information about the gift sub event. Null if notice_type is not sub_gift.
    pub sub_gift: Option<SubGift>,
    /// Information about the community gift sub event. Null if notice_type is not community_sub_gift.
    pub community_sub_gift: Option<CommunitySubGift>,
    /// Information about the community gift paid upgrade event. Null if notice_type is not gift_paid_upgrade.
    pub gift_paid_upgrade: Option<GiftPaidUpgrade>,
    /// Information about the Prime gift paid upgrade event. Null if notice_type is not prime_paid_upgrade.
    pub prime_paid_upgrade: Option<PrimePaidUpgrade>,
    /// Information about the raid event. Null if notice_type is not raid.
    pub raid: Option<Raid>,
    /// Returns an empty payload if notice_type is unraid, otherwise returns null.
    pub unraid: Option<Unraid>,
    /// Information about the pay it forward event. Null if notice_type is not pay_it_forward.
    pub pay_it_forward: Option<PayItForward>,
    /// Information about the announcement event. Null if notice_type is not announcement
    pub announcement: Option<Announcement>,
    /// Information about the charity donation event. Null if notice_type is not charity_donation.
    pub charity_donation: Option<CharityDonation>,
    /// Information about the bits badge tier event. Null if notice_type is not bits_badge_tier.
    pub bits_badge_tier: Option<BitsBadgeTier>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Badge {
    /// An ID that identifies this set of chat badges. For example, Bits or Subscriber.
    pub set_id: types::BadgeSetId,
    /// An ID that identifies this version of the badge. The ID can be any value. For example, for Bits, the ID is the Bits tier level, but for World of Warcraft, it could be Alliance or Horde.
    pub id: types::ChatBadgeId,
    /// Contains metadata related to the chat badges in the badges tag. Currently, this tag contains metadata only for subscriber badges, to indicate the number of months the user has been a subscriber.
    pub info: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Message {
    /// The chat message in plain text.
    pub text: String,
    /// Ordered list of chat message fragments.
    pub fragments: Vec<Fragment>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Fragment {
    /// The type of message fragment. Possible values:
    ///
    /// * `text`
    /// * `cheermote`
    /// * `emote`
    /// * `mention`
    #[serde(rename = "type")]
    pub type_: FragmentType,
    /// Message text in fragment
    pub text: String,
    /// Metadata pertaining to the cheermote.
    pub cheermote: Option<Cheermote>,
    /// Metadata pertaining to the emote.
    pub emote: Option<Emote>,
    /// Metadata pertaining to the mention.
    pub mention: Option<Mention>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum FragmentType {
    Text,
    Cheermote,
    Emote,
    Mention,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Cheermote {
    /// The name portion of the Cheermote string that you use in chat to cheer Bits.
    ///
    /// The full Cheermote string is the concatenation of {prefix} + {number of Bits}.
    /// For example, if the prefix is “Cheer” and you want to cheer 100 Bits, the full Cheermote string is Cheer100.
    /// When the Cheermote string is entered in chat, Twitch converts it to the image associated with the Bits tier that was cheered.
    pub prefix: String,
    /// The amount of bits cheered.
    pub bits: i32,
    /// The tier level of the cheermote.
    pub tier: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Emote {
    /// An ID that uniquely identifies this emote.
    pub id: types::EmoteId,
    /// An ID that identifies the emote set that the emote belongs to.
    pub emote_set_id: types::EmoteSetId,
    /// The ID of the broadcaster who owns the emote.
    pub owner_id: types::UserId,
    /// The formats that the emote is available in. For example, if the emote is available only as a static PNG, the array contains only static. But if the emote is available as a static PNG and an animated GIF, the array contains static and animated. The possible formats are:
    ///
    /// * `animated` — An animated GIF is available for this emote.
    /// * `static` — A static PNG file is available for this emote.
    pub format: Vec<types::EmoteAnimationSetting>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Mention {
    /// The user ID of the mentioned user.
    pub user_id: types::UserId,
    /// The user name of the mentioned user.
    pub user_name: types::DisplayName,
    /// The user login of the mentioned user.
    pub user_login: types::UserName,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum NoticeType {
    Sub,
    Resub,
    SubGift,
    CommunitySubGift,
    GiftPaidUpgrade,
    PrimePaidUpgrade,
    Raid,
    Unraid,
    PayItForward,
    Announcement,
    BitsBadgeTier,
    CharityDonation,
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
    /// Whether or not the gift was anonymous. Null if not a gift.
    pub gifter_is_anonymous: Option<bool>,
    /// The user ID of the subscription gifter. Null if anonymous.
    pub gifter_user_id: Option<types::UserId>,
    /// The user name of the subscription gifter. Null if anonymous.
    pub gifter_user_name: Option<types::DisplayName>,
    /// The user login of the subscription gifter. Null if anonymous.
    pub gifter_user_login: Option<types::UserName>,
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GiftPaidUpgrade {
    /// Whether the gift was given anonymously.
    pub gifter_is_anonymous: bool,
    /// The user ID of the user who gifted the subscription. Null if anonymous.
    pub gifter_user_id: Option<types::UserId>,
    /// The user name of the user who gifted the subscription. Null if anonymous.
    pub gifter_user_name: Option<types::DisplayName>,
    /// The user login of the user who gifted the subscription. Null if anonymous.
    pub gifter_user_login: Option<types::UserName>,
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Unraid {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PayItForward {
    /// Whether the gift was given anonymously.
    pub gifter_is_anonymous: bool,
    /// The user ID of the user who gifted the subscription. Null if anonymous.
    pub gifter_user_id: Option<types::UserId>,
    /// The user name of the user who gifted the subscription. Null if anonymous.
    pub gifter_user_name: Option<types::DisplayName>,
    /// The user login of the user who gifted the subscription. Null if anonymous.
    pub gifter_user_login: Option<types::UserName>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Announcement {
    /// Color of the announcement.
    pub color: types::HexColor,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CharityDonation {
    /// Name of the charity.
    pub charity_name: String,
    /// An object that contains the amount of money that the user paid.
    pub amount: crate::extra::DonationAmount,
}

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

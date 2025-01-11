#![doc(alias = "channel.guest_star_settings.update")]
//! the host preferences for Guest Star have been updated.

use super::*;
/// [`channel.guest_star_settings.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate): the host preferences for Guest Star have been updated.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGuestStarSettingsUpdateBeta {
    /// The broadcaster user ID for the channel you want to receive Guest Star settings update notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID of the moderator or broadcaster of the specified channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelGuestStarSettingsUpdateBeta {
    /// Get notifications for guest star sessions in this channel as a moderator
    pub fn new(
        broadcaster_user_id: impl Into<types::UserId>,
        moderator_user_id: impl Into<types::UserId>,
    ) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
            moderator_user_id: moderator_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelGuestStarSettingsUpdateBeta {
    type Payload = ChannelGuestStarSettingsUpdateBetaPayload;

    const EVENT_TYPE: EventType = EventType::ChannelGuestStarSettingsUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadGuestStar,
        twitch_oauth2::Scope::ChannelManageGuestStar,
        twitch_oauth2::Scope::ModeratorReadGuestStar,
        twitch_oauth2::Scope::ModeratorManageGuestStar,
    )];
    const VERSION: &'static str = "beta";
}

/// [`channel.guest_star_settings.update`](ChannelGuestStarSettingsUpdateBeta) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGuestStarSettingsUpdateBetaPayload {
    /// User ID of the host channel.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster display name
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster login.
    pub broadcaster_user_name: types::DisplayName,

    /// Flag determining if Guest Star moderators have access to control whether a guest is live once assigned to a slot.
    pub is_moderator_send_live_enabled: bool,
    /// Number of slots the Guest Star call interface will allow the host to add to a call.
    pub slot_count: usize,
    /// Flag determining if browser sources subscribed to sessions on this channel should output audio.
    pub is_browser_source_audio_enabled: bool,
    /// This setting determines how the guests within a session should be laid out within a group browser source.
    pub group_layout: GroupLayout,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.guest_star_settings.update",
            "version": "beta",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337",
                "moderator_user_id": "1312"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2023-04-11T10:11:52.123Z"
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "Cool_User",
            "broadcaster_user_login": "cool_user",
            "is_moderator_send_live_enabled": true,
            "slot_count": 5,
            "is_browser_source_audio_enabled": true,
            "group_layout": "tiled"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelGuestStarSettingsUpdateBeta(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert!(notif.is_moderator_send_live_enabled);
    assert_eq!(notif.slot_count, 5);
    assert!(notif.is_browser_source_audio_enabled);
    assert_eq!(notif.group_layout, GroupLayout::Tiled);
}

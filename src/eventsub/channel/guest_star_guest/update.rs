#![doc(alias = "channel.guest_star_guest.update")]
//! the host preferences for Guest Star have been updated.

use super::*;
/// [`channel.guest_star_guest.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate): the host preferences for Guest Star have been updated.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGuestStarGuestUpdateBeta {
    /// The broadcaster user ID for the channel you want to receive Guest Star guest update notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// The user ID of the moderator or broadcaster of the specified channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl ChannelGuestStarGuestUpdateBeta {
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

impl EventSubscription for ChannelGuestStarGuestUpdateBeta {
    type Payload = ChannelGuestStarGuestUpdateBetaPayload;

    const EVENT_TYPE: EventType = EventType::ChannelGuestStarGuestUpdate;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadGuestStar,
        twitch_oauth2::Scope::ChannelManageGuestStar,
        twitch_oauth2::Scope::ModeratorReadGuestStar,
        twitch_oauth2::Scope::ModeratorManageGuestStar,
    )];
    const VERSION: &'static str = "beta";
}

/// [`channel.guest_star_guest.update`](ChannelGuestStarGuestUpdateBeta) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelGuestStarGuestUpdateBetaPayload {
    /// The non-host broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The non-host broadcaster display name.
    pub broadcaster_user_login: types::UserName,
    /// The non-host broadcaster login.
    pub broadcaster_user_name: types::DisplayName,

    /// ID representing the unique session that was started.
    pub session_id: types::GuestStarSessionId,

    /// The user ID of the moderator who updated the guest’s state (could be the host).
    ///
    /// [None] if the update was performed by the guest.
    pub moderator_user_id: Option<types::UserId>,
    /// The moderator display name.
    ///
    /// [None] if the update was performed by the guest.
    pub moderator_user_login: Option<types::UserName>,
    /// The moderator login.
    ///
    /// [None] if the update was performed by the guest.
    pub moderator_user_name: Option<types::DisplayName>,

    /// The user ID of the guest who transitioned states in the session.
    ///
    /// [None] if the slot is now empty.
    pub guest_user_id: Option<types::UserId>,
    /// The guest display name.
    ///
    /// [None] if the slot is now empty.
    pub guest_user_login: Option<types::UserName>,
    /// The guest login.
    ///
    /// [None] if the slot is now empty.
    pub guest_user_name: Option<types::DisplayName>,

    /// The ID of the slot assignment the guest is assigned to.
    ///
    /// [None] if the guest is in the [Invited][GuestState::Invited], [Removed][GuestState::Removed], [Ready][GuestState::Ready], or [Accepted][GuestState::Accepted] state.
    pub slot_id: Option<types::GuestStarSlotId>,
    /// The current state of the user after the update has taken place.
    ///
    /// [None] if the slot is now empty.
    pub state: Option<GuestState>,

    /// User ID of the host channel.
    pub host_user_id: types::UserId,
    /// The host display name.
    pub host_user_login: types::UserName,
    /// The host login.
    pub host_user_name: types::DisplayName,

    /// Flag that signals whether the host is allowing the slot’s video to be seen by participants within the session.
    ///
    /// [None] if the guest is not slotted.
    pub host_video_enabled: Option<bool>,
    /// Flag that signals whether the host is allowing the slot’s audio to be heard by participants within the session.
    ///
    /// [None] if the guest is not slotted.
    pub host_audio_enabled: Option<bool>,
    /// Value between 0-100 that represents the slot’s audio level as heard by participants within the session.
    ///
    /// [None] if the guest is not slotted.
    pub host_volume: Option<u8>,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.guest_star_guest.update",
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
            "created_at": "2023-04-11T10:11:32.123Z"
        },
        "event": {
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "Cool_User",
            "broadcaster_user_login": "cool_user",
            "session_id": "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "moderator_user_id": "1312",
            "moderator_user_name": "Cool_Mod",
            "moderator_user_login": "cool_mod",
            "guest_user_id": "1234",
            "guest_user_name": "Cool_Guest",
            "guest_user_login": "cool_guest",
            "slot_id": "1",
            "state": "live",
            "host_user_id": "4242",
            "host_user_name": "A_host",
            "host_user_login": "a_host",
            "host_video_enabled": true,
            "host_audio_enabled": true,
            "host_volume": 100
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelGuestStarGuestUpdateBeta(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid guest type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert_eq!(notif.moderator_user_id.unwrap().as_str(), "1312");
    assert_eq!(notif.session_id.as_str(), "2KFRQbFtpmfyD3IevNRnCzOPRJI");
    assert_eq!(notif.guest_user_id.unwrap().as_str(), "1234");
    assert_eq!(notif.host_user_id.as_str(), "4242");
    assert_eq!(notif.slot_id.unwrap().as_str(), "1");
    assert_eq!(notif.state, Some(GuestState::Live));
    assert_eq!(notif.host_video_enabled, Some(true));
    assert_eq!(notif.host_audio_enabled, Some(true));
    assert_eq!(notif.host_volume, Some(100));
}

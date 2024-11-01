#![doc(alias = "mod")]
//! Helix endpoints regarding moderation
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Moderation 🟢 23/23</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Check AutoMod Status](https://dev.twitch.tv/docs/api/reference#check-automod-status) | - | [`check_automod_status`] |
//! | [Manage Held AutoMod Messages](https://dev.twitch.tv/docs/api/reference#manage-held-automod-messages) | - | [`manage_held_automod_messages`] |
//! | [Get AutoMod Settings](https://dev.twitch.tv/docs/api/reference#get-automod-settings) | - | [`get_automod_settings`] |
//! | [Update AutoMod Settings](https://dev.twitch.tv/docs/api/reference#update-automod-settings) | - | [`update_automod_settings`] |
//! | [Get Banned Users](https://dev.twitch.tv/docs/api/reference#get-banned-users) | [`HelixClient::get_banned_users_in_channel_from_id`](crate::helix::HelixClient::get_banned_users_in_channel_from_id) | [`get_banned_users`] |
//! | [Ban User](https://dev.twitch.tv/docs/api/reference#ban-user) | [`HelixClient::ban_user`](crate::helix::HelixClient::ban_user) | [`ban_user`] |
//! | [Unban User](https://dev.twitch.tv/docs/api/reference#unban-user) | [`HelixClient::unban_user`](crate::helix::HelixClient::unban_user) | [`unban_user`] |
//! | [Get Unban Requests](https://dev.twitch.tv/docs/api/reference#get-unban-requests) | [`HelixClient::get_unban_requests`](crate::helix::HelixClient::get_unban_requests) | [`get_unban_requests`] |
//! | [Resolve Unban Requests](https://dev.twitch.tv/docs/api/reference#resolve-unban-requests) | - | [`resolve_unban_request`] |
//! | [Get Blocked Terms](https://dev.twitch.tv/docs/api/reference#get-blocked-terms) | - | [`get_blocked_terms`] |
//! | [Add Blocked Term](https://dev.twitch.tv/docs/api/reference#add-blocked-term) | - | [`add_blocked_term`] |
//! | [Remove Blocked Term](https://dev.twitch.tv/docs/api/reference#remove-blocked-term) | - | [`remove_blocked_term`] |
//! | [Delete Chat Messages](https://dev.twitch.tv/docs/api/reference#delete-chat-messages) | [`HelixClient::delete_chat_message`](crate::helix::HelixClient::delete_chat_message) | [`delete_chat_messages`] |
//! | [Get Moderated Channels](https://dev.twitch.tv/docs/api/reference#get-moderated-channels) | [`HelixClient::get_moderated_channels`](crate::helix::HelixClient::get_moderated_channels) | [`get_moderated_channels`] |
//! | [Get Moderators](https://dev.twitch.tv/docs/api/reference#get-moderators) | [`HelixClient::get_moderators_in_channel_from_id`](crate::helix::HelixClient::get_moderators_in_channel_from_id) | [`get_moderators`] |
//! | [Add Channel Moderator](https://dev.twitch.tv/docs/api/reference#add-channel-moderator) | [`HelixClient::add_channel_moderator`](crate::helix::HelixClient::add_channel_moderator) | [`add_channel_moderator`] |
//! | [Remove Channel Moderator](https://dev.twitch.tv/docs/api/reference#remove-channel-moderator) | [`HelixClient::remove_channel_moderator`](crate::helix::HelixClient::remove_channel_moderator) | [`remove_channel_moderator`] |
//! | [Get VIPs](https://dev.twitch.tv/docs/api/reference#get-vips) | [`HelixClient::get_vips_in_channel`](crate::helix::HelixClient::get_vips_in_channel) | [`get_vips`] |
//! | [Add Channel VIP](https://dev.twitch.tv/docs/api/reference#add-channel-vip) | [`HelixClient::add_channel_vip`](crate::helix::HelixClient::add_channel_vip) | [`add_channel_vip`] |
//! | [Remove Channel VIP](https://dev.twitch.tv/docs/api/reference#remove-channel-vip) | [`HelixClient::remove_channel_vip`](crate::helix::HelixClient::remove_channel_vip) | [`remove_channel_vip`] |
//! | [Update Shield Mode Status](https://dev.twitch.tv/docs/api/reference#update-shield-mode-status) | - | [`update_shield_mode_status`] |
//! | [Get Shield Mode Status](https://dev.twitch.tv/docs/api/reference#get-shield-mode-status) | - | [`get_shield_mode_status`] |
//! | [Warn Chat User](https://dev.twitch.tv/docs/api/reference#warn-chat-user) | [`HelixClient::warn_chat_user`](crate::helix::HelixClient::warn_chat_user) | [`warn_chat_user`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->

use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod add_blocked_term;
pub mod add_channel_moderator;
pub mod ban_user;
pub mod check_automod_status;
pub mod delete_chat_messages;
pub mod get_automod_settings;
pub mod get_banned_users;
pub mod get_blocked_terms;
pub mod get_moderated_channels;
pub mod get_moderators;
pub mod get_shield_mode_status;
pub mod get_unban_requests;
pub mod manage_held_automod_messages;
pub mod remove_blocked_term;
pub mod remove_channel_moderator;
pub mod resolve_unban_request;
pub mod unban_user;
pub mod update_automod_settings;
pub mod update_shield_mode_status;
#[cfg(feature = "beta")]
pub mod warn_chat_user;

#[doc(inline)]
pub use add_blocked_term::{AddBlockedTermBody, AddBlockedTermRequest};
#[doc(inline)]
pub use add_channel_moderator::{AddChannelModeratorRequest, AddChannelModeratorResponse};
#[doc(inline)]
pub use ban_user::{BanUser, BanUserBody, BanUserRequest};
#[doc(inline)]
pub use check_automod_status::{
    CheckAutoModStatus, CheckAutoModStatusBody, CheckAutoModStatusRequest,
};
#[doc(inline)]
pub use delete_chat_messages::{DeleteChatMessagesRequest, DeleteChatMessagesResponse};
#[doc(inline)]
pub use get_automod_settings::{AutoModSettings, GetAutoModSettingsRequest};
#[doc(inline)]
pub use get_banned_users::{BannedUser, GetBannedUsersRequest};
#[doc(inline)]
pub use get_moderated_channels::{GetModeratedChannelsRequest, ModeratedChannel};
#[doc(inline)]
pub use get_moderators::{GetModeratorsRequest, Moderator};
#[doc(inline)]
pub use get_shield_mode_status::{GetShieldModeStatusRequest, LastShieldMode, ShieldModeStatus};
#[doc(inline)]
pub use get_unban_requests::{GetUnbanRequestsRequest, UnbanRequest, UnbanRequestStatus};
#[doc(inline)]
pub use manage_held_automod_messages::{
    AutoModAction, ManageHeldAutoModMessages, ManageHeldAutoModMessagesBody,
    ManageHeldAutoModMessagesRequest,
};
#[doc(inline)]
pub use remove_blocked_term::{RemoveBlockedTerm, RemoveBlockedTermRequest};
#[doc(inline)]
pub use remove_channel_moderator::{RemoveChannelModeratorRequest, RemoveChannelModeratorResponse};
#[doc(inline)]
pub use resolve_unban_request::ResolveUnbanRequest;
#[doc(inline)]
pub use unban_user::{UnbanUserRequest, UnbanUserResponse};
#[doc(inline)]
pub use update_automod_settings::{
    UpdateAutoModSettingsBody, UpdateAutoModSettingsIndividual, UpdateAutoModSettingsRequest,
};
#[cfg(feature = "beta")]
#[doc(inline)]
pub use warn_chat_user::{WarnChatUser, WarnChatUserBody, WarnChatUserRequest};
// endpoints defined in other modules.
pub use super::channels::add_channel_vip::{self, AddChannelVipRequest, AddChannelVipResponse};
pub use super::channels::get_vips::{self, GetVipsRequest, Vip};
pub use super::channels::remove_channel_vip::{
    self, RemoveChannelVipRequest, RemoveChannelVipResponse,
};

/// A blocked term in automod
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BlockedTerm {
    /// The broadcaster that owns the list of blocked terms.
    pub broadcaster_id: types::UserId,
    /// The UTC date and time (in RFC3339 format) of when the term was blocked.
    pub created_at: types::Timestamp,
    /// The UTC date and time (in RFC3339 format) of when the blocked term is set to expire. After the block expires, user’s will be able to use the term in the broadcaster’s chat room.
    ///
    /// This field is null if the term was added manually or was permanently blocked by AutoMod.
    pub expires_at: Option<types::Timestamp>,
    /// An ID that uniquely identifies this blocked term.
    pub id: types::BlockedTermId,
    /// The moderator that blocked the word or phrase from being used in the broadcaster’s chat room.
    pub moderator_id: types::UserId,
    /// The blocked word or phrase.
    pub text: String,
    /// The UTC date and time (in RFC3339 format) of when the term was updated.
    ///
    /// When the term is added, this timestamp is the same as created_at. The timestamp changes as AutoMod continues to deny the term.
    pub updated_at: types::Timestamp,
}

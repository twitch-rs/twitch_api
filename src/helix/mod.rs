//! Helix endpoints or the [New Twitch API](https://dev.twitch.tv/docs/api)
//!
//! [`HelixClient`] provides convenience function for sending requests as described on [the crate documentation](crate).
//!
//! ### Manual Usage
//!
//! Aside from using [`HelixClient`], you can decide to use this library without any specific client implementation.
//!
//! ```rust
//! use twitch_api::{helix::{self, Request, RequestGet, users::{GetUsersRequest, User}}, types};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//!
//! let logins: &[&types::UserNameRef] = &["justintv123".into()];
//! let request = GetUsersRequest::logins(logins);
//!
//! // Send it however you want
//! // Create a [`http::Response<hyper::body::Bytes>`] with RequestGet::create_request, which takes an access token and a client_id
//! let response = send_http_request(request.create_request("accesstoken", "client_id")?)?;
//!
//! // then parse the response
//! let uri = request.get_uri()?;
//! let user: helix::Response<_, Vec<User>> = GetUsersRequest::parse_response(Some(request), &uri, response)?;
//! println!("{:#?}", user);
//! # Ok(())
//! # }
//! # fn send_http_request(_: http::Request<hyper::body::Bytes>) -> Result<http::Response<hyper::body::Bytes>,&'static str> {
//! # Ok(http::Response::builder().body(r#"{"data":[{"id":"141981764","login":"twitchdev","display_name":"TwitchDev","type":"","broadcaster_type":"partner","description":"Supportingthird-partydevelopersbuildingTwitchintegrationsfromchatbotstogameintegrations.","profile_image_url":"https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png","offline_image_url":"https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png","view_count":5980557,"email":"not-real@email.com","created_at":"2016-12-14T20:32:28.894263Z"}]}"#.as_bytes().to_owned().into()).unwrap())
//! # }
//! ```
//!
//! ## Implemented Endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details><summary style="cursor: pointer">Ads 🟢 3/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Start Commercial](https://dev.twitch.tv/docs/api/reference#start-commercial) | - | [`channels::start_commercial`] |
//! | [Get Ad Schedule](https://dev.twitch.tv/docs/api/reference#get-ad-schedule) | - | [`channels::get_ad_schedule`] |
//! | [Snooze Next Ad](https://dev.twitch.tv/docs/api/reference#snooze-next-ad) | - | [`channels::snooze_next_ad`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Analytics 🔴 0/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Extension Analytics](https://dev.twitch.tv/docs/api/reference#get-extension-analytics) | - | - |
//! | [Get Game Analytics](https://dev.twitch.tv/docs/api/reference#get-game-analytics) | - | - |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Bits 🟡 2/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Bits Leaderboard](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard) | - | [`bits::get_bits_leaderboard`] |
//! | [Get Cheermotes](https://dev.twitch.tv/docs/api/reference#get-cheermotes) | - | [`bits::get_cheermotes`] |
//! | [Get Extension Transactions](https://dev.twitch.tv/docs/api/reference#get-extension-transactions) | - | - |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">CCLs 🟢 1/1</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Content Classification Labels](https://dev.twitch.tv/docs/api/reference#get-content-classification-labels) | [`HelixClient::get_content_classification_labels`], [`HelixClient::get_content_classification_labels_for_locale`] | [`ccls::get_content_classification_labels`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Channel Points 🟢 6/6</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Create Custom Rewards](https://dev.twitch.tv/docs/api/reference#create-custom-rewards) | - | [`points::create_custom_rewards`] |
//! | [Delete Custom Reward](https://dev.twitch.tv/docs/api/reference#delete-custom-reward) | - | [`points::delete_custom_reward`] |
//! | [Get Custom Reward](https://dev.twitch.tv/docs/api/reference#get-custom-reward) | [`HelixClient::get_custom_rewards`] | [`points::get_custom_reward`] |
//! | [Get Custom Reward Redemption](https://dev.twitch.tv/docs/api/reference#get-custom-reward-redemption) | - | [`points::get_custom_reward_redemption`] |
//! | [Update Custom Reward](https://dev.twitch.tv/docs/api/reference#update-custom-reward) | - | [`points::update_custom_reward`] |
//! | [Update Redemption Status](https://dev.twitch.tv/docs/api/reference#update-redemption-status) | - | [`points::update_redemption_status`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Channels 🟢 5/5</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information) | - | [`channels::get_channel_information`] |
//! | [Modify Channel Information](https://dev.twitch.tv/docs/api/reference#modify-channel-information) | - | [`channels::modify_channel_information`] |
//! | [Get Channel Editors](https://dev.twitch.tv/docs/api/reference#get-channel-editors) | - | [`channels::get_channel_editors`] |
//! | [Get Followed Channels](https://dev.twitch.tv/docs/api/reference#get-followed-channels) | [`HelixClient::get_followed_channels`] | [`channels::get_followed_channels`] |
//! | [Get Channel Followers](https://dev.twitch.tv/docs/api/reference#get-channel-followers) | - | [`channels::get_channel_followers`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Charity 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Charity Campaign](https://dev.twitch.tv/docs/api/reference#get-charity-campaign) | - | [`charity::get_charity_campaign`] |
//! | [Get Charity Campaign Donations](https://dev.twitch.tv/docs/api/reference#get-charity-campaign-donations) | - | [`charity::get_charity_campaign_donations`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Chat 🟢 15/15</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Chatters](https://dev.twitch.tv/docs/api/reference#get-chatters) | [`HelixClient::get_chatters`] | [`chat::get_chatters`] |
//! | [Get Channel Emotes](https://dev.twitch.tv/docs/api/reference#get-channel-emotes) | [`HelixClient::get_channel_emotes_from_id`], [`HelixClient::get_channel_emotes_from_login`] | [`chat::get_channel_emotes`] |
//! | [Get Global Emotes](https://dev.twitch.tv/docs/api/reference#get-global-emotes) | [`HelixClient::get_global_emotes`] | [`chat::get_global_emotes`] |
//! | [Get Emote Sets](https://dev.twitch.tv/docs/api/reference#get-emote-sets) | [`HelixClient::get_emote_sets`] | [`chat::get_emote_sets`] |
//! | [Get Channel Chat Badges](https://dev.twitch.tv/docs/api/reference#get-channel-chat-badges) | - | [`chat::get_channel_chat_badges`] |
//! | [Get Global Chat Badges](https://dev.twitch.tv/docs/api/reference#get-global-chat-badges) | - | [`chat::get_global_chat_badges`] |
//! | [Get Chat Settings](https://dev.twitch.tv/docs/api/reference#get-chat-settings) | [`HelixClient::get_chat_settings`] | [`chat::get_chat_settings`] |
//! | [Get Shared Chat Session](https://dev.twitch.tv/docs/api/reference#get-shared-chat-session) | [`HelixClient::get_shared_chat_session`] | [`chat::get_shared_chat_session`] |
//! | [Get User Emotes](https://dev.twitch.tv/docs/api/reference#get-user-emotes) | [`HelixClient::get_user_emotes`], [`HelixClient::get_user_emotes_in_channel`] | [`chat::get_user_emotes`] |
//! | [Update Chat Settings](https://dev.twitch.tv/docs/api/reference#update-chat-settings) | - | [`chat::update_chat_settings`] |
//! | [Send Chat Announcement](https://dev.twitch.tv/docs/api/reference#send-chat-announcement) | [`HelixClient::send_chat_announcement`] | [`chat::send_chat_announcement`] |
//! | [Send a Shoutout](https://dev.twitch.tv/docs/api/reference#send-a-shoutout) | - | [`chat::send_a_shoutout`] |
//! | [Send Chat Message](https://dev.twitch.tv/docs/api/reference#send-chat-message) | [`HelixClient::send_chat_message`], [`HelixClient::send_chat_message_reply`] | [`chat::send_chat_message`] |
//! | [Get User Chat Color](https://dev.twitch.tv/docs/api/reference#get-user-chat-color) | [`HelixClient::get_user_chat_color`], [`HelixClient::get_users_chat_colors`] | [`chat::get_user_chat_color`] |
//! | [Update User Chat Color](https://dev.twitch.tv/docs/api/reference#update-user-chat-color) | [`HelixClient::update_user_chat_color`] | [`chat::update_user_chat_color`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Clips 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Create Clip](https://dev.twitch.tv/docs/api/reference#create-clip) | - | [`clips::create_clip`] |
//! | [Get Clips](https://dev.twitch.tv/docs/api/reference#get-clips) | - | [`clips::get_clips`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Conduits 🟢 6/6</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Conduits](https://dev.twitch.tv/docs/api/reference#get-conduits) | [`HelixClient::get_conduits`] | [`eventsub::get_conduits`] |
//! | [Create Conduits](https://dev.twitch.tv/docs/api/reference#create-conduits) | [`HelixClient::create_conduit`] | [`eventsub::create_conduit`] |
//! | [Update Conduits](https://dev.twitch.tv/docs/api/reference#update-conduits) | [`HelixClient::update_conduit`] | [`eventsub::update_conduit`] |
//! | [Delete Conduit](https://dev.twitch.tv/docs/api/reference#delete-conduit) | [`HelixClient::delete_conduit`] | [`eventsub::delete_conduit`] |
//! | [Get Conduit Shards](https://dev.twitch.tv/docs/api/reference#get-conduit-shards) | [`HelixClient::get_conduit_shards`] | [`eventsub::get_conduit_shards`] |
//! | [Update Conduit Shards](https://dev.twitch.tv/docs/api/reference#update-conduit-shards) | [`HelixClient::update_conduit_shards`] | [`eventsub::update_conduit_shards`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Entitlements 🔴 0/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Drops Entitlements](https://dev.twitch.tv/docs/api/reference#get-drops-entitlements) | - | - |
//! | [Update Drops Entitlements](https://dev.twitch.tv/docs/api/reference#update-drops-entitlements) | - | - |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">EventSub 🟢 3/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Create EventSub Subscription](https://dev.twitch.tv/docs/api/reference#create-eventsub-subscription) | [`HelixClient::create_eventsub_subscription`] | [`eventsub::create_eventsub_subscription`] |
//! | [Delete EventSub Subscription](https://dev.twitch.tv/docs/api/reference#delete-eventsub-subscription) | [`HelixClient::delete_eventsub_subscription`] | [`eventsub::delete_eventsub_subscription`] |
//! | [Get EventSub Subscriptions](https://dev.twitch.tv/docs/api/reference#get-eventsub-subscriptions) | [`HelixClient::get_eventsub_subscriptions`] | [`eventsub::get_eventsub_subscriptions`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Extensions 🔴 0/12</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Extension Configuration Segment](https://dev.twitch.tv/docs/api/reference#get-extension-configuration-segment) | - | - |
//! | [Set Extension Configuration Segment](https://dev.twitch.tv/docs/api/reference#set-extension-configuration-segment) | - | - |
//! | [Set Extension Required Configuration](https://dev.twitch.tv/docs/api/reference#set-extension-required-configuration) | - | - |
//! | [Send Extension PubSub Message](https://dev.twitch.tv/docs/api/reference#send-extension-pubsub-message) | - | - |
//! | [Get Extension Live Channels](https://dev.twitch.tv/docs/api/reference#get-extension-live-channels) | - | - |
//! | [Get Extension Secrets](https://dev.twitch.tv/docs/api/reference#get-extension-secrets) | - | - |
//! | [Create Extension Secret](https://dev.twitch.tv/docs/api/reference#create-extension-secret) | - | - |
//! | [Send Extension Chat Message](https://dev.twitch.tv/docs/api/reference#send-extension-chat-message) | - | - |
//! | [Get Extensions](https://dev.twitch.tv/docs/api/reference#get-extensions) | - | - |
//! | [Get Released Extensions](https://dev.twitch.tv/docs/api/reference#get-released-extensions) | - | - |
//! | [Get Extension Bits Products](https://dev.twitch.tv/docs/api/reference#get-extension-bits-products) | - | - |
//! | [Update Extension Bits Product](https://dev.twitch.tv/docs/api/reference#update-extension-bits-product) | - | - |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Games 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Top Games](https://dev.twitch.tv/docs/api/reference#get-top-games) | - | [`games::get_top_games`] |
//! | [Get Games](https://dev.twitch.tv/docs/api/reference#get-games) | [`HelixClient::get_games_by_id`] | [`games::get_games`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Goals 🟢 1/1</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Creator Goals](https://dev.twitch.tv/docs/api/reference#get-creator-goals) | - | [`goals::get_creator_goals`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Guest Star 🔴 0/12</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Channel Guest Star Settings](https://dev.twitch.tv/docs/api/reference#get-channel-guest-star-settings) | - | - |
//! | [Update Channel Guest Star Settings](https://dev.twitch.tv/docs/api/reference#update-channel-guest-star-settings) | - | - |
//! | [Get Guest Star Session](https://dev.twitch.tv/docs/api/reference#get-guest-star-session) | - | - |
//! | [Create Guest Star Session](https://dev.twitch.tv/docs/api/reference#create-guest-star-session) | - | - |
//! | [End Guest Star Session](https://dev.twitch.tv/docs/api/reference#end-guest-star-session) | - | - |
//! | [Get Guest Star Invites](https://dev.twitch.tv/docs/api/reference#get-guest-star-invites) | - | - |
//! | [Send Guest Star Invite](https://dev.twitch.tv/docs/api/reference#send-guest-star-invite) | - | - |
//! | [Delete Guest Star Invite](https://dev.twitch.tv/docs/api/reference#delete-guest-star-invite) | - | - |
//! | [Assign Guest Star Slot](https://dev.twitch.tv/docs/api/reference#assign-guest-star-slot) | - | - |
//! | [Update Guest Star Slot](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot) | - | - |
//! | [Delete Guest Star Slot](https://dev.twitch.tv/docs/api/reference#delete-guest-star-slot) | - | - |
//! | [Update Guest Star Slot Settings](https://dev.twitch.tv/docs/api/reference#update-guest-star-slot-settings) | - | - |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Hype Train 🟡 1/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Hype Train Events](https://dev.twitch.tv/docs/api/reference#get-hype-train-events) | - | [`hypetrain::get_hypetrain_events`] |
//! | [Get Hype Train Status](https://dev.twitch.tv/docs/api/reference#get-hype-train-status) | - | - |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Moderation 🟢 23/23</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Check AutoMod Status](https://dev.twitch.tv/docs/api/reference#check-automod-status) | - | [`moderation::check_automod_status`] |
//! | [Manage Held AutoMod Messages](https://dev.twitch.tv/docs/api/reference#manage-held-automod-messages) | - | [`moderation::manage_held_automod_messages`] |
//! | [Get AutoMod Settings](https://dev.twitch.tv/docs/api/reference#get-automod-settings) | - | [`moderation::get_automod_settings`] |
//! | [Update AutoMod Settings](https://dev.twitch.tv/docs/api/reference#update-automod-settings) | - | [`moderation::update_automod_settings`] |
//! | [Get Banned Users](https://dev.twitch.tv/docs/api/reference#get-banned-users) | [`HelixClient::get_banned_users_in_channel_from_id`] | [`moderation::get_banned_users`] |
//! | [Ban User](https://dev.twitch.tv/docs/api/reference#ban-user) | [`HelixClient::ban_user`] | [`moderation::ban_user`] |
//! | [Unban User](https://dev.twitch.tv/docs/api/reference#unban-user) | [`HelixClient::unban_user`] | [`moderation::unban_user`] |
//! | [Get Unban Requests](https://dev.twitch.tv/docs/api/reference#get-unban-requests) | [`HelixClient::get_unban_requests`] | [`moderation::get_unban_requests`] |
//! | [Resolve Unban Requests](https://dev.twitch.tv/docs/api/reference#resolve-unban-requests) | - | [`moderation::resolve_unban_request`] |
//! | [Get Blocked Terms](https://dev.twitch.tv/docs/api/reference#get-blocked-terms) | - | [`moderation::get_blocked_terms`] |
//! | [Add Blocked Term](https://dev.twitch.tv/docs/api/reference#add-blocked-term) | - | [`moderation::add_blocked_term`] |
//! | [Remove Blocked Term](https://dev.twitch.tv/docs/api/reference#remove-blocked-term) | - | [`moderation::remove_blocked_term`] |
//! | [Delete Chat Messages](https://dev.twitch.tv/docs/api/reference#delete-chat-messages) | [`HelixClient::delete_chat_message`] | [`moderation::delete_chat_messages`] |
//! | [Get Moderated Channels](https://dev.twitch.tv/docs/api/reference#get-moderated-channels) | [`HelixClient::get_moderated_channels`] | [`moderation::get_moderated_channels`] |
//! | [Get Moderators](https://dev.twitch.tv/docs/api/reference#get-moderators) | [`HelixClient::get_moderators_in_channel_from_id`] | [`moderation::get_moderators`] |
//! | [Add Channel Moderator](https://dev.twitch.tv/docs/api/reference#add-channel-moderator) | [`HelixClient::add_channel_moderator`] | [`moderation::add_channel_moderator`] |
//! | [Remove Channel Moderator](https://dev.twitch.tv/docs/api/reference#remove-channel-moderator) | [`HelixClient::remove_channel_moderator`] | [`moderation::remove_channel_moderator`] |
//! | [Get VIPs](https://dev.twitch.tv/docs/api/reference#get-vips) | [`HelixClient::get_vips_in_channel`] | [`channels::get_vips`] |
//! | [Add Channel VIP](https://dev.twitch.tv/docs/api/reference#add-channel-vip) | [`HelixClient::add_channel_vip`] | [`channels::add_channel_vip`] |
//! | [Remove Channel VIP](https://dev.twitch.tv/docs/api/reference#remove-channel-vip) | [`HelixClient::remove_channel_vip`] | [`channels::remove_channel_vip`] |
//! | [Update Shield Mode Status](https://dev.twitch.tv/docs/api/reference#update-shield-mode-status) | - | [`moderation::update_shield_mode_status`] |
//! | [Get Shield Mode Status](https://dev.twitch.tv/docs/api/reference#get-shield-mode-status) | - | [`moderation::get_shield_mode_status`] |
//! | [Warn Chat User](https://dev.twitch.tv/docs/api/reference#warn-chat-user) | [`HelixClient::warn_chat_user`] | [`moderation::warn_chat_user`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Polls 🟢 3/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Polls](https://dev.twitch.tv/docs/api/reference#get-polls) | - | [`polls::get_polls`] |
//! | [Create Poll](https://dev.twitch.tv/docs/api/reference#create-poll) | - | [`polls::create_poll`] |
//! | [End Poll](https://dev.twitch.tv/docs/api/reference#end-poll) | - | [`polls::end_poll`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Predictions 🟢 3/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Predictions](https://dev.twitch.tv/docs/api/reference#get-predictions) | - | [`predictions::get_predictions`] |
//! | [Create Prediction](https://dev.twitch.tv/docs/api/reference#create-prediction) | - | [`predictions::create_prediction`] |
//! | [End Prediction](https://dev.twitch.tv/docs/api/reference#end-prediction) | - | [`predictions::end_prediction`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Raids 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Start a raid](https://dev.twitch.tv/docs/api/reference#start-a-raid) | [`HelixClient::start_a_raid`] | [`raids::start_a_raid`] |
//! | [Cancel a raid](https://dev.twitch.tv/docs/api/reference#cancel-a-raid) | [`HelixClient::cancel_a_raid`] | [`raids::cancel_a_raid`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Schedule 🟡 5/6</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Channel Stream Schedule](https://dev.twitch.tv/docs/api/reference#get-channel-stream-schedule) | - | [`schedule::get_channel_stream_schedule`] |
//! | [Get Channel iCalendar](https://dev.twitch.tv/docs/api/reference#get-channel-icalendar) | - | - |
//! | [Update Channel Stream Schedule](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule) | - | [`schedule::update_channel_stream_schedule`] |
//! | [Create Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment) | - | [`schedule::create_channel_stream_schedule_segment`] |
//! | [Update Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#update-channel-stream-schedule-segment) | - | [`schedule::update_channel_stream_schedule_segment`] |
//! | [Delete Channel Stream Schedule Segment](https://dev.twitch.tv/docs/api/reference#delete-channel-stream-schedule-segment) | - | [`schedule::delete_channel_stream_schedule_segment`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Search 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Search Categories](https://dev.twitch.tv/docs/api/reference#search-categories) | [`HelixClient::search_categories`] | [`search::search_categories`] |
//! | [Search Channels](https://dev.twitch.tv/docs/api/reference#search-channels) | [`HelixClient::search_channels`] | [`search::search_channels`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Streams 🟢 5/5</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Stream Key](https://dev.twitch.tv/docs/api/reference#get-stream-key) | [`HelixClient::get_stream_key`] | [`streams::get_stream_key`] |
//! | [Get Streams](https://dev.twitch.tv/docs/api/reference#get-streams) | [`HelixClient::get_streams_from_ids`], [`HelixClient::get_streams_from_logins`] | [`streams::get_streams`] |
//! | [Get Followed Streams](https://dev.twitch.tv/docs/api/reference#get-followed-streams) | [`HelixClient::get_followed_streams`] | [`streams::get_followed_streams`] |
//! | [Create Stream Marker](https://dev.twitch.tv/docs/api/reference#create-stream-marker) | [`HelixClient::create_stream_marker`] | [`streams::create_stream_marker`] |
//! | [Get Stream Markers](https://dev.twitch.tv/docs/api/reference#get-stream-markers) | - | [`streams::get_stream_markers`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Subscriptions 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Broadcaster Subscriptions](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions) | [`HelixClient::get_broadcaster_subscriptions`] | [`subscriptions::get_broadcaster_subscriptions`] |
//! | [Check User Subscription](https://dev.twitch.tv/docs/api/reference#check-user-subscription) | - | [`subscriptions::check_user_subscription`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Tags 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get All Stream Tags](https://dev.twitch.tv/docs/api/reference#get-all-stream-tags) | - | [`tags::get_all_stream_tags`] |
//! | [Get Stream Tags](https://dev.twitch.tv/docs/api/reference#get-stream-tags) | - | [`streams::get_stream_tags`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Teams 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Channel Teams](https://dev.twitch.tv/docs/api/reference#get-channel-teams) | - | [`teams::get_channel_teams`] |
//! | [Get Teams](https://dev.twitch.tv/docs/api/reference#get-teams) | - | [`teams::get_teams`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Users 🟢 8/8</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Users](https://dev.twitch.tv/docs/api/reference#get-users) | [`HelixClient::get_user_from_id`], [`HelixClient::get_user_from_login`], [`HelixClient::get_users_from_ids`], [`HelixClient::get_users_from_logins`] | [`users::get_users`] |
//! | [Update User](https://dev.twitch.tv/docs/api/reference#update-user) | [`HelixClient::update_user_description`] | [`users::update_user`] |
//! | [Get User Block List](https://dev.twitch.tv/docs/api/reference#get-user-block-list) | - | [`users::get_user_block_list`] |
//! | [Block User](https://dev.twitch.tv/docs/api/reference#block-user) | [`HelixClient::block_user`] | [`users::block_user`] |
//! | [Unblock User](https://dev.twitch.tv/docs/api/reference#unblock-user) | [`HelixClient::unblock_user`] | [`users::unblock_user`] |
//! | [Get User Extensions](https://dev.twitch.tv/docs/api/reference#get-user-extensions) | [`HelixClient::get_user_extensions`] | [`users::get_user_extensions`] |
//! | [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions) | [`HelixClient::get_user_active_extensions`] | [`users::get_user_active_extensions`] |
//! | [Update User Extensions](https://dev.twitch.tv/docs/api/reference#update-user-extensions) | - | [`users::update_user_extensions`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Videos 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Videos](https://dev.twitch.tv/docs/api/reference#get-videos) | - | [`videos::get_videos`] |
//! | [Delete Videos](https://dev.twitch.tv/docs/api/reference#delete-videos) | - | [`videos::delete_videos`] |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer">Whispers 🟢 1/1</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Send Whisper](https://dev.twitch.tv/docs/api/reference#send-whisper) | [`HelixClient::send_whisper`] | [`whispers::send_whisper`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->

use serde_derive::Deserialize;
#[doc(no_inline)]
#[cfg(feature = "twitch_oauth2")]
pub use twitch_oauth2::Scope;
#[cfg(feature = "twitch_oauth2")]
use twitch_oauth2::TwitchToken;

#[cfg(feature = "client")]
pub mod client;
mod endpoints;
pub mod request;
pub mod response;

#[cfg(feature = "client")]
#[doc(inline)]
pub use client::{client_ext::make_stream, *};
pub use endpoints::*;
#[cfg(feature = "client")]
#[doc(inline)]
pub use request::errors::ClientRequestError;
#[doc(inline)]
pub use request::errors::{
    CreateRequestError, HelixRequestDeleteError, HelixRequestGetError, HelixRequestPatchError,
    HelixRequestPostError, HelixRequestPutError, InvalidUri, SerializeError,
};
#[doc(inline)]
pub use request::{Request, RequestDelete, RequestGet, RequestPatch, RequestPost, RequestPut};
#[doc(inline)]
pub use response::Response;

pub(crate) mod ser;
pub(crate) use crate::deserialize_default_from_null;
use crate::parse_json;
pub(crate) use request::parse_single_return;

#[derive(PartialEq, Deserialize, Debug)]
struct InnerResponse<D> {
    data: D,
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    #[serde(default)]
    pagination: Pagination,
    #[serde(default)]
    total: Option<i64>,
    #[serde(default, flatten)]
    other: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Deserialize, Debug)]
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
struct CustomInnerResponse<'a> {
    #[serde(borrow)]
    data: &'a serde_json::value::RawValue,
    #[serde(default)]
    pagination: Pagination,
    #[serde(default)]
    total: Option<i64>,
    // FIXME: There is an issue with RawValue on flatten maps. https://github.com/serde-rs/json/issues/599
    #[serde(flatten, default)]
    other: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize, Clone, Debug)]
struct HelixRequestError {
    error: String,
    status: u16,
    message: String,
}

/// Deserialize 0, "0" or "" as None
fn deserialize_none_from_empty_or_zero_string<'de, D, S>(
    deserializer: D,
) -> Result<Option<S>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: serde::Deserialize<'de>, {
    use serde::de::IntoDeserializer;
    struct Inner<S>(std::marker::PhantomData<S>);
    impl<'de, S> serde::de::Visitor<'de> for Inner<S>
    where S: serde::Deserialize<'de>
    {
        type Value = Option<S>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("any string or integer")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: serde::de::Error {
            match value {
                "" => Ok(None),
                "0" => Ok(None),
                v => S::deserialize(v.into_deserializer()).map(Some),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where E: serde::de::Error {
            match &*value {
                "" => Ok(None),
                "0" => Ok(None),
                v => S::deserialize(v.into_deserializer()).map(Some),
            }
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where E: serde::de::Error {
            if v == 0 {
                Ok(None)
            } else {
                S::deserialize(v.into_deserializer()).map(Some)
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where E: serde::de::Error {
            if v == 0 {
                Ok(None)
            } else {
                S::deserialize(v.into_deserializer()).map(Some)
            }
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where E: serde::de::Error {
            Ok(None)
        }
    }

    deserializer.deserialize_any(Inner(std::marker::PhantomData))
}

/// A request that can be paginated.
pub trait Paginated: Request {
    /// Should returns the current pagination cursor.
    ///
    /// # Notes
    ///
    /// Pass [`Option::None`] if no cursor is found.
    fn set_pagination(&mut self, cursor: Option<Cursor>);
}

/// A cursor for pagination. This is needed because of how pagination is represented in the [New Twitch API](https://dev.twitch.tv/docs/api)
#[derive(PartialEq, Deserialize, Debug, Clone, Default)]
struct Pagination {
    #[serde(default)]
    cursor: Option<Cursor>,
}

/// A cursor is a pointer to the current "page" in the twitch api pagination
#[aliri_braid::braid(serde)]
pub struct Cursor;

impl CursorRef {
    /// Get a borrowed [`Cow<'_, CursorRef>`](std::borrow::Cow::Borrowed)
    pub fn as_cow(&self) -> ::std::borrow::Cow<'_, Self> { self.into() }
}

impl Cursor {
    /// Get a owned [`Cow<'_, CursorRef>`](std::borrow::Cow::Owned)
    const fn into_cow<'a>(self) -> std::borrow::Cow<'a, CursorRef> { std::borrow::Cow::Owned(self) }
}

/// Errors that can happen when creating a body
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum BodyError {
    /// could not serialize as json
    JsonError(#[from] serde_json::Error),
    /// could not serialize to query
    QuerySerializeError(#[from] ser::Error),
    /// uri is invalid
    InvalidUri(#[from] InvalidUri),
}

/// Create a body. Used for specializing request bodies
pub trait HelixRequestBody {
    /// Create the body
    fn try_to_body(&self) -> Result<hyper::body::Bytes, BodyError>;
}

/// An empty body.
///
/// Implements [`HelixRequestBody::try_to_body`], returning an empty vector
#[derive(Default, Clone, Copy)]
pub struct EmptyBody;

impl HelixRequestBody for EmptyBody {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, BodyError> { Ok(<_>::default()) }
}

// TODO: I would want specialization for this. For now, to override this behavior for a body, we specify a sealed trait
impl<T> HelixRequestBody for T
where T: serde::Serialize + private::SealedSerialize
{
    fn try_to_body(&self) -> Result<hyper::body::Bytes, BodyError> {
        serde_json::to_vec(&self)
            .map_err(Into::into)
            .map(Into::into)
    }
}

pub(crate) mod private {
    pub trait SealedSerialize {}
}

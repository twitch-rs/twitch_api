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
//!
//! ## Implemented Subscriptions
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">automod.*</code> 🟢 6/6</summary>
//!
//! | Name | Subscription<br>Payload |
//! |---|:---|
//! | [`automod.message.hold`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#automodmessagehold) (v1) | [AutomodMessageHoldV1](automod::AutomodMessageHoldV1)<br>[AutomodMessageHoldV1Payload](automod::AutomodMessageHoldV1Payload) |
//! | [`automod.message.hold`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#automodmessagehold-v2) (v2) | [AutomodMessageHoldV2](automod::AutomodMessageHoldV2)<br>[AutomodMessageHoldV2Payload](automod::AutomodMessageHoldV2Payload) |
//! | [`automod.message.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#automodmessageupdate) (v1) | [AutomodMessageUpdateV1](automod::AutomodMessageUpdateV1)<br>[AutomodMessageUpdateV1Payload](automod::AutomodMessageUpdateV1Payload) |
//! | [`automod.message.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#automodmessageupdate-v2) (v2) | [AutomodMessageUpdateV2](automod::AutomodMessageUpdateV2)<br>[AutomodMessageUpdateV2Payload](automod::AutomodMessageUpdateV2Payload) |
//! | [`automod.settings.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#automodsettingsupdate) (v1) | [AutomodSettingsUpdateV1](automod::AutomodSettingsUpdateV1)<br>[AutomodSettingsUpdateV1Payload](automod::AutomodSettingsUpdateV1Payload) |
//! | [`automod.terms.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#automodtermsupdate) (v1) | [AutomodTermsUpdateV1](automod::AutomodTermsUpdateV1)<br>[AutomodTermsUpdateV1Payload](automod::AutomodTermsUpdateV1Payload) |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">channel.*</code> 🟡 65/67</summary>
//!
//! | Name | Subscription<br>Payload |
//! |---|:---|
//! | [`channel.ad_break.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelad_breakbegin) (v1) | [ChannelAdBreakBeginV1](channel::ChannelAdBreakBeginV1)<br>[ChannelAdBreakBeginV1Payload](channel::ChannelAdBreakBeginV1Payload) |
//! | [`channel.ban`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelban) (v1) | [ChannelBanV1](channel::ChannelBanV1)<br>[ChannelBanV1Payload](channel::ChannelBanV1Payload) |
//! | [`channel.bits.use`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelbitsuse) (beta) | -<br>- |
//! | [<span style="font-size: 0.9em">`channel.channel_points_automatic_reward_redemption.add`</span>](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_automatic_reward_redemptionadd) (v1) | [ChannelPointsAutomaticRewardRedemptionAddV1](channel::ChannelPointsAutomaticRewardRedemptionAddV1)<br>[ChannelPointsAutomaticRewardRedemptionAddV1Payload](channel::ChannelPointsAutomaticRewardRedemptionAddV1Payload) |
//! | [<span style="font-size: 0.9em">`channel.channel_points_automatic_reward_redemption.add`</span>](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_automatic_reward_redemptionadd-v2) (beta) | -<br>- |
//! | [<span style="font-size: 0.9em">`channel.channel_points_custom_reward.add`</span>](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_rewardadd) (v1) | [ChannelPointsCustomRewardAddV1](channel::ChannelPointsCustomRewardAddV1)<br>[ChannelPointsCustomRewardAddV1Payload](channel::ChannelPointsCustomRewardAddV1Payload) |
//! | [<span style="font-size: 0.9em">`channel.channel_points_custom_reward.remove`</span>](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_rewardremove) (v1) | [ChannelPointsCustomRewardRemoveV1](channel::ChannelPointsCustomRewardRemoveV1)<br>[ChannelPointsCustomRewardRemoveV1Payload](channel::ChannelPointsCustomRewardRemoveV1Payload) |
//! | [<span style="font-size: 0.9em">`channel.channel_points_custom_reward.update`</span>](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_rewardupdate) (v1) | [ChannelPointsCustomRewardUpdateV1](channel::ChannelPointsCustomRewardUpdateV1)<br>[ChannelPointsCustomRewardUpdateV1Payload](channel::ChannelPointsCustomRewardUpdateV1Payload) |
//! | [<span style="font-size: 0.9em">`channel.channel_points_custom_reward_redemption.add`</span>](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_reward_redemptionadd) (v1) | [ChannelPointsCustomRewardRedemptionAddV1](channel::ChannelPointsCustomRewardRedemptionAddV1)<br>[ChannelPointsCustomRewardRedemptionAddV1Payload](channel::ChannelPointsCustomRewardRedemptionAddV1Payload) |
//! | [<span style="font-size: 0.9em">`channel.channel_points_custom_reward_redemption.update`</span>](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_reward_redemptionupdate) (v1) | [ChannelPointsCustomRewardRedemptionUpdateV1](channel::ChannelPointsCustomRewardRedemptionUpdateV1)<br>[ChannelPointsCustomRewardRedemptionUpdateV1Payload](channel::ChannelPointsCustomRewardRedemptionUpdateV1Payload) |
//! | [`channel.charity_campaign.donate`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcharity_campaigndonate) (v1) | [ChannelCharityCampaignDonateV1](channel::ChannelCharityCampaignDonateV1)<br>[ChannelCharityCampaignDonateV1Payload](channel::ChannelCharityCampaignDonateV1Payload) |
//! | [`channel.charity_campaign.progress`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcharity_campaignprogress) (v1) | [ChannelCharityCampaignProgressV1](channel::ChannelCharityCampaignProgressV1)<br>[ChannelCharityCampaignProgressV1Payload](channel::ChannelCharityCampaignProgressV1Payload) |
//! | [`channel.charity_campaign.start`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcharity_campaignstart) (v1) | [ChannelCharityCampaignStartV1](channel::ChannelCharityCampaignStartV1)<br>[ChannelCharityCampaignStartV1Payload](channel::ChannelCharityCampaignStartV1Payload) |
//! | [`channel.charity_campaign.stop`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcharity_campaignstop) (v1) | [ChannelCharityCampaignStopV1](channel::ChannelCharityCampaignStopV1)<br>[ChannelCharityCampaignStopV1Payload](channel::ChannelCharityCampaignStopV1Payload) |
//! | [`channel.chat.clear`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatclear) (v1) | [ChannelChatClearV1](channel::ChannelChatClearV1)<br>[ChannelChatClearV1Payload](channel::ChannelChatClearV1Payload) |
//! | [`channel.chat.clear_user_messages`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatclear_user_messages) (v1) | [ChannelChatClearUserMessagesV1](channel::ChannelChatClearUserMessagesV1)<br>[ChannelChatClearUserMessagesV1Payload](channel::ChannelChatClearUserMessagesV1Payload) |
//! | [`channel.chat.message`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatmessage) (v1) | [ChannelChatMessageV1](channel::ChannelChatMessageV1)<br>[ChannelChatMessageV1Payload](channel::ChannelChatMessageV1Payload) |
//! | [`channel.chat.message_delete`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatmessage_delete) (v1) | [ChannelChatMessageDeleteV1](channel::ChannelChatMessageDeleteV1)<br>[ChannelChatMessageDeleteV1Payload](channel::ChannelChatMessageDeleteV1Payload) |
//! | [`channel.chat.notification`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatnotification) (v1) | [ChannelChatNotificationV1](channel::ChannelChatNotificationV1)<br>[ChannelChatNotificationV1Payload](channel::ChannelChatNotificationV1Payload) |
//! | [`channel.chat.user_message_hold`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatuser_message_hold) (v1) | [ChannelChatUserMessageHoldV1](channel::ChannelChatUserMessageHoldV1)<br>[ChannelChatUserMessageHoldV1Payload](channel::ChannelChatUserMessageHoldV1Payload) |
//! | [`channel.chat.user_message_update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchatuser_message_update) (v1) | [ChannelChatUserMessageUpdateV1](channel::ChannelChatUserMessageUpdateV1)<br>[ChannelChatUserMessageUpdateV1Payload](channel::ChannelChatUserMessageUpdateV1Payload) |
//! | [`channel.chat_settings.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchat_settingsupdate) (v1) | [ChannelChatSettingsUpdateV1](channel::ChannelChatSettingsUpdateV1)<br>[ChannelChatSettingsUpdateV1Payload](channel::ChannelChatSettingsUpdateV1Payload) |
//! | [`channel.cheer`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelcheer) (v1) | [ChannelCheerV1](channel::ChannelCheerV1)<br>[ChannelCheerV1Payload](channel::ChannelCheerV1Payload) |
//! | [`channel.follow`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelfollow) (v2) | [ChannelFollowV2](channel::ChannelFollowV2)<br>[ChannelFollowV2Payload](channel::ChannelFollowV2Payload) |
//! | [`channel.goal.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelgoalbegin) (v1) | [ChannelGoalBeginV1](channel::ChannelGoalBeginV1)<br>[ChannelGoalBeginV1Payload](channel::ChannelGoalBeginV1Payload) |
//! | [`channel.goal.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelgoalend) (v1) | [ChannelGoalEndV1](channel::ChannelGoalEndV1)<br>[ChannelGoalEndV1Payload](channel::ChannelGoalEndV1Payload) |
//! | [`channel.goal.progress`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelgoalprogress) (v1) | [ChannelGoalProgressV1](channel::ChannelGoalProgressV1)<br>[ChannelGoalProgressV1Payload](channel::ChannelGoalProgressV1Payload) |
//! | [`channel.guest_star_guest.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelguest_star_guestupdate) (beta) | [ChannelGuestStarGuestUpdateBeta](channel::ChannelGuestStarGuestUpdateBeta)<br>[ChannelGuestStarGuestUpdateBetaPayload](channel::ChannelGuestStarGuestUpdateBetaPayload) |
//! | [`channel.guest_star_session.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelguest_star_sessionbegin) (beta) | [ChannelGuestStarSessionBeginBeta](channel::ChannelGuestStarSessionBeginBeta)<br>[ChannelGuestStarSessionBeginBetaPayload](channel::ChannelGuestStarSessionBeginBetaPayload) |
//! | [`channel.guest_star_session.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelguest_star_sessionend) (beta) | [ChannelGuestStarSessionEndBeta](channel::ChannelGuestStarSessionEndBeta)<br>[ChannelGuestStarSessionEndBetaPayload](channel::ChannelGuestStarSessionEndBetaPayload) |
//! | [`channel.guest_star_settings.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelguest_star_settingsupdate) (beta) | [ChannelGuestStarSettingsUpdateBeta](channel::ChannelGuestStarSettingsUpdateBeta)<br>[ChannelGuestStarSettingsUpdateBetaPayload](channel::ChannelGuestStarSettingsUpdateBetaPayload) |
//! | [`channel.hype_train.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelhype_trainbegin) (v1) | [ChannelHypeTrainBeginV1](channel::ChannelHypeTrainBeginV1)<br>[ChannelHypeTrainBeginV1Payload](channel::ChannelHypeTrainBeginV1Payload) |
//! | [`channel.hype_train.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelhype_trainend) (v1) | [ChannelHypeTrainEndV1](channel::ChannelHypeTrainEndV1)<br>[ChannelHypeTrainEndV1Payload](channel::ChannelHypeTrainEndV1Payload) |
//! | [`channel.hype_train.progress`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelhype_trainprogress) (v1) | [ChannelHypeTrainProgressV1](channel::ChannelHypeTrainProgressV1)<br>[ChannelHypeTrainProgressV1Payload](channel::ChannelHypeTrainProgressV1Payload) |
//! | [`channel.moderate`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelmoderate) (v1) | [ChannelModerateV1](channel::ChannelModerateV1)<br>[ChannelModerateV1Payload](channel::ChannelModerateV1Payload) |
//! | [`channel.moderate`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelmoderate-v2) (v2) | [ChannelModerateV2](channel::ChannelModerateV2)<br>[ChannelModerateV2Payload](channel::ChannelModerateV2Payload) |
//! | [`channel.moderator.add`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelmoderatoradd) (v1) | [ChannelModeratorAddV1](channel::ChannelModeratorAddV1)<br>[ChannelModeratorAddV1Payload](channel::ChannelModeratorAddV1Payload) |
//! | [`channel.moderator.remove`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelmoderatorremove) (v1) | [ChannelModeratorRemoveV1](channel::ChannelModeratorRemoveV1)<br>[ChannelModeratorRemoveV1Payload](channel::ChannelModeratorRemoveV1Payload) |
//! | [`channel.poll.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpollbegin) (v1) | [ChannelPollBeginV1](channel::ChannelPollBeginV1)<br>[ChannelPollBeginV1Payload](channel::ChannelPollBeginV1Payload) |
//! | [`channel.poll.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpollend) (v1) | [ChannelPollEndV1](channel::ChannelPollEndV1)<br>[ChannelPollEndV1Payload](channel::ChannelPollEndV1Payload) |
//! | [`channel.poll.progress`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpollprogress) (v1) | [ChannelPollProgressV1](channel::ChannelPollProgressV1)<br>[ChannelPollProgressV1Payload](channel::ChannelPollProgressV1Payload) |
//! | [`channel.prediction.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpredictionbegin) (v1) | [ChannelPredictionBeginV1](channel::ChannelPredictionBeginV1)<br>[ChannelPredictionBeginV1Payload](channel::ChannelPredictionBeginV1Payload) |
//! | [`channel.prediction.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpredictionend) (v1) | [ChannelPredictionEndV1](channel::ChannelPredictionEndV1)<br>[ChannelPredictionEndV1Payload](channel::ChannelPredictionEndV1Payload) |
//! | [`channel.prediction.lock`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpredictionlock) (v1) | [ChannelPredictionLockV1](channel::ChannelPredictionLockV1)<br>[ChannelPredictionLockV1Payload](channel::ChannelPredictionLockV1Payload) |
//! | [`channel.prediction.progress`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpredictionprogress) (v1) | [ChannelPredictionProgressV1](channel::ChannelPredictionProgressV1)<br>[ChannelPredictionProgressV1Payload](channel::ChannelPredictionProgressV1Payload) |
//! | [`channel.raid`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelraid) (v1) | [ChannelRaidV1](channel::ChannelRaidV1)<br>[ChannelRaidV1Payload](channel::ChannelRaidV1Payload) |
//! | [`channel.shared_chat.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelshared_chatbegin) (v1) | [ChannelSharedChatBeginV1](channel::ChannelSharedChatBeginV1)<br>[ChannelSharedChatBeginV1Payload](channel::ChannelSharedChatBeginV1Payload) |
//! | [`channel.shared_chat.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelshared_chatend) (v1) | [ChannelSharedChatEndV1](channel::ChannelSharedChatEndV1)<br>[ChannelSharedChatEndV1Payload](channel::ChannelSharedChatEndV1Payload) |
//! | [`channel.shared_chat.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelshared_chatupdate) (v1) | [ChannelSharedChatUpdateV1](channel::ChannelSharedChatUpdateV1)<br>[ChannelSharedChatUpdateV1Payload](channel::ChannelSharedChatUpdateV1Payload) |
//! | [`channel.shield_mode.begin`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelshield_modebegin) (v1) | [ChannelShieldModeBeginV1](channel::ChannelShieldModeBeginV1)<br>[ChannelShieldModeBeginV1Payload](channel::ChannelShieldModeBeginV1Payload) |
//! | [`channel.shield_mode.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelshield_modeend) (v1) | [ChannelShieldModeEndV1](channel::ChannelShieldModeEndV1)<br>[ChannelShieldModeEndV1Payload](channel::ChannelShieldModeEndV1Payload) |
//! | [`channel.shoutout.create`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelshoutoutcreate) (v1) | [ChannelShoutoutCreateV1](channel::ChannelShoutoutCreateV1)<br>[ChannelShoutoutCreateV1Payload](channel::ChannelShoutoutCreateV1Payload) |
//! | [`channel.shoutout.receive`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelshoutoutreceive) (v1) | [ChannelShoutoutReceiveV1](channel::ChannelShoutoutReceiveV1)<br>[ChannelShoutoutReceiveV1Payload](channel::ChannelShoutoutReceiveV1Payload) |
//! | [`channel.subscribe`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsubscribe) (v1) | [ChannelSubscribeV1](channel::ChannelSubscribeV1)<br>[ChannelSubscribeV1Payload](channel::ChannelSubscribeV1Payload) |
//! | [`channel.subscription.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsubscriptionend) (v1) | [ChannelSubscriptionEndV1](channel::ChannelSubscriptionEndV1)<br>[ChannelSubscriptionEndV1Payload](channel::ChannelSubscriptionEndV1Payload) |
//! | [`channel.subscription.gift`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsubscriptiongift) (v1) | [ChannelSubscriptionGiftV1](channel::ChannelSubscriptionGiftV1)<br>[ChannelSubscriptionGiftV1Payload](channel::ChannelSubscriptionGiftV1Payload) |
//! | [`channel.subscription.message`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsubscriptionmessage) (v1) | [ChannelSubscriptionMessageV1](channel::ChannelSubscriptionMessageV1)<br>[ChannelSubscriptionMessageV1Payload](channel::ChannelSubscriptionMessageV1Payload) |
//! | [`channel.suspicious_user.message`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsuspicious_usermessage) (v1) | [ChannelSuspiciousUserMessageV1](channel::ChannelSuspiciousUserMessageV1)<br>[ChannelSuspiciousUserMessageV1Payload](channel::ChannelSuspiciousUserMessageV1Payload) |
//! | [`channel.suspicious_user.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelsuspicious_userupdate) (v1) | [ChannelSuspiciousUserUpdateV1](channel::ChannelSuspiciousUserUpdateV1)<br>[ChannelSuspiciousUserUpdateV1Payload](channel::ChannelSuspiciousUserUpdateV1Payload) |
//! | [`channel.unban`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelunban) (v1) | [ChannelUnbanV1](channel::ChannelUnbanV1)<br>[ChannelUnbanV1Payload](channel::ChannelUnbanV1Payload) |
//! | [`channel.unban_request.create`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelunban_requestcreate) (v1) | [ChannelUnbanRequestCreateV1](channel::ChannelUnbanRequestCreateV1)<br>[ChannelUnbanRequestCreateV1Payload](channel::ChannelUnbanRequestCreateV1Payload) |
//! | [`channel.unban_request.resolve`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelunban_requestresolve) (v1) | [ChannelUnbanRequestResolveV1](channel::ChannelUnbanRequestResolveV1)<br>[ChannelUnbanRequestResolveV1Payload](channel::ChannelUnbanRequestResolveV1Payload) |
//! | [`channel.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelupdate) (v2) | [ChannelUpdateV2](channel::ChannelUpdateV2)<br>[ChannelUpdateV2Payload](channel::ChannelUpdateV2Payload) |
//! | [`channel.vip.add`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelvipadd) (v1) | [ChannelVipAddV1](channel::ChannelVipAddV1)<br>[ChannelVipAddV1Payload](channel::ChannelVipAddV1Payload) |
//! | [`channel.vip.remove`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelvipremove) (v1) | [ChannelVipRemoveV1](channel::ChannelVipRemoveV1)<br>[ChannelVipRemoveV1Payload](channel::ChannelVipRemoveV1Payload) |
//! | [`channel.warning.acknowledge`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelwarningacknowledge) (v1) | [ChannelWarningAcknowledgeV1](channel::ChannelWarningAcknowledgeV1)<br>[ChannelWarningAcknowledgeV1Payload](channel::ChannelWarningAcknowledgeV1Payload) |
//! | [`channel.warning.send`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelwarningsend) (v1) | [ChannelWarningSendV1](channel::ChannelWarningSendV1)<br>[ChannelWarningSendV1Payload](channel::ChannelWarningSendV1Payload) |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">conduit.*</code> 🟢 1/1</summary>
//!
//! | Name | Subscription<br>Payload |
//! |---|:---|
//! | [`conduit.shard.disabled`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#conduitsharddisabled) (v1) | [ConduitShardDisabledV1](conduit::ConduitShardDisabledV1)<br>[ConduitShardDisabledV1Payload](conduit::ConduitShardDisabledV1Payload) |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">drop.*</code> 🔴 0/1</summary>
//!
//! | Name | Subscription<br>Payload |
//! |---|:---|
//! | [`drop.entitlement.grant`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#dropentitlementgrant) (v1) | -<br>- |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">extension.*</code> 🔴 0/1</summary>
//!
//! | Name | Subscription<br>Payload |
//! |---|:---|
//! | [`extension.bits_transaction.create`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#extensionbits_transactioncreate) (v1) | -<br>- |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">stream.*</code> 🟢 2/2</summary>
//!
//! | Name | Subscription<br>Payload |
//! |---|:---|
//! | [`stream.offline`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#streamoffline) (v1) | [StreamOfflineV1](stream::StreamOfflineV1)<br>[StreamOfflineV1Payload](stream::StreamOfflineV1Payload) |
//! | [`stream.online`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#streamonline) (v1) | [StreamOnlineV1](stream::StreamOnlineV1)<br>[StreamOnlineV1Payload](stream::StreamOnlineV1Payload) |
//!
//! </details>
//!
//! <details><summary style="cursor: pointer"><code style="color: var(--link-color)">user.*</code> 🟢 4/4</summary>
//!
//! | Name | Subscription<br>Payload |
//! |---|:---|
//! | [`user.authorization.grant`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userauthorizationgrant) (v1) | [UserAuthorizationGrantV1](user::UserAuthorizationGrantV1)<br>[UserAuthorizationGrantV1Payload](user::UserAuthorizationGrantV1Payload) |
//! | [`user.authorization.revoke`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userauthorizationrevoke) (v1) | [UserAuthorizationRevokeV1](user::UserAuthorizationRevokeV1)<br>[UserAuthorizationRevokeV1Payload](user::UserAuthorizationRevokeV1Payload) |
//! | [`user.update`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userupdate) (v1) | [UserUpdateV1](user::UserUpdateV1)<br>[UserUpdateV1Payload](user::UserUpdateV1Payload) |
//! | [`user.whisper.message`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#userwhispermessage) (v1) | [UserWhisperMessageV1](user::UserWhisperMessageV1)<br>[UserWhisperMessageV1Payload](user::UserWhisperMessageV1Payload) |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->

use std::borrow::Cow;

use crate::types;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

use crate::parse_json;

pub mod automod;
pub mod channel;
pub mod conduit;
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
/// Conduit transport
pub struct ConduitTransport {
    /// An ID that identifies the conduit to send notifications to.
    ///
    /// When you create a conduit, the server returns the conduit ID.
    pub conduit_id: String,
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
    /// Conduit transport
    Conduit(ConduitTransport),
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

    /// Convenience method for making a conduit transport
    pub fn conduit(conduit_id: impl std::string::ToString) -> Transport {
        Transport::Conduit(ConduitTransport {
            conduit_id: conduit_id.to_string(),
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

    /// Returns `true` if the transport is [`Conduit`].
    ///
    /// [`Conduit`]: Transport::Conduit
    #[must_use]
    pub fn is_conduit(&self) -> bool { matches!(self, Self::Conduit(..)) }

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

    /// Returns `Some(&ConduitTransport)` if this transport is a [conduit](ConduitTransport)
    pub fn as_conduit(&self) -> Option<&ConduitTransport> {
        if let Self::Conduit(v) = self {
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

    /// Returns `Some(ConduitTransport)` if this transport is a [conduit](ConduitTransport), `Err(())` if not
    pub fn try_into_conduit(self) -> Option<ConduitTransport> {
        if let Self::Conduit(v) = self {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
/// Conduit transport
pub struct ConduitTransportResponse {
    /// The conduit ID
    pub conduit_id: String,
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
    /// Conduit transport response
    Conduit(ConduitTransportResponse),
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

    /// Returns `true` if the transport response is [`Conduit`].
    ///
    /// [`Conduit`]: TransportResponse::Conduit
    #[must_use]
    pub fn is_conduit(&self) -> bool { matches!(self, Self::Conduit(..)) }

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

    /// Returns `Some(&ConduitTransport)` if this transport response is a [conduit](ConduitTransportResponse)
    pub fn as_conduit(&self) -> Option<&ConduitTransportResponse> {
        if let Self::Conduit(v) = self {
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

    /// Returns `Ok(ConduitTransport)` if this transport response is a [conduit](ConduitTransportResponse)
    pub fn try_into_conduit(self) -> Result<ConduitTransportResponse, Self> {
        if let Self::Conduit(v) = self {
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
    pub id: types::ConduitId,
    /// Number of shards associated with this conduit
    pub shard_count: usize,
}

/// General information about a [Shard](https://dev.twitch.tv/docs/eventsub/handling-conduit-events/)
///
/// A shard is a Webhook or WebSocket connection, while a conduit is a collection of shards. The conduit transport type is for backend server applications and requires app access tokens.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
#[cfg(feature = "eventsub")]
#[cfg_attr(nightly, doc(cfg(feature = "eventsub")))]
pub struct Shard {
    /// Shard ID.
    pub id: types::ConduitShardId,

    /// The transport details that you want Twitch to use when sending you notifications.
    pub transport: Transport,
}

impl Shard {
    /// Create a shard with a transport set
    pub fn new(id: impl Into<types::ConduitShardId>, transport: Transport) -> Self {
        Self {
            id: id.into(),

            transport,
        }
    }
}

/// The shard status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ShardStatus {
    /// The shard is enabled.
    Enabled,

    /// The shard is pending verification of the specified callback URL.
    WebhookCallbackVerificationPending,

    /// The specified callback URL failed verification.
    WebhookCallbackVerificationFailed,

    /// The notification delivery failure rate was too high.
    NotificationFailuresExceeded,

    /// The client closed the connection.
    WebsocketDisconnected,

    /// The client failed to respond to a ping message.
    WebsocketFailedPingPong,

    /// The client sent a non-pong message. Clients may only send pong messages (and only in response to a ping message).
    WebsocketReceivedInboundTraffic,

    /// The Twitch WebSocket server experienced an unexpected error.
    WebsocketInternalError,

    /// The Twitch WebSocket server timed out writing the message to the client.
    WebsocketNetworkTimeout,

    /// The Twitch WebSocket server experienced a network error writing the message to the client.
    WebsocketNetworkError,

    /// The client failed to reconnect to the Twitch WebSocket server within the required time after a Reconnect Message.
    WebsocketFailedToReconnect,
}

/// A structured error that occurred with a shard
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ShardError {
    /// Shard ID.
    pub id: types::ConduitShardId,

    /// The error that occurred while updating the shard.
    pub message: String,

    /// Error codes used to represent a specific error condition while attempting to update shards.
    pub code: String,
}

/// A shard when described by Twitch
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ShardResponse {
    /// Shard ID.
    pub id: types::ConduitShardId,

    /// The shard status. The subscriber receives events only for enabled shards.
    pub status: ShardStatus,

    /// The transport details that you want Twitch to use when sending you notifications.
    pub transport: TransportResponse,
}

pub(crate) trait NamedField {
    const NAME: &'static str;
    const OPT_PREFIX: Option<&'static str> = None;
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

/// Deserialize {"field": field} as { field ...} and serialize in reverse
/// utilizes OPT_PREFIX for prefixing the field name
mod enum_field_as_inner_prefixed {
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
                let name = S::OPT_PREFIX.unwrap().to_string() + S::NAME;
                while let Some(key) = map.next_key::<String>()? {
                    if key == name {
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
        map.serialize_entry(&(T::OPT_PREFIX.unwrap().to_string() + T::NAME), value)?;
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

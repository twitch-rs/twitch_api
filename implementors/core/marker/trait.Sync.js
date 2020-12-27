(function() {var implementors = {};
implementors["twitch_api2"] = [{"text":"impl Sync for TwitchCategory","synthetic":true,"types":[]},{"text":"impl Sync for SubscriptionTier","synthetic":true,"types":[]},{"text":"impl Sync for BroadcasterType","synthetic":true,"types":[]},{"text":"impl Sync for UserType","synthetic":true,"types":[]},{"text":"impl Sync for VideoPeriod","synthetic":true,"types":[]},{"text":"impl Sync for VideoType","synthetic":true,"types":[]},{"text":"impl Sync for VideoPrivacy","synthetic":true,"types":[]},{"text":"impl Sync for CommercialLength","synthetic":true,"types":[]},{"text":"impl Sync for CommercialLengthParseError","synthetic":true,"types":[]},{"text":"impl Sync for User","synthetic":true,"types":[]},{"text":"impl Sync for GetBitsLeaderboardRequest","synthetic":true,"types":[]},{"text":"impl Sync for BitsLeaderboard","synthetic":true,"types":[]},{"text":"impl Sync for DateRange","synthetic":true,"types":[]},{"text":"impl Sync for LeaderboardUser","synthetic":true,"types":[]},{"text":"impl Sync for GetCheermotesRequest","synthetic":true,"types":[]},{"text":"impl Sync for Cheermote","synthetic":true,"types":[]},{"text":"impl Sync for CheermoteType","synthetic":true,"types":[]},{"text":"impl Sync for Tiers","synthetic":true,"types":[]},{"text":"impl Sync for CheermoteImages","synthetic":true,"types":[]},{"text":"impl Sync for CheermoteImage","synthetic":true,"types":[]},{"text":"impl Sync for CheermoteImageArray","synthetic":true,"types":[]},{"text":"impl Sync for Level","synthetic":true,"types":[]},{"text":"impl Sync for GetChannelInformationRequest","synthetic":true,"types":[]},{"text":"impl Sync for ChannelInformation","synthetic":true,"types":[]},{"text":"impl Sync for ModifyChannelInformationRequest","synthetic":true,"types":[]},{"text":"impl Sync for ModifyChannelInformationBody","synthetic":true,"types":[]},{"text":"impl Sync for ModifyChannelInformation","synthetic":true,"types":[]},{"text":"impl Sync for StartCommercialRequest","synthetic":true,"types":[]},{"text":"impl Sync for StartCommercialBody","synthetic":true,"types":[]},{"text":"impl Sync for StartCommercial","synthetic":true,"types":[]},{"text":"impl Sync for GetClipsRequest","synthetic":true,"types":[]},{"text":"impl Sync for Clip","synthetic":true,"types":[]},{"text":"impl Sync for GetGamesRequest","synthetic":true,"types":[]},{"text":"impl Sync for GetTopGamesRequest","synthetic":true,"types":[]},{"text":"impl Sync for CheckAutoModStatusRequest","synthetic":true,"types":[]},{"text":"impl Sync for CheckAutoModStatusBody","synthetic":true,"types":[]},{"text":"impl Sync for CheckAutoModStatus","synthetic":true,"types":[]},{"text":"impl Sync for GetBannedEventsRequest","synthetic":true,"types":[]},{"text":"impl Sync for BannedEvent","synthetic":true,"types":[]},{"text":"impl Sync for GetBannedUsersRequest","synthetic":true,"types":[]},{"text":"impl Sync for BannedUser","synthetic":true,"types":[]},{"text":"impl Sync for GetModeratorEventsRequest","synthetic":true,"types":[]},{"text":"impl Sync for ModeratorEvent","synthetic":true,"types":[]},{"text":"impl Sync for GetModeratorsRequest","synthetic":true,"types":[]},{"text":"impl Sync for Moderator","synthetic":true,"types":[]},{"text":"impl Sync for GetCustomRewardRedemptionRequest","synthetic":true,"types":[]},{"text":"impl Sync for CustomRewardRedemption","synthetic":true,"types":[]},{"text":"impl Sync for Reward","synthetic":true,"types":[]},{"text":"impl Sync for UpdateRedemptionStatusRequest","synthetic":true,"types":[]},{"text":"impl Sync for UpdateRedemptionStatusBody","synthetic":true,"types":[]},{"text":"impl Sync for UpdateRedemptionStatusInformation","synthetic":true,"types":[]},{"text":"impl Sync for CustomRewardRedemptionStatus","synthetic":true,"types":[]},{"text":"impl Sync for SearchCategoriesRequest","synthetic":true,"types":[]},{"text":"impl Sync for SearchChannelsRequest","synthetic":true,"types":[]},{"text":"impl Sync for Channel","synthetic":true,"types":[]},{"text":"impl Sync for GetStreamTagsRequest","synthetic":true,"types":[]},{"text":"impl Sync for GetStreamsRequest","synthetic":true,"types":[]},{"text":"impl Sync for Stream","synthetic":true,"types":[]},{"text":"impl Sync for StreamType","synthetic":true,"types":[]},{"text":"impl Sync for GetBroadcasterSubscriptionsRequest","synthetic":true,"types":[]},{"text":"impl Sync for BroadcasterSubscription","synthetic":true,"types":[]},{"text":"impl Sync for GetAllStreamTagsRequest","synthetic":true,"types":[]},{"text":"impl Sync for AutoGenerated","synthetic":true,"types":[]},{"text":"impl Sync for TwitchTag","synthetic":true,"types":[]},{"text":"impl Sync for CreateUserFollowsRequest","synthetic":true,"types":[]},{"text":"impl Sync for CreateUserFollowsBody","synthetic":true,"types":[]},{"text":"impl Sync for CreateUserFollows","synthetic":true,"types":[]},{"text":"impl Sync for DeleteUserFollowsRequest","synthetic":true,"types":[]},{"text":"impl Sync for DeleteUserFollow","synthetic":true,"types":[]},{"text":"impl Sync for GetUsersRequest","synthetic":true,"types":[]},{"text":"impl Sync for User","synthetic":true,"types":[]},{"text":"impl Sync for GetUsersFollowsRequest","synthetic":true,"types":[]},{"text":"impl Sync for UsersFollow","synthetic":true,"types":[]},{"text":"impl Sync for GetVideosRequest","synthetic":true,"types":[]},{"text":"impl Sync for Video","synthetic":true,"types":[]},{"text":"impl Sync for Sort","synthetic":true,"types":[]},{"text":"impl Sync for VideoPeriod","synthetic":true,"types":[]},{"text":"impl Sync for VideoTypeFilter","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl&lt;'a, C&gt; Sync for HelixClient&lt;'a, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;R, D&gt; Sync for Response&lt;R, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for ClientRequestError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl Sync for CreateRequestError","synthetic":true,"types":[]},{"text":"impl Sync for InvalidUri","synthetic":true,"types":[]},{"text":"impl Sync for HelixRequestGetError","synthetic":true,"types":[]},{"text":"impl Sync for HelixRequestPutError","synthetic":true,"types":[]},{"text":"impl Sync for HelixRequestPostError","synthetic":true,"types":[]},{"text":"impl Sync for HelixRequestPatchError","synthetic":true,"types":[]},{"text":"impl Sync for HelixRequestDeleteError","synthetic":true,"types":[]},{"text":"impl&lt;'a, C&gt; Sync for TMIClient&lt;'a, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for GetChatters","synthetic":true,"types":[]},{"text":"impl Sync for Chatters","synthetic":true,"types":[]},{"text":"impl Sync for HostsRequestId","synthetic":true,"types":[]},{"text":"impl Sync for GetHosts","synthetic":true,"types":[]},{"text":"impl Sync for Host","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for RequestError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl Sync for ChannelBitsEventsV2","synthetic":true,"types":[]},{"text":"impl Sync for ChannelBitsEventsV2Reply","synthetic":true,"types":[]},{"text":"impl Sync for BitsEventData","synthetic":true,"types":[]},{"text":"impl Sync for BadgeEntitlement","synthetic":true,"types":[]},{"text":"impl Sync for BitsContext","synthetic":true,"types":[]},{"text":"impl Sync for ChannelBitsBadgeUnlocks","synthetic":true,"types":[]},{"text":"impl Sync for ChannelBitsBadgeUnlocksReply","synthetic":true,"types":[]},{"text":"impl Sync for ChannelCheerEventsPublicV1","synthetic":true,"types":[]},{"text":"impl Sync for ChannelCheerEventsPublicV1Reply","synthetic":true,"types":[]},{"text":"impl Sync for TriggerType","synthetic":true,"types":[]},{"text":"impl Sync for ChannelPointsChannelV1","synthetic":true,"types":[]},{"text":"impl Sync for Redemption","synthetic":true,"types":[]},{"text":"impl Sync for RedemptionStatus","synthetic":true,"types":[]},{"text":"impl Sync for Reward","synthetic":true,"types":[]},{"text":"impl Sync for Image","synthetic":true,"types":[]},{"text":"impl Sync for GlobalCooldown","synthetic":true,"types":[]},{"text":"impl Sync for Max","synthetic":true,"types":[]},{"text":"impl Sync for Progress","synthetic":true,"types":[]},{"text":"impl Sync for ChannelPointsChannelV1Reply","synthetic":true,"types":[]},{"text":"impl Sync for ChannelSubGiftsV1","synthetic":true,"types":[]},{"text":"impl Sync for MysteryGiftPurchase","synthetic":true,"types":[]},{"text":"impl Sync for ChannelSubGiftsV1Reply","synthetic":true,"types":[]},{"text":"impl Sync for ChannelSubscribeEventsV1","synthetic":true,"types":[]},{"text":"impl Sync for Sub","synthetic":true,"types":[]},{"text":"impl Sync for ReSub","synthetic":true,"types":[]},{"text":"impl Sync for SubGift","synthetic":true,"types":[]},{"text":"impl Sync for ResubGift","synthetic":true,"types":[]},{"text":"impl Sync for ChannelSubscribeEventsV1Reply","synthetic":true,"types":[]},{"text":"impl Sync for Emote","synthetic":true,"types":[]},{"text":"impl Sync for SubMessage","synthetic":true,"types":[]},{"text":"impl Sync for CommunityPointsChannelV1","synthetic":true,"types":[]},{"text":"impl Sync for Following","synthetic":true,"types":[]},{"text":"impl Sync for FollowingReply","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainEventsV1","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainEventsV1Rewards","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainRewards","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainStart","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainEnd","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainConductorUpdate","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainProgression","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainLevelUp","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainEventsV1Reply","synthetic":true,"types":[]},{"text":"impl Sync for Config","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainDifficulty","synthetic":true,"types":[]},{"text":"impl Sync for Kickoff","synthetic":true,"types":[]},{"text":"impl Sync for ParticipationConversionRates","synthetic":true,"types":[]},{"text":"impl Sync for NotificationThresholds","synthetic":true,"types":[]},{"text":"impl Sync for ConductorRewards","synthetic":true,"types":[]},{"text":"impl Sync for BitsRewards","synthetic":true,"types":[]},{"text":"impl Sync for SubsRewards","synthetic":true,"types":[]},{"text":"impl Sync for Participations","synthetic":true,"types":[]},{"text":"impl Sync for Conductors","synthetic":true,"types":[]},{"text":"impl Sync for HypeTrainProgress","synthetic":true,"types":[]},{"text":"impl Sync for Level","synthetic":true,"types":[]},{"text":"impl Sync for Reward","synthetic":true,"types":[]},{"text":"impl Sync for SourceType","synthetic":true,"types":[]},{"text":"impl Sync for ActionType","synthetic":true,"types":[]},{"text":"impl Sync for EndingReason","synthetic":true,"types":[]},{"text":"impl Sync for ChatModeratorActions","synthetic":true,"types":[]},{"text":"impl Sync for ModerationAction","synthetic":true,"types":[]},{"text":"impl Sync for ModeratorAdded","synthetic":true,"types":[]},{"text":"impl Sync for ChatModeratorActionsReply","synthetic":true,"types":[]},{"text":"impl Sync for UnbanRequest","synthetic":true,"types":[]},{"text":"impl Sync for ModerationActionCommand","synthetic":true,"types":[]},{"text":"impl Sync for ModerationType","synthetic":true,"types":[]},{"text":"impl Sync for Raid","synthetic":true,"types":[]},{"text":"impl Sync for RaidGoV2","synthetic":true,"types":[]},{"text":"impl Sync for RaidUpdateV2","synthetic":true,"types":[]},{"text":"impl Sync for RaidCancelV2","synthetic":true,"types":[]},{"text":"impl Sync for RaidReply","synthetic":true,"types":[]},{"text":"impl Sync for VideoPlayback","synthetic":true,"types":[]},{"text":"impl Sync for VideoPlaybackById","synthetic":true,"types":[]},{"text":"impl Sync for VideoPlaybackReply","synthetic":true,"types":[]},{"text":"impl Sync for Vod","synthetic":true,"types":[]},{"text":"impl Sync for WatchpartyType","synthetic":true,"types":[]},{"text":"impl Sync for BroadcastType","synthetic":true,"types":[]},{"text":"impl Sync for TopicSubscribe","synthetic":true,"types":[]},{"text":"impl Sync for TwitchResponse","synthetic":true,"types":[]},{"text":"impl Sync for TopicData","synthetic":true,"types":[]},{"text":"impl Sync for Response","synthetic":true,"types":[]},{"text":"impl Sync for SurfError","synthetic":true,"types":[]},{"text":"impl Sync for DummyHttpClient","synthetic":true,"types":[]},{"text":"impl&lt;'a, C&gt; Sync for TwitchClient&lt;'a, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["twitch_oauth2"] = [{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl&lt;EF, TT&gt; Sync for TwitchTokenResponse&lt;EF, TT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;EF: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;TT: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for TwitchTokenErrorResponse","synthetic":true,"types":[]},{"text":"impl Sync for Scope","synthetic":true,"types":[]},{"text":"impl Sync for AppAccessToken","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for TokenError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for ValidationError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for RevokeTokenError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for RefreshTokenError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl Sync for UserToken","synthetic":true,"types":[]},{"text":"impl Sync for ValidatedToken","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
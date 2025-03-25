# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/twitch-rs/twitch_api/compare/v0.7.1...Unreleased)

## [v0.7.1] - 2025-03-19

[Commits](https://github.com/twitch-rs/twitch_api/compare/v0.7.0...v0.7.1)

### Added

- Added the following eventsub topics:
  `Channel Bits Use V1`
- Added `HelixClient::get_users_from_logins`

### Fixed

- Eventsub `channel.moderate` action `Timeout` and `Ban` field `reason` is now `None` when the string is empty.
- `ClientExt::get_eventsub_subscriptions` now returns all pages (previously, only the first page was returned).
- Corrected the names of types for `ChannelSuspiciousUserMessageV1Payload::types`
- Fixed incorrect handling of nullable fields in `channel.suspicious_user.message`

## [v0.7.0] - 2025-01-22

[Commits](https://github.com/twitch-rs/twitch_api/compare/v0.6.1...v0.7.0)

### Breaking changes

- MSRV: 1.77.0
- Requests for helix endpoints have been converted to take `Cow`s.
  This change means the `builder()` methods are harder to use, consider using the new methods on
  each request which provide the same functionality but with better ergonomics.
  See the top-level documentation for a endpoint for more examples.
- Crate name changed: `twitch_api2` -> `twitch_api`, also changed to new org `twitch-rs`
- All (most) types are now living in their own crate `twitch_types`
- Features for clients are now named after the client, e.g feature `reqwest_client` is now simply `reqwest`
- Fixed wrong type on `UserAuthorizationGrantV1::client_id`
- Removed `bits_voting_enabled`, `bits_per_vote` and `bits_votes` on `Create Poll` and `Poll`
- `Get Clips` takes and returns `ClipId` instead of a string
- Made `GetCustomRewardRedemptionRequest::reward_id` optional
- `typed-builder` is now optional and not enabled by default, enable feature `typed-builder` to use the `::builder()` methods
- Client ext methods that take a `IntoIterator<Item = T>` now takes a `IntoIterator<Item = impl Into<T>>`
- `Get Channel Information` can now take multiple ids
- Simplified lifetimes for `Client`. Fixes an issue where &'1 Thing<'static> where: Thing<'static> would wrongly lower '1 to be specific. See <https://github.com/twitch-rs/twitch_api/issues/236>
- `TeamInformation::thumbnail_url` is now optional (`Option<String>`).
- Made many structs & enums non exhaustive
- Renamed `GetBlockedTerms` -> `GetBlockedTermsRequest`
- Changed `SCOPES` on `helix::Request`, `pubsub::Topic` and `eventsub::EventSubscription` to be `twitch_oauth2::Validator`
- Updated `twitch_oauth2` dependency
- Added EventSub WebSocket support, changing many methods on transport and eventsub to take this into account.
- Removed `tmi` feature ([shut down in April 2023](https://discuss.dev.twitch.com/t/legacy-chatters-endpoint-shutdown-details-and-timeline-april-2023/43161))

### Changes

- Deprecate `Get User` `view_count`
- Deprecate `Check AutoMod status` `user_id`
- Removed deprecated `limit` on `GetEventSubSubscriptionsRequest` response
- Removed deprecated `limit` on `CreateEventSubSubscription`
- Removed deprecated `AddBlockedTerm`, `DeleteBlockedTerm`, `DeletePermittedTerm` and `AddPermittedTerm` on `ModerationActionCommand`
- Deprecated TMI
- Deprecated `channel.follow` v1 eventsub event
- Deprecated `Get User Follows` and associated follower related extension methods
- Deprecated Twitch-defined tags: `Get All Stream Tags`, `Get Stream Tags`, `Replace Stream Tags` and `TwitchTag`

### Added

- Added the following helix endpoints:
  `Get Bits Leaderboard`, `Get Cheermotes`, `Get Content Classification Labels`, `Add Channel Vip`,
  `Get Ad Schedule`, `Get Channel Editors`, `Get Channel Followers`, `Get Channel Information`,
  `Get VIPs`, `Modify Channel Information`, `Remove Channel Vip`, `Snooze Next Ad`, `Start Commercial`,
  `Get Charity Campaign`, `Get Charity Campaign Donations`, `Get Channel Chat Badges`,
  `Get Channel Emotes`, `Get Chat Settings`, `Get Chatters`, `Get Emote Sets`, `Get Shared Chat Session`,
  `Get User Chat Color`, `Get User Emotes`, `Send A Shoutout`, `Send Chat Announcement`,
  `Send Chat Message`, `Update Chat Settings`, `Update User Chat Color`, `Create Clip`, `Get Clips`,
  `Delete Conduit`, `Delete Eventsub Subscription`, `Get Conduit Shards`, `Get Eventsub Subscriptions`,
  `Update Conduit`, `Update Conduit Shards`, `Get Games`, `Get Top Games`, `Get Creator Goals`,
  `Get Hypetrain Events`, `Add Blocked Term`, `Add Channel Moderator`, `Ban User`, `Check Automod Status`,
  `Delete Chat Messages`, `Get Automod Settings`, `Get Banned Users`, `Get Blocked Terms`,
  `Get Moderated Channels`, `Get Moderators`, `Get Shield Mode Status`, `Get Unban Requests`,
  `Manage Held Automod Messages`, `Remove Blocked Term`, `Remove Channel Moderator`, `Resolve Unban Request`,
  `Unban User`, `Update Automod Settings`, `Update Shield Mode Status`, `Warn Chat User`,
  `Create Custom Rewards`, `Delete Custom Reward`, `Get Custom Reward`, `Get Custom Reward Redemption`,
  `Update Custom Reward`, `Update Redemption Status`, `Create Poll`, `End Poll`, `Get Polls`,
  `Create Prediction`, `End Prediction`, `Get Predictions`, `Cancel A Raid`, `Start A Raid`,
  `Create Channel Stream Schedule Segment`, `Delete Channel Stream Schedule Segment`,
  `Get Channel Stream Schedule`, `Update Channel Stream Schedule`, `Update Channel Stream Schedule Segment`,
  `Search Categories`, `Search Channels`, `Create Stream Marker`, `Get Followed Streams`, `Get Stream Key`,
  `Get Stream Markers`, `Get Stream Tags`, `Get Streams`, `Replace Stream Tags`, `Check User Subscription`,
  `Get Broadcaster Subscriptions`, `Get Broadcaster Subscriptions Events`, `Get All Stream Tags`,
  `Get Channel Teams`, `Get Teams`, `Block User`, `Get User Active Extensions`, `Get User Block List`,
  `Get Users`, `Get Users Follows`, `Unblock User`, `Update User`, `Update User Extensions`,
  `Delete Videos`, `Get Videos`, `Send Whisper`
- Added the following eventsub topics:
  `Automod Message Hold V1`, `Automod Message Hold V2`, `Automod Message Update V1`,
  `Automod Message Update V2`, `Automod Settings Update V1`, `Automod Terms Update V1`,
  `Channel Ad Break Begin V1`, `Channel Points Automatic Reward Redemption Add V1`,
  `Channel Charity Campaign Donate V1`, `Channel Charity Campaign Progress V1`,
  `Channel Charity Campaign Start V1`, `Channel Charity Campaign Stop V1`,
  `Channel Chat Clear V1`, `Channel Chat Clear User Messages V1`, `Channel Chat Message V1`,
  `Channel Chat Message Delete V1`, `Channel Chat Notification V1`, `Channel Chat User Message Hold V1`,
  `Channel Chat User Message Update V1`, `Channel Chat Settings Update V1`,
  `Channel Follow V2`, `Channel Guest Star Guest Update Beta`, `Channel Guest Star Session Begin Beta`,
  `Channel Guest Star Session End Beta`, `Channel Guest Star Settings Update Beta`,
  `Channel Moderate V1`, `Channel Moderate V2`, `Channel Moderator Add V1`,
  `Channel Moderator Remove V1`, `Channel Shared Chat Begin V1`, `Channel Shared Chat End V1`,
  `Channel Shared Chat Update V1`, `Channel Shield Mode Begin V1`, `Channel Shield Mode End V1`,
  `Channel Shoutout Create V1`, `Channel Shoutout Receive V1`, `Channel Suspicious User Message V1`,
  `Channel Suspicious User Update V1`, `Channel Unban Request Create V1`,
  `Channel Unban Request Resolve V1`, `Channel Update V2`, `Channel Vip Add V1`,
  `Channel Vip Remove V1`, `Channel Warning Acknowledge V1`, `Channel Warning Send V1`,
  `Conduit Shard Disabled V1`, `User Whisper Message V1`
- Added `tags` to `Modify Channel Information`, `Get Channel Information`, `Search Channels`, `Get Followed Streams` and `Get Streams`
- Added `beta` feature to specifically enable beta endpoints
- Added `type` and `user_id` query params to `GetEventSubSubscriptionsRequest`
- Added `vod_offset` to `Clip`s
- Added `Delete Channel Stream Schedule Segment` that was accidentally not included in `0.6.0`
- Added `sort` and `id` to `Get Custom Reward Redemption`
- Added contribution type `OTHER` for Get Hype Train Events
- Added `tower` implementation for `HttpClient`
- Added `helix::Response::new` and `helix::Response::with_data` to make it possible to create your own responses.
- Added `is_branded_content` and `content_classification_labels` to `Get Channel Information` and `Modify Channel information`
- Added `HelixClient` functions `get_streams_from_logins` and `get_streams_from_ids`
- Added `is_featured` to Get Clips
- Added `title` and `description` as fields in the response of `Get Channel Chat Badges` and `Get Global Chat Badges`

### Fixed

- Handle `teams/channel` (Get Channel Teams) returning a null value for data

## [v0.6.1] - 2022-04-29

- No changes, only a fix for docs.rs output and some version bumps in Cargo.toml

[Commits](https://github.com/twitch-rs/twitch_api/compare/v0.6.0...v0.6.1)

## [v0.6.0] - 2022-04-28

[Commits](https://github.com/twitch-rs/twitch_api/compare/v0.5.0...v0.6.0)

### Added

- Added Helix endpoints:
  - `Manage Held AutoMod Messages`
  - `Get Global Chat Badges`
  - `Get Channel Chat Badges`
  - `Get Channel Emotes`
  - `Get Global Emotes`
  - `Get Emote Sets`
  - `Get Channel Stream Schedule`
  - `Update Channel Stream Schedule`
  - `Create Channel Stream Schedule Segment`
  - `Delete Channel Stream Schedule Segment`
  - `Get Creator Goals`
  - `Add Blocked Term`
  - `Get Blocked Terms`
  - `Remove Blocked Term`
  - `Unban User`
  - `Ban User`
  - `Update Chat Settings`
- Added Channel Terms to pubsub `chat_moderator_actions`
- Added `user-moderation-notifications` topic to pubsub
- Added `extendsub` to pubsub `channel-subscribe-events-v1`
- Added `delay` to `Get Channel Information`
- Added `serde::Serialize` to all helix endpoint return values
- Added `channel.subscription.end` to EventSub
- Added `channel.subscription.gift` to EventSub
- Added `channel.subscription.message` to EventSub
- Added `user.authorization.grant` to EventSub
- Added `channel.goal.begin`, `channel.goal.progress` and `channel.goal.end` to EventSub
- Added `helix::make_stream` to make streams out of paginated responses.
- Added fields `moderator_id`,`moderator_login`,`moderator_name` and `reason` to `BannedUser`
- Added `pubsub::unlisten_command`
- Added `moderator_removed` as a moderation action to PubSub.
- Added `EmoteUrlBuilder` to make an url with `EmoteId::url()` and `ChannelEmote::url()`
- Added methods to `Timestamp` for constructing and handling them. Uses the `time` crate behind the `time` feature.
- `twitch_oauth2` has been upgraded, and following this upgrade, `HelixClient`, `TmiClient` and `TwitchClient` can be used as clients for token requests.
- Added field `game_name` to `Streams`
- Added function `get_follow_relationships`, `get_broadcaster_subscribers`, `get_global_emotes`, `get_channel_emotes_from_id`, `get_channel_emotes_from_login` and `get_emote_sets` to `HelixClient`
- Added fields `format`, `scale`, `theme_mode` and `template` to `ChannelEmote`, `GetEmoteSets` and `GlobalEmote`
- Added functions `HelixClient::req_<method>_custom` to return a specific struct/enum defined by the user. This also enables references in responses for these functions.
- Added `HypeTrainId` to relevant eventsub and helix endpoints.

### Changed

- MSRV: 1.60.0
- Made all specific "string" types actual types with [`aliri_braid`](https://crates.io/crates/aliri_braid)
- Deprecated specific term actions in `ChatModeratorActionsReply`, replacing them with `ChannelTermsAction`
- Deprecated `Vip` action in `ChatModeratorActionsReply`, replacing it with `VipAdded`
- Removed some derived impls and fixed builders that assumed a default wrongly.
- `HelixClient::new`, `TmiClient::new` and `TwitchClient::new` now give a more specified client.
- Added total and gifter information to helix `Get Broadcaster Subscriptions`
- `HelixClient` methods `search_categories`, `search_channels`, `get_followed_streams` and `get_moderators_in_channel_from_id` now use streams to provide paginated response.
- Renamed `BroadcasterType::Affiliated` -> `BroadcasterType::Affiliate`
- Client extension methods that are paginated are now paginated lazily using a stream.
- `pubsub::listen_command` now accepts `Into<Option<&str>>` as the `auth_token`.
- `pubsub::Topics` and all topics now implement `Clone` and `Hash`.
- `TWITCH_HELIX_URL`, `TWITCH_TMI_URL` and `TWITCH_PUBSUB_URL` are now `url::Url`s and can be overridden with environment variables. See the docs for more information.
- Added field total to `helix::Response`
- Changed return type of `GetBroadcasterSubscriptions` to be a vector of `BroadcasterSubscription`
- Made `Payload::verify` and `Payload::parse_http` generic on the body type for `AsRef<[u8]>`
- Made `Payload` in EventSub enumerate over the different message types, `verification`, `revocation` and `notification`. Adds a new `Notification` type which holds the different notification payloads. `Payload` is now `Event` and old `NotificationPayload` is now called `Payload`, this is because a payload can be a revocation, notification or verification.

### Removed

- Removed enum variants for a lot of error states in helix endpoint responses. Most of these are returned by `HelixRequest_Error::Error`
- Removed deprecated follow endpoints `Create Users Follows` and `Delete Users Follows`
- Removed helix `webhook`s and `HelixRequestGetError::InvalidUri`
- Removed TMI `get_hosts`
- Removed Helix `Get Banned Events` and `Get Moderator Events`

## [v0.5.0] - 2021-05-08

[Commits](https://github.com/twitch-rs/twitch_api/compare/v0.4.1...v0.5.0)

### Added

- Made crate runtime agnostic with custom clients using feature `client`.
- Added `unsupported` feature to enable experimental/undocumented APIs/endpoints/topics.
- Made most fields deny unknown fields by enabling feature `deny_unknown_fields`.
- Added `all` feature to enable all feature sans `unsupported` and `deny_unknown_fields`.
- Added most PubSub topics.
  - channel-bits-badge-unlocks
  - channel-bits-events-v2
  - channel-cheer-events-public-v1
  - channel-points-channel-v1
  - channel-sub-gifts-v1
  - channel-subscribe-events-v1
  - chat_moderator_actions
  - community-points-channel-v1
  - following
  - hype-train-events-v1
  - hype-train-events-v1.rewards
  - raid
  - video-playback
  - video-playback-by-id
- Added most EventSub Event Subscriptions.
  - Channel Ban V1 Event
  - Channel Cheer V1 Event
  - Channel Follow V1 Event
  - Channel Hype Train Begin V1 Event
  - Channel Hype Train End V1 Event
  - Channel Hype Train Progress V1 Event
  - Channel Points Custom Reward Add V1 Event
  - Channel Points Custom Reward Redemption Add V1 Event
  - Channel Points Custom Reward Redemption Update V1 Event
  - Channel Points Custom Reward Remove V1 Event
  - Channel Points Custom Reward Update V1 Event
  - Channel Raid V1 Event
  - Channel Subscribe V1 Event
  - Channel Unban V1 Event
  - Channel Update V1 Event
  - StreamOffline V1 Event
  - StreamOnline V1 Event
  - User Authorization Revoke V1 Event
  - User Update V1 Event
- Added most Webhook topics.
  - Channel Ban Change Events
  - Get Channel Editors
  - Hype Train Event
  - Moderator Change Events
  - Stream Changed
  - Subscription Events
  - User Changed
  - User Follows
- Added tmi endpoint `get_hosts` thanks to [waridley](https://github.com/Waridley).
- Added Helix endpoints
  - Block User
  - Check User Subscription
  - Create Custom Rewards
  - Create User Follows
  - Delete Custom Reward
  - Delete User Follows
  - Delete Videos
  - Get All Stream Tags
  - Get Bits Leaderboard
  - Get Broadcaster Subscriptions Events
  - Get Channel Editors
  - Get Cheermote
  - Get Custom Reward
  - Get Custom Reward Redemption (thanks [Dinnerbone](https://github.com/Dinnerbone))
  - Get Games
  - Get Hype Train Events
  - Get Stream Tags
  - Get User Block List
  - Get User Follows
  - Get Videos
  - Get Webhook Subscriptions
  - Search Categories
  - Search Channels
  - Unblock User
  - Update Custom Reward (thanks [FoxLisk](https://github.com/FoxLisk))
  - Update Redemption Status (thanks [Dinnerbone](https://github.com/Dinnerbone))
  - Replace Stream Tags (thanks [ModProg](https://github.com/ModProg))

### Changed

- MSRV: 1.51.0
- BREAKING: Removed `helix` and `tmi` features from default-features.
- BREAKING: Renamed `TMIClient` -> `TmiClient`
- Improved documentation
- Renamed some helix endpoint replies. [#18]
- `twitch_oauth2` dependency is now gated behind its feature flag.

## [End of Changelog]

Changelog starts on v0.5.0

# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/Emilgardis/twitch_api2/compare/0.5.0...Unreleased)

### Added

* Added Helix endpoints:
  *  `Manage Held AutoMod Messages`
  *  `Get Global Chat Badges`
  *  `Get Channel Chat Badges`
  *  `Get Channel Emotes`
  *  `Get Global Emotes`
  *  `Get Emote Sets`
  *  `Get Channel Stream Schedule`
  *  `Update Channel Stream Schedule`
  *  `Create Channel Stream Schedule Segment`
  *  `Delete Channel Stream Schedule Segment`
* Added Channel Terms to pubsub `chat_moderator_actions`
* Added `user-moderation-notifications` topic to pubsub
* Added `extendsub` to pubsub `channel-subscribe-events-v1`
* Added `delay` to `Get Channel Information`
* Added `serde::Serialize` to all helix endpoint return values
* Added `channel.subscription.end` to EventSub
* Added `channel.subscription.gift` to EventSub
* Added `channel.subscription.message` to EventSub
* Added `user.authorization.grant` to EventSub
* Added `helix::make_stream` to make streams out of paginated responses.
* Added fields `moderator_id`,`moderator_login`,`moderator_name` and `reason` to `BannedUser`
* Added `pubsub::unlisten_command`
* Added `moderator_removed` as a moderation action to PubSub.
* Added `EmoteUrlBuilder` to make an url with `EmoteId::url()`
* Added methods to `Timestamp` for constructing and handling them. Can use the `chrono` crate behind the `chrono` feature.
* New `HelixClient` method `get_follow_relationships` for getting follows.
* `twitch_oauth2` has been upgraded, and following this upgrade, `HelixClient`, `TmiClient` and `TwitchClient` can be used as clients for token requests.

### Changed

* Deprecated specific term actions in `ChatModeratorActionsReply`, replacing them with `ChannelTermsAction`
* Deprecated `Vip` action in `ChatModeratorActionsReply`, replacing it with `VipAdded`
* Removed some derived impls and fixed builders that assumed a default wrongly.
* `HelixClient::new`, `TmiClient::new` and `TwitchClient::new` now give a more specified client.
* Added total and gifter information to helix `Get Broadcaster Subscriptions`
* Made all specific "string" types actual types with [`aliri_braid`](https://crates.io/crates/aliri_braid)
* `HelixClient` methods `search_categories`, `search_channels`, `get_followed_streams` and `get_moderators_in_channel_from_id` now use streams to provide paginated response. 
* Renamed `BroadcasterType::Affiliated` -> `BroadcasterType::Affiliate`
* Client extension methods that are paginated are now paginated lazily using a stream.
* `pubsub::listen_command` now accepts `Into<Option<&str>>` as the `auth_token`.
* `pubsub::Topics` and all topics now implement `Clone` and `Hash`.
* `TWITCH_HELIX_URL`, `TWITCH_TMI_URL` and `TWITCH_PUBSUB_URL` are now `url::Url`s and can be overridden with environment variables. See the docs for more information.
* Added field total to `helix::Response`

### Removed

* Removed enum variants for a lot of error states in helix endpoint responses. Most of these are returned by `HelixRequest_Error::Error`
* Removed deprecated follow endpoints `Create Users Follows` and `Delete Users Follows`

## [v0.5.0] - 2021-05-08

[Commits](https://github.com/Emilgardis/twitch_api2/compare/v0.4.1...v0.5.0)

### Added

* Made crate runtime agnostic with custom clients using feature `client`.
* Added `unsupported` feature to enable experimental/undocumented APIs/endpoints/topics.
* Made most fields deny unknown fields by enabling feature `deny_unknown_fields`.
* Added `all` feature to enable all feature sans `unsupported` and `deny_unknown_fields`.
* Added most PubSub topics.
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
* Added most EventSub Event Subscriptions.
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
* Added most Webhook topics.
    - Channel Ban Change Events
    - Get Channel Editors
    - Hype Train Event
    - Moderator Change Events
    - Stream Changed
    - Subscription Events
    - User Changed
    - User Follows
* Added tmi endpoint `get_hosts` thanks to [waridley](https://github.com/Waridley).
* Added Helix endpoints
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

* MSRV: 1.51.0
* BREAKING: Removed `helix` and `tmi` features from default-features.
* BREAKING: Renamed `TMIClient` -> `TmiClient`
* Improved documentation
* Renamed some helix endpoint replies. [#18]
* `twitch_oauth2` dependency is now gated behind its feature flag.

## [End of Changelog] 

Changelog starts on v0.5.0
# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/Emilgardis/twitch_api2/compare/v0.4.1...Unreleased)

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
    - Channel Raid Beta Event
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
    - Create User Follows
    - Delete User Follows
    - Delete Videos
    - Get All Stream Tags
    - Get Bits Leaderboard
    - Get Broadcaster Subscriptions Events
    - Get Channel Editors
    - Get Cheermote
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
    - Update Redemption Status (thanks [Dinnerbone](https://github.com/Dinnerbone))

### Changed

* MSRV = 1.48.0
* BREAKING: Removed `helix` and `tmi` features from default-features.
* BREAKING: Renamed `TMIClient` -> `TmiClient`
* Improved documentation
* Renamed some helix endpoint replies. [#18]
* `twitch_oauth2` dependency is now gated behind its feature flag.

## [End of Changelog] 

Changelog starts on Unreleased
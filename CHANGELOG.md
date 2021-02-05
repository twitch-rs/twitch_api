# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/Emilgardis/twitch_api2/compare/v0.4.1...Unreleased)

### Added

* Made crate runtime agnostic with custom clients using feature `client`.
* Added `unsupported` feature to enable experimental/undocumented APIs/endpoints/topics.
* Made most fields deny unknown fields by default, specify feature `allow_unknown_fields` to ignore unknowns.
* Added `all` feature to enable all feature sans `unsupported` and `allow_unknown_fields`.
* Added most PubSub topics.
    - channel-bits-events-v2
    - channel-bits-badge-unlocks
    - channel-cheer-events-public-v1
    - channel-points-channel-v1
    - channel-sub-gifts-v1
    - channel-subscribe-events-v1
    - community-points-channel-v1
    - following
    - hype-train-events-v1
    - hype-train-events-v1.rewards
    - chat_moderator_actions
    - raid
    - video-playback
    - video-playback-by-id
* Added most EventSub Event Subscriptions.
    - Channel Update V1 Event
    - Channel Follow V1 Event
    - Channel Subscribe V1 Event
    - Channel Cheer V1 Event
    - Channel Ban V1 Event
    - Channel Unban V1 Event
    - Channel Points Custom Reward Add V1 Event
    - Channel Points Custom Reward Update V1 Event
    - Channel Points Custom Reward Remove V1 Event
    - Channel Points Custom Reward Redemption Add V1 Event
    - Channel Points Custom Reward Redemption Update V1 Event
    - Channel Hype Train Begin V1 Event
    - Channel Hype Train Progress V1 Event
    - Channel Hype Train End V1 Event
    - StreamOnline V1 Event
    - StreamOffline V1 Event
    - User Update V1 Event
    - User Authorization Revoke V1 Event
    - Channel Raid Beta Event
* Added most Webhook topics.
    - Hype Train Event
    - Channel Ban Change Events
    - Moderator Change Events
    - Subscription Events
    - User Changed
    - User Follows
    - Stream Changed
    - Get Channel Editors
* Added tmi endpoint `get_hosts` thanks to [waridley](https://github.com/Waridley).
* Added Helix endpoints
    - Get User Follows
    - Get Stream Tags
    - Get All Stream Tags
    - Get Games
    - Delete User Follows
    - Create User Follows
    - Search Categories
    - Search Channels
    - Get Videos
    - Get Cheermote
    - Get Bits Leaderboard
    - Get Custom Reward Redemption (thanks [Dinnerbone](https://github.com/Dinnerbone))
    - Update Redemption Status (thanks [Dinnerbone](https://github.com/Dinnerbone))
    - Get Broadcaster Subscriptions Events
    - Get Hype Train Events
    - Get Webhook Subscriptions
    - Get Channel Editors
    - Delete Videos
    - Get User Block List
    - Block User
    - Unblock User

### Changed

* MSRV = 1.48.0
* BREAKING: Removed `helix` and `tmi` features from default-features.
* Improved documentation
* Renamed some helix endpoint replies. [#18]
* `twitch_oauth2` dependency is now gated behind its feature flag.

## [End of Changelog] 

Changelog starts on Unreleased
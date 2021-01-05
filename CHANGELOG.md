# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

* Made crate runtime agnostic with custom clients using feature `client`.
* Removed `helix` and `tmi` features from default-features.
* Added `unsupported` feature to enable experimental/undocumented APIs/endpoints/topics.
* Made most fields deny unknown fields by default, specify feature `allow_unknown_fields` to ignore unknowns.
* Added `all` feature to enable all feature sans `unsupported` and `allow_unknown_fields`.
* Added most PubSub topics.
* Added most EventSub Event Subscriptions.
* Added most Webhook topics.
* Added tmi endpoint `get_hosts` thanks to [@waridley](https://github.com/Waridley).
* Helix Endpoints.
  * Added:
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
    - Get Custom Reward Redemption (thanks [@Dinnerbone])
    - Update Redemption Status (thanks [@Dinnerbone])
    - Get Broadcaster Subscriptions Events
    - Get Hype Train Events
    - Get Webhook Subscriptions
* Many more. See [comparison](https://github.com/Emilgardis/twitch_api2/compare/v0.4.1...v0.5.0) for all changes/additions


[@Dinnerbone]: https://github.com/Dinnerbone

### Changed

* Improved documentation
* Renamed some helix endpoint replies. [#18]
* `twitch_oauth2` dependency is now gated behind it's feature flag.
* More changes

## End of Changelog 

Changelog starts on v0.5.0
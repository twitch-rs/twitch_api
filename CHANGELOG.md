# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

* Made crate runtime agnostic with custom clients using feature `client`.
* Added pubsub topics. [#34] and others
* BREAKING: Removed `helix` and `tmi` features from default-features.
* Added `unsupported` feature to enable experimental/undocumented APIs/endpoints/topics.
* Added `all` feature to enable all feature sans `unsupported`
* Added tmi endpoint `get_hosts` thanks to [@waridley](https://github.com/Waridley).
* Implemented more helix endpoints.


### Changed

* Improved documentation
* Renamed some helix endpoint replies. [#18]
* `twitch_oauth2` dependency is now gated behind it's feature flag.
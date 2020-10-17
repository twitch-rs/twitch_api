Twitch API2 | Rust library for talking with the new Twitch API aka. "Helix" and TMI.
============================================

[![github]](https://github.com/emilgardis/twitch_api2)&ensp;[![crates-io]](https://crates.io/crates/twitch_api2)&ensp;[![docs-rs-big]](https://docs.rs/twitch_api2/0.4.1/twitch_api2)

 [github]: https://img.shields.io/badge/github-emilgardis/twitch__api2-8da0cb?style=for-the-badge&labelColor=555555&logo=github
 [crates-io]: https://img.shields.io/crates/v/twitch_api2.svg?style=for-the-badge&color=fc8d62&logo=rust
 [docs-rs-big]: https://img.shields.io/badge/docs.rs-twitch__api2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

See [documentation](https://docs.rs/twitch_api2) for more info.

You can see current unpublished docs here: [![local-docs]](https://emilgardis.github.io/twitch_api2/twitch_api2)

See [examples](./examples) for examples.

## Implemented endpoints

### Helix

<details><summary>Click to expand</summary><p>

#### Moderation

| Endpoint               |                                                                   |                                                                                                                                                                                    |
| :--------------------- | :---------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✔ Check AutoMod Status | `POST https://api.twitch.tv/helix/moderation/enforcements/status` | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/moderation/check_automod_status) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#check-automod-status) |
| ✔ Get Banned Users     | `GET https://api.twitch.tv/helix/moderation/banned`               | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/moderation/get_banned_users) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-banned-users)         |
| ✔ Get Banned Events    | `GET https://api.twitch.tv/helix/moderation/banned/events`        | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/moderation/get_banned_events) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-banned-events)       |
| ✔ Get Moderators       | `GET https://api.twitch.tv/helix/moderation/moderators`           | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/moderation/get_moderators) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-moderators)             |
| ✔ Get Moderator Events | `GET https://api.twitch.tv/helix/moderation/moderators/events`    | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/moderation/get_moderator_events) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-moderator-events) |


#### Channels

| Endpoint                     |                                                        |                                                                                                                                                                                              |
| :--------------------------- | :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✔ Start Commercial           | `POST https://api.twitch.tv/helix/channels/commercial` | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/channels/start_commercial) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#start-commercial)                     |
| ✔ Get Channel Information    | `GET https://api.twitch.tv/helix/channels`             | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/channels/get_channel_information) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-channel-information)       |
| ✔ Modify Channel Information | `PATCH https://api.twitch.tv/helix/channels`           | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/channels/modify_channel_information) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#modify-channel-information) |


#### Analytics

| Endpoint                  |                                                        |                                                                                                                                                                                                |
| :------------------------ | :----------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 🔨 Get Extension Analytics | `GET https://api.twitch.tv/helix/analytics/extensions` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/analytics/get_extension_analytics)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-extension-analytics) |
| 🔨 Get Game Analytics      | `GET https://api.twitch.tv/helix/analytics/games`      | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/analytics/get_game_analytics)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-game-analytics)           |


#### Bits

| Endpoint               |                                                    |                                                                                                                                                                                     |
| :--------------------- | :------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 🔨 Get Cheermotes       | `GET https://api.twitch.tv/helix/bits/cheermotes`  | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/bits/get_cheermotes)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-cheermotes)             |
| 🔨 Get Bits Leaderboard | `GET https://api.twitch.tv/helix/bits/leaderboard` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/bits/get_bits_leaderboard)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard) |


#### Extensions

| Endpoint                     |                                                           |                                                                                                                                                                                                       |
| :--------------------------- | :-------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 🔨 Get Extension Transactions | `GET https://api.twitch.tv/helix/extensions/transactions` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/extensions/get_extension_transactions)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-extension-transactions) |


#### Clips

| Endpoint      |                                          |                                                                                                                                                                    |
| :------------ | :--------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 🔨 Create Clip | `POST https://api.twitch.tv/helix/clips` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/clips/create_clip)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#create-clip) |
| ✔ Get Clips   | `GET https://api.twitch.tv/helix/clips`  | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/clips/get_clips) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-clips)            |


#### Entitlements

| Endpoint                               |                                                        |                                                                                                                                                                                                                |
| :------------------------------------- | :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 🔨 Create Entitlement Grants Upload URL | `POST https://api.twitch.tv/helix/entitlements/upload` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/create_entitlement_grants_upload_url)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#create-entitlement-grants-upload-url) |
| 🔨 Get Code Status                      | `GET https://api.twitch.tv/helix/entitlements/codes`   | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/get_code_status)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-code-status)                                           |
| 🔨 Redeem Code                          | `POST https://api.twitch.tv/helix/entitlements/code`   | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/redeem_code)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#redeem-code)                                                   |


#### Games

| Endpoint        |                                             |                                                                                                                                                                 |
| :-------------- | :------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✔ Get Top Games | `GET https://api.twitch.tv/helix/games/top` | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/games/get_top_games) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-top-games) |
| ✔ Get Games     | `GET https://api.twitch.tv/helix/games`     | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/games/get_games) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-games)         |


#### Search

| Endpoint            |                                                     |                                                                                                                                                                             |
| :------------------ | :-------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✔ Search Categories | `GET https://api.twitch.tv/helix/search/categories` | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/search/search_categories) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#search-categories)    |
| ✔ Search Channels   | `GET https://api.twitch.tv/helix/search/channels`   | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/search/search_channels) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#search-channels) |


#### Streams

| Endpoint               |                                                    |                                                                                                                                                                                        |
| :--------------------- | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 🔨 Get Stream Key       | `https://api.twitch.tv/helix/streams/key`          | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/streams/get_stream_key)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-stream-key)             |
| ✔ Get Streams          | `GET https://api.twitch.tv/helix/streams`          | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/streams/get_streams) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-streams)                          |
| 🔨 Create Stream Marker | `POST https://api.twitch.tv/helix/streams/markers` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/streams/create_stream_marker)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#create-stream-marker) |
| 🔨 Get Stream Markers   | `GET https://api.twitch.tv/helix/streams/markers`  | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/streams/get_stream_markers)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-stream-markers)     |
| ✔ Get Stream Tags      | `GET https://api.twitch.tv/helix/streams/tags`     | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/streams/get_stream_tags) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-stream-tags)                  |
| 🔨 Replace Stream Tags  | `PUT https://api.twitch.tv/helix/streams/tags`     | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/streams/replace_stream_tags)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#replace-stream-tags)   |


#### Subscriptions

| Endpoint                        |                                                 |                                                                                                                                                                                                         |
| :------------------------------ | :---------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| ✔ Get Broadcaster Subscriptions | `GET https://api.twitch.tv/helix/subscriptions` | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/subscriptions/get_broadcaster_subscriptions) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions) |


#### Tags

| Endpoint              |                                                |                                                                                                                                                                            |
| :-------------------- | :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✔ Get All Stream Tags | `GET https://api.twitch.tv/helix/tags/streams` | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/tags/get_all_stream_tags) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-all-stream-tags) |


#### Users

| Endpoint                     |                                                         |                                                                                                                                                                                                  |
| :--------------------------- | :------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✔ Create User Follows        | `POST https://api.twitch.tv/helix/users/follows`&nbsp;  | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/users/create_user_follows) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#create-user-follows)                      |
| ✔ Delete User Follows        | `DELETE https://api.twitch.tv/helix/users/follows`      | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/users/delete_user_follows) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#delete-user-follows)                      |
| ✔ Get Users                  | `GET https://api.twitch.tv/helix/users`                 | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/users/get_users) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-users)                                          |
| ✔ Get Users Follows          | `GET https://api.twitch.tv/helix/users/follows`         | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/users/get_users_follows) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-users-follows)                          |
| 🔨 Update User                | `PUT https://api.twitch.tv/helix/users`                 | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/users/update_user)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#update-user)                               |
| 🔨 Get User Extensions        | `GET https://api.twitch.tv/helix/users/extensions/list` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/users/get_user_extensions)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-user-extensions)               |
| 🔨 Get User Active Extensions | `GET https://api.twitch.tv/helix/users/extensions`      | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/users/get_user_active_extensions)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions) |
| 🔨 Update User Extensions     | `PUT https://api.twitch.tv/helix/users/extensions`      | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/users/update_user_extensions)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#update-user-extensions)         |


#### Videos

| Endpoint     |                                          |                                                                                                                                                            |
| :----------- | :--------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✔ Get Videos | `GET https://api.twitch.tv/helix/videos` | [![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/videos/get_videos) [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-videos) |


#### Webhooks

| Endpoint                    |                                                          |                                                                                                                                                                                                   |
| :-------------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 🔨 Get Webhook Subscriptions | `GET https://api.twitch.tv/helix/webhooks/subscriptions` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/webhooks/get_webhook_subscriptions)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-webhook-subscriptions) |
        
#### Hypetrain

| Endpoint                |                                                    |                                                                                                                                                                                             |
| :---------------------- | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 🔨 Get Hype Train Events | `GET https://api.twitch.tv/helix/hypetrain/events` | <!--[![docs-rs]](https://docs.rs/twitch_api2/*/twitch_api2/helix/hypetrain/get_hype_train_events)--> [![twitch-reference]](https://dev.twitch.tv/docs/api/reference#get-hype-train-events)) |

</p></details>

### TMI

<details><summary>Click to expand</summary><p>

| Endpoint       |                                                           |
| :------------- | :-------------------------------------------------------- |
| ✔ Get Chatters | `https://tmi.twitch.tv/group/user/{broadcaster}/chatters` |
| ✔ Get Hosts    | `https://tmi.twitch.tv/hosts`                             |

</p></details>

<h5> License </h5>

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[local-docs]: https://img.shields.io/github/workflow/status/Emilgardis/twitch_api2/github%20pages/master?label=docs&style=flat-square&event=push
[docs-rs]: https://img.shields.io/badge/docs-66c2a5?style=flat-square&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
[twitch-reference]: https://img.shields.io/badge/api-blue?style=flat-square&logoColor=white&logo=twitch

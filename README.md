# Twitch API | Rust library for talking with the new Twitch API aka. "Helix", EventSub and more!

[![github]](https://github.com/twitch-rs/twitch_api)&ensp;[![crates-io]](https://crates.io/crates/twitch_api)&ensp;[![docs-rs-big]](https://docs.rs/twitch_api/0.7.1/twitch_api/)&ensp;[![discord]](https://discord.gg/7APWQeEmnK)

[github]: https://img.shields.io/badge/github-twitch--rs/twitch__api-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/crates/v/twitch_api.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-rs-big]: https://img.shields.io/badge/docs.rs-twitch__api-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
[discord]: https://img.shields.io/discord/325552783787032576?logo=discord&style=for-the-badge

See [documentation](https://docs.rs/twitch_api) for more info.

You can see current unpublished docs for the main branch here: [![local-docs]](https://twitch-rs.github.io/twitch_api/twitch_api)

See [examples](./examples) for examples. If you want to run them locally,
make sure you [get the git submodules](./CONTRIBUTING.md#fetching-the-git-submodules) first.

[local-docs]: https://img.shields.io/github/actions/workflow/status/twitch-rs/twitch_api/gh-pages.yml?label=dev%20docs&style=flat-square&event=push

```rust ,no_run
use twitch_api::helix::HelixClient;
use twitch_api::twitch_oauth2::{AccessToken, UserToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Create the HelixClient, which is used to make requests to the Twitch API
    let client: HelixClient<reqwest::Client> = HelixClient::default();
    // Create a UserToken, which is used to authenticate requests
    let token = UserToken::from_token(&client, AccessToken::from("mytoken")).await?;

    println!(
        "Channel: {:?}",
        client.get_channel_from_login("twitchdev", &token).await?
    );

    Ok(())
}
```

## Notes

This crate was previously available as `twitch_api2` and has since been renamed to `twitch_api`

## Goals

This crate aims to target

- [Helix](https://dev.twitch.tv/docs/api/reference)
  - See [implemented endpoints](https://github.com/twitch-rs/twitch_api/wiki/Implemented-Features#helix)
- [EventSub](https://dev.twitch.tv/docs/eventsub/eventsub-reference)
  - See [implemented eventsub Helix endpoints](https://github.com/twitch-rs/twitch_api/wiki/Implemented-Features#eventsub)
- [Extensions](https://dev.twitch.tv/docs/extensions/reference)
  - No functionality implemented yet
- [PubSub](https://dev.twitch.tv/docs/pubsub) (without a client)
  - See [implemented topics](https://github.com/twitch-rs/twitch_api/wiki/Implemented-Features#pubsub)
  - PubSub [is deprecated](https://discuss.dev.twitch.com/t/legacy-pubsub-deprecation-and-shutdown-timeline/58043), replaced with EventSub websockets

There are no current plans to support

- [GraphQL](https://github.com/mauricew/twitch-graphql-api)
- [Drops](https://dev.twitch.tv/docs/drops) (except what is in Helix)
- [Twitch IRC Chat](https://dev.twitch.tv/docs/irc), use [museun/twitch_message](https://github.com/museun/twitch_message) or [robotty/twitch-irc](https://github.com/robotty/twitch-irc-rs/)
- [Authentication](https://dev.twitch.tv/docs/authentication), use [twitch-rs/twitch_oauth2](https://github.com/twitch-rs/twitch_oauth2)
- Undocumented Helix endpoints, i.e endpoints mobile Twitch app uses. Including [working "hidden" endpoints](https://thomassen.sh/twitch-api-endpoints/)

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

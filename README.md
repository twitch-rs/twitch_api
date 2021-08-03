Twitch API2 | Rust library for talking with the new Twitch API aka. "Helix", TMI and more!
============================================

[![github]](https://github.com/emilgardis/twitch_api2)&ensp;[![crates-io]](https://crates.io/crates/twitch_api2)&ensp;[![docs-rs-big]](https://docs.rs/twitch_api2/0.6.0-rc.2/twitch_api2)

 [github]: https://img.shields.io/badge/github-emilgardis/twitch__api2-8da0cb?style=for-the-badge&labelColor=555555&logo=github
 [crates-io]: https://img.shields.io/crates/v/twitch_api2.svg?style=for-the-badge&color=fc8d62&logo=rust
 [docs-rs-big]: https://img.shields.io/badge/docs.rs-twitch__api2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

See [documentation](https://docs.rs/twitch_api2) for more info.

You can see current unpublished docs here: [![local-docs]](https://emilgardis.github.io/twitch_api2/twitch_api2)

See [examples](./examples) for examples.

[local-docs]: https://img.shields.io/github/workflow/status/Emilgardis/twitch_api2/github%20pages/master?label=master%20docs&style=flat-square&event=push

```rust ,no_run
use twitch_api2::helix::HelixClient;
use twitch_oauth2::{AccessToken, UserToken};
use reqwest::Client as ReqwestClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
  let client: HelixClient<ReqwestClient> =  HelixClient::default();

    let token = UserToken::from_existing(
        &client,
        AccessToken::new("mytoken".to_string()),
        None, // Refresh Token
        None, // Client Secret
    )
    .await?;

    println!("Channel: {:?}",
            client.get_channel_from_login("twitchdev", &token).await?
    );

    Ok(())
}
```

## Goals

This crate aims to target

* [Helix](https://dev.twitch.tv/docs/api/reference)
  * See [implemented endpoints](https://github.com/Emilgardis/twitch_api2/wiki/Implemented-Features#helix)
* TMI
  * See [implemented endpoints](https://github.com/Emilgardis/twitch_api2/wiki/Implemented-Features#tmi)
* [EventSub](https://dev.twitch.tv/docs/eventsub/eventsub-reference)
  * See [implemented eventsub Helix endpoints](https://github.com/Emilgardis/twitch_api2/wiki/Implemented-Features#eventsub)
* [Webhooks](https://dev.twitch.tv/docs/api/webhooks-reference)
  * See [implemented webhooks](https://github.com/Emilgardis/twitch_api2/wiki/Implemented-Features#webhooks)
* [PubSub](https://dev.twitch.tv/docs/pubsub) (without a client)
  * See [implemented topics](https://github.com/Emilgardis/twitch_api2/wiki/Implemented-Features#pubsub)
* [Extensions](https://dev.twitch.tv/docs/extensions/reference)

This crate should also be able to be used for

* some [v5 Kraken services](https://dev.twitch.tv/docs/v5)
* [GraphQL](https://github.com/mauricew/twitch-graphql-api)
* Useful undocumented Helix endpoints, i.e endpoints mobile Twitch app uses. Including [working "hidden" endpoints](https://thomassen.sh/twitch-api-endpoints/)

There are no current plans to support

* [Drops](https://dev.twitch.tv/docs/drops) (except what is in Helix)
* [Twitch IRC Chat](https://dev.twitch.tv/docs/irc), use [museun/twitchchat](https://github.com/museun/twitchchat)
* [Authentication](https://dev.twitch.tv/docs/authentication), use [Emilgardis/twitch_oauth2](https://github.com/Emilgardis/twitch_oauth2)


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


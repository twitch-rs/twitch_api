# EventSub Websocket Example

This example shows how to use the EventSub websocket to listen to events.

Ensure you've [gotten the git submodules](../../CONTRIBUTING.md#fetching-the-git-submodules) to [run this example](../README.md#running-the-examples)

```sh
git clone https://github.com/twitch-rs/twitch_api.git --recurse-submodules
cd twitch_api
# if you didn't get the submodules, run
# git submodule update --init --recursive
cargo run -p eventsub_websocket -- --access-token <token>
```

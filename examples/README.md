# Examples

## Getting a token

To run the examples, you will need to have a Twitch OAuth token. You can get one by following the [Twitch OAuth guide](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth).

There are sites available that can help you generate these tokens, or you can use the official [Twitch CLI](https://github.com/twitchdev/twitch-cli), `twitch_oauth2::UserToken::builder()` or `twitch_oauth2::tokens::ImplicitUserTokenBuilder`

## Running the examples

To run an example, ensure you've gotten the [submodules](../CONTRIBUTING.md#fetching-the-git-submodules) and have a [token](#getting-a-token) available.

```sh
git clone https://github.com/twitch-rs/twitch_api.git --recurse-submodules
cd twitch_api
# if you didn't get the submodules, run
# git submodule update --init --recursive
cargo run --example <example_name> -- <token>
```

Some examples are their own crates/workspace members, you can run these with

```sh
cargo run -p <example> -- <args>
```

## .env

Instead of passing a token to every example, you can also create a `.env` file in the root of the repository with the following contents

```txt
# .env
TWITCH_TOKEN=mytoken
CLIENT_ID=myclientid
CLIENT_SECRET=myclientid
```

All of these are optional, you do not need to specify a value if the example doesn't need it or it's supplied as an argument in the commandline or from the environment.

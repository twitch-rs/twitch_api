#![doc(alias = "user")]
#![allow(deprecated)]
//! Helix endpoints regarding users
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::{helix::{HelixClient, users::GetUsersRequest}, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetUsersRequest::logins(&["justintvfan"]);
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Users 🟡 4/8</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Users](https://dev.twitch.tv/docs/api/reference#get-users) | [`HelixClient::get_user_from_id`](crate::helix::HelixClient::get_user_from_id), [`HelixClient::get_user_from_login`](crate::helix::HelixClient::get_user_from_login), [`HelixClient::get_users_from_ids`](crate::helix::HelixClient::get_users_from_ids) | [`get_users`] |
//! | [Update User](https://dev.twitch.tv/docs/api/reference#update-user) | - | - |
//! | [Get User Block List](https://dev.twitch.tv/docs/api/reference#get-user-block-list) | - | [`get_user_block_list`] |
//! | [Block User](https://dev.twitch.tv/docs/api/reference#block-user) | [`HelixClient::block_user`](crate::helix::HelixClient::block_user) | [`block_user`] |
//! | [Unblock User](https://dev.twitch.tv/docs/api/reference#unblock-user) | [`HelixClient::unblock_user`](crate::helix::HelixClient::unblock_user) | [`unblock_user`] |
//! | [Get User Extensions](https://dev.twitch.tv/docs/api/reference#get-user-extensions) | - | - |
//! | [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions) | - | - |
//! | [Update User Extensions](https://dev.twitch.tv/docs/api/reference#update-user-extensions) | - | - |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->
use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod block_user;
pub mod get_user_block_list;
pub mod get_users;
pub mod get_users_follows;
pub mod unblock_user;

#[doc(inline)]
pub use block_user::{BlockUser, BlockUserRequest};
#[doc(inline)]
pub use get_user_block_list::{GetUserBlockListRequest, UserBlock};
#[doc(inline)]
pub use get_users::{GetUsersRequest, User};
#[doc(inline)]
pub use get_users_follows::{FollowRelationship, GetUsersFollowsRequest, UsersFollows};
#[doc(inline)]
pub use unblock_user::{UnblockUser, UnblockUserRequest};

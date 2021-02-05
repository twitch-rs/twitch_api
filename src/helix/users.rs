#![doc(alias = "user")]
//! Helix endpoints regarding users
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, users::GetUsersRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetUsersRequest::builder()
//!     .login(vec!["justinfan1337".to_string()])
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
use crate::{helix, types};
use serde::{Deserialize, Serialize};

pub mod block_user;
pub mod create_user_follows;
pub mod delete_user_follows;
pub mod get_user_block_list;
pub mod get_users;
pub mod get_users_follows;
pub mod unblock_user;

#[doc(inline)]
pub use block_user::{BlockUser, BlockUserRequest};
#[doc(inline)]
pub use create_user_follows::{CreateUserFollows, CreateUserFollowsBody, CreateUserFollowsRequest};
#[doc(inline)]
pub use delete_user_follows::{DeleteUserFollow, DeleteUserFollowsRequest};
#[doc(inline)]
pub use get_user_block_list::{GetUserBlockListRequest, UserBlockList};
#[doc(inline)]
pub use get_users::{GetUsersRequest, User};
#[doc(inline)]
pub use get_users_follows::{GetUsersFollowsRequest, UsersFollow};
#[doc(inline)]
pub use unblock_user::{UnblockUser, UnblockUserRequest};

//! Endpoints and topics for webhooks
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, webhooks::{hub::{self, WebhookHubRequest, WebhookHubBody}, topics}};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = WebhookHubRequest::<topics::users::UserFollowsTopic>::builder().build();
//! let body = WebhookHubBody::builder()
//!     .callback("https://example.com/this-is-a-callback")
//!     .lease_seconds(864000)
//!     .mode(hub::WebhookSubscriptionMode::Subscribe)
//!     .secret("12233213890390".to_string())
//!     .topic(topics::users::UserFollowsTopic::builder().from_id(1336.to_string()).build())
//!     .build();
//!
//! client.req_post(req, body, &token).await?;
//! # Ok(())
//! # }
//! ```

use crate::{helix, types};
use serde::{Deserialize, Serialize};

pub mod get_webhook_subscriptions;
pub mod hub;
pub mod topics;

pub use topics::Topic;

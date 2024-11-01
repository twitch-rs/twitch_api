#![doc(alias = "channel")]
//! Helix endpoints regarding channels
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::helix::{HelixClient, channels::GetChannelInformationRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let ids: &[&twitch_types::UserIdRef] = &["1234".into()];
//! let req = GetChannelInformationRequest::broadcaster_ids(ids);
//!
//! // Get Channel Information Request only returns one entry.
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Ads 🟢 3/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Start Commercial](https://dev.twitch.tv/docs/api/reference#start-commercial) | - | [`start_commercial`] |
//! | [Get Ad Schedule](https://dev.twitch.tv/docs/api/reference#get-ad-schedule) | - | [`get_ad_schedule`] |
//! | [Snooze Next Ad](https://dev.twitch.tv/docs/api/reference#snooze-next-ad) | - | [`snooze_next_ad`] |
//!
//! </details>
//!
//! <details open><summary style="cursor: pointer">Channels 🟢 5/5</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information) | - | [`get_channel_information`] |
//! | [Modify Channel Information](https://dev.twitch.tv/docs/api/reference#modify-channel-information) | - | [`modify_channel_information`] |
//! | [Get Channel Editors](https://dev.twitch.tv/docs/api/reference#get-channel-editors) | - | [`get_channel_editors`] |
//! | [Get Followed Channels](https://dev.twitch.tv/docs/api/reference#get-followed-channels) | [`HelixClient::get_followed_channels`](crate::helix::HelixClient::get_followed_channels) | [`get_followed_channels`] |
//! | [Get Channel Followers](https://dev.twitch.tv/docs/api/reference#get-channel-followers) | - | [`get_channel_followers`] |
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

pub mod add_channel_vip;
pub mod get_ad_schedule;
pub mod get_channel_editors;
pub mod get_channel_followers;
pub mod get_channel_information;
pub mod get_followed_channels;
pub mod get_vips;
pub mod modify_channel_information;
pub mod remove_channel_vip;
pub mod snooze_next_ad;
pub mod start_commercial;

#[doc(inline)]
pub use add_channel_vip::{AddChannelVipRequest, AddChannelVipResponse};
#[doc(inline)]
pub use get_ad_schedule::{AdSchedule, GetAdScheduleRequest};
#[doc(inline)]
pub use get_channel_editors::{Editor, GetChannelEditorsRequest};
#[doc(inline)]
pub use get_channel_followers::{Follower, GetChannelFollowersRequest};
#[doc(inline)]
pub use get_channel_information::{ChannelInformation, GetChannelInformationRequest};
#[doc(inline)]
pub use get_followed_channels::{FollowedBroadcaster, GetFollowedChannels};
#[doc(inline)]
pub use get_vips::{GetVipsRequest, Vip};
#[doc(inline)]
pub use modify_channel_information::{
    ContentClassificationLabel, ModifyChannelInformation, ModifyChannelInformationBody,
    ModifyChannelInformationRequest,
};
#[doc(inline)]
pub use remove_channel_vip::{RemoveChannelVipRequest, RemoveChannelVipResponse};
pub use snooze_next_ad::{SnoozeNextAdRequest, SnoozedAdSchedule};
#[doc(inline)]
pub use start_commercial::{StartCommercial, StartCommercialBody, StartCommercialRequest};

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
//! <details open><summary style="cursor: pointer">Users 🟡 8/9</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Users](https://dev.twitch.tv/docs/api/reference#get-users) | [`HelixClient::get_user_from_id`](crate::helix::HelixClient::get_user_from_id), [`HelixClient::get_user_from_login`](crate::helix::HelixClient::get_user_from_login), [`HelixClient::get_users_from_ids`](crate::helix::HelixClient::get_users_from_ids), [`HelixClient::get_users_from_logins`](crate::helix::HelixClient::get_users_from_logins) | [`get_users`] |
//! | [Update User](https://dev.twitch.tv/docs/api/reference#update-user) | [`HelixClient::update_user_description`](crate::helix::HelixClient::update_user_description) | [`update_user`] |
//! | [Get Authorization By User](https://dev.twitch.tv/docs/api/reference#get-authorization-by-user) | - | - |
//! | [Get User Block List](https://dev.twitch.tv/docs/api/reference#get-user-block-list) | - | [`get_user_block_list`] |
//! | [Block User](https://dev.twitch.tv/docs/api/reference#block-user) | [`HelixClient::block_user`](crate::helix::HelixClient::block_user) | [`block_user`] |
//! | [Unblock User](https://dev.twitch.tv/docs/api/reference#unblock-user) | [`HelixClient::unblock_user`](crate::helix::HelixClient::unblock_user) | [`unblock_user`] |
//! | [Get User Extensions](https://dev.twitch.tv/docs/api/reference#get-user-extensions) | [`HelixClient::get_user_extensions`](crate::helix::HelixClient::get_user_extensions) | [`get_user_extensions`] |
//! | [Get User Active Extensions](https://dev.twitch.tv/docs/api/reference#get-user-active-extensions) | [`HelixClient::get_user_active_extensions`](crate::helix::HelixClient::get_user_active_extensions) | [`get_user_active_extensions`] |
//! | [Update User Extensions](https://dev.twitch.tv/docs/api/reference#update-user-extensions) | - | [`update_user_extensions`] |
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
pub mod get_user_active_extensions;
pub mod get_user_block_list;
pub mod get_user_extensions;
pub mod get_users;
pub mod get_users_follows;
pub mod unblock_user;
pub mod update_user;
pub mod update_user_extensions;

#[doc(inline)]
pub use block_user::{BlockUser, BlockUserRequest};
#[doc(inline)]
pub use get_user_active_extensions::{ExtensionConfiguration, GetUserActiveExtensionsRequest};
#[doc(inline)]
pub use get_user_block_list::{GetUserBlockListRequest, UserBlock};
#[doc(inline)]
pub use get_user_extensions::{Extension, ExtensionType, GetUserExtensionsRequest};
#[doc(inline)]
pub use get_users::{GetUsersRequest, User};
#[doc(inline)]
pub use get_users_follows::{FollowRelationship, GetUsersFollowsRequest, UsersFollows};
#[doc(inline)]
pub use unblock_user::{UnblockUser, UnblockUserRequest};
#[doc(inline)]
pub use update_user::UpdateUserRequest;
#[doc(inline)]
pub use update_user_extensions::{
    ExtensionSpecification, UpdateUserExtensionsBody, UpdateUserExtensionsRequest,
};

/// A slot for an extension to be active in
#[derive(PartialEq, Eq, Debug, Clone)]
#[non_exhaustive]
pub enum ExtensionSlot<T> {
    /// The slot is not configured
    Inactive,
    /// The slot is configured
    Active(T),
}

impl<T: serde::Serialize> serde::Serialize for ExtensionSlot<T> {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Inactive => {
                let mut state = serde::Serializer::serialize_struct(ser, "ExtensionSlot", 1)?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "active", &false)?;
                serde::ser::SerializeStruct::end(state)
            }
            Self::Active(it) => it.serialize(ActiveTaggedSerializer { delegate: ser }),
        }
    }
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for ExtensionSlot<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::de::Deserializer<'de> {
        let mut map = serde_json::Map::deserialize(deserializer)?;

        let active = map
            .remove("active")
            .ok_or_else(|| serde::de::Error::missing_field("active"))
            .map(serde::Deserialize::deserialize)?
            .map_err(serde::de::Error::custom)?;
        let rest = serde_json::Value::Object(map);

        if active {
            T::deserialize(rest)
                .map(Self::Active)
                .map_err(serde::de::Error::custom)
        } else {
            Ok(Self::Inactive)
        }
    }
}

/// Serializes a struct with an additional `"active": true` (used for active extensions).
///
/// Modelled after serde's internal TaggedSerializer
struct ActiveTaggedSerializer<S> {
    delegate: S,
}

impl<S: serde::Serializer> ActiveTaggedSerializer<S> {
    fn bad_type<T>(self) -> Result<T, S::Error> {
        Err(serde::ser::Error::custom(
            "cannot serialize with anything other than a struct",
        ))
    }
}

impl<S: serde::Serializer> serde::Serializer for ActiveTaggedSerializer<S> {
    type Error = S::Error;
    type Ok = S::Ok;
    type SerializeMap = serde::ser::Impossible<S::Ok, S::Error>;
    type SerializeSeq = serde::ser::Impossible<S::Ok, S::Error>;
    type SerializeStruct = S::SerializeStruct;
    type SerializeStructVariant = serde::ser::Impossible<S::Ok, S::Error>;
    type SerializeTuple = serde::ser::Impossible<S::Ok, S::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<S::Ok, S::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<S::Ok, S::Error>;

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let mut state = self.delegate.serialize_struct(name, len + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "active", &true)?;
        Ok(state)
    }

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_some<T: ?Sized + serde::Serialize>(self, _: &T) -> Result<Self::Ok, Self::Error> {
        self.bad_type()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { self.bad_type() }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        self.bad_type()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.bad_type()
    }

    fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.bad_type()
    }

    fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.bad_type()
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.bad_type()
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.bad_type()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.bad_type()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.bad_type()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.bad_type()
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.bad_type()
    }
}

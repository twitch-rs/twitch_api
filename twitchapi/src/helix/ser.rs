//! application/x-www-form-urlencoded serializer. that follows twitch spec
//!
//! `foo=1&foo=2&foo=3`

use std::borrow::Cow;

use serde::ser::{self, Impossible, Serialize};
use url::form_urlencoded::Serializer as UrlEncodedSerializer;

pub fn to_string<T: ser::Serialize>(input: T) -> Result<String, Error> {
    let mut urlencoder = UrlEncodedSerializer::new("".to_owned());
    input.serialize(Serializer::new(&mut urlencoder))?;
    Ok(urlencoder.finish())
}

pub struct Serializer<'input, 'output> {
    urlencoder: &'output mut UrlEncodedSerializer<'input, String>,
}

impl<'input, 'output> Serializer<'input, 'output> {
    fn new(urlencoder: &'output mut UrlEncodedSerializer<'input, String>) -> Self {
        Serializer { urlencoder }
    }
}

#[derive(Debug, thiserror::Error, displaydoc::Display)]
/// Errors from the query serializer
pub enum Error {
    /// {0}
    Custom(Cow<'static, str>),
    /// serializer only supports structs and maps on top-level
    TopLevelNotSupported {
        /// Location this was triggered
        location: &'static std::panic::Location<'static>,
    },
    /// field serializer only supports ...
    FieldNotSupported {
        /// Location this was triggered
        location: &'static std::panic::Location<'static>,
    },
    /// pair serializer only supports ...
    PairNotSupported {
        /// Location this was triggered
        location: &'static std::panic::Location<'static>,
    },
    /// value serializer only supports primitive types
    ValueNotSupported {
        /// Location this was triggered
        location: &'static std::panic::Location<'static>,
    },
}

impl Error {
    #[track_caller]
    fn top_level_not_supported() -> Self {
        Error::TopLevelNotSupported {
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    fn field_not_supported() -> Self {
        Error::FieldNotSupported {
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    fn pair_not_supported() -> Self {
        Error::PairNotSupported {
            location: std::panic::Location::caller(),
        }
    }

    #[track_caller]
    fn value_not_supported() -> Self {
        Error::PairNotSupported {
            location: std::panic::Location::caller(),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self { Error::Custom(msg.to_string().into()) }
}

impl<'input, 'output> ser::Serializer for Serializer<'input, 'output> {
    type Error = Error;
    type Ok = &'output mut UrlEncodedSerializer<'input, String>;
    type SerializeMap = MapSerializer<'input, 'output>;
    type SerializeSeq = Impossible<Self::Ok, Error>;
    type SerializeStruct = StructSerializer<'input, 'output>;
    type SerializeStructVariant = Impossible<Self::Ok, Error>;
    // FIXME: This should be implemented.
    type SerializeTuple = Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Error>;

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        Ok(StructSerializer {
            urlencoder: self.urlencoder,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer {
            urlencoder: self.urlencoder,
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>
    {
        Err(Error::top_level_not_supported())
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: serde::Serialize {
        Err(Error::top_level_not_supported())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>
    {
        Err(Error::top_level_not_supported())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::top_level_not_supported())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::top_level_not_supported())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::top_level_not_supported())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>
    {
        Err(Error::top_level_not_supported())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>
    {
        Err(Error::top_level_not_supported())
    }
}

pub struct MapSerializer<'input, 'output> {
    urlencoder: &'output mut UrlEncodedSerializer<'input, String>,
}

impl<'input, 'output> ser::SerializeMap for MapSerializer<'input, 'output> {
    type Error = Error;
    type Ok = &'output mut UrlEncodedSerializer<'input, String>;

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: serde::Serialize,
        V: serde::Serialize,
    {
        self.urlencoder.append_pair(
            &key.serialize(ValueSerializer)?,
            &value.serialize(ValueSerializer)?,
        );
        Ok(())
    }

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where T: serde::Serialize {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where T: serde::Serialize {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(self.urlencoder) }
}

pub struct StructSerializer<'input, 'output> {
    urlencoder: &'output mut UrlEncodedSerializer<'input, String>,
}

impl<'input, 'output> ser::SerializeStruct for StructSerializer<'input, 'output> {
    type Error = Error;
    type Ok = &'output mut UrlEncodedSerializer<'input, String>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(FieldSerializer {
            key,
            urlencoder: self.urlencoder,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(self.urlencoder) }
}

pub struct FieldSerializer<'input, 'output> {
    key: &'static str,
    urlencoder: &'output mut UrlEncodedSerializer<'input, String>,
}

impl<'input, 'output> ser::Serializer for FieldSerializer<'input, 'output> {
    type Error = Error;
    type Ok = &'output mut UrlEncodedSerializer<'input, String>;
    type SerializeMap = MapSerializer<'input, 'output>;
    type SerializeSeq = Self;
    type SerializeStruct = Impossible<Self::Ok, Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Error>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Error>;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        //self.urlencoder.append_pair(self.key, v);
        value.serialize(PairSerializer {
            key: self.key,
            urlencoder: self.urlencoder,
        })?;
        Ok(self.urlencoder)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Ok(self.urlencoder) }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where T: serde::Serialize {
        //self.urlencoder.append_pair(self.key, v);
        value.serialize(PairSerializer {
            key: self.key,
            urlencoder: self.urlencoder,
        })?;
        Ok(self.urlencoder)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer {
            urlencoder: self.urlencoder,
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> { Ok(self) }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err(Error::field_not_supported()) }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::field_not_supported())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>
    {
        Err(Error::field_not_supported())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::field_not_supported())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::field_not_supported())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>
    {
        Err(Error::field_not_supported())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>
    {
        Err(Error::field_not_supported())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        Err(Error::field_not_supported())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>
    {
        Err(Error::field_not_supported())
    }
}

impl<'input, 'output> ser::SerializeSeq for FieldSerializer<'input, 'output> {
    type Error = Error;
    type Ok = &'output mut UrlEncodedSerializer<'input, String>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where T: serde::Serialize {
        value.serialize(PairSerializer {
            key: self.key,
            urlencoder: self.urlencoder,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(self.urlencoder) }
}

impl<'input, 'output> ser::SerializeTuple for FieldSerializer<'input, 'output> {
    type Error = Error;
    type Ok = &'output mut UrlEncodedSerializer<'input, String>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where T: serde::Serialize {
        value.serialize(PairSerializer {
            key: self.key,
            urlencoder: self.urlencoder,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(self.urlencoder) }
}

struct PairSerializer<'input, 'output> {
    key: &'static str,
    urlencoder: &'output mut UrlEncodedSerializer<'input, String>,
}

impl<'input, 'output> ser::Serializer for PairSerializer<'input, 'output> {
    type Error = Error;
    type Ok = &'output mut UrlEncodedSerializer<'input, String>;
    type SerializeMap = Impossible<Self::Ok, Error>;
    type SerializeSeq = Impossible<Self::Ok, Error>;
    type SerializeStruct = Impossible<Self::Ok, Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Error>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Error>;

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.urlencoder
            .append_pair(self.key, &v.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Ok(self.urlencoder) }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where T: serde::Serialize {
        self.urlencoder
            .append_pair(self.key, &value.serialize(ValueSerializer)?);
        Ok(self.urlencoder)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::pair_not_supported())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::pair_not_supported())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err(Error::pair_not_supported()) }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::pair_not_supported())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>
    {
        Err(Error::pair_not_supported())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::pair_not_supported())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::pair_not_supported())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::pair_not_supported())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> { Ok(self) }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>
    {
        Err(Error::pair_not_supported())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>
    {
        Err(Error::pair_not_supported())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::pair_not_supported())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        Err(Error::pair_not_supported())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>
    {
        Err(Error::pair_not_supported())
    }
}

impl<'input, 'output> ser::SerializeTuple for PairSerializer<'input, 'output> {
    type Error = Error;
    type Ok = &'output mut UrlEncodedSerializer<'input, String>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where T: serde::Serialize {
        self.urlencoder
            .append_pair(self.key, &value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(self.urlencoder) }
}

struct ValueSerializer;

impl ser::Serializer for ValueSerializer {
    type Error = Error;
    type Ok = Cow<'static, str>;
    type SerializeMap = Impossible<Self::Ok, Error>;
    type SerializeSeq = Impossible<Self::Ok, Error>;
    type SerializeStruct = Impossible<Self::Ok, Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Error>;
    type SerializeTuple = Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> { Ok(Cow::Owned(v.to_string())) }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> { Ok(Cow::Owned(v.to_string())) }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // FIXME: eh
        Ok(Cow::Owned(v.to_string()))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::value_not_supported())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err(Error::value_not_supported()) }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: serde::Serialize {
        Err(Error::value_not_supported())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err(Error::value_not_supported()) }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::value_not_supported())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>
    {
        Err(Error::value_not_supported())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::value_not_supported())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(Error::value_not_supported())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::value_not_supported())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::value_not_supported())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>
    {
        Err(Error::value_not_supported())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>
    {
        Err(Error::value_not_supported())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::value_not_supported())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        Err(Error::value_not_supported())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>
    {
        Err(Error::value_not_supported())
    }
}

#[test]
fn serialize_query() {
    #[derive(serde::Serialize)]
    struct Request {
        filter: String,
        maybe: Option<String>,
        possibly: Option<(String,)>,
        ids: Vec<Option<String>>,
        ids2: Vec<u64>,
        stuff: (u8, f32, &'static str),
        extras: std::collections::BTreeMap<i32, &'static str>,
    }

    let req = Request {
        filter: "1".to_string(),
        maybe: None,
        possibly: Some(("sure thing".to_string(),)),
        ids: vec![Some("2".to_string()), Some("3".to_string())],
        ids2: vec![4],
        stuff: (32, -35f32, "ha"),
        extras: [(1i32, "one"), (2, "two")].iter().copied().collect(),
    };
    assert_eq!(
        to_string(req).unwrap(),
        "filter=1&possibly=sure+thing&ids=2&ids=3&ids2=4&stuff=32&stuff=-35&stuff=ha&1=one&2=two"
    )
}

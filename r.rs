#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod ser {
    use serde::{ser, Serialize};
    use std::error::Error as StdError;
    use std::{
        fmt::{self, Display},
        io,
    };
    /// Struct that will handle the output of serialization
    pub struct RlpSerializer {
        pub output: Vec<u8>,
        pub list_output: Vec<u8>,
        pub actual_list_output: Vec<u8>,
        pub is_list: bool,
        pub total_list: u8,
    }
    pub fn to_rlp_bytes<T>(value: &T) -> Result<Vec<u8>, ()>
    where
        T: Serialize,
    {
        let mut serializer = RlpSerializer {
            output: Vec::new(),
            list_output: Vec::new(),
            actual_list_output: Vec::new(),
            is_list: false,
            total_list: 0,
        };
        value.serialize(&mut serializer).unwrap();
        Ok(serializer.output)
    }
    impl RlpSerializer {
        pub fn serialize_number(&mut self, v: u64) -> Result<(), Error> {
            match v {
                0 => {
                    self.output.push(0x80);
                }
                1..=127 => {
                    self.output.push(v as u8);
                }
                _ => {
                    let mut buff = [0u8; 9];
                    let l = self.write_bytes(v, &mut buff[1..]);
                    buff[0] = 0x80u8 + l;
                    self.output
                        .extend_from_slice(&buff[0usize..l as usize + 1 as usize]);
                }
            }
            Ok(())
        }
        /// Returns the total number of bytes written to the array that is provided
        fn write_bytes(&self, v: u64, buff: &mut [u8]) -> u8 {
            match v {
                v if v < (v << 8) => {
                    buff[0] = v as u8;
                    1
                }
                v if v < (v << 16) => {
                    buff[0] = (v >> 8) as u8;
                    buff[1] = v as u8;
                    2
                }
                v if v < (v << 24) => {
                    buff[0] = (v >> 16) as u8;
                    buff[1] = (v >> 8) as u8;
                    buff[2] = v as u8;
                    3
                }
                v if v < (v << 32) => {
                    buff[0] = (v >> 24) as u8;
                    buff[1] = (v >> 16) as u8;
                    buff[2] = (v >> 8) as u8;
                    buff[3] = v as u8;
                    4
                }
                v if v < (v << 40) => {
                    buff[0] = (v >> 32) as u8;
                    buff[1] = (v >> 24) as u8;
                    buff[2] = (v >> 16) as u8;
                    buff[3] = (v >> 8) as u8;
                    buff[4] = v as u8;
                    5
                }
                v if v < (v << 48) => {
                    buff[0] = (v >> 40) as u8;
                    buff[1] = (v >> 32) as u8;
                    buff[2] = (v >> 24) as u8;
                    buff[3] = (v >> 16) as u8;
                    buff[4] = (v >> 8) as u8;
                    buff[5] = v as u8;
                    6
                }
                v if v < (v << 56) => {
                    buff[0] = (v >> 48) as u8;
                    buff[1] = (v >> 40) as u8;
                    buff[2] = (v >> 32) as u8;
                    buff[3] = (v >> 24) as u8;
                    buff[4] = (v >> 16) as u8;
                    buff[5] = (v >> 8) as u8;
                    buff[6] = v as u8;
                    7
                }
                _ => {
                    buff[0] = (v >> 56) as u8;
                    buff[1] = (v >> 48) as u8;
                    buff[2] = (v >> 40) as u8;
                    buff[3] = (v >> 32) as u8;
                    buff[4] = (v >> 24) as u8;
                    buff[5] = (v >> 16) as u8;
                    buff[6] = (v >> 8) as u8;
                    buff[7] = v as u8;
                    8
                }
            }
        }
    }
    pub struct Error;
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Error")
        }
    }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                _ => f.write_fmt(format_args!("Failed")),
            }
        }
    }
    impl StdError for Error {}
    impl serde::ser::Error for Error {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self {
            Error
        }
    }
    impl<'a> serde::ser::Serializer for &'a mut RlpSerializer {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Self;
        type SerializeTuple = Self;
        type SerializeTupleStruct = Self;
        type SerializeTupleVariant = Self;
        type SerializeMap = Self;
        type SerializeStruct = Self;
        type SerializeStructVariant = Self;
        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            if v {
                self.output.push(0x01);
            } else {
                self.output.push(0x80);
            }
            Ok(())
        }
        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            {
                ::std::io::_print(format_args!("Serializing I8\n"));
            };
            self.serialize_number(v as u64)
        }
        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            {
                ::std::io::_print(format_args!("Serializing U74\n"));
            };
            self.serialize_number(v as u64)
        }
        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
            self.serialize_number(v as u64)
        }
        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            self.serialize_number(v as u64)
        }
        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            self.serialize_number(v as u64)
        }
        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            self.serialize_number(v as u64)
        }
        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            self.serialize_u64(v as u64)
        }
        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            self.serialize_number(v as u64)
        }
        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            let v_string = v.to_string();
            self.serialize_str(&v_string)
        }
        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            let v_string = v.to_string();
            self.serialize_str(&v_string)
        }
        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            let str_bytes = v.as_bytes();
            {
                ::std::io::_print(format_args!("Ser STR\n"));
            };
            self.serialize_bytes(str_bytes)
        }
        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
            if v.len() == 1 && v[0] < 0x7f {
                if self.is_list {
                    self.list_output.push(v[0]);
                } else {
                    self.output.push(v[0]);
                }
            } else if v.len() < 55 {
                let prefix = 0x80u8 + v.len() as u8;
                if self.is_list {
                    self.list_output.push(prefix);
                    self.list_output.extend_from_slice(v);
                } else {
                    self.output.push(prefix);
                    self.output.extend_from_slice(v);
                }
            } else {
                let mut buff = [0u8; 9];
                let size_str_len_bytes = self
                    .write_bytes(v.len() as u64, &mut buff[1..]);
                buff[0] = 0xB7 + size_str_len_bytes;
                if self.is_list {
                    self.list_output
                        .extend_from_slice(
                            &buff[..size_str_len_bytes as usize + 1 as usize],
                        );
                    self.list_output.extend_from_slice(v);
                } else {
                    self.output
                        .extend_from_slice(
                            &buff[..size_str_len_bytes as usize + 1 as usize],
                        );
                    self.output.extend_from_slice(v);
                }
            }
            Ok(())
        }
        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
            self.serialize_u128(v as u128)
        }
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            return Err(Error);
        }
        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
            let mut s = [0u8; 4];
            v.encode_utf8(&mut s);
            self.serialize_bytes(&s[..v.len_utf8()])
        }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            self.output.push(0xc0);
            Ok(())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.serialize_unit()
        }
        fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }
        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(self)
        }
        fn serialize_seq(
            self,
            len: Option<usize>,
        ) -> Result<Self::SerializeSeq, Self::Error> {
            self.is_list = true;
            self.total_list += 1;
            Ok(self)
        }
        fn serialize_map(
            self,
            len: Option<usize>,
        ) -> Result<Self::SerializeMap, Self::Error> {
            Err(Error)
        }
        fn serialize_unit_struct(
            self,
            name: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Err(Error)
        }
        fn serialize_newtype_struct<T>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(Error)
        }
        fn serialize_newtype_variant<T>(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(Error)
        }
        fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err(Error)
        }
        fn serialize_tuple(
            self,
            len: usize,
        ) -> Result<Self::SerializeTuple, Self::Error> {
            Err(Error)
        }
        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(Error)
        }
        fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err(Error)
        }
        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Err(Error)
        }
    }
    impl<'a> ser::SerializeSeq for &'a mut RlpSerializer {
        type Ok = ();
        type Error = Error;
        fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            let prefix = 0xc0 + self.list_output.len() as u8;
            self.total_list -= 1;
            if self.total_list != 0 {
                self.actual_list_output.push(prefix);
                self.actual_list_output.extend_from_slice(&self.list_output);
            } else {
                if self.list_output.len() > 1 {
                    self.actual_list_output.push(prefix);
                    self.actual_list_output.extend_from_slice(&self.list_output);
                }
                let prefix = 0xc0 + self.actual_list_output.len() as u8;
                self.output.push(prefix);
                self.output.extend_from_slice(&self.actual_list_output);
                self.is_list = false;
            }
            self.list_output.clear();
            Ok(())
        }
    }
    impl<'a> ser::SerializeTuple for &'a mut RlpSerializer {
        type Ok = ();
        type Error = Error;
        fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    impl<'a> ser::SerializeTupleStruct for &'a mut RlpSerializer {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    impl<'a> ser::SerializeTupleVariant for &'a mut RlpSerializer {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    impl<'a> ser::SerializeMap for &'a mut RlpSerializer {
        type Ok = ();
        type Error = Error;
        fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            key.serialize(&mut **self)
        }
        fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    impl<'a> ser::SerializeStruct for &'a mut RlpSerializer {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(&mut **self)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    impl<'a> ser::SerializeStructVariant for &'a mut RlpSerializer {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            key.serialize(&mut **self)?;
            value.serialize(&mut **self)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
}
pub mod des {
    use std::char;
    use serde::de::SeqAccess;
    use serde::de::Visitor;
    use serde::Deserializer;
    use serde::{de, Deserialize};
    use crate::error::Error;
    pub struct RlpDeserializer<'de> {
        pub input: &'de [u8],
        pub track: u8,
        pub initial: bool,
    }
    pub fn from_rlp_bytes<'de, T>(v: &'de [u8]) -> Result<T, Error>
    where
        T: de::Deserialize<'de>,
    {
        let mut deserializer = RlpDeserializer {
            input: v,
            track: 1,
            initial: false,
        };
        let value = de::Deserialize::deserialize(&mut deserializer);
        value
    }
    impl<'de> RlpDeserializer<'de> {
        pub fn next_byte(&mut self) -> Result<u8, Error> {
            let byte = self.input[0];
            self.input = &self.input[1..];
            Ok(byte)
        }
        pub fn read_bytes(&mut self, count: usize) -> &[u8] {
            let bytes = &self.input[0..count];
            self.input = &self.input[count..];
            bytes
        }
        pub fn peak(&self) -> u8 {
            self.input[0]
        }
    }
    impl<'a, 'de> de::Deserializer<'de> for &'a mut RlpDeserializer<'de> {
        type Error = Error;
        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let byte = self.next_byte().unwrap();
            if byte == 0x01 {
                return visitor.visit_bool(true);
            } else if byte == 0x80 {
                return visitor.visit_bool(false);
            } else {
                {
                    ::std::io::_print(format_args!("Not supppp\n"));
                };
                return Err(Error);
            }
        }
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("any is Not Supported\n"));
            };
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let next_byte = self.next_byte().unwrap();
            {
                ::std::io::_print(format_args!("Next Byte = {0}\n", next_byte));
            };
            if next_byte <= 0x7f {
                visitor.visit_bytes(&[next_byte])
            } else if next_byte >= 0x80 && next_byte <= 0xb7 {
                let length_of_arr = next_byte - 0x80u8;
                let bytes = self.read_bytes(length_of_arr as usize);
                {
                    ::std::io::_print(format_args!("Visiting = {0:?}\n", bytes));
                };
                visitor.visit_bytes(bytes)
            } else if next_byte <= 0xbf {
                let length_of_array_length_bytes = next_byte - 0xB7;
                let array_length_bytes: [u8; 8] = self
                    .read_bytes(length_of_array_length_bytes as usize)
                    .try_into()
                    .unwrap();
                let array_length = usize::from_be_bytes(array_length_bytes);
                let data_bytes = self.read_bytes(array_length);
                visitor.visit_bytes(data_bytes)
            } else {
                {
                    ::std::io::_print(format_args!("HI\n"));
                };
                self.initial = false;
                self.deserialize_seq(visitor)
            }
        }
        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            self.deserialize_bytes(visitor)
        }
        /// Char is Always 4 byte
        /// Deserialise_byte cannot be used here because of that reason
        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let next_byte = self.next_byte().unwrap();
            if next_byte < 0x7f {
                let bytes = [next_byte, 0, 0, 0];
                let num = u32::from_le_bytes(bytes);
                let actual_char = char::from_u32(num).unwrap();
                visitor.visit_char(actual_char)
            } else {
                let length_of_arr = next_byte - 0x80u8;
                let bytes: [u8; 4] = self
                    .read_bytes(length_of_arr as usize)
                    .try_into()
                    .unwrap();
                let num = u32::from_le_bytes(bytes);
                let actual_char = char::from_u32(num).unwrap();
                visitor.visit_char(actual_char)
            }
        }
        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("Enum is Not Supported\n"));
            };
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("identifier is Not Supported\n"));
            };
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("ignore Any is Not Supported\n"));
            };
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("Map is Not Supported\n"));
            };
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_newtype_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("NewType Struct is Not Supported\n"));
            };
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("Option is Not Supported\n"));
            };
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("Str is Not Supported\n"));
            };
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            self.deserialize_bytes(visitor)
        }
        fn deserialize_struct<V>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            self.deserialize_seq(visitor)
        }
        fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            ::core::panicking::panic("not implemented")
        }
        fn deserialize_u64<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.visit_u64(0);
                }
                1..=127 => {
                    return visitor.visit_u64(byte as u64);
                }
                _ => {
                    let byte_len = byte - 0x80u8;
                    let num_bytes: [u8; 8] = self
                        .input[0..byte_len as usize]
                        .try_into()
                        .unwrap();
                    let num = <u64>::from_be_bytes(num_bytes);
                    return visitor.visit_u64(num);
                }
            }
        }
        fn deserialize_u32<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.visit_u32(0);
                }
                1..=127 => {
                    return visitor.visit_u32(byte as u32);
                }
                _ => {
                    let byte_len = byte - 0x80u8;
                    let num_bytes: [u8; 4] = self
                        .input[0..byte_len as usize]
                        .try_into()
                        .unwrap();
                    let num = <u32>::from_be_bytes(num_bytes);
                    return visitor.visit_u32(num);
                }
            }
        }
        fn deserialize_u16<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.visit_u16(0);
                }
                1..=127 => {
                    return visitor.visit_u16(byte as u16);
                }
                _ => {
                    let byte_len = byte - 0x80u8;
                    let num_bytes: [u8; 2] = self
                        .input[0..byte_len as usize]
                        .try_into()
                        .unwrap();
                    let num = <u16>::from_be_bytes(num_bytes);
                    return visitor.visit_u16(num);
                }
            }
        }
        fn deserialize_u8<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.visit_u8(0);
                }
                1..=127 => {
                    return visitor.visit_u8(byte as u8);
                }
                _ => {
                    let byte_len = byte - 0x80u8;
                    let num_bytes: [u8; 1] = self
                        .input[0..byte_len as usize]
                        .try_into()
                        .unwrap();
                    let num = <u8>::from_be_bytes(num_bytes);
                    return visitor.visit_u8(num);
                }
            }
        }
        fn deserialize_i64<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.visit_i64(0);
                }
                1..=127 => {
                    return visitor.visit_i64(byte as i64);
                }
                _ => {
                    let byte_len = byte - 0x80u8;
                    let num_bytes: [u8; 8] = self
                        .input[0..byte_len as usize]
                        .try_into()
                        .unwrap();
                    let num = <i64>::from_be_bytes(num_bytes);
                    return visitor.visit_i64(num);
                }
            }
        }
        fn deserialize_i32<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.visit_i32(0);
                }
                1..=127 => {
                    return visitor.visit_i32(byte as i32);
                }
                _ => {
                    let byte_len = byte - 0x80u8;
                    let num_bytes: [u8; 4] = self
                        .input[0..byte_len as usize]
                        .try_into()
                        .unwrap();
                    let num = <i32>::from_be_bytes(num_bytes);
                    return visitor.visit_i32(num);
                }
            }
        }
        fn deserialize_i16<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.visit_i16(0);
                }
                1..=127 => {
                    return visitor.visit_i16(byte as i16);
                }
                _ => {
                    let byte_len = byte - 0x80u8;
                    let num_bytes: [u8; 2] = self
                        .input[0..byte_len as usize]
                        .try_into()
                        .unwrap();
                    let num = <i16>::from_be_bytes(num_bytes);
                    return visitor.visit_i16(num);
                }
            }
        }
        fn deserialize_i8<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.visit_i8(0);
                }
                1..=127 => {
                    return visitor.visit_i8(byte as i8);
                }
                _ => {
                    let byte_len = byte - 0x80u8;
                    let num_bytes: [u8; 1] = self
                        .input[0..byte_len as usize]
                        .try_into()
                        .unwrap();
                    let num = <i8>::from_be_bytes(num_bytes);
                    return visitor.visit_i8(num);
                }
            }
        }
        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            if !self.initial {
                self.initial = true;
                visitor.visit_seq(self)
            } else {
                self.deserialize_bytes(visitor)
            }
        }
        fn deserialize_tuple<V>(
            self,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("Tuple is Not Supported\n"));
            };
            Err(Error)
        }
        fn deserialize_tuple_struct<V>(
            self,
            name: &'static str,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("Tuple Struct is Not Supported\n"));
            };
            Err(Error)
        }
        ///TODO:
        fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("Tuple is Not Supported\n"));
            };
            return Err(Error);
        }
        fn deserialize_unit_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            {
                ::std::io::_print(format_args!("Unit Struct is Not Supported\n"));
            };
            Err(Error)
        }
    }
    impl<'a, 'de> SeqAccess<'de> for &'a mut RlpDeserializer<'de> {
        type Error = Error;
        fn next_element_seed<T>(
            &mut self,
            seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            {
                ::std::io::_print(format_args!("Seeding {0:?}\n", self.input));
            };
            if self.input.is_empty() {
                return Ok(None);
            }
            seed.deserialize(&mut **self).map(Some)
        }
    }
    struct Point {
        y: Vec<Vec<String>>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Point {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "y" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"y" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Point>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Point;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Point",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<Vec<String>>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Point with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Point { y: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<Vec<String>>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("y"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<Vec<String>>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("y")?
                            }
                        };
                        _serde::__private::Ok(Point { y: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["y"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Point",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Point>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Point {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Point", "y", &&self.y)
        }
    }
    fn testing() {
        let bytes = from_rlp_bytes::<
            Point,
        >(
            &[
                210,
                200,
                131,
                99,
                97,
                116,
                131,
                100,
                111,
                103,
                200,
                131,
                99,
                97,
                116,
                131,
                100,
                111,
                103,
            ],
        );
    }
}
pub mod error {
    use core::fmt;
    use std::error::Error as StdError;
    pub struct Error;
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Error")
        }
    }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                _ => f.write_fmt(format_args!("Failed")),
            }
        }
    }
    impl StdError for Error {}
    impl serde::de::Error for Error {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self {
            Error
        }
    }
}

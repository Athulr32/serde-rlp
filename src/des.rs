use core::num;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};
use std::error::Error as StdError;
use std::fmt;

pub struct RlpDeserializer<'de> {
    pub input: &'de [u8],
}

pub fn from_rlp_bytes<'de, T>(v: &'de [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    let mut deserializer = RlpDeserializer { input: v };
    let value = de::Deserialize::deserialize(&mut deserializer);

    value
}

macro_rules! deserialize_int {
    ($method:ident, $visit:ident, $inttype:ty,$bits:expr) => {
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            let byte = self.next_byte().unwrap();
            match byte {
                0x80 => {
                    return visitor.$visit(0);
                }
                1..=127 => {
                    //Single byte
                    return visitor.$visit(byte as $inttype);
                }
                _ => {
                    let byte_len = byte - 0x80u8;

                    let num_bytes: [u8; $bits] =
                        self.input[0..byte_len as usize].try_into().unwrap(); // Number in Bytes
                    let num = <$inttype>::from_be_bytes(num_bytes);
                    return visitor.$visit(num);
                }
            }
        }
    };
}

impl<'de> RlpDeserializer<'de> {
    pub fn next_byte(&mut self) -> Result<u8, Error> {
        let byte = self.input[0];
        self.input = &self.input[1..];

        Ok(byte)
    }
}

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "Failed"),
        }
    }
}

impl StdError for Error {}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(_msg: T) -> Self {
        Error
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
            return Err(Error);
        }
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
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
        unimplemented!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
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

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        println!("{:?}",len);
        unimplemented!()
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
        unimplemented!()
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    deserialize_int!(deserialize_u64, visit_u64, u64, 8);
    deserialize_int!(deserialize_u32, visit_u32, u32, 4);
    deserialize_int!(deserialize_u16, visit_u16, u16, 2);
    deserialize_int!(deserialize_u8, visit_u8, u8, 1);

    deserialize_int!(deserialize_i64, visit_i64, i64, 8);
    deserialize_int!(deserialize_i32, visit_i32, i32, 4);
    deserialize_int!(deserialize_i16, visit_i16, i16, 2);
    deserialize_int!(deserialize_i8, visit_i8, i8, 1);

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }
}

impl<'a, 'de> SeqAccess<'de> for &'a mut RlpDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut **self).map(Some)
    }
}

#[derive(Deserialize, Debug)]
struct Point {
    x: u8,
    y: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: u64,
        y: [u8;10],
    }

    #[test]
    fn des_test() {
        let bytes = from_rlp_bytes::<Point>(&[5, 128]);

        println!("{:?}", bytes);
    }
}

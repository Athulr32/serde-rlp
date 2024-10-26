use serde::{ser, Serialize};
use std::error::Error as StdError;
use std::{
    fmt::{self, Display},
    io,
};

/// Struct that will handle the output of serialization and serialization
pub struct RlpSerializer {
    pub output: Vec<u8>,
}

pub fn to_rlp_bytes<T>(value: &T) -> Result<Vec<u8>, ()>
where
    T: Serialize,
{
    let mut serializer = RlpSerializer { output: Vec::new() };
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
                //Single byte
                self.output.push(v as u8);
            }
            _ => {
                // More than a single byte
                // Convert the integer into shorted byte array and get the length of it
                let mut buff = [0u8; 9]; //Can have a maximum of 9 bytes
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
        // Can Represent v as 1 to 8 bytes
        // Calculate how many byte does v requires
        match v {
            // 255 - 1 byte
            v if v < (v << 8) => {
                buff[0] = v as u8;
                1
            }
            // 2 byte
            v if v < (v << 16) => {
                buff[0] = (v >> 8) as u8;
                buff[1] = v as u8;
                2
            }
            // 3 byte
            v if v < (v << 24) => {
                buff[0] = (v >> 16) as u8;
                buff[1] = (v >> 8) as u8;
                buff[2] = v as u8;
                3
            }
            // 4 bytes
            v if v < (v << 32) => {
                buff[0] = (v >> 24) as u8;
                buff[1] = (v >> 16) as u8;
                buff[2] = (v >> 8) as u8;
                buff[3] = v as u8;
                4
            }
            // 5 bytes
            v if v < (v << 40) => {
                buff[0] = (v >> 32) as u8;
                buff[1] = (v >> 24) as u8;
                buff[2] = (v >> 16) as u8;
                buff[3] = (v >> 8) as u8;
                buff[4] = v as u8;
                5
            }
            // 6 bytes
            v if v < (v << 48) => {
                buff[0] = (v >> 40) as u8;
                buff[1] = (v >> 32) as u8;
                buff[2] = (v >> 24) as u8;
                buff[3] = (v >> 16) as u8;
                buff[4] = (v >> 8) as u8;
                buff[5] = v as u8;
                6
            }
            // 7 bytes
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
            // 8 Bytes
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

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(_msg: T) -> Self {
        Error
    }
}

impl<'a> serde::ser::Serializer for &'a mut RlpSerializer {
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;


    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        // The encoded value is the byte of the boolean itself
        if v {
            self.output.push(0x01);
        } else {
            self.output.push(0x80);
        }
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_number(v as u64)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
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
        //Convert string to byte array
        let str_bytes = v.as_bytes();
        self.serialize_bytes(str_bytes)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if v.len() == 1 && v[0] < 0x7f {
            //Single byte
            self.output.push(v[0]);
        } else if v.len() < 55 {
            //This constitutes of empty array if string is just empty ("")
            let prefix = 0x80u8 + v.len() as u8;
            self.output.push(prefix);
            self.output.extend_from_slice(v);
        } else {
            let mut buff = [0u8; 9];
            let size_str_len_bytes = self.write_bytes(v.len() as u64, &mut buff[1..]);
            buff[0] = 0xB7 + size_str_len_bytes;
            //Write both the Prefix (Added with size_str_len_bytes) and the length of the string (the byte representation)
            self.output
                .extend_from_slice(&buff[..size_str_len_bytes as usize + 1 as usize]);
            //Append the actual string
            self.output.extend_from_slice(v);
        }

        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.serialize_u128(v as u128)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        //TODO: Implementation left
        return Err(Error);
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut s = [0u8; 4];
        let char_bytes = v.encode_utf8(&mut s);
        self.serialize_bytes(&s[..v.len_utf8()])
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        //Emtpy List
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

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
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

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
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

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
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


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Point {
        x: u8,
        y: u8,
    }

    #[test]
    fn test_struct_serialization() {
        let point = Point { x: 5, y: 10 };

        let bytes = to_rlp_bytes(&String::from("hi"));

        println!("{:?}", bytes);
    }
}

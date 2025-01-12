use bytes::{Buf, Bytes};

use crate::types::types;

use super::varint::decode_varint32;

/// This is simply a wrapper around a u8 slice to safely read data, and also add method to read varint and varlong
#[derive(Debug)]
pub struct Decoder<'a> {
    cursor: &'a [u8],
}

impl<'a> Decoder<'a> {
    pub fn new(cursor: &'a [u8]) -> Self {
        Decoder { cursor }
    }

    pub fn read_u8(&mut self) -> types::Result<u8> {
        if self.cursor.remaining() > 0 {
            return Ok(self.cursor.get_u8());
        }

        Err("not enough data to get u8".into())
    }

    pub fn read_u16(&mut self) -> types::Result<u16> {
        if self.cursor.remaining() > 1 {
            return Ok(self.cursor.get_u16());
        }

        Err("not enough data to get u16".into())
    }

    pub fn read_u32(&mut self) -> types::Result<u32> {
        if self.cursor.remaining() > 3 {
            return Ok(self.cursor.get_u32());
        }

        Err("not enough data to get u32".into())
    }

    pub fn read_u64(&mut self) -> types::Result<u64> {
        if self.cursor.remaining() > 7 {
            return Ok(self.cursor.get_u64());
        }

        Err("not enough data to get u64".into())
    }

    pub fn read_i8(&mut self) -> types::Result<i8> {
        if self.cursor.remaining() > 0 {
            return Ok(self.cursor.get_i8());
        }

        Err("not enough data to get i8".into())
    }

    pub fn read_i16(&mut self) -> types::Result<i16> {
        if self.cursor.remaining() > 1 {
            return Ok(self.cursor.get_i16());
        }

        Err("not enough data to get i16".into())
    }

    pub fn read_i32(&mut self) -> types::Result<i32> {
        if self.cursor.remaining() > 3 {
            return Ok(self.cursor.get_i32());
        }

        Err("not enough data to get i32".into())
    }

    pub fn read_i64(&mut self) -> types::Result<i64> {
        if self.cursor.remaining() > 7 {
            return Ok(self.cursor.get_i64());
        }

        Err("not enough data to get i64".into())
    }

    pub fn read_bytes(&mut self) -> types::Result<Bytes> {
        if !self.cursor.has_remaining() {
            return Err("not enough data to read bytes".into());
        }

        if let Ok(bytes_len) = self.read_varint() {
            let bytes_len = bytes_len as usize;
            if self.cursor.len() < bytes_len {
                return Err("not enough data to read bytes, specified by varint".into());
            }

            let slice = &self.cursor[..bytes_len];
            let buffer = Bytes::copy_from_slice(slice);
            return Ok(buffer);
        }

        Err("invalid bytes value".into())
    }

    pub fn read_string(&mut self) -> types::Result<String> {
        if !self.cursor.has_remaining() {
            return Err("not enough data to read string".into());
        }

        if let Ok(str_len) = self.read_varint() {
            let str_len = str_len as usize;
            if self.cursor.len() < str_len {
                return Err("not enough data to read string, specified by varint".into());
            }

            let slice = &self.cursor[..str_len];
            let str = String::from_utf8(slice.into())?;
            return Ok(str);
        }

        Err("invalid string value".into())
    }

    pub fn read_varint(&mut self) -> types::Result<u32> {
        if !self.cursor.has_remaining() {
            return Err("not enough data to get varint".into());
        }

        if let Some((varint, len)) = decode_varint32(self.cursor) {
            self.cursor.advance(len);
            return Ok(varint);
        }

        Err("invalid varint value".into())
    }

    pub fn read_bool(&mut self) -> types::Result<bool> {
        if self.cursor.remaining() > 0 {
            return match self.cursor.get_u8() {
                0x01 => Ok(true),
                0x00 => Ok(false),
                _ => Err("not a valid byte representing a boolean".into()),
            };
        }

        Err("not enough data to get bool".into())
    }

    pub fn remaining(&self) -> usize {
        return self.cursor.remaining();
    }
}

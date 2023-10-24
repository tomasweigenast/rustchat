use bytes::Buf;

use crate::types::types;

use super::{CONTINUE_BIT, SEGMENT_BITS};

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

    pub fn read_varint(&mut self) -> types::Result<i32> {
        let mut value: i32 = 0;
        let mut pos = 0;
        loop {
            if !self.cursor.has_remaining() {
                return Err("not enough data to get varint".into());
            }

            let byte = self.cursor.get_u8();
            value |= ((byte & SEGMENT_BITS) as i32) << pos;

            if byte & CONTINUE_BIT == 0 {
                break;
            }

            pos += 7;
            if pos >= 32 {
                return Err("varint is too big".into());
            }
        }

        return Ok(value);
    }

    pub fn read_varlong(&mut self) -> types::Result<i64> {
        let mut value: i64 = 0;
        let mut pos = 0;
        loop {
            if !self.cursor.has_remaining() {
                return Err("not enough data to get varlong".into());
            }

            let byte = self.cursor.get_u8();
            value |= ((byte & SEGMENT_BITS) as i64) << pos;

            if byte & CONTINUE_BIT == 0 {
                break;
            }

            pos += 7;
            if pos >= 64 {
                return Err("varlong is too big".into());
            }
        }

        return Ok(value);
    }

    pub fn read_string(&mut self) -> types::Result<String> {
        if !self.cursor.has_remaining() {
            return Err("not enough data to read string".into());
        }

        let str_len = self.read_varint()? as usize;
        if self.cursor.len() < str_len {
            return Err("not enough data to read string, specified by varint".into());
        }

        let slice = &self.cursor[..str_len];
        let str = String::from_utf8(slice.into())?;
        return Ok(str);
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

use bytes::{BufMut, Bytes, BytesMut};

use super::varint;

#[derive(Debug)]
pub struct Encoder {
    buf: BytesMut,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {
            buf: BytesMut::with_capacity(1024),
        }
    }

    pub fn write_bool(&mut self, value: bool) {
        match value {
            true => self.buf.put_u8(0x01),
            false => self.buf.put_u8(0x00),
        }
    }

    #[inline]
    pub fn write_i8(&mut self, value: i8) {
        self.buf.put_i8(value)
    }

    #[inline]
    pub fn write_u8(&mut self, value: u8) {
        self.buf.put_u8(value)
    }

    #[inline]
    pub fn write_i16(&mut self, value: i16) {
        self.buf.put_i16(value);
    }

    #[inline]
    pub fn write_u16(&mut self, value: u16) {
        self.buf.put_u16(value);
    }

    #[inline]
    pub fn write_i32(&mut self, value: i32) {
        self.buf.put_i32(value);
    }

    #[inline]
    pub fn write_u32(&mut self, value: u32) {
        self.buf.put_u32(value);
    }

    #[inline]
    pub fn write_i64(&mut self, value: i64) {
        self.buf.put_i64(value);
    }

    #[inline]
    pub fn write_u64(&mut self, value: u64) {
        self.buf.put_u64(value);
    }

    #[inline]
    pub fn write_f32(&mut self, value: f32) {
        self.buf.put_f32(value);
    }

    #[inline]
    pub fn write_f64(&mut self, value: f64) {
        self.buf.put_f64(value);
    }

    pub fn write_string(&mut self, value: String) {
        let buffer = value.as_bytes();
        if buffer.len() > 32767 {
            panic!("Maximum string length exceeded");
        }

        self.write_varint(buffer.len() as u32);
        self.buf.put_slice(buffer);
    }

    pub fn write_string_ref(&mut self, value: &String) {
        let buffer = value.as_bytes();
        if buffer.len() > 32767 {
            panic!("Maximum string length exceeded");
        }

        self.write_varint(buffer.len() as u32);
        self.buf.put_slice(buffer);
    }

    pub fn write_varint(&mut self, value: u32) {
        varint::encode_varint32(value, &mut self.buf);
    }

    pub fn write_bytes(&mut self, value: &Bytes) {
        self.write_varint(value.len() as u32);
        self.buf.put_slice(value);
    }

    pub fn take_bytes(self) -> Bytes {
        self.buf.freeze()
    }
}

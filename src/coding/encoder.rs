use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct Encoder {
    buf: BytesMut,
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

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

    pub fn write_sbyte(&mut self, value: i8) {
        self.buf.put_i8(value)
    }

    pub fn write_ubyte(&mut self, value: u8) {
        self.buf.put_u8(value)
    }

    pub fn write_sshort(&mut self, value: i16) {
        self.buf.put_i16(value);
    }

    pub fn write_ushort(&mut self, value: u16) {
        self.buf.put_u16(value);
    }

    pub fn write_sint(&mut self, value: i32) {
        self.buf.put_i32(value);
    }

    pub fn write_uint(&mut self, value: u32) {
        self.buf.put_u32(value);
    }

    pub fn write_slong(&mut self, value: i64) {
        self.buf.put_i64(value);
    }

    pub fn write_ulong(&mut self, value: u64) {
        self.buf.put_u64(value);
    }

    pub fn write_float(&mut self, value: f32) {
        self.buf.put_f32(value);
    }

    pub fn write_double(&mut self, value: f64) {
        self.buf.put_f64(value);
    }

    pub fn write_varint(&mut self, value: i32) {
        let mut n = value;

        loop {
            if n & (!SEGMENT_BITS as i32) == 0 {
                self.buf.put_u8(n as u8);
                return;
            }

            self.buf.put_u8((n as u8 & SEGMENT_BITS) | CONTINUE_BIT);
            n >>= 7;
        }
    }

    pub fn write_varlong(&mut self, value: i64) {
        let mut n = value;

        loop {
            if n & (!SEGMENT_BITS as i64) == 0 {
                self.buf.put_u8(n as u8);
                return;
            }

            self.buf.put_u8((n as u8 & SEGMENT_BITS) | CONTINUE_BIT);
            n >>= 7;
        }
    }

    pub fn write_string(&mut self, value: String) {
        let buffer = value.as_bytes();
        if buffer.len() > 32767 {
            panic!("Maximum string length exceeded");
        }

        self.write_varint(buffer.len() as i32);
        self.buf.put_slice(buffer);
    }

    pub fn take_bytes(self) -> Bytes {
        self.buf.freeze()
    }
}

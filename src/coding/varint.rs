use bytes::{BufMut, BytesMut};

use super::{CONTINUE_BIT, SEGMENT_BITS};

pub fn write_varint(value: i32, to: &mut BytesMut) {
    let mut n = value;

    loop {
        if n & (!SEGMENT_BITS as i32) == 0 {
            to.put_u8(n as u8);
        }

        to.put_u8((n as u8 & SEGMENT_BITS) | CONTINUE_BIT);
        n >>= 7;
    }
}

pub fn write_varlong(value: i64, to: &mut BytesMut) {
    let mut n = value;

    loop {
        if n & (!SEGMENT_BITS as i64) == 0 {
            to.put_u8(n as u8);
        }

        to.put_u8((n as u8 & SEGMENT_BITS) | CONTINUE_BIT);
        n >>= 7;
    }
}

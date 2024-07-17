use bytes::{Buf, BufMut, BytesMut};

pub fn encode_varint32(mut value: u32, buf: &mut BytesMut) -> usize {
    let mut len = 0;

    while value >= 0x80 {
        buf.put_u8((value as u8) | 0x80);
        value >>= 7;
        len += 1;
    }

    buf.put_u8(value as u8);
    len + 1
}

pub fn decode_varint32(buf: &mut BytesMut) -> Option<u32> {
    let mut value = 0u32;
    let mut shift = 0;

    for _i in 0..5 {
        if buf.is_empty() {
            return None; // Not enough bytes in the buffer to decode a complete VARINT
        }

        let byte = buf.get_u8();
        value |= ((byte & 0x7F) as u32) << shift;

        if (byte & 0x80) == 0 {
            return Some(value);
        }

        shift += 7;
    }

    None // If more than 5 bytes are needed, the VARINT is too large
}

#[cfg(test)]
mod test {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_encode_varint32() {
        let test_cases = vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (127, vec![0x7F]),
            (128, vec![0x80, 0x01]),
            (300, vec![0xAC, 0x02]),
            (16384, vec![0x80, 0x80, 0x01]),
            (u32::MAX, vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F]),
        ];

        for (value, expected) in test_cases {
            let mut buf = BytesMut::with_capacity(5);
            let size = encode_varint32(value, &mut buf);
            assert_eq!(&buf[..], &expected[..]);
            assert_eq!(size, expected.len());
        }
    }

    #[test]
    fn test_decode_varint32() {
        let test_cases = vec![
            (vec![0x00], Some(0)),
            (vec![0x01], Some(1)),
            (vec![0x7F], Some(127)),
            (vec![0x80, 0x01], Some(128)),
            (vec![0xAC, 0x02], Some(300)),
            (vec![0x80, 0x80, 0x01], Some(16384)),
            (vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F], Some(u32::MAX)),
        ];

        for (encoded, expected) in test_cases {
            let mut buf = BytesMut::from(&encoded[..]);
            let decoded = decode_varint32(&mut buf);
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_decode_varint32_incomplete() {
        let test_cases = vec![
            vec![0x80],                   // incomplete
            vec![0x80, 0x80],             // incomplete
            vec![0x80, 0x80, 0x80],       // incomplete
            vec![0x80, 0x80, 0x80, 0x80], // incomplete
        ];

        for encoded in test_cases {
            let mut buf = BytesMut::from(&encoded[..]);
            let decoded = decode_varint32(&mut buf);
            assert_eq!(decoded, None);
        }
    }
}

use bytes::{BufMut, BytesMut};

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

pub fn decode_varint32(buf: &[u8]) -> Option<(u32, usize)> {
    let mut value = 0u32;
    let mut shift = 0;
    let mut len = 0;

    for &byte in buf.iter() {
        value |= ((byte & 0x7F) as u32) << shift;
        len += 1;

        if (byte & 0x80) == 0 {
            return Some((value, len));
        }

        shift += 7;

        if shift >= 32 {
            return None; // Too many bytes
        }
    }

    None // If we exit the loop, it means the VARINT is incomplete
}

#[cfg(test)]
mod tests {
    use super::*;

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
            (vec![0x00], Some((0, 1))),
            (vec![0x01], Some((1, 1))),
            (vec![0x7F], Some((127, 1))),
            (vec![0x80, 0x01], Some((128, 2))),
            (vec![0xAC, 0x02], Some((300, 2))),
            (vec![0x80, 0x80, 0x01], Some((16384, 3))),
            (vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F], Some((u32::MAX, 5))),
        ];

        for (encoded, expected) in test_cases {
            let decoded = decode_varint32(&encoded);
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
            let decoded = decode_varint32(&encoded);
            assert_eq!(decoded, None);
        }
    }
}

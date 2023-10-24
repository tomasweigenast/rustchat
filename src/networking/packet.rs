use bytes::{Buf, Bytes, BytesMut};

use crate::{coding::Decoder, types};

use super::error::NetworkingError;

#[derive(Debug)]
pub struct Packet {
    id: u8,
    size: u64,
    data: Bytes,
}

impl Packet {
    /// Creates a new Packet
    pub fn new(id: u8, data: Bytes) -> Packet {
        Packet {
            id,
            size: data.len() as u64,
            data,
        }
    }

    /// Creates a new packet from a BytesMut reference
    pub fn from(buffer: &BytesMut) -> types::Result<Packet> {
        if !buffer.has_remaining() {
            return Err(NetworkingError::InvalidPacketFormat.into());
        }

        let buffer = buffer.chunk();
        let mut decoder = Decoder::new(buffer);

        // read packet id
        let packet_id: u8 = decoder.read_u8()?;
        let packet_size: u64 = decoder.read_varlong()?.try_into()?;

        let data =
            Bytes::copy_from_slice(&buffer[buffer.len() - decoder.remaining()..buffer.len()]);

        Ok(Packet {
            id: packet_id,
            size: packet_size,
            data,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::coding::Encoder;

    use super::*;

    #[test]
    fn packet_from() {
        let mut encoder = Encoder::new();
        encoder.write_ubyte(0x85);
        encoder.write_varlong(255);
        encoder.write_string("hola mundo".into());
        let bytes = encoder.take_bytes();

        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(bytes.chunk());

        let packet = Packet::from(&buffer);
        if packet.is_err() {
            println!("error: {}", packet.as_ref().unwrap_err());
        }
        assert!(packet.is_ok());

        let packet = packet.unwrap();
        assert_eq!(packet.id, 0x85);
        assert_eq!(packet.size, 255);

        let mut decoder = Decoder::new(&packet.data);
        let value = decoder.read_string().expect("failed to read string");
        assert_eq!(value, Bytes::from_static(b"hola mundo"));
    }
}

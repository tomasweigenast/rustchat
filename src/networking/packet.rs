use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{coding::Decoder, types};

use super::error::NetworkingError;

const PACKET_HEADER_SIZE: usize = 12;

#[derive(Debug)]
pub struct Packet {
    pub id: u8,
    pub payload_size: u16,
    pub seq_ack_number: u32,
    pub mac_number: u32,
    pub control_bits: u8,
    pub payload: Bytes,
}

impl Packet {
    /// Creates a new Packet
    pub fn new(id: u8, data: Bytes) -> Packet {
        Packet {
            id,
            payload_size: data.len() as u16,
            control_bits: 0,
            seq_ack_number: 0,
            mac_number: 0,
            payload: data,
        }
    }

    /// Encodes this packet to a byte buffer
    pub fn encode(&self) -> Bytes {
        // Create a BytesMut buffer with the exact required capacity
        let mut buffer = BytesMut::with_capacity(self.total_size());

        // Write fields to the buffer
        buffer.put_u8(self.id);
        buffer.put_u16(self.payload_size);
        buffer.put_u8(self.control_bits);
        buffer.put_u32(self.seq_ack_number);
        buffer.put_u32(self.mac_number);
        buffer.put_slice(&self.payload);

        // Convert BytesMut to Bytes for immutability
        buffer.freeze()
    }

    /// Creates a new packet from a BytesMut reference, parsing the contents.
    pub fn from(buffer: &BytesMut) -> types::Result<Packet> {
        if !buffer.has_remaining() {
            return Err(NetworkingError::InvalidPacketFormat.into());
        }

        let buffer = buffer.chunk();
        let mut decoder = Decoder::new(buffer);

        // read packet id
        let packet_id = decoder.read_u8()?;
        let payload_size = decoder.read_u16()?;
        let control_bits = decoder.read_u8()?;
        let seq_ack_number = decoder.read_u32()?;
        let mac_number = decoder.read_u32()?;

        let data =
            Bytes::copy_from_slice(&buffer[buffer.len() - decoder.remaining()..buffer.len()]);

        Ok(Packet {
            id: packet_id,
            payload_size,
            control_bits,
            mac_number,
            seq_ack_number,
            payload: data,
        })
    }

    /// Returns the total size of this packet, considering the payload.
    pub fn total_size(&self) -> usize {
        PACKET_HEADER_SIZE + self.payload.len()
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
        assert_eq!(packet.payload_size, 255);

        let mut decoder = Decoder::new(&packet.payload);
        let value = decoder.read_string().expect("failed to read string");
        assert_eq!(value, Bytes::from_static(b"hola mundo"));
    }
}

use std::{any::Any, fmt};

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{
    coding::{Decoder, Encoder},
    types,
};

use super::{error::NetworkingError, packet_type::PacketType};

pub const MAX_PACKET_SIZE: usize = 1024 * 4; // 4 kB
pub const PACKET_HEADER_SIZE: usize = 4 + 1 + 1;

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub id: u8,
    pub control_bits: u8,
    pub payload: Bytes,
}

impl Packet {
    /// Encodes this packet to a byte buffer
    pub fn encode(&self) -> Bytes {
        // Create a BytesMut buffer with the exact required capacity
        let mut buffer = BytesMut::with_capacity(self.total_size());

        // Write fields to the buffer
        buffer.put_u8(self.id);
        buffer.put_u8(self.control_bits);
        buffer.put_slice(&self.payload);

        // Convert BytesMut to Bytes for immutability
        buffer.freeze()
    }

    /// Creates a new packet with the given payload.
    pub fn new(id: u8, control_bits: u8, payload: Bytes) -> Self {
        Self {
            id,
            control_bits,
            payload,
        }
    }

    /// Creates a new packet with the given packet_type.
    pub fn new_from_type(control_bits: u8, packet_type: Box<dyn PacketType>) -> Self {
        let mut encoder = Encoder::new();
        packet_type.serialize(&mut encoder);

        Self {
            id: packet_type.packet_id(),
            control_bits,
            payload: encoder.take_bytes(),
        }
    }

    /// Creates a new packet from Bytes, parsing the contents.
    pub fn from(buffer: Bytes) -> types::Result<Packet> {
        if !buffer.has_remaining() {
            return Err(NetworkingError::InvalidPacketFormat.into());
        }

        let packet_id = buffer[0];
        let control_bits = buffer[1];
        let payload = &buffer[2..];

        Ok(Packet {
            id: packet_id,
            control_bits,
            payload: Bytes::copy_from_slice(payload),
        })
    }

    /// Returns the total size of this packet, considering the payload.
    pub fn total_size(&self) -> usize {
        PACKET_HEADER_SIZE + self.payload.len()
    }

    /// Converts the packet payload to the given PacketType
    pub fn receive_payload<T: PacketType>(&self, packet_type: &mut T) -> types::Result<()> {
        let mut decoder = Decoder::new(&self.payload);
        packet_type.deserialize(&mut decoder)?;
        Ok(())
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Packet\n")?;
        write!(f, "    - Type Id: {}\n", self.id)?;
        write!(f, "    - Payload Size: {}\n", self.payload.len())?;

        write!(f, "    - Control bits: {:08b}\n", self.control_bits)?;

        write!(f, "    - Payload: ")?;
        for i in 0..std::cmp::min(50, self.payload.len()) {
            write!(f, "{} ", self.payload[i])?;
        }
        write!(f, "\n")?;

        write!(f, "    - String Payload: ")?;
        let utf8_payload = std::str::from_utf8(&self.payload);
        match utf8_payload {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "Unable to decode as UTF-8"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn packet_parse() {
        let payload = Bytes::from("hello world!");
        let packet1 = Packet {
            id: 1,
            control_bits: 0b0000_0010,
            payload,
        };

        let buffer = packet1.encode();
        let packet = Packet::from(buffer.clone());
        assert!(packet.is_ok());

        let packet2 = packet.unwrap();
        assert_eq!(packet1, packet2);
    }
}

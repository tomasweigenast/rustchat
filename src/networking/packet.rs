use std::fmt;

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{coding::Decoder, types};

use super::error::NetworkingError;

pub const MAX_PACKET_SIZE: usize = 576;
const PACKET_HEADER_SIZE: usize = 12;
const ACK_FLAG: u8 = 0b0000_0001;
const SYN_FLAG: u8 = 0b0000_0010;

#[derive(Debug, PartialEq)]
pub struct Packet {
    pub id: u8,
    pub payload_size: u16,
    pub seq_ack_number: u32,
    pub mac_number: u32,
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
        buffer.put_u16(self.payload_size);
        buffer.put_u8(self.control_bits);
        buffer.put_u32(self.seq_ack_number);
        buffer.put_u32(self.mac_number);
        buffer.put_slice(&self.payload);

        // Convert BytesMut to Bytes for immutability
        buffer.freeze()
    }

    /// Creates a new packet from a BytesMut reference, parsing the contents.
    pub fn from(buffer: Bytes) -> types::Result<Packet> {
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

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Packet\n")?;
        write!(f, "    - Type Id: {}\n", self.id)?;
        write!(f, "    - Payload Size: {}\n", self.payload_size)?;
        write!(f, "    - Sequence/Ack: {}\n", self.seq_ack_number)?;
        write!(f, "    - Mac Number: {}\n", self.mac_number)?;

        let ack_bit = (self.control_bits & ACK_FLAG) != 0;
        let syn_bit = (self.control_bits & SYN_FLAG) != 0;

        write!(f, "    - Control bits: {:08b}\n", self.control_bits)?;
        write!(f, "            - ACK: {}\n", ack_bit)?;
        write!(f, "            - SYN: {}\n", syn_bit)?;

        write!(f, "    - Payload: ")?;
        for i in 0..std::cmp::min(self.payload_size as usize, self.payload.len()) {
            write!(f, "{:02X} ", self.payload[i])?;
        }
        write!(f, "\n")?;

        write!(f, "    - String Payload: ")?;
        let utf8_payload = std::str::from_utf8(
            &self.payload[..std::cmp::min(self.payload_size as usize, self.payload.len())],
        );
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
            mac_number: 123456789,
            payload_size: payload.len() as u16,
            seq_ack_number: 12,
            payload,
        };

        let buffer = packet1.encode();
        let packet = Packet::from(buffer.clone());
        assert!(packet.is_ok());

        let packet2 = packet.unwrap();
        assert_eq!(packet1, packet2);
    }
}

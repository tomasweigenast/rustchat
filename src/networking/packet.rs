use bytes::Bytes;

use super::errors::NetworkingError;

#[derive(Debug)]
pub struct Packet {
    id: u8,
    data: Bytes,
}

impl Packet {
    /// Creates a new Packet
    pub fn new(id: u8, data: Bytes) -> Packet {
        Packet { id, data }
    }

    /// Creates a new packet from a Bytes
    pub fn from(buffer: Bytes) -> Result<Packet, NetworkingError> {
        if buffer.len() < 2 {
            return Err(NetworkingError::InvalidPacketFormat);
        }

        Ok(Packet {
            id: buffer[0],
            data: buffer.slice(1..),
        })
    }
}

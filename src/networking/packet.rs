use crate::{coding::Decoder, types::types};

use super::{
    packet_type::{MessagePacket, PacketData, MESSAGE},
    raw_packet::RawPacket,
};

#[derive(Debug)]
pub enum Packet {
    Message(MessagePacket),
}

impl Packet {
    pub fn from(raw_packet: RawPacket) -> types::Result<Self> {
        let mut decoder = Decoder::new(&raw_packet.payload);

        match raw_packet.packet_type {
            MESSAGE => {
                let mut packet = MessagePacket::default();
                packet.deserialize(&mut decoder)?;
                return Ok(Packet::Message(packet));
            }
            _ => Err("invalid-packet".into()),
        }
    }
}

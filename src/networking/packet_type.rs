use crate::{
    coding::{Decoder, Encoder},
    types::Result,
};

use super::message_payload::{DestinationType, MessagePayload};

/// A user trying to sign in to the server.
pub const SIGN_IN: u8 = 1;

/// A user trying to sign out from the server.
pub const SIGN_OUT: u8 = 2;

/// A message from an user
pub const MESSAGE: u8 = 3;

pub trait PacketData: Send {
    fn packet_id(&self) -> u8;
    fn deserialize(&mut self, data: &mut Decoder) -> Result<()>;
    fn serialize(&self, encoder: &mut Encoder);
}

#[derive(Debug, Default, PartialEq)]
pub struct LoginPacket {
    username: String,
    password: String,
}

impl PacketData for LoginPacket {
    fn deserialize(&mut self, data: &mut Decoder) -> Result<()> {
        self.username = data.read_string()?;
        self.password = data.read_string()?;
        Ok(())
    }

    fn serialize(&self, encoder: &mut Encoder) {
        encoder.write_string_ref(&self.username);
        encoder.write_string_ref(&self.password);
    }

    fn packet_id(&self) -> u8 {
        SIGN_IN
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct LogoutPacket {
    session_id: i64,
}

impl PacketData for LogoutPacket {
    fn deserialize(&mut self, data: &mut Decoder) -> Result<()> {
        self.session_id = data.read_i64()?;
        Ok(())
    }

    fn serialize(&self, encoder: &mut Encoder) {
        encoder.write_i64(self.session_id);
    }

    fn packet_id(&self) -> u8 {
        SIGN_OUT
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct MessagePacket {
    /// The id of the destination. Can be an individual user or a channel
    pub destination: i32,

    /// Indicates the destination type: Channel/User
    pub destination_type: DestinationType,

    /// The payload of the message
    pub message_payload: MessagePayload,
}

impl PacketData for MessagePacket {
    fn deserialize(&mut self, data: &mut Decoder) -> Result<()> {
        self.destination = data.read_i32()?;
        self.destination_type = DestinationType::from(data.read_u8()?);

        match data.read_i8()? {
            1 => {
                let message = data.read_string()?;
                self.message_payload = MessagePayload::Text(message);
                Ok(())
            }
            2 => {
                let buffer = data.read_bytes()?;
                self.message_payload = MessagePayload::File(buffer);
                Ok(())
            }
            _ => Err("unknown-message-type".into()),
        }
    }

    fn serialize(&self, encoder: &mut Encoder) {
        encoder.write_i32(self.destination);
        encoder.write_u8(self.destination_type.to_code());

        match &self.message_payload {
            MessagePayload::Text(text) => {
                encoder.write_i8(1);
                encoder.write_string_ref(text);
            }
            MessagePayload::File(buffer) => {
                encoder.write_i8(2);
                encoder.write_bytes(buffer);
            }
            MessagePayload::Invalid => todo!(),
        }
    }

    fn packet_id(&self) -> u8 {
        MESSAGE
    }
}

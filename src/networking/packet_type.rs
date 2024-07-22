use crate::{
    coding::{Decoder, Encoder},
    types::Result,
};

/// A user trying to sign in to the server.
pub const SIGN_IN: u8 = 1;

/// A user trying to sign out from the server.
pub const SIGN_OUT: u8 = 2;

pub trait PacketType: Send {
    fn packet_id(&self) -> u8;
    fn deserialize(&mut self, data: &mut Decoder) -> Result<()>;
    fn serialize(&self, encoder: &mut Encoder);
}

#[derive(Debug, Default, PartialEq)]
pub struct LoginPacket {
    username: String,
    password: String,
}

impl PacketType for LoginPacket {
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
    session_id: String,
}

impl PacketType for LogoutPacket {
    fn deserialize(&mut self, data: &mut Decoder) -> Result<()> {
        self.session_id = data.read_string()?;
        Ok(())
    }

    fn serialize(&self, encoder: &mut Encoder) {
        encoder.write_string_ref(&self.session_id);
    }

    fn packet_id(&self) -> u8 {
        SIGN_OUT
    }
}

pub enum PacketPayload {
    Login(LoginPacket),
    Logout(LogoutPacket),
}

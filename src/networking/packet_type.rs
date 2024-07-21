use bytes::Bytes;

use crate::types::Result;

/// A user trying to sign in to the server.
pub const SIGN_IN: u8 = 1;

/// A user trying to sign out from the server.
pub const SIGN_OUT: u8 = 2;

// Define a trait for packet deserialization
pub trait DeserializePacket {
    fn deserialize(data: &Bytes) -> Result<Self>
    where
        Self: Sized;
}

pub struct LoginPacket {
    username: String,
    password: String,
}

impl DeserializePacket for LoginPacket {
    fn deserialize(data: &Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
        // Ok(LoginPacket {})
    }
}

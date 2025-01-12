use bytes::Bytes;

#[derive(Debug, PartialEq)]
pub enum MessagePayload {
    Text(String),
    File(Bytes),
}

#[derive(Debug, PartialEq)]
pub enum DestinationType {
    User,
    Channel,
    Unknown,
}

impl DestinationType {
    pub fn from(code: u8) -> Self {
        match code {
            1 => Self::User,
            2 => Self::Channel,
            _ => Self::Unknown,
        }
    }

    pub fn to_code(&self) -> u8 {
        match &self {
            DestinationType::User => 1,
            DestinationType::Channel => 2,
            DestinationType::Unknown => 0,
        }
    }
}

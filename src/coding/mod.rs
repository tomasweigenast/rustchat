pub mod decoder;
pub mod encoder;
pub mod varint;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub use decoder::Decoder;
pub use encoder::Encoder;

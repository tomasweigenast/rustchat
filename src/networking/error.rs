use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum NetworkingError {
    #[snafu(display("invalid packet format"))]
    InvalidPacketFormat,
}

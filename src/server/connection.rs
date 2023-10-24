use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{networking::packet::Packet, types};

pub struct Connection {
    /// the outbound connection
    pub stream: TcpStream,

    /// a buffer used to store received packets
    pub read_buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            read_buffer: BytesMut::with_capacity(1024 * 4),
            stream,
        }
    }

    pub async fn read_packet(&mut self) -> types::Result<Packet> {
        let n: usize = self.stream.read_buf(&mut self.read_buffer).await?;
        if n == 0 {
            return Err("connection reset by peer".into());
        }

        return self.parse_packet().await;
    }

    async fn parse_packet(&mut self) -> types::Result<Packet> {
        let buf = &self.read_buffer;
        Packet::from(buf)
    }
}

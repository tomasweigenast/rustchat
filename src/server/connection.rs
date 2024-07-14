use std::net::SocketAddr;

use bytes::{BufMut, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use crate::{
    coding::varint::{self},
    networking::packet::Packet,
    types,
};

#[derive(Debug)]
pub struct Connection {
    /// The outbound connection
    pub stream: BufWriter<TcpStream>,

    /// A buffer used to store received packets
    pub read_buffer: BytesMut,

    /// The socket address
    pub address: SocketAddr,
}

impl Connection {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        Self {
            address,
            read_buffer: BytesMut::with_capacity(1024 * 4),
            stream: BufWriter::new(stream),
        }
    }

    pub async fn read_packet(&mut self) -> types::Result<Packet> {
        let n: usize = self.stream.read_buf(&mut self.read_buffer).await?;
        if n == 0 {
            return Err("connection reset by peer".into());
        }

        return self.parse_packet().await;
    }

    pub async fn write_packet(&mut self, packet: Packet) -> types::Result<()> {
        self.stream.write_u8(packet.id).await?;

        let mut buffer = BytesMut::with_capacity(packet.size.try_into()?);
        varint::write_varlong(packet.size.try_into()?, &mut buffer);
        buffer.put(packet.data);

        let mut freezed = buffer.freeze();
        self.stream.write_buf(&mut freezed).await?;
        self.stream.flush().await?;
        Ok(())
    }

    async fn parse_packet(&mut self) -> types::Result<Packet> {
        let buf = &self.read_buffer;
        Packet::from(buf)
    }
}

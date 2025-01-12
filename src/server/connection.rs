use std::net::SocketAddr;

use futures::SinkExt;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};
use tokio_stream::StreamExt;
use tokio_util::codec::*;

use crate::{networking::packet::Packet, networking::packet::MAX_PACKET_SIZE, types};

#[derive(Debug)]
pub struct Connection {
    /// The outbound connection
    pub stream: Framed<TcpStream, LengthDelimitedCodec>,

    /// The socket address
    pub address: SocketAddr,
}

impl Connection {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        let transport = LengthDelimitedCodec::builder()
            .big_endian()
            .max_frame_length(MAX_PACKET_SIZE)
            .length_field_type::<u32>()
            .length_adjustment(0)
            .length_field_offset(0)
            .length_field_length(4)
            .new_framed(stream);

        Self {
            address,
            stream: transport,
        }
    }

    pub async fn read_packet(&mut self) -> types::Result<Packet> {
        let result: Option<bytes::BytesMut> = self.stream.try_next().await?;
        if let Some(buffer) = result {
            return Ok(Packet::from(buffer.freeze())?);
        }

        Err("no data available".into())
    }

    pub async fn write_packet(&mut self, packet: Packet) -> types::Result<()> {
        let buffer = packet.encode();
        self.stream.send(buffer).await?;
        Ok(())
    }
}

use std::net::SocketAddr;

use futures::SinkExt;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};
use tokio_stream::StreamExt;
use tokio_tungstenite::WebSocketStream;
use tokio_util::codec::*;

use crate::{networking::packet::Packet, networking::packet::MAX_PACKET_SIZE, types};

use super::framed_websocket::WebSocketAdapter;

pub trait ConnectionHandle {
    async fn read_packet(&mut self) -> types::Result<Packet>;
    async fn write_packet(&mut self, packet: Packet) -> types::Result<()>;
}

#[derive(Debug)]
pub struct Connection<T: AsyncRead + AsyncWrite + Unpin> {
    /// The outbound connection
    pub stream: Framed<T, LengthDelimitedCodec>,

    /// The socket address
    pub address: SocketAddr,
}

impl<T> Connection<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(address: SocketAddr, transport: T) -> Self {
        let codec = LengthDelimitedCodec::builder()
            .big_endian()
            .max_frame_length(MAX_PACKET_SIZE)
            .length_field_type::<u32>()
            .length_adjustment(0)
            .length_field_offset(0)
            .length_field_length(4)
            .new_framed(transport);

        Self {
            address,
            stream: codec,
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

impl Connection<TcpStream> {
    pub fn from_tcp(address: SocketAddr, stream: TcpStream) -> Self {
        Self::new(address, stream)
    }
}

impl Connection<WebSocketAdapter> {
    pub fn from_websocket(address: SocketAddr, stream: WebSocketStream<TcpStream>) -> Self {
        Self::new(address, WebSocketAdapter::new(stream))
    }
}

impl<T> ConnectionHandle for Connection<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    async fn read_packet(&mut self) -> types::Result<Packet> {
        self.read_packet().await
    }

    async fn write_packet(&mut self, packet: Packet) -> types::Result<()> {
        return self.write_packet(packet).await;
    }
}

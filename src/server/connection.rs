use std::net::SocketAddr;

use async_trait::async_trait;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_tungstenite::WebSocketStream;
use tokio_util::codec::*;

use crate::{networking::packet::Packet, networking::packet::MAX_PACKET_SIZE, types};

use super::framed_websocket::WebSocketAdapter;

#[async_trait]
pub trait ConnectionHandle {
    async fn read_packet(&mut self) -> types::Result<Packet>;
    async fn write_packet(&mut self, packet: Packet) -> types::Result<()>;
    fn socket(&self) -> SocketAddr;
}

#[derive(Debug)]
pub struct TcpConnection {
    stream: Framed<TcpStream, LengthDelimitedCodec>,
    address: SocketAddr,
}

impl TcpConnection {
    pub fn new(address: SocketAddr, transport: TcpStream) -> Self {
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
}

#[async_trait]
impl ConnectionHandle for TcpConnection {
    async fn read_packet(&mut self) -> types::Result<Packet> {
        let result: Option<bytes::BytesMut> = self.stream.try_next().await?;
        if let Some(buffer) = result {
            return Ok(Packet::from(buffer.freeze())?);
        }

        Err("no data available".into())
    }

    async fn write_packet(&mut self, packet: Packet) -> types::Result<()> {
        let buffer = packet.encode();
        self.stream.send(buffer).await?;
        Ok(())
    }

    fn socket(&self) -> SocketAddr {
        self.address
    }
}

#[derive(Debug)]
pub struct WebSocketConnection {
    stream: Framed<WebSocketAdapter, LengthDelimitedCodec>,
    address: SocketAddr,
}

impl WebSocketConnection {
    pub fn new(address: SocketAddr, transport: WebSocketStream<TcpStream>) -> Self {
        let transport = WebSocketAdapter::new(transport);
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
}

#[async_trait]
impl ConnectionHandle for WebSocketConnection {
    async fn read_packet(&mut self) -> types::Result<Packet> {
        let result: Option<bytes::BytesMut> = self.stream.try_next().await?;
        if let Some(buffer) = result {
            return Ok(Packet::from(buffer.freeze())?);
        }

        Err("no data available".into())
    }

    async fn write_packet(&mut self, packet: Packet) -> types::Result<()> {
        let buffer = packet.encode();
        self.stream.send(buffer).await?;
        Ok(())
    }

    fn socket(&self) -> SocketAddr {
        self.address
    }
}

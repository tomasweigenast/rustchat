use std::net::SocketAddr;

use bytes::{Bytes, BytesMut};
use tokio::net::{TcpStream, UdpSocket};

use crate::{
    networking::packet::{Packet, MAX_PACKET_SIZE},
    types::types::{self},
};

/// Server is meant to be a singleton that maintains the actual server state
#[derive(Debug)]
pub struct Server {
    _listener: UdpSocket,
}

/// Run the server.
pub async fn run(listener: UdpSocket) {
    let mut server = Server {
        _listener: listener,
    };

    tokio::select! {
        res = server.run() => {
            // If an error is received here, accepting connections from the TCP
            // listener failed multiple times and the server is giving up and
            // shutting down.
            //
            // Errors encountered when handling individual connections do not
            // bubble up to this point.
            if let Err(err) = res {
                println!("failed to accept, err {}", err);
            }
        }
    }
}

impl Server {
    pub async fn new(endpoint: &str) -> Result<Server, Box<dyn std::error::Error>> {
        // create the UDP socket
        let listener = UdpSocket::bind(endpoint).await?;
        println!("UDP server started at {}", endpoint);

        Ok(Server {
            _listener: listener, // now listener is owned by Server
        })
    }

    pub async fn run(&mut self) -> types::Result<()> {
        let mut buf = [0; MAX_PACKET_SIZE];
        loop {
            let (len, addr) = self._listener.recv_from(&mut buf).await?;

            // copy buffer
            // let mut buffer = BytesMut::with_capacity(256);
            // buffer.extend_from_slice(&buf);

            // parse packet
            let buffer = Bytes::copy_from_slice(&buf);
            let packet = Packet::from(buffer).unwrap();

            println!("{}", packet);

            // answer with the same packet
            self._listener.send_to(&packet.payload, addr).await?;
        }
    }

    async fn accept(&mut self) -> types::Result<(SocketAddr, TcpStream)> {
        unimplemented!();
        // TODO: add backoff to try multiple times
        // loop {
        //     match self._listener.accept().await {
        //         Ok((socket, address)) => return Ok((address, socket)),
        //         Err(err) => {
        //             return Err(err.into());
        //         }
        //     }
        // }
    }
}

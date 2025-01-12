use std::{net::SocketAddr, sync::Arc};

use futures::StreamExt;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};
use tokio_tungstenite::accept_async;

use crate::types::types::{self};

use super::{
    connection::{Connection, ConnectionHandle},
    user::User,
};

/// Server is meant to be a singleton that maintains the actual server state
#[derive(Debug)]
pub struct Server {
    _listener: TcpListener,

    /// The list of clients in the server
    pub clients: Vec<Arc<RwLock<User>>>,
}

/// Run the server.
pub async fn run(listener: TcpListener) {
    let mut server = Server {
        _listener: listener,
        clients: vec![],
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
        let listener = TcpListener::bind(endpoint).await?;

        Ok(Server {
            _listener: listener,
            clients: vec![],
        })
    }

    pub async fn run(&mut self) -> types::Result<()> {
        println!("Running on {}", self._listener.local_addr().unwrap());

        loop {
            let (socket, stream) = self.accept().await?;
            self.handle_connection(socket, stream).await?;

            // let mut client = User::new(Connection::new(socket, stream));

            // tokio::spawn(async move {
            //     // TODO: If an error is encountered, log it.
            //     // Process the connection.
            //     client.run().await.unwrap();
            // });
        }
    }

    pub async fn handle_connection(
        &mut self,
        socket: SocketAddr,
        stream: TcpStream,
    ) -> types::Result<Box<dyn ConnectionHandle>> {
        let mut buf = [0; 4];

        // Peek first 4 bytes to check if its a GET (websocket connection)
        let is_websocket: bool = match stream.peek(&mut buf).await {
            Ok(read) => {
                read >= 4 && &buf[..4] == b"GET"
                // This is a websocket connection

                // if let Ok(ws) = accept_async(stream).await {
                //     // let connection = Connection::from_websocket(socket, ws);
                //     return true
                // } else {
                //     // invalid websocket handshake, drop the connection
                //     return false
                // }

                // false
            }
            Err(_) => false,
        };

        if is_websocket {
            Ok(Box::new(Connection::from_tcp(socket, stream)))
        } else {
            let ws = accept_async(stream).await?;
            Ok(Box::new(Connection::from_websocket(socket, ws)))
        }
    }

    async fn accept(&mut self) -> types::Result<(SocketAddr, TcpStream)> {
        // TODO: add backoff to try multiple times
        match self._listener.accept().await {
            Ok((socket, address)) => return Ok((address, socket)),
            Err(err) => {
                return Err(err.into());
            }
        }
    }
}

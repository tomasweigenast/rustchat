use std::{net::SocketAddr, sync::Arc};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};
use tokio_tungstenite::accept_async;

use crate::types::types::{self};

use super::{
    connection::{ConnectionHandle, TcpConnection, WebSocketConnection},
    user::User,
};

/// Server is meant to be a singleton that maintains the actual server state
pub struct Server {
    _listener: TcpListener,

    /// The list of clients in the server
    pub clients: Vec<Arc<RwLock<User>>>,
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
            // accept tcp connection
            let (socket, stream) = self.accept().await?;

            // determine the source of the connection
            let mut buf = [0; 3];

            // Peek first 4 bytes to check if its a GET (websocket connection)
            let is_websocket: bool = match stream.peek(&mut buf).await {
                Ok(read) => read >= 3 && &buf[..3] == b"GET",
                Err(_) => false,
            };

            let connection_handle: Box<dyn ConnectionHandle + Send>;
            if is_websocket {
                match accept_async(stream).await {
                    Ok(ws) => {
                        connection_handle = Box::new(WebSocketConnection::new(socket, ws));
                    }
                    Err(err) => {
                        println!("could not connect via websocket: {}", err);

                        // continue to the next loop to avoid using connection_handle
                        // if its not initialized
                        continue;
                    }
                }
            } else {
                connection_handle = Box::new(TcpConnection::new(socket, stream));
            }

            // create a client and spawn a new task to handle it
            println!("Client connected from {}", socket);
            let mut client = User::new(connection_handle);
            tokio::spawn(async move {
                // TODO: If an error is encountered, log it.
                // Process the connection.
                client.run().await.unwrap();
                println!("Client disconnected.");
            });
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

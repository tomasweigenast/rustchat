use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use tokio::{net::TcpListener, sync::RwLock, time::sleep};
use tokio_tungstenite::accept_async;

use crate::types::types::{self};

use super::{
    connection::{ConnectionHandle, TcpConnection, WebSocketConnection},
    user::User,
};

/// Server is meant to be a singleton that maintains the actual server state
pub struct Server {
    listener: TcpListener,
    metadata: Arc<ServerMetadata>,
}

pub struct ServerMetadata {
    /// The list of clients in the server
    pub clients: RwLock<HashMap<SocketAddr, Arc<RwLock<User>>>>,
}

impl Server {
    pub async fn new(endpoint: &str) -> Result<Server, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(endpoint).await?;

        Ok(Server {
            listener,
            metadata: Arc::new(ServerMetadata {
                clients: RwLock::new(HashMap::new()),
            }),
        })
    }

    pub async fn run(&mut self) -> types::Result<()> {
        println!("Running on {}", self.listener.local_addr().unwrap());
        self.stats();

        loop {
            // accept tcp connection
            let (stream, socket) = self.listener.accept().await?;

            // determine the source of the connection
            let mut buf = [0; 3];

            // Peek first 4 bytes to check if its a GET (websocket connection)
            let is_websocket: bool = match stream.peek(&mut buf).await {
                Ok(read) => read >= 3 && &buf[..3] == b"GET",
                Err(_) => false,
            };

            let connection_handle: Box<dyn ConnectionHandle + Send + Sync>;
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
            let client = Arc::new(RwLock::new(User::new(connection_handle)));
            self.metadata.add_client(socket, client.clone()).await;

            // Clone server metadata to give it to the tokio task
            let server_metadata = self.metadata.clone();
            tokio::spawn(async move {
                // TODO: If an error is encountered, log it.

                // Process the connection.
                let mut client = client.write().await;
                if let Err(err) = client.run().await {
                    println!("An error occurred handling user: {}", err);
                }

                println!("Client {} disconnected.", socket);
                server_metadata.remove_client(&socket).await;
            });
        }
    }

    pub fn stats(&self) {
        let server_metadata = self.metadata.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await;
                println!("---------- Server Stats ----------");
                println!(
                    "Connected users: {}",
                    server_metadata.clients.read().await.len()
                );
                println!("----------------------------------");
            }
        });
    }
}

impl ServerMetadata {
    async fn add_client(self: &Arc<Self>, socket: SocketAddr, user: Arc<RwLock<User>>) {
        self.clients.write().await.insert(socket, user);
    }

    async fn remove_client(self: &Arc<Self>, socket: &SocketAddr) {
        self.clients.write().await.remove(socket);
    }
}

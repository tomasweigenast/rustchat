use std::{sync::Arc, time::Duration};

use tokio::{
    net::{unix::uid_t, TcpListener},
    sync::RwLock,
    time::sleep,
};
use tokio_tungstenite::accept_async;

use crate::types::types::{self};

use super::{
    connection::{ConnectionHandle, TcpConnection, WebSocketConnection},
    database::Database,
    user::User,
};

/// Server is meant to be a singleton that maintains the actual server state
pub struct Server {
    listener: TcpListener,
    db: Arc<Database>,
}

impl Server {
    pub async fn new(endpoint: &str) -> Result<Server, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(endpoint).await?;

        Ok(Server {
            listener,
            db: Arc::new(Database::new()),
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
            let user = User::new(connection_handle);
            let uid = &user.id();
            let client = Arc::new(RwLock::new(user));
            self.db.add_client(uid, client.clone()).await;

            // Clone server metadata to give it to the tokio task
            let db = self.db.clone();
            tokio::spawn(async move {
                // TODO: If an error is encountered, log it.

                // Process the connection.
                let mut client = client.write().await;
                if let Err(err) = client.run().await {
                    println!("An error occurred handling user: {}", err);
                }

                println!("Client {} disconnected.", socket);
                db.remove_client(&client.id()).await;
            });
        }
    }

    pub fn stats(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await;
                println!("---------- Server Stats ----------");
                println!("Connected users: {}", db.total_clients().await);
                println!("----------------------------------");
            }
        });
    }
}

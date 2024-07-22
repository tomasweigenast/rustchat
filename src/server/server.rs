use std::{net::SocketAddr, sync::Arc};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

use crate::types::types::{self};

use super::{connection::Connection, user::User};

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
            let (socket, address) = self.accept().await?;
            let mut client = User::new(Connection::new(socket, address));

            tokio::spawn(async move {
                // TODO: If an error is encountered, log it.
                // Process the connection.
                client.run().await.unwrap();
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

use std::{net::SocketAddr, sync::Arc};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

use super::user::User;

/// Server is meant to be a singleton that maintains the actual server state
#[derive(Debug)]
pub struct Server {
    _listener: TcpListener,

    /// The list of clients in the server
    pub clients: Vec<User>,
}

/*lazy_static! {
    /// Singleton instance of the server
    pub static ref SERVER: RwLock<Server> = RwLock::new(Server::create());
}*/

impl Server {
    pub async fn new(endpoint: &str) -> Result<Server, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(endpoint).await?;

        Ok(Server {
            _listener: listener,
            clients: vec![],
        })
    }

    pub async fn run(self) {
        let sv = Arc::new(RwLock::new(self));

        while let Ok((stream, address)) = sv.read().await._listener.accept().await {
            let sv_clone = sv.clone();
            tokio::spawn(async move {
                sv_clone.write().await.handle_client(stream, address).await;
            });
        }
    }

    async fn handle_client(&mut self, stream: TcpStream, address: SocketAddr) {
        match stream.set_nodelay(true) {
            Ok(_) => (),
            Err(err) => eprintln!("set_nodelay call fail, error: {}", err),
        }

        // register the new client
        match User::new(stream, address) {
            Ok(user) => self.clients.push(user),
            Err(err) => eprintln!("new user call fail, error: {}", err),
        }
    }
}

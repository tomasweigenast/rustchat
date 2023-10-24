use std::{net::SocketAddr, sync::Arc};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        RwLock,
    },
};

use super::user::User;

/// Server is meant to be a singleton that maintains the actual server state
#[derive(Debug)]
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

    pub async fn run(self) {
        let sv = Arc::new(RwLock::new(self));
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32);

        // start task to listen for messages
        // let sv_clone = sv.clone();
        // tokio::spawn(async move {
        //     sv_clone.write().await.handle_messages(&mut rx).await;
        // });

        // start listening for incoming connections
        let sv_clone = sv.clone(); // Clone for reading outside the loop
        tokio::spawn(async move {
            let sv_reader = sv_clone.read().await;
            while let Ok((stream, address)) = sv_reader._listener.accept().await {
                let sv_clone = sv.clone(); // Clone for writing within the spawned task
                let tx_clone = tx.clone();

                tokio::spawn(async move {
                    let mut sv_writer = sv_clone.write().await;
                    sv_writer.handle_client(tx_clone, stream, address).await;
                });
            }
        })
        .await
        .expect("error running main task");
    }

    async fn handle_messages(&mut self, rx: &mut Receiver<Vec<u8>>) {
        while let Some(message) = rx.recv().await {
            let pretty_msg = String::from_utf8(message).expect("not an string");

            println!("Received message: {}", pretty_msg);
        }
    }

    async fn handle_client(&mut self, tx: Sender<Vec<u8>>, stream: TcpStream, address: SocketAddr) {
        println!("received new connection from {:}", address);

        match stream.set_nodelay(true) {
            Ok(_) => (),
            Err(err) => eprintln!("set_nodelay call fail, error: {}", err),
        }

        // register the new client
        match User::new(tx, stream, address) {
            Ok(user) => {
                let user_arc = Arc::new(RwLock::new(user));
                self.clients.push(user_arc.clone());
                user_arc.write().await.begin().await;
            }
            Err(err) => eprintln!("new user call fail, error: {}", err),
        }
    }
}

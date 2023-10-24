use std::{io::Error, net::SocketAddr, time::Instant};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

// User represents a person that is connected to the server
#[derive(Debug)]
pub struct User {
    /// The current state of the user
    state: UserState,

    /// The stats of the user
    stats: UserStats,

    /// The remote address of the user
    address: SocketAddr,

    /// The read/write stream of the user
    stream: TcpStream,

    tx: mpsc::Sender<Vec<u8>>,
}

#[derive(Debug)]
pub struct UserStats {
    /// The instant when the user joined, measured from the Join state
    join_at: Option<Instant>,

    /// The instant when the user made the last interaction with the server
    last_interaction: Option<Instant>,
}

#[derive(Debug, PartialEq)]
pub enum UserState {
    /// The default state of an user. Indicates the user is issuing a handshake with the server
    Handshake,

    /// The user was accepted in the server and is interacting in it
    Join,

    /// The user is being disconnected from the server
    Disconnect,
}

impl User {
    /// Creates a new user for the given TcpStream, which is borrowed and owned by User
    pub fn new(
        tx: mpsc::Sender<Vec<u8>>,
        stream: TcpStream,
        address: SocketAddr,
    ) -> Result<Self, Error> {
        Ok(User {
            state: UserState::Handshake,
            stats: UserStats {
                join_at: None,
                last_interaction: None,
            },
            tx,
            address,
            stream,
        })
    }

    pub async fn begin(&mut self) {
        println!("Listening for user {:} messages", self.address);
        let mut buffer = [0; 1024];
        loop {
            match self.stream.read(&mut buffer).await {
                Ok(n) => {
                    if n == 0 {
                        // End of stream, the client has disconnected.
                        break;
                    }

                    // let received_message =
                    //     String::from_utf8_lossy(&buffer[0..n]).trim().to_string();
                    self.tx
                        .send(buffer.into())
                        .await
                        .expect("Message channel closed");
                }
                Err(err) => eprintln!("Failed to read from stream: {:}", err),
            }
        }
    }

    /// Disconnects the user
    pub async fn disconnect(&mut self) -> Result<(), Error> {
        if self.state != UserState::Join {
            return Ok(());
        }

        let result = self.stream.shutdown().await;
        match result {
            Ok(_) => {
                self.state = UserState::Disconnect;
                self.stats.last_interaction = Some(Instant::now());
                return Ok(());
            }
            Err(_) => result,
        }
    }
}

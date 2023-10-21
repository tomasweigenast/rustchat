use std::{io::Error, net::SocketAddr, time::Instant};

use tokio::{io::AsyncWriteExt, net::TcpStream};

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
    pub fn new(stream: TcpStream, address: SocketAddr) -> Result<User, Error> {
        Ok(User {
            state: UserState::Handshake,
            stats: UserStats {
                join_at: None,
                last_interaction: None,
            },
            address,
            stream,
        })
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

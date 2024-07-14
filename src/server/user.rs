use std::time::Instant;

use crate::types::types;

use super::connection::Connection;

// User represents a person that is connected to the server
#[derive(Debug)]
pub struct User {
    /// The current state of the user
    state: UserState,

    /// The stats of the user
    stats: UserStats,

    /// The read/write stream of the user
    connection: Connection,
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
    /// Creates a new user for the given Connection, which is borrowed and owned by User
    pub fn new(connection: Connection) -> Self {
        User {
            state: UserState::Handshake,
            stats: UserStats {
                join_at: None,
                last_interaction: None,
            },
            connection,
        }
    }

    /// Process a single connection
    pub async fn run(&mut self) -> types::Result<()> {
        // while !self.shutdown.is_shutdown() {

        // here tokio::select! is used to await until self.connection.read_packet() OR self.shutdown.recv() completes
        let packet = tokio::select! {
            res = self.connection.read_packet() => res?,
            // _ = self.shutdown.recv() => {
            //     // If a shutdown signal is received, return from `run`.
            //     // This will result in the task terminating.
            //     return Ok(());
            // }
        };

        println!(
            "Received packet {:?} from {}",
            packet, self.connection.address
        );

        self.connection.write_packet(packet).await.unwrap();
        println!("Response sent.");

        Ok(())
    }

    // Disconnects the user
    // pub async fn disconnect(&mut self) -> Result<(), Error> {
    //     if self.state != UserState::Join {
    //         return Ok(());
    //     }

    //     let result = self.stream.shutdown().await;
    //     match result {
    //         Ok(_) => {
    //             self.state = UserState::Disconnect;
    //             self.stats.last_interaction = Some(Instant::now());
    //             return Ok(());
    //         }
    //         Err(_) => result,
    //     }
    // }
}

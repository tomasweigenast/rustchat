use std::{any::Any, time::Instant};

use bytes::Bytes;
use uuid::Uuid;

use crate::{
    networking::{packet::Packet, raw_packet::RawPacket},
    types::types,
};

use super::connection::ConnectionHandle;

// User represents a person that is connected to the server
pub struct User {
    id: Uuid,

    /// The stats of the user
    stats: UserStats,

    /// The read/write stream of the user
    connection: Box<dyn ConnectionHandle + Send + Sync>,
}

#[derive(Debug)]
pub struct UserStats {
    /// The instant when the user joined, measured from the Join state
    join_at: Instant,

    /// The instant when the user made the last interaction with the server
    last_interaction: Option<Instant>,
}

impl User {
    /// Creates a new user for the given Connection, which is borrowed and owned by User
    pub fn new(connection: Box<dyn ConnectionHandle + Send + Sync>) -> Self {
        User {
            id: Uuid::new_v4(),
            stats: UserStats {
                join_at: Instant::now(),
                last_interaction: None,
            },
            connection,
        }
    }

    pub fn id(&self) -> Uuid {
        return self.id;
    }

    /// Process a single connection
    pub async fn run(&mut self) -> types::Result<()> {
        // while !self.shutdown.is_shutdown() {

        loop {
            // here tokio::select! is used to await until self.connection.read_packet() OR self.shutdown.recv() completes
            let packet = tokio::select! {
                res = self.connection.read_packet() => res,
                // _ = self.shutdown.recv() => {
                //     // If a shutdown signal is received, return from `run`.
                //     // This will result in the task terminating.
                //     return Ok(());
                // }
            };

            match packet {
                Ok(raw_packet) => {
                    println!(
                        "Received packet [{}] from {}",
                        raw_packet,
                        self.connection.socket()
                    );

                    if let Ok(packet) = Packet::from(raw_packet) {
                        self.handle_packet(packet);
                    }

                    // TODO: ignore invalid packets?

                    // packet.receive_payload(packet_type)

                    // self.connection.write_packet(packet).await.unwrap();
                    // println!("Response sent.");
                }
                Err(err) => {
                    println!("Failed to decode packet: {}", err);

                    // TODO: check kind of error before disconnecting
                    self.connection
                        .write_packet(RawPacket::new(1, Bytes::from("wrong packet")))
                        .await
                        .unwrap_or_default();
                    println!("Response sent.");

                    // break the loop and return to the caller
                    return Ok(());
                }
            }
        }
    }

    pub async fn send_packet(&mut self, packet: Packet) -> types::Result<()> {
        let raw_packet = RawPacket::from(packet);
        match self.connection.write_packet(raw_packet).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn handle_packet(&self, packet: Packet) {
        match packet {
            Packet::Message(message_packet) => {}
        }
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

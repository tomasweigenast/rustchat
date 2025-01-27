use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    networking::{message_payload::MessagePayload, packet::Packet, packet_type::MessagePacket},
    types::types,
};

use super::user::User;

pub struct ServerChannel {
    subscribers: HashMap<Uuid, Arc<RwLock<User>>>,
}

impl ServerChannel {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub async fn add_subscriber(&mut self, user: Arc<RwLock<User>>) {
        let user_id = user.read().await.id();
        self.subscribers.insert(user_id, user);
    }

    pub async fn broadcast(&mut self, message: MessagePayload) -> types::Result<()> {
        for (key, value) in &self.subscribers {
            let user = value.write().await;
            user.send_packet(Packet::Message(MessagePacket {}))
        }

        Ok(())
    }
}

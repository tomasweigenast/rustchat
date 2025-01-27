use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use super::{channel::ServerChannel, user::User};

/// Acts as a simple server database
pub struct Database {
    clients: RwLock<HashMap<Uuid, Arc<RwLock<User>>>>,
    channels: RwLock<HashMap<Uuid, Arc<ServerChannel>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            clients: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_client(self: &Arc<Self>, id: &Uuid, user: Arc<RwLock<User>>) {
        self.clients.write().await.insert(*id, user);
    }

    pub async fn remove_client(self: &Arc<Self>, id: &Uuid) {
        self.clients.write().await.remove(id);
    }

    pub async fn get_channel(self: &Arc<Self>, id: Uuid) -> Option<Arc<ServerChannel>> {
        self.channels.read().await.get(&id).cloned()
    }

    pub async fn total_clients(self: &Arc<Self>) -> usize {
        self.clients.read().await.len()
    }
}

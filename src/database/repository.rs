use redis::Commands;
use uuid::Uuid;

use crate::{server::channel::ServerChannel, types};

pub struct Repository {
    client: redis::Client,
}

impl Repository {
    pub fn new(conn_string: &str) -> types::Result<Self> {
        Ok(Self {
            client: redis::Client::open(conn_string)?,
        })
    }

    pub async fn get_channel(&self, id: Uuid) -> types::Result<ServerChannel> {
        self.client.get(key)
    }
}

use std::sync::RwLock;

use lazy_static::lazy_static;

use super::user::User;

/// Server is meant to be a singleton that maintains the actual server state
#[derive(Debug)]
pub struct Server {
    /// The list of clients in the server
    pub clients: Vec<User>,
}

lazy_static! {
    pub static ref SERVER: RwLock<Server> = RwLock::new(Server::create());
}

impl Server {
    fn create() -> Server {
        Server { clients: vec![] }
    }
}

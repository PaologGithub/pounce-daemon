use uuid::Uuid;

use crate::config::node::ServerConfig;

pub struct Server {
    id: Uuid,
    pub name: String
}

impl Server {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name
        }
    }

    pub fn from(config: ServerConfig) -> Self {
        Self {
            id: config.uuid,
            name: config.name
        }
    }
}
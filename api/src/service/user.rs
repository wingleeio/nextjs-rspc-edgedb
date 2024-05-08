use std::sync::Arc;

use edgedb_tokio::Client;

struct User {
    client: Arc<Client>,
}

impl User {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

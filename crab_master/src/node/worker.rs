use common::types::client_id::ClientId;

pub struct Worker {
    client_id: ClientId,
}

impl Worker {
    pub fn new(client_id: ClientId) -> Worker {
        Worker { client_id }
    }
}

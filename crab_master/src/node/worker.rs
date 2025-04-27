use common::types::client_id::NodeId;

pub struct Worker {
    node_id: NodeId,
}

impl Worker {
    pub fn new(node_id: NodeId) -> Worker {
        Worker { node_id }
    }
}

use common::types::node_id::NodeId;

#[derive(Debug)]
pub struct Worker {
    node_id: NodeId,
}

impl Worker {
    pub fn new(node_id: NodeId) -> Worker {
        Worker { node_id }
    }
}

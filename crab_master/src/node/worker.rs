use std::time::Instant;

use common::types::node_id::NodeId;

#[derive(Debug)]
pub struct Worker {
    node_id: NodeId,

    last_heartbeat: Instant,
}

impl Worker {
    pub fn new(node_id: NodeId) -> Worker {
        Worker {
            node_id,
            last_heartbeat: Instant::now(),
        }
    }
}

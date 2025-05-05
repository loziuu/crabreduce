use std::time::SystemTime;

use common::types::node_id::NodeId;

#[derive(Debug)]
pub struct Worker {
    node_id: NodeId,

    last_heartbeat: SystemTime,
}

impl Worker {
    pub fn new(node_id: NodeId) -> Worker {
        Worker {
            node_id,
            last_heartbeat: SystemTime::now(),
        }
    }

    pub fn last_heartbeat(&self) -> &SystemTime {
        &self.last_heartbeat
    }

    pub(crate) fn update_heartbeat(&mut self) {
        self.last_heartbeat = SystemTime::now();
    }
}

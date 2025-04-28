use super::worker::Worker;
use common::types::node_id::NodeId;
use common::types::node_id::NodeIdError;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum ServerError {
    ValidationError(&'static str),
}

pub struct MasterConfiguration {}

#[derive(Debug)]
pub struct MasterNode {
    workers: Arc<Mutex<Vec<Worker>>>,
}

impl Default for MasterNode {
    fn default() -> Self {
        MasterNode::new()
    }
}

impl MasterNode {
    pub fn new() -> MasterNode {
        MasterNode {
            workers: Arc::new(Mutex::new(vec![])),
        }
    }

    // TODO:What if node with given id is already registered?
    pub async fn register_worker(&self, node_id: NodeId) {
        let b = self.workers.clone();
        let mut lock = b.lock().await;
        lock.push(Worker::new(node_id));
    }
}

impl From<NodeIdError> for ServerError {
    fn from(value: NodeIdError) -> Self {
        match value {
            NodeIdError::InvalidValue(msg) => ServerError::ValidationError(msg),
        }
    }
}

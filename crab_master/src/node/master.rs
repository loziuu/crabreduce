use super::worker::Worker;
use common::types::node_id::NodeId;
use common::types::node_id::NodeIdError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub enum ServerError {
    ValidationError(&'static str),
}

#[derive(Debug)]
pub struct MasterConfiguration {
    pub(crate) node_timeout: Duration,
}

impl Default for MasterConfiguration {
    fn default() -> Self {
        Self {
            node_timeout: Duration::from_secs(60),
        }
    }
}

#[derive(Debug)]
pub struct MasterNode {
    workers: Arc<Mutex<HashMap<NodeId, Worker>>>,
    // TODO:Queue with workers?
    config: MasterConfiguration,
}

impl MasterNode {
    pub fn config(&self) -> &MasterConfiguration {
        &self.config
    }

    pub fn borrow_workers(&self) -> Arc<Mutex<HashMap<NodeId, Worker>>> {
        self.workers.clone()
    }
}

impl Default for MasterNode {
    fn default() -> Self {
        MasterNode::new()
    }
}

impl MasterNode {
    pub fn new() -> MasterNode {
        MasterNode {
            // TODO: Use different hashmap?
            workers: Arc::new(Mutex::new(HashMap::new())),
            config: MasterConfiguration::default(),
        }
    }

    // TODO:What if node with given id is already registered?
    pub async fn register_worker(&self, node_id: NodeId) {
        {
            let mut lock = self.workers.lock().await;
            lock.insert(node_id.clone(), Worker::new(node_id.clone()));
        }
    }

    pub async fn register_heartbeat(&self, node_id: NodeId) {
        {
            let mut lock = self.workers.lock().await;

            match lock.get_mut(&node_id) {
                Some(worker) => {
                    worker.update_heartbeat();
                }
                None => todo!(),
            }
        }
    }
}

impl From<NodeIdError> for ServerError {
    fn from(value: NodeIdError) -> Self {
        match value {
            NodeIdError::InvalidValue(msg) => ServerError::ValidationError(msg),
        }
    }
}

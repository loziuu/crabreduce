use super::worker::Worker;
use crate::server::RegisterRequest;
use crate::server::RegisterResponse;
use crate::server::crab_master_service_server::CrabMasterService;
use common::types::client_id::NodeId;
use common::types::client_id::NodeIdError;
use std::sync::{Arc, Mutex};
use tonic::Response;

pub struct MasterConfiguration {}

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

    pub fn register_worker(&self, node_id: NodeId) {
        println!("Registering {}.", node_id.id());

        let b = self.workers.clone();
        let mut lock = b.lock().unwrap();
        lock.push(Worker::new(node_id));

        println!("Registered succesfully");
    }
}

#[tonic::async_trait]
impl CrabMasterService for MasterNode {
    async fn register(
        &self,
        request: tonic::Request<RegisterRequest>,
    ) -> std::result::Result<tonic::Response<RegisterResponse>, tonic::Status> {
        let req = request.get_ref();

        let node_id = match &req.worker_id {
            Some(id) => NodeId::try_from(id.clone().id).map_err(ServerError::from)?, // Prettify that?
            None => return Err(tonic::Status::invalid_argument("Client_id is missing")),
        };

        self.register_worker(node_id);
        Ok(Response::new(RegisterResponse {}))
    }
}

enum ServerError {
    ValidationError(&'static str),
}

impl From<ServerError> for tonic::Status {
    fn from(value: ServerError) -> Self {
        match value {
            ServerError::ValidationError(msg) => tonic::Status::invalid_argument(msg),
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

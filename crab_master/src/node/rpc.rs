use common::types::node_id::NodeId;
use tonic::Response;
use tracing::{info, instrument};

use crate::server::{
    HeartbeatRequest, HeartbeatResponse, RegisterRequest, RegisterResponse,
    crab_master_service_server::CrabMasterService,
};

use super::master::{MasterNode, ServerError};

#[tonic::async_trait]
impl CrabMasterService for MasterNode {
    #[instrument(name = "register_worker", skip_all)]
    async fn register(
        &self,
        request: tonic::Request<RegisterRequest>,
    ) -> std::result::Result<tonic::Response<RegisterResponse>, tonic::Status> {
        info!("Registering worker: {:?}", request.get_ref());
        let req = request.get_ref();

        let node_id = match &req.worker_id {
            Some(id) => NodeId::try_from(id.clone().id).map_err(ServerError::from)?, // Prettify that?
            None => return Err(tonic::Status::invalid_argument("Client_id is missing")),
        };

        self.register_worker(node_id).await;

        info!("Worker registered.");
        Ok(Response::new(RegisterResponse {}))
    }

    // TODO: Define one field to contain worker_id
    #[instrument(name = "heartbeat_worker", skip_all)]
    async fn heartbeat(
        &self,
        request: tonic::Request<HeartbeatRequest>,
    ) -> std::result::Result<tonic::Response<HeartbeatResponse>, tonic::Status> {
        let body = request.get_ref();

        info!(
            "Received worker {} heartbeat",
            &body.id.as_ref().unwrap().id
        );

        Ok(Response::new(HeartbeatResponse {}))
    }
}

impl From<ServerError> for tonic::Status {
    fn from(value: ServerError) -> Self {
        match value {
            ServerError::ValidationError(msg) => tonic::Status::invalid_argument(msg),
        }
    }
}

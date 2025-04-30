use thiserror::Error;
use tonic::Response;

use crate::rpc::{
    HeartbeatRequest, HeartbeatResponse, RegisterRequest, RegisterResponse,
    crab_master_service_client::CrabMasterServiceClient,
};

#[derive(Clone)]
pub struct MasterClient {
    inner: CrabMasterServiceClient<tonic::transport::Channel>,
}

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("Cannot connect to server")]
    CannotConnect,
}

const MAX_RETRIES: usize = 10;

impl MasterClient {
    pub async fn connect<T>(uri: T) -> Result<MasterClient, RpcError>
    where
        T: Into<String>,
    {
        match CrabMasterServiceClient::connect(uri.into()).await {
            Ok(inner) => Ok(Self { inner }),
            Err(err) => panic!("{:?}", err),
        }
    }

    pub async fn register(&mut self, request: RegisterRequest) -> RegisterResponse {
        let resp = self
            .inner
            .register(request)
            .await
            .expect("Failed to register worker");
        resp.into_inner()
    }

    pub async fn heartbeat(
        &mut self,
        request: HeartbeatRequest,
    ) -> Result<Response<HeartbeatResponse>> {
        self.inner.heartbeat(request).await;
    }
}

use thiserror::Error;

use crate::rpc::{
    Id, RegisterRequest, RegisterResponse, crab_master_service_client::CrabMasterServiceClient,
};
use std::net::SocketAddr;

pub struct RpcClient {
    inner: CrabMasterServiceClient<tonic::transport::Channel>,
}

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("Cannot connect to server")]
    CannotConnect,
}

const MAX_RETRIES: usize = 10;

impl RpcClient {
    pub async fn connect(addr: SocketAddr) -> Result<RpcClient, RpcError> {
        //let address: String = format!("http://127.0.0.1:{}", addr.port());

        // Retry
        match CrabMasterServiceClient::connect("http://[::1]:50420").await {
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
}

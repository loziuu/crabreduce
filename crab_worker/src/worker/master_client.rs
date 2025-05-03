use thiserror::Error;

use crate::rpc::{
    HeartbeatRequest, RegisterRequest, crab_master_service_client::CrabMasterServiceClient,
};

type ClientResult<T> = Result<T, RpcError>;

#[derive(Clone)]
pub struct MasterClient {
    inner: CrabMasterServiceClient<tonic::transport::Channel>,
}

/// NOTE: Maybe rename it so it's not obvious that it's RPC underneath it?
#[derive(Error, Debug)]
pub enum RpcError {
    #[error("Cannot connect to server")]
    CannotConnect,
}

const MAX_RETRIES: usize = 10;

impl MasterClient {
    pub async fn connect<T>(uri: T) -> ClientResult<MasterClient>
    where
        T: Into<String>,
    {
        // TODO: Retries...
        match CrabMasterServiceClient::connect(uri.into()).await {
            Ok(inner) => Ok(Self { inner }),
            Err(err) => panic!("{:?}", err),
        }
    }

    pub async fn register(&mut self, request: RegisterRequest) -> ClientResult<()> {
        self.inner
            .register(request)
            .await
            .map_err(|_| RpcError::CannotConnect)?;
        Ok(())
    }

    pub async fn heartbeat(&mut self, request: HeartbeatRequest) -> ClientResult<()> {
        self.inner
            .heartbeat(request)
            .await
            .map_err(|_| RpcError::CannotConnect)?;
        Ok(())
    }
}

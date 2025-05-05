use mockall::automock;
use thiserror::Error;

use crate::rpc::{
    HeartbeatRequest, RegisterRequest, crab_master_service_client::CrabMasterServiceClient,
};

pub(crate) type ClientResult<T> = Result<T, RpcError>;

#[automock]
pub trait CrabMaster: Send {
    fn connect(&mut self) -> impl std::future::Future<Output = ClientResult<()>> + Send;

    fn register(
        &mut self,
        request: RegisterRequest,
    ) -> impl std::future::Future<Output = ClientResult<()>> + Send;

    fn heartbeat(
        &mut self,
        request: HeartbeatRequest,
    ) -> impl std::future::Future<Output = ClientResult<()>> + Send;
}

#[derive(Clone)]
pub struct RpcCrabMaster {
    inner: Option<CrabMasterServiceClient<tonic::transport::Channel>>,

    // TODO: Use something better, validateable?
    uri: String,
}

/// NOTE: Maybe rename it so it's not obvious that it's RPC underneath it?
#[derive(Error, Debug, Clone, Copy)]
pub enum RpcError {
    #[error("Cannot connect to the server")]
    CannotConnect,

    #[error("Not connnected to the server")]
    NotConnected,
}

const MAX_RETRIES: usize = 10;

impl RpcCrabMaster {
    pub fn new(uri: String) -> RpcCrabMaster {
        Self { uri, inner: None }
    }
}

impl CrabMaster for RpcCrabMaster {
    async fn connect(&mut self) -> ClientResult<()> {
        if self.inner.is_some() {
            return Ok(());
        }

        // TODO: Use max retries here.
        // TODO: Remove this uri.clone()?
        match CrabMasterServiceClient::connect(self.uri.clone()).await {
            Ok(inner) => {
                self.inner = Some(inner);
                Ok(())
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    async fn register(&mut self, request: RegisterRequest) -> ClientResult<()> {
        match self.inner.as_mut() {
            Some(rpc) => {
                rpc.register(request)
                    .await
                    .map_err(|_| RpcError::CannotConnect)?;
                Ok(())
            }
            None => Err(RpcError::NotConnected),
        }
    }

    async fn heartbeat(&mut self, request: HeartbeatRequest) -> ClientResult<()> {
        match self.inner.as_mut() {
            Some(rpc) => {
                rpc.heartbeat(request)
                    .await
                    .map_err(|_| RpcError::CannotConnect)?;
                Ok(())
            }
            None => Err(RpcError::NotConnected),
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO: This is to be tested with integration tests?
}

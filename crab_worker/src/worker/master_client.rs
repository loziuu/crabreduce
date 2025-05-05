use thiserror::Error;

use crate::rpc::{
    HeartbeatRequest, RegisterRequest, crab_master_service_client::CrabMasterServiceClient,
};

pub(crate) type ClientResult<T> = Result<T, RpcError>;

// NOTE: Rename it to maybe... just MASTER? Who cares for 'client' suffix?
pub trait MasterClient: Send {
    fn connect<T: Into<String>>(
        uri: T,
    ) -> impl std::future::Future<Output = ClientResult<impl MasterClient>>;

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
pub struct RpcMasterClient {
    inner: CrabMasterServiceClient<tonic::transport::Channel>,
}

/// NOTE: Maybe rename it so it's not obvious that it's RPC underneath it?
#[derive(Error, Debug)]
pub enum RpcError {
    #[error("Cannot connect to server")]
    CannotConnect,
}

const MAX_RETRIES: usize = 10;

impl MasterClient for RpcMasterClient {
    async fn connect<T: Into<String>>(uri: T) -> ClientResult<impl MasterClient> {
        // TODO: Use max retries here.
        match CrabMasterServiceClient::connect(uri.into()).await {
            Ok(inner) => Ok(Self { inner }),
            Err(err) => panic!("{:?}", err),
        }
    }

    async fn register(&mut self, request: RegisterRequest) -> ClientResult<()> {
        self.inner
            .register(request)
            .await
            .map_err(|_| RpcError::CannotConnect)?;
        Ok(())
    }

    async fn heartbeat(&mut self, request: HeartbeatRequest) -> ClientResult<()> {
        self.inner
            .heartbeat(request)
            .await
            .map_err(|_| RpcError::CannotConnect)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::rpc::{HeartbeatRequest, RegisterRequest};

    use super::{ClientResult, MasterClient};

    pub(crate) struct MockMasterClient {}

    impl MasterClient for MockMasterClient {
        async fn connect<T: Into<String>>(_uri: T) -> ClientResult<impl MasterClient> {
            Ok(Self {})
        }

        async fn register(&mut self, _request: RegisterRequest) -> ClientResult<()> {
            Ok(())
        }

        async fn heartbeat(&mut self, _request: HeartbeatRequest) -> ClientResult<()> {
            Ok(())
        }
    }
}

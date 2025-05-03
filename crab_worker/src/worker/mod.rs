pub(crate) mod heartbeat;
pub mod master_client;

pub mod uni_worker;

pub trait Worker: Send {
    fn register(&mut self) -> impl std::future::Future<Output = Result<(), WorkerError>> + Send;
    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), WorkerError>> + Send;

    fn heartbeat(&mut self) -> impl std::future::Future<Output = Result<(), WorkerError>> + Send;
}

#[derive(Debug)]
pub enum WorkerError {
    ConnectionError,
    NotRegistered,
}

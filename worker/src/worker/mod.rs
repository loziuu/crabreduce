pub mod daemon;
mod heartbeat;
pub mod master_client;
pub mod uni_worker;

pub trait Worker {
    async fn register(&mut self);
    async fn shutdown(&mut self);
}

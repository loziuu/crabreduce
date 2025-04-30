use std::sync::Arc;

use heartbeat::HeartbeatManager;
use tokio::sync::Mutex;

pub mod daemon;
mod heartbeat;
pub mod master_client;
pub mod uni_worker;

trait Worker {
    async fn register(&mut self);
    async fn shutdown(&mut self);
}

struct WorkerManager<T: Worker> {
    worker: Arc<Mutex<T>>,
    heartbeat: HeartbeatManager<T>,
}

impl<T: Worker> WorkerManager<T> {
    fn new(worker: T) -> Self {
        let worker = Arc::new(Mutex::new(worker));

        let manager = Self {
            worker: worker.clone(),
            heartbeat: HeartbeatManager::new(worker.clone()),
        };

        manager
    }

    async fn start(&mut self) {
        let mut w = self.worker.lock().await;
        w.register();
    }

    async fn shutdown(&self) {
        let mut worker = self.worker.lock().await;
        worker.shutdown().await;
    }
}

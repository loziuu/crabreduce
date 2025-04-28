use std::sync::Arc;

use tokio::sync::Mutex;

pub mod daemon;
pub mod heartbeat;
pub mod master_client;
pub mod uni_worker;

trait Worker {
    async fn shutdown(&mut self);
}

struct WorkerManager<T: Worker> {
    worker: Arc<Mutex<T>>,
    heartbeat_manager: HeartbeatManager,
}

impl<T: Worker> WorkerManager<T> {
    fn new(worker: T) -> Self {
        let manager = Self {
            worker: Arc::new(Mutex::new(worker)),
        }

        tokio::spawn(|| {
        });
    }

    async fn shutdown(&self) {
        let mut worker = self.worker.lock().await;
        worker.shutdown().await;
    }
}

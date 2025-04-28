use std::{sync::Arc, time::Instant};

use tokio::sync::Mutex;

use super::Worker;

const HEARTBEAT_INTERVAL: u64 = 5;

struct HeartbeatManager<T: Worker> {
    worker: Arc<Mutex<T>>,
    last_time: Instant,
}

impl<T: Worker> HeartbeatManager<T> {
    pub fn new(worker: Arc<Mutex<T>>) -> Self {
        Self {
            worker,
            last_time: Instant::now(),
        }
    }

    pub async fn send_heartbeat(&mut self) {
        let w = self.worker.lock().await;

        self.last_time = Instant::now();
    }
}

use std::{sync::Arc, time::Duration};

use common::types::mutex::AsyncMutex;
use tokio::time::sleep;
use tracing::{info, warn};

use super::Worker;

// TODO: Move it to config struct?
const HEARTBEAT_INTERVAL: u64 = 5;
const SHUTDOWN_THRESHOLD: usize = 10;

pub struct HeartbeatManager<W: Worker + Send + 'static> {
    worker: Arc<AsyncMutex<W>>,
}

impl<W: Worker + Send + 'static> HeartbeatManager<W> {
    pub fn new(worker: Arc<AsyncMutex<W>>) -> Self {
        Self { worker }
    }

    /// Start firing heartbeats and publish an shutdown signal when it fails N times?
    // TODO: Abstract away sender so it's not depending on tokio API that much...
    pub fn start(self) {
        let mut retries = 0;
        let w = self.worker.clone();

        tokio::spawn(async move {
            loop {
                {
                    let mut lock = w.lock().await;

                    info!("Sending heartbeat...");
                    if (lock.heartbeat().await).is_err() {
                        info!("Heartbeat failed.");
                        retries += 1;
                    } else {
                        retries = 0;
                    }
                }

                if retries == SHUTDOWN_THRESHOLD {
                    warn!("Heartbeat threshold exceeded. Shutting down...");
                    match w.lock().await.shutdown().await {
                        Ok(_) => return,
                        Err(_) => panic!("Failed to gracefully shutdown worker"),
                    }
                }

                retries += 1;
                sleep(Duration::from_secs(HEARTBEAT_INTERVAL)).await;
            }
        });
    }
}

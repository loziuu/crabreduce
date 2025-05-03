use std::{sync::Arc, time::Duration};

use common::types::mutex::AsyncMutex;
use tokio::time::sleep;
use tracing::{info, warn};
use worker::{Worker, heartbeat::HeartbeatManager};

pub mod worker;

pub mod rpc {
    tonic::include_proto!("crabmaster");
}

pub async fn start_worker<W>(mut w: W)
where
    W: Worker + Send + 'static,
{
    info!("Starting CrabReduce worker...");
    if (w.register().await).is_err() {
        warn!("Failed to register ");
        return;
    }
    info!("Worker started successfully");

    let arc = Arc::new(AsyncMutex::new(w));
    HeartbeatManager::new(arc.clone()).start();

    info!("Awaiting tasks...");
    loop {
        sleep(Duration::from_secs(10)).await;
    }
}

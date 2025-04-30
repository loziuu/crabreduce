use crate::rpc::Id;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::info;

use common::types::node_id::NodeId;
use tokio::{
    sync::{Mutex, mpsc::Sender},
    time::sleep,
};

use crate::rpc::HeartbeatRequest;

use super::{Worker, master_client::MasterClient, uni_worker::WorkerSignal};

// TODO: Move it to config struct?
const HEARTBEAT_INTERVAL: u64 = 5;
const SHUTDOWN_THRESHOLD: usize = 10;

pub struct HeartbeatManager {
    id: NodeId,
    client: MasterClient,
    last_time: Instant,
}

impl HeartbeatManager {
    pub fn new(id: NodeId, client: MasterClient) -> Self {
        Self {
            id,
            client,
            last_time: Instant::now(),
        }
    }

    pub async fn send_heartbeat(&mut self) {
        self.last_time = Instant::now();
    }

    /// Start firing heartbeats and publish an shutdown signal when it fails N times?
    // TODO: Abstract away sender so it's not depending on tokio API that much...
    pub fn start(mut self, sender: Sender<WorkerSignal>) {
        let mut retries = 0;
        tokio::spawn(async move {
            loop {
                if retries >= 10 {
                    // TODO: UNWRAP
                    sender.send(WorkerSignal::Shutdown).await.unwrap();
                    return;
                }

                let req = HeartbeatRequest {
                    id: Some(Id {
                        id: self.id.id().to_string(),
                    }),
                    state: 0,
                };

                match self.client.heartbeat(req).await {
                    Ok(_) => retries = 0,
                    Err(e) => {
                        if retries == SHUTDOWN_THRESHOLD {
                            // TODO: UNWRAP
                            sender.send(WorkerSignal::Shutdown).await.unwrap();
                            return;
                        }

                        info!("Failed heartbeat: {:?}. Will retry.", e);
                        retries += 1;
                    }
                }
                sleep(Duration::from_secs(HEARTBEAT_INTERVAL)).await;
            }
        });
    }
}

pub struct WorkerAwareContextManager<W: Worker> {
    worker: Arc<Mutex<W>>,
    last_time: Instant,
}

impl<W: Worker> WorkerAwareContextManager<W> {
    pub fn new(worker: Arc<Mutex<W>>) -> Self {
        Self {
            worker,
            last_time: Instant::now(),
        }
    }

    pub fn start(mut self) {
        let mut retries = 0;
        tokio::spawn(async move {
            loop {
                let req = HeartbeatRequest {
                    id: Some(Id {
                        id: self.worker.id().to_string(),
                    }),
                    state: 0,
                };

                match self.client.heartbeat(req).await {
                    Ok(_) => retries = 0,
                    Err(e) => {
                        if retries == SHUTDOWN_THRESHOLD {
                            // TODO: UNWRAP
                            self.worker.lock().await.shutdown().await;
                            return;
                        }

                        info!("Failed heartbeat: {:?}. Will retry.", e);
                        retries += 1;
                    }
                }
                sleep(Duration::from_secs(HEARTBEAT_INTERVAL)).await;
            }
        });
    }
}
